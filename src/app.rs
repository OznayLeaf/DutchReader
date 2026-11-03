//! Wiring between the Slint UI and the plain-Rust modules.
//!
//! This is the only module that touches both Slint and the business logic. It
//! owns the navigation stack, the open reader session, the 30-second page rule,
//! the lookup worker, narration, and persistence.

use crate::dictionary::{Dictionary, Sort, format_date};
use crate::lookup::{self, CacheEntry, LookupCache, LookupOutcome, MyMemoryProvider};
use crate::platform::lifecycle::Lifecycle;
use crate::platform::share;
use crate::platform::timezone;
use crate::platform::tts::{TtsEvent, TtsHandle};
use crate::reader::layout::{
    PageLayout, PageSpan, READER_FONT_SIZE, REFERENCE_LARGE_HEIGHT, REFERENCE_LARGE_WIDTH, RectBox,
    page_for_offset, paginate, paginate_story, story_text,
};
use crate::reader::page_timer::PageTimer;
use crate::storage::Storage;
use crate::storage::progress::Progress;
use crate::storage::settings::{DEFAULT_TTS_SPEED, MeaningsSource, Settings, Theme};
use crate::stories::index::StoryIndex;
use crate::stories::model::{Level, Story, normalize_word};
use crate::stories::starter::StarterWords;
use crate::writing::draft::Draft;
use crate::writing::share_text::share_text;
use crate::writing::stats::{self, Stats};
use crate::writing::store::WritingStore;
use crate::writing::{display_title, format_duration, is_empty_text, preview, word_count};
use crate::{
    AppWindow, DictionaryRow, LevelCardData, MonthRow, ReaderMark, ReaderWord, StoryRow, WritingRow,
};
use slint::{ComponentHandle, ModelRc, VecModel};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Open reader state. Everything here is replaced when a story opens or closes.
struct ReaderSession {
    story: Story,
    /// Joined authored text; page ranges index into this string.
    text: String,
    /// Computed pages for the current width/height.
    pages: Vec<PageSpan>,
    page: usize,
    /// Text offset the reader is anchored to. Repagination maps this offset, so
    /// several resizes in a row can never drift backwards through the story.
    anchor: usize,
    layout: PageLayout,
    /// Dimensions the current pagination was measured for. A resize repaginates.
    width: f32,
    height: f32,
    timer: PageTimer,
    overlay_visible: bool,
    /// When the story opened. Taps in the first moments are ignored so the tap
    /// that opened the story cannot also toggle the overlay or turn a page.
    opened_at_ms: u64,
    /// When the lookup sheet was last closed. The release that closes it can
    /// reach the reader underneath, so taps are ignored briefly.
    popup_closed_at_ms: u64,
    popup: Option<PopupState>,
    tts_generation: u64,
    tts_active: bool,
    tts_sentence: i32,
}

struct PopupState {
    /// Matches the in-flight lookup job; stale results are ignored.
    job: u64,
    word: String,
    article: String,
    meaning: String,
    base: String,
    source: String,
    note: String,
    loading: bool,
}

struct LookupJob {
    id: u64,
    token: String,
    occurrence: u32,
    started: Instant,
    rx: Receiver<(LookupOutcome, Option<CacheEntry>)>,
}

/// Where the editor should start once a Continue / Start new prompt is answered.
#[derive(Debug, Clone)]
enum EditorIntent {
    New,
    Edit(String),
}

/// An open editor session. The draft itself is the persisted data; the timing
/// fields are UI-only and are never shown as a timer.
struct EditorSession {
    draft: Draft,
    last_tick: Instant,
    pending_ms: u64,
    last_save: Instant,
    dirty: bool,
}

struct AppState {
    index: StoryIndex,
    storage: Storage,
    settings: Settings,
    progress: Progress,
    dictionary: Dictionary,
    lookup_cache: LookupCache,
    /// Starter lemma -> first story (in reading order) that uses it.
    starter_origins: HashMap<String, (String, Level)>,
    /// Level currently shown by the Level screen.
    current_level: Level,
    /// Previous screens, most recent last. Back pops this stack.
    nav: Vec<String>,
    /// Reader dimensions the Home/Level page labels were computed with. When the
    /// window changes size the labels are refreshed on the next tick.
    home_dims: (f32, f32),
    reader: Option<ReaderSession>,
    dictionary_query: String,
    dictionary_sort: Sort,
    lookup: Option<LookupJob>,
    lookup_seq: u64,
    /// Bumped for every toast so an old hide timer cannot hide a newer toast.
    toast_seq: Rc<Cell<u64>>,
    /// Last draft loaded from disk. While the editor is open, `editor` is the
    /// source of truth; autosave keeps this copy in sync.
    draft: Option<Draft>,
    editor: Option<EditorSession>,
    pending_editor: Option<EditorIntent>,
    writing: WritingStore,
    writing_selected: Option<String>,
    writing_detail_visible: bool,
    editor_toast_seq: Rc<Cell<u64>>,
    writing_toast_seq: Rc<Cell<u64>>,
    tts: TtsHandle,
    tts_events: Receiver<TtsEvent>,
    lifecycle: Lifecycle,
    lifecycle_epoch: u64,
}

type SharedState = Rc<RefCell<AppState>>;

/// Start the app. `android_data_dir` is the app-private directory supplied by
/// `android_main`; desktop preview passes `None`.
pub fn run(android_data_dir: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    run_inner(android_data_dir, Lifecycle::new())
}

/// Android entry point variant that receives the lifecycle handle updated by
/// the `android-activity` event listener.
pub fn run_with_lifecycle(
    android_data_dir: Option<PathBuf>,
    lifecycle: Lifecycle,
) -> Result<(), Box<dyn std::error::Error>> {
    run_inner(android_data_dir, lifecycle)
}

fn run_inner(
    android_data_dir: Option<PathBuf>,
    lifecycle: Lifecycle,
) -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(debug_assertions) {
        crate::stories::validate::startup_check();
    }

    let index = StoryIndex::load_embedded()?;
    let starter_origins = starter_origins(&index);
    let storage = Storage::new(crate::storage::resolve_data_dir(android_data_dir));
    let settings = storage.load_settings();
    let progress = storage.load_progress();
    let dictionary = storage.load_dictionary();
    let lookup_cache = storage.load_lookup_cache();
    let writing = WritingStore::load(&storage.writings_path());
    let draft = Draft::load(&storage.draft_path());
    let (tts, tts_events) = TtsHandle::start(settings.tts_speed);
    let lifecycle_epoch = lifecycle.epoch();

    let ui = AppWindow::new()?;
    let state: SharedState = Rc::new(RefCell::new(AppState {
        index,
        storage,
        settings,
        progress,
        dictionary,
        lookup_cache,
        starter_origins,
        current_level: Level::A1,
        nav: Vec::new(),
        home_dims: (0.0, 0.0),
        reader: None,
        dictionary_query: String::new(),
        dictionary_sort: Sort::Alphabetical,
        lookup: None,
        lookup_seq: 0,
        toast_seq: Rc::new(Cell::new(0)),
        draft,
        editor: None,
        pending_editor: None,
        writing,
        writing_selected: None,
        writing_detail_visible: false,
        editor_toast_seq: Rc::new(Cell::new(0)),
        writing_toast_seq: Rc::new(Cell::new(0)),
        tts,
        tts_events,
        lifecycle,
        lifecycle_epoch,
    }));

    state.borrow_mut().init(&ui);

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_go_back(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().go_back(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_open_level(move |code| {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().open_level(&ui, &code);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_open_story(move |id, _title| {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            // Open on the next event-loop turn. Creating the reader inside the
            // click callback makes Slint replay the same press/release to the
            // brand-new reader TouchArea, which would toggle the overlay.
            let id = id.to_string();
            let state = state.clone();
            slint::Timer::single_shot(Duration::ZERO, move || {
                state.borrow_mut().open_story(&ui, &id);
            });
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_resume_story(move || {
            let id = state.borrow().progress.last_story.clone();
            let (Some(id), Some(ui)) = (id, ui_weak.upgrade()) else {
                return;
            };
            let state = state.clone();
            slint::Timer::single_shot(Duration::ZERO, move || {
                state.borrow_mut().open_story(&ui, &id);
            });
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_open_dictionary(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().open_dictionary(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_open_settings(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().open_settings(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_open_write(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().open_editor_new(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_open_writing(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().open_writing(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_editor_edited(move |text| {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().editor_edited(&ui, &text);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_editor_title_edited(move |text| {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().editor_title_edited(&ui, &text);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_editor_share(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().editor_share(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_editor_continue(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().continue_draft(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_editor_start_new(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().start_new_draft(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_writing_open_story(move |id| {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().open_writing_detail(&ui, &id);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_writing_close_detail(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().close_writing_detail(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_writing_share_detail(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().share_writing_detail(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_writing_copy_detail(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state
                    .borrow()
                    .show_writing_toast(&ui, "Copied to clipboard.".into());
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_writing_edit_detail(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().edit_writing_detail(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_writing_delete_request(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_writing_delete_visible(true);
                state.borrow_mut().writing_detail_visible = true;
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        ui.on_writing_delete_cancel(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_writing_delete_visible(false);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_writing_delete_confirm(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().delete_writing(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_writing_open_write(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().open_editor_new(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_toggle_theme(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let mut state = state.borrow_mut();
                let theme = if ui.get_dark_mode() {
                    Theme::Light
                } else {
                    Theme::Dark
                };
                state.set_theme(&ui, theme);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_reader_tap(move |x, y| {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().reader_tap(&ui, x, y, &state);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_reader_next(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().request_turn(&ui, 1, &state);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_reader_previous(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().request_turn(&ui, -1, &state);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_reader_toggle_tts(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().toggle_tts(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_reader_close_popup(move || {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().close_popup(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_dictionary_search(move |text| {
            if let Some(ui) = ui_weak.upgrade() {
                let mut state = state.borrow_mut();
                state.dictionary_query = text.to_string();
                state.refresh_dictionary(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_dictionary_sort_changed(move |code| {
            if let Some(ui) = ui_weak.upgrade() {
                let mut state = state.borrow_mut();
                state.dictionary_sort = Sort::from_code(&code);
                ui.set_dictionary_sort(state.dictionary_sort.code().into());
                state.refresh_dictionary(&ui);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_settings_theme(move |code| {
            if let Some(ui) = ui_weak.upgrade() {
                let mut state = state.borrow_mut();
                let theme = theme_from_code(&code);
                state.set_theme(&ui, theme);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_settings_speed(move |value| {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().set_tts_speed(&ui, value, false);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_settings_speed_released(move |value| {
            if let Some(ui) = ui_weak.upgrade() {
                state.borrow_mut().set_tts_speed(&ui, value, true);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        ui.on_settings_meanings(move |code| {
            if let Some(ui) = ui_weak.upgrade() {
                let mut state = state.borrow_mut();
                state.settings.meanings_source = meanings_from_code(&code);
                state.save_settings();
                ui.set_settings_meanings_source(
                    meanings_code(state.settings.meanings_source).into(),
                );
            }
        });
    }

    // One timer drives the 30-second rule, lifecycle resets, lookup results,
    // TTS events, and resize relayouts.
    let tick = slint::Timer::default();
    {
        let ui_weak = ui.as_weak();
        let state = state.clone();
        tick.start(
            slint::TimerMode::Repeated,
            Duration::from_millis(200),
            move || {
                if let Some(ui) = ui_weak.upgrade() {
                    state.borrow_mut().tick(&ui);
                }
            },
        );
    }

    ui.run()?;
    drop(tick);
    Ok(())
}

impl AppState {
    fn init(&mut self, ui: &AppWindow) {
        ui.set_theme_setting(theme_code(self.settings.theme).into());
        ui.set_settings_tts_speed(self.settings.tts_speed);
        ui.set_settings_tts_speed_label(format!("{:.1}×", self.settings.tts_speed).into());
        ui.set_settings_meanings_source(meanings_code(self.settings.meanings_source).into());
        ui.set_dictionary_sort(self.dictionary_sort.code().into());
        ui.set_current_level(self.current_level.as_str().into());
        ui.set_current_level_label(level_label(self.current_level).into());
        self.refresh_home(ui);
        self.refresh_dictionary(ui);
        self.refresh_writing(ui);
    }

    /// Recompute everything that depends on stored progress.
    fn refresh_home(&mut self, ui: &AppWindow) {
        let (width, height) = reader_dims(ui);
        self.home_dims = (width, height);
        ui.set_levels(level_cards(&self.index, &self.progress));
        ui.set_stories(story_rows(
            &self.index,
            &self.progress,
            self.current_level,
            width,
            height,
        ));
        apply_resume(ui, &self.index, &self.progress, width, height);
    }

    fn refresh_dictionary(&self, ui: &AppWindow) {
        let total = self.dictionary.len();
        let entries = self
            .dictionary
            .search(&self.dictionary_query, self.dictionary_sort);
        let rows: Vec<DictionaryRow> = entries
            .iter()
            .map(|entry| {
                let story_title = self
                    .index
                    .get(&entry.story_id)
                    .map(|loaded| loaded.story.title.clone())
                    .unwrap_or_else(|| entry.story_id.clone());
                DictionaryRow {
                    word: entry.display_word().into(),
                    meaning: entry.meaning.clone().into(),
                    meta: format!(
                        "{} · {} · {}",
                        entry.level,
                        story_title,
                        format_date(entry.added_at, timezone::offset_seconds(entry.added_at))
                    )
                    .into(),
                    pos: entry.pos.to_uppercase().into(),
                }
            })
            .collect();
        ui.set_dictionary_total(total as i32);
        ui.set_dictionary_count_text(count_words(total).into());
        ui.set_dictionary_entries(Rc::new(VecModel::from(rows)).into());
    }

    fn navigate(&mut self, ui: &AppWindow, target: &str, forward: bool) {
        let current = ui.get_screen().to_string();
        if current == target {
            return;
        }
        if forward {
            self.nav.push(current.clone());
        }
        ui.set_screen(target.into());
    }

    fn go_back(&mut self, ui: &AppWindow) {
        let current = ui.get_screen().to_string();
        match current.as_str() {
            "reader" => self.close_reader(ui),
            "settings" => self.save_settings(),
            "editor" => {
                // A visible prompt means no session has started yet; leave
                // without touching the saved draft.
                if !ui.get_editor_prompt_visible() {
                    self.save_editor_draft(ui);
                }
                self.close_editor(ui);
            }
            "writing" => {
                // Back first dismisses the delete confirmation, then the story
                // detail, and only then leaves My Writing.
                if ui.get_writing_delete_visible() {
                    ui.set_writing_delete_visible(false);
                    return;
                }
                if self.writing_detail_visible {
                    self.close_writing_detail(ui);
                    return;
                }
            }
            _ => {}
        }
        let target = self.nav.pop().unwrap_or_else(|| "home".to_string());
        let leaving_reader = current == "reader";
        ui.set_screen(target.into());
        if leaving_reader {
            self.refresh_home(ui);
        }
    }

    fn open_level(&mut self, ui: &AppWindow, code: &str) {
        let level = Level::from_code(code).unwrap_or(Level::A1);
        self.current_level = level;
        ui.set_current_level(level.as_str().into());
        ui.set_current_level_label(level_label(level).into());
        self.refresh_home(ui);
        self.navigate(ui, "level", true);
    }

    fn open_story(&mut self, ui: &AppWindow, id: &str) {
        let Some(loaded) = self.index.get(id) else {
            eprintln!("[navigation] unknown story id `{id}`");
            return;
        };
        let story = loaded.story.clone();
        let text = story_text(&story);
        let (width, height) = reader_dims(ui);
        let pages = paginate(&story, &text, width, height, READER_FONT_SIZE);
        let offset = self.progress.last_offset_of(&story.id);
        let page = page_for_offset(&pages, offset);
        let layout = pages[page].layout.clone();

        self.stop_tts(ui);
        self.lookup = None;
        self.reader = Some(ReaderSession {
            story,
            text,
            pages,
            page,
            anchor: offset,
            layout,
            width,
            height,
            timer: PageTimer::new(now_ms()),
            overlay_visible: true,
            opened_at_ms: now_ms(),
            popup_closed_at_ms: 0,
            popup: None,
            tts_generation: 0,
            tts_active: false,
            tts_sentence: -1,
        });
        self.current_level = self.reader.as_ref().expect("reader").story.level;
        ui.set_current_level(self.current_level.as_str().into());
        ui.set_current_level_label(level_label(self.current_level).into());

        self.navigate(ui, "reader", true);
        self.apply_page(ui, page, true);
    }

    fn open_dictionary(&mut self, ui: &AppWindow) {
        self.refresh_dictionary(ui);
        self.navigate(ui, "dictionary", true);
    }

    fn open_settings(&mut self, ui: &AppWindow) {
        ui.set_theme_setting(theme_code(self.settings.theme).into());
        ui.set_settings_tts_speed(self.settings.tts_speed);
        ui.set_settings_tts_speed_label(format!("{:.1}×", self.settings.tts_speed).into());
        ui.set_settings_meanings_source(meanings_code(self.settings.meanings_source).into());
        self.navigate(ui, "settings", true);
    }

    // ---- writing editor ---------------------------------------------------

    fn open_editor_new(&mut self, ui: &AppWindow) {
        self.open_editor(ui, EditorIntent::New);
    }

    fn open_editor_edit(&mut self, ui: &AppWindow, id: &str) {
        if self.writing.get(id).is_none() {
            return;
        }
        self.open_editor(ui, EditorIntent::Edit(id.to_string()));
    }

    fn open_editor(&mut self, ui: &AppWindow, intent: EditorIntent) {
        if let Some(draft) = self.draft.clone().filter(|draft| !draft.is_empty()) {
            let body = match &intent {
                EditorIntent::Edit(_) => {
                    "You have an unfinished draft. Continue it, or start new and replace it with the story you chose."
                }
                EditorIntent::New => {
                    "You have an unfinished story. Continue where you left off, or start a new one."
                }
            };
            self.pending_editor = Some(intent);
            self.editor = None;
            ui.set_editor_prompt_title("Continue your draft?".into());
            ui.set_editor_prompt_body(body.into());
            ui.set_editor_prompt_visible(true);
            ui.set_editor_title(draft.title.clone().into());
            ui.set_editor_text(draft.text.clone().into());
            // An unfinished draft keeps the date it was started, not today.
            let placeholder_at = if draft.created_at == 0 {
                now_unix()
            } else {
                draft.created_at
            };
            ui.set_editor_title_placeholder(
                display_title("", placeholder_at, timezone::offset_seconds(placeholder_at)).into(),
            );
            ui.set_editor_word_count_text(count_words(word_count(&draft.text)).into());
            ui.set_editor_can_share(false);
            ui.set_editor_toast_visible(false);
            self.navigate(ui, "editor", true);
            return;
        }
        self.start_editor(ui, intent);
    }

    fn start_editor(&mut self, ui: &AppWindow, intent: EditorIntent) {
        let now = now_unix();
        let draft = match intent {
            EditorIntent::New => Draft {
                text: String::new(),
                title: String::new(),
                created_at: now,
                updated_at: now,
                elapsed_seconds: 0,
                editing_id: None,
            },
            EditorIntent::Edit(id) => {
                let Some(story) = self.writing.get(&id).cloned() else {
                    return;
                };
                Draft {
                    text: story.text,
                    title: story.title,
                    created_at: story.created_at,
                    updated_at: now,
                    elapsed_seconds: story.elapsed_seconds,
                    editing_id: Some(story.id),
                }
            }
        };
        self.start_editor_with_draft(ui, draft);
    }

    fn start_editor_with_draft(&mut self, ui: &AppWindow, mut draft: Draft) {
        let now = now_unix();
        if draft.created_at == 0 {
            draft.created_at = now;
        }
        draft.updated_at = now;
        ui.set_editor_prompt_visible(false);
        ui.set_editor_title_placeholder(
            display_title(
                "",
                draft.created_at,
                timezone::offset_seconds(draft.created_at),
            )
            .into(),
        );
        ui.set_editor_title(draft.title.clone().into());
        ui.set_editor_text(draft.text.clone().into());
        ui.set_editor_word_count_text(count_words(word_count(&draft.text)).into());
        ui.set_editor_can_share(!is_empty_text(&draft.text));
        ui.set_editor_toast_visible(false);
        self.pending_editor = None;
        self.editor = Some(EditorSession {
            draft,
            last_tick: Instant::now(),
            pending_ms: 0,
            last_save: Instant::now(),
            dirty: false,
        });
        self.navigate(ui, "editor", true);
    }

    fn continue_draft(&mut self, ui: &AppWindow) {
        let draft = self.draft.clone().filter(|draft| !draft.is_empty());
        self.pending_editor = None;
        ui.set_editor_prompt_visible(false);
        match draft {
            Some(draft) => self.start_editor_with_draft(ui, draft),
            None => self.start_editor(ui, EditorIntent::New),
        }
    }

    fn start_new_draft(&mut self, ui: &AppWindow) {
        if let Err(error) = Draft::clear(&self.storage.draft_path()) {
            eprintln!("[writing] could not clear the old draft: {error}");
        }
        self.draft = None;
        let intent = self.pending_editor.take().unwrap_or(EditorIntent::New);
        ui.set_editor_prompt_visible(false);
        self.start_editor(ui, intent);
    }

    fn editor_edited(&mut self, ui: &AppWindow, text: &str) {
        let Some(editor) = self.editor.as_mut() else {
            return;
        };
        editor.draft.text = text.to_string();
        editor.dirty = true;
        ui.set_editor_word_count_text(count_words(word_count(text)).into());
        ui.set_editor_can_share(!is_empty_text(text));
    }

    fn editor_title_edited(&mut self, _ui: &AppWindow, text: &str) {
        let Some(editor) = self.editor.as_mut() else {
            return;
        };
        editor.draft.title = text.to_string();
        editor.dirty = true;
    }

    fn editor_share(&mut self, ui: &AppWindow) {
        self.flush_editor_time();
        let Some(editor) = self.editor.as_ref() else {
            return;
        };
        if is_empty_text(&editor.draft.text) {
            self.show_editor_toast(ui, "Write something before sharing.".into());
            return;
        }
        let body = share_text(&editor.draft.text);

        match share::share_text(&body) {
            Ok(()) => {
                let now = now_unix();
                let draft = editor.draft.clone();
                let story = self.writing.upsert(&draft, now);
                if let Err(error) = self.writing.save(&self.storage.writings_path()) {
                    eprintln!("[writing] could not save the story: {error}");
                }
                if let Err(error) = Draft::clear(&self.storage.draft_path()) {
                    eprintln!("[writing] could not clear the shared draft: {error}");
                }
                self.draft = None;
                self.editor = None;
                self.pending_editor = None;
                ui.set_editor_prompt_visible(false);
                ui.set_editor_toast_visible(false);
                self.refresh_writing(ui);
                self.open_writing_detail(ui, &story.id);

                // The editor is not part of the history after a successful
                // share; returning from My Writing goes to the screen before it.
                if self.nav.last().is_some_and(|screen| screen == "editor") {
                    self.nav.pop();
                }
                ui.set_screen("writing".into());
                self.show_writing_toast(ui, "Story saved to My Writing.".into());
            }
            Err(message) => {
                self.save_editor_draft(ui);
                self.show_editor_toast(ui, message);
            }
        }
    }

    /// Add whole foreground seconds to the current editor session. Sub-second
    /// time stays in `pending_ms` until the next tick.
    fn flush_editor_time(&mut self) {
        let now = Instant::now();
        let foreground = self.lifecycle.is_foreground();
        if let Some(editor) = self.editor.as_mut() {
            if foreground {
                let delta = now.saturating_duration_since(editor.last_tick);
                editor.pending_ms = editor.pending_ms.saturating_add(delta.as_millis() as u64);
                let seconds = editor.pending_ms / 1_000;
                if seconds > 0 {
                    editor.draft.elapsed_seconds =
                        editor.draft.elapsed_seconds.saturating_add(seconds);
                    editor.pending_ms %= 1_000;
                    editor.dirty = true;
                }
            } else {
                editor.pending_ms = 0;
            }
            editor.last_tick = now;
        }
    }

    /// Persist the editor text, or remove the draft when the text is empty.
    fn save_editor_draft(&mut self, ui: &AppWindow) {
        self.flush_editor_time();
        let path = self.storage.draft_path();
        let now = now_unix();
        let (is_empty, save_result, draft_copy) = {
            let Some(editor) = self.editor.as_mut() else {
                return;
            };
            editor.draft.updated_at = now;
            let is_empty = editor.draft.is_empty();
            let draft_copy = editor.draft.clone();
            let save_result = if is_empty {
                Draft::clear(&path)
            } else {
                editor.draft.save(&path)
            };
            if save_result.is_ok() {
                editor.dirty = false;
                editor.last_save = Instant::now();
            }
            (is_empty, save_result, draft_copy)
        };

        match save_result {
            Ok(()) => {
                self.draft = if is_empty { None } else { Some(draft_copy) };
            }
            Err(error) => {
                eprintln!("[writing] could not save the draft: {error}");
                self.show_editor_toast(ui, "Could not save the draft.".into());
            }
        }
    }

    fn tick_editor(&mut self, ui: &AppWindow) {
        self.flush_editor_time();
        let should_save = self.editor.as_ref().is_some_and(|editor| {
            editor.dirty && editor.last_save.elapsed() >= Duration::from_secs(3)
        });
        if should_save {
            self.save_editor_draft(ui);
        }
    }

    fn close_editor(&mut self, ui: &AppWindow) {
        self.editor = None;
        self.pending_editor = None;
        ui.set_editor_prompt_visible(false);
        ui.set_editor_toast_visible(false);
    }

    // ---- my writing -------------------------------------------------------

    fn open_writing(&mut self, ui: &AppWindow) {
        self.refresh_writing(ui);
        self.writing_detail_visible = false;
        self.writing_selected = None;
        ui.set_writing_detail_visible(false);
        ui.set_writing_delete_visible(false);
        ui.set_writing_toast_visible(false);
        self.navigate(ui, "writing", true);
    }

    fn refresh_writing(&mut self, ui: &AppWindow) {
        let now = now_unix();
        let stats = Stats::compute(&self.writing.stories, now, |unix| {
            timezone::offset_seconds(unix)
        });
        let rows: Vec<WritingRow> = self
            .writing
            .newest_first()
            .into_iter()
            .map(|story| {
                let offset = timezone::offset_seconds(story.created_at);
                WritingRow {
                    id: story.id.clone().into(),
                    title: display_title(&story.title, story.created_at, offset).into(),
                    date: stats::format_date_time(story.created_at, offset).into(),
                    preview: preview(&story.text, 120).into(),
                    meta: format!(
                        "{} · {}",
                        count_words(story.word_count),
                        format_duration(story.elapsed_seconds)
                    )
                    .into(),
                }
            })
            .collect();
        let months: Vec<MonthRow> = stats
            .months
            .iter()
            .map(|month| MonthRow {
                label: month.label.clone().into(),
                stories_text: count_stories(month.stories).into(),
                words_text: count_words(month.words).into(),
                ratio: month.ratio,
            })
            .collect();

        ui.set_writing_stories(Rc::new(VecModel::from(rows)).into());
        ui.set_writing_months(Rc::new(VecModel::from(months)).into());
        ui.set_writing_total_stories_text(stats.total_stories.to_string().into());
        ui.set_writing_total_words_text(count_words(stats.total_words).into());
        ui.set_writing_streak_text(count_days(stats.streak_days).into());
    }

    fn open_writing_detail(&mut self, ui: &AppWindow, id: &str) {
        let Some(story) = self.writing.get(id).cloned() else {
            return;
        };
        let offset = timezone::offset_seconds(story.created_at);
        ui.set_writing_detail_title(display_title(&story.title, story.created_at, offset).into());
        ui.set_writing_detail_date(stats::format_date_time(story.created_at, offset).into());
        ui.set_writing_detail_meta(
            format!(
                "{} · {}",
                count_words(story.word_count),
                format_duration(story.elapsed_seconds)
            )
            .into(),
        );
        ui.set_writing_detail_text(story.text.clone().into());
        ui.set_writing_detail_visible(true);
        ui.set_writing_delete_visible(false);
        self.writing_selected = Some(story.id);
        self.writing_detail_visible = true;
    }

    fn close_writing_detail(&mut self, ui: &AppWindow) {
        self.writing_selected = None;
        self.writing_detail_visible = false;
        ui.set_writing_detail_visible(false);
        ui.set_writing_delete_visible(false);
    }

    fn share_writing_detail(&mut self, ui: &AppWindow) {
        let Some(id) = self.writing_selected.clone() else {
            return;
        };
        let Some(story) = self.writing.get(&id) else {
            return;
        };
        let body = share_text(&story.text);
        match share::share_text(&body) {
            Ok(()) => self.show_writing_toast(ui, "Opening the share sheet…".into()),
            Err(message) => self.show_writing_toast(ui, message),
        }
    }

    fn edit_writing_detail(&mut self, ui: &AppWindow) {
        let Some(id) = self.writing_selected.clone() else {
            return;
        };
        self.open_editor_edit(ui, &id);
    }

    fn delete_writing(&mut self, ui: &AppWindow) {
        let Some(id) = self.writing_selected.take() else {
            return;
        };
        if !self.writing.remove(&id) {
            return;
        }
        if let Err(error) = self.writing.save(&self.storage.writings_path()) {
            eprintln!("[writing] could not save the story list: {error}");
        }
        self.writing_detail_visible = false;
        ui.set_writing_detail_visible(false);
        ui.set_writing_delete_visible(false);
        self.refresh_writing(ui);
        self.show_writing_toast(ui, "Story deleted.".into());
    }

    fn show_editor_toast(&self, ui: &AppWindow, text: String) {
        let seq = self.editor_toast_seq.get() + 1;
        self.editor_toast_seq.set(seq);
        ui.set_editor_toast_text(text.into());
        ui.set_editor_toast_visible(true);

        let ui_weak = ui.as_weak();
        let toast_seq = self.editor_toast_seq.clone();
        slint::Timer::single_shot(Duration::from_millis(2800), move || {
            if toast_seq.get() == seq
                && let Some(ui) = ui_weak.upgrade()
            {
                ui.set_editor_toast_visible(false);
            }
        });
    }

    fn show_writing_toast(&self, ui: &AppWindow, text: String) {
        let seq = self.writing_toast_seq.get() + 1;
        self.writing_toast_seq.set(seq);
        ui.set_writing_toast_text(text.into());
        ui.set_writing_toast_visible(true);

        let ui_weak = ui.as_weak();
        let toast_seq = self.writing_toast_seq.clone();
        slint::Timer::single_shot(Duration::from_millis(2800), move || {
            if toast_seq.get() == seq
                && let Some(ui) = ui_weak.upgrade()
            {
                ui.set_writing_toast_visible(false);
            }
        });
    }

    /// Paint `page` and save its text offset as the resume position.
    fn apply_page(&mut self, ui: &AppWindow, page: usize, reset_timer: bool) {
        let now = now_ms();
        let Some(reader) = self.reader.as_mut() else {
            return;
        };
        if page >= reader.pages.len() {
            return;
        }
        reader.page = page;
        reader.layout = reader.pages[page].layout.clone();
        reader.anchor = reader.pages[page].start;
        if reset_timer {
            reader.timer.reset(now);
        }
        reader.tts_sentence = -1;

        let story_id = reader.story.id.clone();
        let story_title = reader.story.title.clone();
        let level = reader.story.level;
        let page_count = reader.pages.len();
        let offset = reader.pages[page].start;
        let completed = self.progress.is_completed(&story_id);

        ui.set_reader_title(story_title.into());
        ui.set_reader_level(level.as_str().into());
        ui.set_reader_page_index(page as i32);
        ui.set_reader_page_count(page_count as i32);
        ui.set_reader_page_label(format!("Page {} of {page_count}", page + 1).into());
        ui.set_reader_words(reader_words(&reader.layout));
        ui.set_reader_underlines(reader_marks(&reader.layout.underlines));
        ui.set_reader_sentence_marks(reader_marks(&reader.layout.sentence_marks));
        ui.set_reader_font_size(reader.layout.font_size);
        ui.set_reader_line_height(reader.layout.line_height);
        ui.set_reader_overlay_visible(reader.overlay_visible);
        ui.set_reader_completed(completed);
        ui.set_reader_tts_sentence(-1);

        self.progress.record_offset(&story_id, offset);
        if let Err(error) = self.storage.save_progress(&self.progress) {
            eprintln!("[progress] could not save progress: {error}");
        }
        self.refresh_home(ui);
    }

    /// Turn the page instantly. There is no flip animation and no two-phase
    /// state, so a tap can never leave a page half-turned.
    fn request_turn(&mut self, ui: &AppWindow, delta: i32, _shared: &SharedState) {
        let target = {
            let Some(reader) = self.reader.as_ref() else {
                return;
            };
            if reader.popup.is_some() {
                return;
            }
            let now = now_ms();
            if now.saturating_sub(reader.opened_at_ms) < OPEN_TAP_GUARD_MS
                || now.saturating_sub(reader.popup_closed_at_ms) < POPUP_CLOSE_GUARD_MS
            {
                return;
            }
            let target = reader.page as i32 + delta;
            if target < 0 || target >= reader.pages.len() as i32 {
                return;
            }
            target as usize
        };

        self.apply_page(ui, target, true);
        self.restart_tts_after_turn(ui);
    }

    /// After a page turn, keep narrating the new page if Listen was active.
    fn restart_tts_after_turn(&mut self, ui: &AppWindow) {
        let Some(reader) = self.reader.as_mut() else {
            return;
        };
        if !reader.tts_active {
            return;
        }
        let sentences = reader.layout.sentences.clone();
        if sentences.is_empty() {
            self.tts.stop();
            reader.tts_active = false;
            reader.tts_sentence = -1;
            ui.set_reader_tts_active(false);
            ui.set_reader_tts_sentence(-1);
            return;
        }
        let generation = self.tts.speak(sentences);
        reader.tts_generation = generation;
        reader.tts_sentence = -1;
        ui.set_reader_tts_active(true);
        ui.set_reader_tts_sentence(-1);
    }

    fn reader_tap(&mut self, ui: &AppWindow, x: f32, y: f32, shared: &SharedState) {
        enum Action {
            Lookup { token: String, occurrence: u32 },
            Turn(i32),
            ToggleOverlay,
            None,
        }

        let action = {
            let Some(reader) = self.reader.as_ref() else {
                return;
            };
            if reader.popup.is_some() {
                return;
            }
            let now = now_ms();
            if now.saturating_sub(reader.opened_at_ms) < OPEN_TAP_GUARD_MS
                || now.saturating_sub(reader.popup_closed_at_ms) < POPUP_CLOSE_GUARD_MS
            {
                return;
            }
            if let Some(index) = reader.layout.hit_test(x, y) {
                let word = &reader.layout.words[index];
                if !word.token.is_empty() {
                    Action::Lookup {
                        token: word.token.clone(),
                        occurrence: word.occurrence,
                    }
                } else {
                    Action::None
                }
            } else if x < reader.width * 0.3 {
                Action::Turn(-1)
            } else if x > reader.width * 0.7 {
                Action::Turn(1)
            } else {
                Action::ToggleOverlay
            }
        };

        match action {
            Action::Lookup { token, occurrence } => self.open_popup(ui, token, occurrence),
            Action::Turn(delta) => self.request_turn(ui, delta, shared),
            Action::ToggleOverlay => {
                if let Some(reader) = self.reader.as_mut() {
                    reader.overlay_visible = !reader.overlay_visible;
                    ui.set_reader_overlay_visible(reader.overlay_visible);
                }
            }
            Action::None => {}
        }
    }

    fn open_popup(&mut self, ui: &AppWindow, token: String, occurrence: u32) {
        let mode = self.settings.meanings_source;
        let Some(reader) = self.reader.as_ref() else {
            return;
        };
        let story = reader.story.clone();
        let cache = self.lookup_cache.clone();

        self.lookup_seq += 1;
        let id = self.lookup_seq;
        let loading = mode == MeaningsSource::Online;

        ui.set_reader_popup_word(token.clone().into());
        ui.set_reader_popup_article("".into());
        ui.set_reader_popup_meaning("".into());
        ui.set_reader_popup_base("".into());
        ui.set_reader_popup_source("".into());
        ui.set_reader_popup_note("".into());
        ui.set_reader_popup_loading(loading);
        ui.set_reader_popup_visible(true);

        if let Some(reader) = self.reader.as_mut() {
            reader.popup = Some(PopupState {
                job: id,
                word: token.clone(),
                article: String::new(),
                meaning: String::new(),
                base: String::new(),
                source: String::new(),
                note: String::new(),
                loading,
            });
        }
        self.lookup = None;

        if loading {
            let (tx, rx) = mpsc::channel();
            let worker_token = token.clone();
            let worker_story = story.clone();
            let worker_cache = cache;
            let now = now_unix();
            std::thread::spawn(move || {
                let provider = MyMemoryProvider::new();
                let (outcome, cache_entry) = lookup::resolve(
                    &worker_story,
                    &worker_token,
                    occurrence,
                    MeaningsSource::Online,
                    &worker_cache,
                    Some(&provider),
                    now,
                );
                let _ = tx.send((outcome, cache_entry));
            });
            self.lookup = Some(LookupJob {
                id,
                token,
                occurrence,
                started: Instant::now(),
                rx,
            });
        } else {
            let (outcome, cache_entry) = lookup::resolve(
                &story,
                &token,
                occurrence,
                MeaningsSource::Offline,
                &cache,
                None,
                now_unix(),
            );
            self.finish_lookup(ui, id, outcome, cache_entry);
        }
    }

    fn close_popup(&mut self, ui: &AppWindow) {
        self.lookup = None;
        if let Some(reader) = self.reader.as_mut() {
            reader.popup = None;
            reader.popup_closed_at_ms = now_ms();
        }
        ui.set_reader_popup_visible(false);
        ui.set_reader_popup_loading(false);
    }

    fn finish_lookup(
        &mut self,
        ui: &AppWindow,
        id: u64,
        outcome: LookupOutcome,
        cache_entry: Option<CacheEntry>,
    ) {
        if let Some(entry) = cache_entry {
            self.lookup_cache.insert(entry);
            if let Err(error) = self.storage.save_lookup_cache(&self.lookup_cache) {
                eprintln!("[lookup] could not save the cache: {error}");
            }
        }

        let Some(reader) = self.reader.as_mut() else {
            return;
        };
        let Some(popup) = reader.popup.as_mut() else {
            return;
        };
        if popup.job != id {
            return;
        }

        let article = outcome.article.clone().unwrap_or_default();
        let base = outcome.base.clone().unwrap_or_default();
        let note = outcome.note.clone().unwrap_or_default();
        popup.loading = false;
        popup.word = outcome.word.clone();
        popup.article = article.clone();
        popup.meaning = outcome.meaning.clone();
        popup.base = base.clone();
        popup.source = outcome.source.label().to_string();
        popup.note = note.clone();

        ui.set_reader_popup_loading(false);
        ui.set_reader_popup_word(popup.word.clone().into());
        ui.set_reader_popup_article(article.into());
        ui.set_reader_popup_meaning(popup.meaning.clone().into());
        ui.set_reader_popup_base(base.into());
        ui.set_reader_popup_source(popup.source.clone().into());
        ui.set_reader_popup_note(note.into());
    }

    fn toggle_tts(&mut self, ui: &AppWindow) {
        let active = self.reader.as_ref().is_some_and(|reader| reader.tts_active);
        if active {
            self.stop_tts(ui);
            return;
        }
        if !self.speak_current_page(ui) {
            self.show_toast(ui, "There is nothing to read on this page.".into());
        }
    }

    /// Speak the page that is currently shown. Returns false when the page has
    /// no sentences.
    fn speak_current_page(&mut self, ui: &AppWindow) -> bool {
        let Some(reader) = self.reader.as_mut() else {
            return false;
        };
        let sentences = reader.layout.sentences.clone();
        if sentences.is_empty() {
            return false;
        }
        let generation = self.tts.speak(sentences);
        reader.tts_generation = generation;
        reader.tts_active = true;
        reader.tts_sentence = -1;
        ui.set_reader_tts_active(true);
        ui.set_reader_tts_sentence(-1);
        true
    }

    fn stop_tts(&mut self, ui: &AppWindow) {
        self.tts.stop();
        if let Some(reader) = self.reader.as_mut() {
            reader.tts_active = false;
            reader.tts_sentence = -1;
        }
        ui.set_reader_tts_active(false);
        ui.set_reader_tts_sentence(-1);
    }

    fn close_reader(&mut self, ui: &AppWindow) {
        self.lookup = None;
        if self.reader.take().is_some() {
            self.tts.stop();
        }
        ui.set_reader_popup_visible(false);
        ui.set_reader_popup_loading(false);
        ui.set_reader_tts_active(false);
        ui.set_reader_tts_sentence(-1);
        ui.set_reader_toast_visible(false);
        ui.set_reader_overlay_visible(true);
    }

    fn set_theme(&mut self, ui: &AppWindow, theme: Theme) {
        self.settings.theme = theme;
        self.save_settings();
        ui.set_theme_setting(theme_code(theme).into());
    }

    fn set_tts_speed(&mut self, ui: &AppWindow, value: f32, persist: bool) {
        let speed = if value.is_finite() {
            value.clamp(0.5, 2.0)
        } else {
            DEFAULT_TTS_SPEED
        };
        self.settings.tts_speed = speed;
        self.tts.set_speed(speed);
        ui.set_settings_tts_speed(speed);
        ui.set_settings_tts_speed_label(format!("{speed:.1}×").into());
        if persist {
            self.save_settings();
        }
    }

    fn save_settings(&mut self) {
        self.settings.sanitize();
        if let Err(error) = self.storage.save_settings(&self.settings) {
            eprintln!("[settings] could not save settings: {error}");
        }
    }

    fn show_toast(&self, ui: &AppWindow, text: String) {
        let seq = self.toast_seq.get() + 1;
        self.toast_seq.set(seq);
        ui.set_reader_toast_text(text.into());
        ui.set_reader_toast_visible(true);

        let ui_weak = ui.as_weak();
        let toast_seq = self.toast_seq.clone();
        slint::Timer::single_shot(Duration::from_millis(2800), move || {
            if toast_seq.get() == seq
                && let Some(ui) = ui_weak.upgrade()
            {
                ui.set_reader_toast_visible(false);
            }
        });
    }

    fn tick(&mut self, ui: &AppWindow) {
        self.poll_lifecycle(ui);
        self.tick_editor(ui);
        self.poll_tts(ui);
        self.poll_lookup(ui);
        self.ensure_reader_size(ui);
        self.poll_page_timer(ui);
        // The first Home render can happen before the window has a size. Once
        // the real size is known, recompute the page labels so Home and Level
        // agree with the reader on this device.
        let dims = reader_dims(ui);
        if dims != self.home_dims {
            self.refresh_home(ui);
        }
    }

    fn poll_lifecycle(&mut self, ui: &AppWindow) {
        let epoch = self.lifecycle.epoch();
        if epoch == self.lifecycle_epoch {
            return;
        }
        self.lifecycle_epoch = epoch;
        let now = now_ms();
        if let Some(reader) = self.reader.as_mut() {
            reader.timer.reset(now);
        }
        if self.lifecycle.is_foreground() {
            // Resume: restart the editor clock now so background time is not
            // counted.
            if let Some(editor) = self.editor.as_mut() {
                editor.last_tick = Instant::now();
            }
        } else {
            self.stop_tts(ui);
            self.close_popup(ui);
            // Backgrounding saves the draft; `save_editor_draft` flushes the
            // foreground time first.
            if self.editor.is_some() {
                self.save_editor_draft(ui);
            }
        }
    }

    fn poll_tts(&mut self, ui: &AppWindow) {
        while let Ok(event) = self.tts_events.try_recv() {
            if let Some(message) = self.handle_tts_event(ui, event) {
                self.show_toast(ui, message);
            }
        }
    }

    /// Returns a message to toast, if the event needs one.
    fn handle_tts_event(&mut self, ui: &AppWindow, event: TtsEvent) -> Option<String> {
        let generation = match &event {
            TtsEvent::Sentence { generation, .. }
            | TtsEvent::Finished { generation }
            | TtsEvent::Error { generation, .. } => *generation,
        };
        let current = match self.reader.as_ref() {
            Some(reader) if reader.tts_active => reader.tts_generation,
            _ => return None,
        };
        if generation != current {
            return None;
        }

        match event {
            TtsEvent::Sentence { index, .. } => {
                if let Some(reader) = self.reader.as_mut() {
                    reader.tts_sentence = index as i32;
                }
                ui.set_reader_tts_sentence(index as i32);
                None
            }
            TtsEvent::Finished { .. } => {
                // Listening continues onto the next computed page instead of
                // stopping at every page break.
                let next = self
                    .reader
                    .as_ref()
                    .map(|reader| reader.page + 1)
                    .filter(|next| {
                        self.reader
                            .as_ref()
                            .is_some_and(|reader| *next < reader.pages.len())
                    });
                if let Some(next) = next {
                    self.apply_page(ui, next, true);
                    self.speak_current_page(ui);
                    return None;
                }
                if let Some(reader) = self.reader.as_mut() {
                    reader.tts_active = false;
                    reader.tts_sentence = -1;
                }
                ui.set_reader_tts_active(false);
                ui.set_reader_tts_sentence(-1);
                None
            }
            TtsEvent::Error { message, .. } => {
                if let Some(reader) = self.reader.as_mut() {
                    reader.tts_active = false;
                    reader.tts_sentence = -1;
                }
                ui.set_reader_tts_active(false);
                ui.set_reader_tts_sentence(-1);
                Some(message)
            }
        }
    }

    fn poll_lookup(&mut self, ui: &AppWindow) {
        let mut result = None;
        let mut fallback = false;
        if let Some(job) = self.lookup.as_ref() {
            match job.rx.try_recv() {
                Ok(value) => result = Some(value),
                Err(TryRecvError::Empty) => {
                    fallback = job.started.elapsed() > Duration::from_secs(8);
                }
                Err(TryRecvError::Disconnected) => fallback = true,
            }
        }

        if let Some((outcome, cache_entry)) = result {
            if let Some(job) = self.lookup.take() {
                self.finish_lookup(ui, job.id, outcome, cache_entry);
            }
            return;
        }

        if fallback {
            let Some(job) = self.lookup.take() else {
                return;
            };
            let Some(reader) = self.reader.as_ref() else {
                return;
            };
            let (mut outcome, _) = lookup::resolve(
                &reader.story,
                &job.token,
                job.occurrence,
                MeaningsSource::Offline,
                &self.lookup_cache,
                None,
                now_unix(),
            );
            outcome.note = Some(
                "The online dictionary did not answer in time. Showing the story glossary.".into(),
            );
            self.finish_lookup(ui, job.id, outcome, None);
        }
    }

    fn ensure_reader_size(&mut self, ui: &AppWindow) {
        let (width, height) = reader_dims(ui);
        let needs_relayout = self.reader.as_ref().is_some_and(|reader| {
            (reader.width - width).abs() > 1.0 || (reader.height - height).abs() > 1.0
        });
        if !needs_relayout {
            return;
        }
        let mut restart_tts = false;
        let mut new_offset = None;
        if let Some(reader) = self.reader.as_mut() {
            // Map the anchor (not the current page's start) so several resizes
            // in a row always land on the same place in the text.
            let offset = reader.anchor;
            let old_page = reader.page;
            let pages = paginate(&reader.story, &reader.text, width, height, READER_FONT_SIZE);
            let page = page_for_offset(&pages, offset);
            reader.width = width;
            reader.height = height;
            reader.pages = pages;
            reader.page = page;
            reader.layout = reader.pages[page].layout.clone();
            if page != old_page {
                reader.timer.reset(now_ms());
            }
            new_offset = Some((reader.story.id.clone(), reader.pages[page].start));
            restart_tts = reader.tts_active;
            ui.set_reader_page_index(page as i32);
            ui.set_reader_page_count(reader.pages.len() as i32);
            ui.set_reader_page_label(format!("Page {} of {}", page + 1, reader.pages.len()).into());
            ui.set_reader_font_size(reader.layout.font_size);
            ui.set_reader_line_height(reader.layout.line_height);
            ui.set_reader_words(reader_words(&reader.layout));
            ui.set_reader_underlines(reader_marks(&reader.layout.underlines));
            ui.set_reader_sentence_marks(reader_marks(&reader.layout.sentence_marks));
        }
        if let Some((story_id, offset)) = new_offset {
            self.progress.record_offset(&story_id, offset);
            if let Err(error) = self.storage.save_progress(&self.progress) {
                eprintln!("[progress] could not save progress: {error}");
            }
        }
        if restart_tts {
            self.restart_tts_after_turn(ui);
        }
    }

    fn poll_page_timer(&mut self, ui: &AppWindow) {
        if !self.lifecycle.is_foreground() {
            return;
        }
        let now = now_ms();
        let fired = self
            .reader
            .as_mut()
            .is_some_and(|reader| reader.timer.poll(now));
        if !fired {
            return;
        }

        let Some((story, page_text, page_index, last_page)) = self.reader.as_ref().map(|reader| {
            (
                reader.story.clone(),
                reader.pages[reader.page].text(&reader.text).to_string(),
                reader.page,
                reader.pages.len().saturating_sub(1),
            )
        }) else {
            return;
        };

        let added =
            self.dictionary
                .add_page_words(&story, &page_text, now / 1000, &self.starter_origins);
        if added > 0 {
            if let Err(error) = self.storage.save_dictionary(&self.dictionary) {
                eprintln!("[dictionary] could not save the dictionary: {error}");
            }
            self.refresh_dictionary(ui);
        }

        let completed_now = page_index == last_page && !self.progress.is_completed(&story.id) && {
            self.progress.mark_completed_at(&story.id, now / 1000);
            true
        };
        if completed_now {
            if let Err(error) = self.storage.save_progress(&self.progress) {
                eprintln!("[progress] could not save completion: {error}");
            }
            self.refresh_home(ui);
            ui.set_reader_completed(true);
        }

        if completed_now {
            self.show_toast(
                ui,
                if added > 0 {
                    format!("Story finished! {added} new words added to your dictionary.")
                } else {
                    "Story finished!".to_string()
                },
            );
        } else if added > 0 {
            self.show_toast(ui, format!("{added} new words added to your dictionary."));
        }
    }
}

fn reader_dims(ui: &AppWindow) -> (f32, f32) {
    let mut width = ui.get_reader_text_width();
    let mut height = ui.get_reader_text_height();
    // Before the first frame the window has no size. Fall back to the large
    // reference phone so Home, Level, and the first page all agree; a later
    // tick repaginates for the real size.
    if !width.is_finite() || width < 120.0 {
        width = REFERENCE_LARGE_WIDTH;
    }
    if !height.is_finite() || height < 200.0 {
        height = REFERENCE_LARGE_HEIGHT;
    }
    (width, height)
}

/// Where `offset` lands in the pagination for the current screen. Home and Level
/// use this so their page labels agree with what the reader will show on this
/// device, even after the text has been repaginated for a new screen size.
fn device_position(story: &Story, offset: usize, width: f32, height: f32) -> (usize, usize) {
    let pages = paginate_story(story, width, height);
    (page_for_offset(&pages, offset), pages.len())
}

fn reader_words(layout: &PageLayout) -> ModelRc<ReaderWord> {
    let words: Vec<ReaderWord> = layout
        .words
        .iter()
        .map(|word| ReaderWord {
            text: word.text.clone().into(),
            x: word.x,
            y: word.y,
            width: word.width,
            height: word.height,
        })
        .collect();
    Rc::new(VecModel::from(words)).into()
}

fn reader_marks(rects: &[RectBox]) -> ModelRc<ReaderMark> {
    let marks: Vec<ReaderMark> = rects
        .iter()
        .map(|rect| ReaderMark {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: rect.height,
            sentence: rect.sentence as i32,
        })
        .collect();
    Rc::new(VecModel::from(marks)).into()
}

/// Ignore reader input for this long after a story opens. The tap that opened
/// the story can otherwise reach the reader underneath and toggle the overlay.
const OPEN_TAP_GUARD_MS: u64 = 300;

/// Ignore reader input for this long after the lookup sheet closes. The release
/// that dismisses the sheet can otherwise turn the page underneath it.
const POPUP_CLOSE_GUARD_MS: u64 = 250;

/// Map every starter lemma to the first story (in reading order) that uses it.
/// The personal dictionary uses this for attribution, so `de` is credited to
/// A1-01 even when it is added while reading A2.
fn starter_origins(index: &StoryIndex) -> HashMap<String, (String, Level)> {
    let starter = StarterWords::embedded();
    let mut origins = HashMap::new();
    for loaded in index.stories() {
        let story = &loaded.story;
        let lexicon = story.lexicon();
        for token in story.tokens() {
            let Some(entry) = lexicon.resolve(token.text) else {
                continue;
            };
            let lemma = normalize_word(&entry.lemma);
            if starter.is_lemma(&lemma) {
                origins
                    .entry(lemma)
                    .or_insert_with(|| (story.id.clone(), story.level));
            }
        }
    }
    origins
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}

fn now_unix() -> u64 {
    now_ms() / 1000
}

fn theme_code(theme: Theme) -> &'static str {
    match theme {
        Theme::System => "system",
        Theme::Light => "light",
        Theme::Dark => "dark",
    }
}

fn theme_from_code(code: &str) -> Theme {
    match code {
        "light" => Theme::Light,
        "dark" => Theme::Dark,
        _ => Theme::System,
    }
}

fn meanings_code(source: MeaningsSource) -> &'static str {
    match source {
        MeaningsSource::Offline => "offline",
        MeaningsSource::Online => "online",
    }
}

fn meanings_from_code(code: &str) -> MeaningsSource {
    match code {
        "online" => MeaningsSource::Online,
        _ => MeaningsSource::Offline,
    }
}

fn level_label(level: Level) -> &'static str {
    match level {
        Level::A1 => "Beginner",
        Level::A2 => "Elementary",
        Level::B1 => "Intermediate",
        Level::B2 => "Upper intermediate",
    }
}

/// `1 of 1 story`, `0 of 1 story`, `2 of 5 stories`, `No stories yet`.
fn count_text(completed: usize, total: usize) -> String {
    if total == 0 {
        return "No stories yet".to_string();
    }
    format!(
        "{completed} of {total} {}",
        if total == 1 { "story" } else { "stories" }
    )
}

/// `1 word`, `0 words`, `12 words`.
fn count_words(total: usize) -> String {
    format!("{total} {}", if total == 1 { "word" } else { "words" })
}

/// `1 story`, `0 stories`, `12 stories`.
fn count_stories(total: usize) -> String {
    format!("{total} {}", if total == 1 { "story" } else { "stories" })
}

/// `1 day`, `0 days`, `12 days`.
fn count_days(total: usize) -> String {
    format!("{total} {}", if total == 1 { "day" } else { "days" })
}

fn level_cards(index: &StoryIndex, progress: &Progress) -> ModelRc<LevelCardData> {
    let cards: Vec<LevelCardData> = Level::ALL
        .iter()
        .map(|&level| {
            let ids: Vec<&str> = index
                .level(level)
                .map(|loaded| loaded.story.id.as_str())
                .collect();
            let total = ids.len();
            let completed = progress.completed_count(ids.iter().copied());
            let ratio = if total == 0 {
                0.0
            } else {
                completed as f32 / total as f32
            };
            LevelCardData {
                code: level.as_str().into(),
                label: level_label(level).into(),
                count_text: count_text(completed, total).into(),
                progress: ratio,
            }
        })
        .collect();
    Rc::new(VecModel::from(cards)).into()
}

fn story_rows(
    index: &StoryIndex,
    progress: &Progress,
    level: Level,
    width: f32,
    height: f32,
) -> ModelRc<StoryRow> {
    let rows: Vec<StoryRow> = index
        .level(level)
        .map(|loaded| {
            let story = &loaded.story;
            let completed = progress.is_completed(&story.id);
            let started = progress.has_started(&story.id);
            let offset = progress.last_offset_of(&story.id);
            let (page, pages) = device_position(story, offset, width, height);
            let progress_text = if completed {
                format!("Finished · {pages} pages · tap to read again")
            } else if started {
                format!("Continue on page {} of {pages}", (page + 1).min(pages))
            } else {
                format!("Not started · {pages} pages")
            };
            StoryRow {
                id: story.id.clone().into(),
                number: format!("STORY {:02}", story.order).into(),
                title: story.title.clone().into(),
                blurb: story.blurb_nl.clone().into(),
                progress: progress_text.into(),
                completed,
                started,
            }
        })
        .collect();
    Rc::new(VecModel::from(rows)).into()
}

/// Fill the "Continue reading" card on Home, or hide it when there is nothing
/// to resume.
fn apply_resume(ui: &AppWindow, index: &StoryIndex, progress: &Progress, width: f32, height: f32) {
    let loaded = progress.last_story.as_deref().and_then(|id| index.get(id));

    match loaded {
        Some(loaded) => {
            let story = &loaded.story;
            let offset = progress.last_offset_of(&story.id);
            let (page, pages) = device_position(story, offset, width, height);
            let detail = if progress.is_completed(&story.id) {
                format!("{} · finished · {pages} pages", story.level)
            } else {
                format!(
                    "{} · page {} of {pages}",
                    story.level,
                    (page + 1).min(pages)
                )
            };
            ui.set_resume_visible(true);
            ui.set_resume_title(story.title.clone().into());
            ui.set_resume_detail(detail.into());
        }
        None => {
            ui.set_resume_visible(false);
            ui.set_resume_title("".into());
            ui.set_resume_detail("".into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slint::Model;

    #[test]
    fn count_text_uses_singular_for_one_story() {
        assert_eq!(count_text(0, 1), "0 of 1 story");
        assert_eq!(count_text(1, 1), "1 of 1 story");
    }

    #[test]
    fn count_text_uses_plural_and_empty_state() {
        assert_eq!(count_text(2, 5), "2 of 5 stories");
        assert_eq!(count_text(0, 0), "No stories yet");
    }

    #[test]
    fn count_words_uses_singular_for_one_word() {
        assert_eq!(count_words(0), "0 words");
        assert_eq!(count_words(1), "1 word");
        assert_eq!(count_words(12), "12 words");
    }

    #[test]
    fn settings_codes_round_trip() {
        for theme in [Theme::System, Theme::Light, Theme::Dark] {
            assert_eq!(theme_from_code(theme_code(theme)), theme);
        }
        for source in [MeaningsSource::Offline, MeaningsSource::Online] {
            assert_eq!(meanings_from_code(meanings_code(source)), source);
        }
    }

    /// The Level screen renders exactly the rows produced here.
    #[test]
    fn level_rows_match_the_embedded_index() {
        use crate::reader::layout::{REFERENCE_LARGE_HEIGHT, REFERENCE_LARGE_WIDTH};

        let index = StoryIndex::load_embedded().expect("embedded stories");
        let progress = Progress::default();
        for level in Level::ALL {
            let rows = story_rows(
                &index,
                &progress,
                level,
                REFERENCE_LARGE_WIDTH,
                REFERENCE_LARGE_HEIGHT,
            );
            assert_eq!(
                rows.row_count(),
                index.level_count(level),
                "rows for {level}"
            );
        }
    }

    #[test]
    fn story_rows_show_resume_page_then_completion() {
        use crate::reader::layout::{
            REFERENCE_LARGE_HEIGHT, REFERENCE_LARGE_WIDTH, paginate_story,
        };

        let index = StoryIndex::load_embedded().expect("embedded stories");
        let story = &index.get("a1-01").expect("a1-01 exists").story;
        let pages = paginate_story(story, REFERENCE_LARGE_WIDTH, REFERENCE_LARGE_HEIGHT);
        let offset = pages[2].start;
        let mut progress = Progress::default();

        progress.record_offset_at("a1-01", offset, 100);
        let rows = story_rows(
            &index,
            &progress,
            Level::A1,
            REFERENCE_LARGE_WIDTH,
            REFERENCE_LARGE_HEIGHT,
        );
        let row = rows.row_data(0).expect("first A1 row");
        assert!(row.started);
        assert!(!row.completed);
        assert_eq!(
            row.progress.as_str(),
            format!("Continue on page 3 of {}", pages.len())
        );

        progress.mark_completed_at("a1-01", 200);
        let rows = story_rows(
            &index,
            &progress,
            Level::A1,
            REFERENCE_LARGE_WIDTH,
            REFERENCE_LARGE_HEIGHT,
        );
        let row = rows.row_data(0).expect("first A1 row");
        assert!(row.completed);
        assert!(row.progress.as_str().starts_with("Finished"));
    }
}
