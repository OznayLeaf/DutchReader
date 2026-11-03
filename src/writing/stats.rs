//! Writing statistics: totals, per-month stories and words, and the current
//! streak.
//!
//! All dates are **local** civil dates. The caller supplies the UTC offset that
//! applies at each stored timestamp, so a story written late at night is grouped
//! under the day the writer saw, and DST changes are handled per timestamp.
//!
//! **Streak definition:** a day counts when at least one story was saved that
//! local day. The current streak is the run of consecutive days ending today. If
//! nothing was written today but something was written yesterday, the streak is
//! still alive through today and is counted from yesterday. If neither today
//! nor yesterday has a story, the streak is zero. Multiple stories on one day
//! count once.

use super::store::StoryEntry;
use crate::time::{format_month, local_date_time, local_days};
use std::collections::{BTreeMap, BTreeSet};

pub use crate::time::{civil_from_days, days_from_civil, format_date, format_date_time};

#[derive(Debug, Clone, PartialEq)]
pub struct MonthStat {
    pub year: i32,
    pub month: u32,
    /// e.g. `May 2026`.
    pub label: String,
    pub stories: usize,
    pub words: usize,
    /// Bar width relative to the busiest month, 0.0 - 1.0.
    pub ratio: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub total_stories: usize,
    pub total_words: usize,
    pub months: Vec<MonthStat>,
    pub streak_days: usize,
}

impl Stats {
    /// Compute statistics from saved stories. `now` is a Unix timestamp in
    /// seconds. `offset_at` returns the phone's UTC offset at a timestamp; tests
    /// pass fixed values so month, year, and DST boundaries are exact.
    pub fn compute<F>(stories: &[StoryEntry], now: u64, offset_at: F) -> Stats
    where
        F: Fn(u64) -> i32,
    {
        let mut by_month: BTreeMap<(i32, u32), (usize, usize)> = BTreeMap::new();
        let mut days: BTreeSet<i64> = BTreeSet::new();
        let mut total_words = 0;

        for story in stories {
            let offset = offset_at(story.created_at);
            total_words += story.word_count;
            days.insert(local_days(story.created_at, offset));
            let (year, month, _, _, _) = local_date_time(story.created_at, offset);
            let entry = by_month.entry((year, month)).or_default();
            entry.0 += 1;
            entry.1 += story.word_count;
        }

        let max_words = by_month
            .values()
            .map(|(_, words)| *words)
            .max()
            .unwrap_or(0);
        let months = by_month
            .into_iter()
            .rev()
            .map(|((year, month), (stories, words))| MonthStat {
                year,
                month,
                label: format_month(year, month),
                stories,
                words,
                ratio: if max_words == 0 {
                    0.0
                } else {
                    words as f32 / max_words as f32
                },
            })
            .collect();

        Stats {
            total_stories: stories.len(),
            total_words,
            months,
            streak_days: current_streak(&days, now, offset_at(now)),
        }
    }
}

fn current_streak(days: &BTreeSet<i64>, now: u64, offset_seconds: i32) -> usize {
    let today = local_days(now, offset_seconds);
    let mut cursor = if days.contains(&today) {
        today
    } else if days.contains(&(today - 1)) {
        today - 1
    } else {
        return 0;
    };

    let mut streak = 0;
    while days.contains(&cursor) {
        streak += 1;
        cursor -= 1;
    }
    streak
}

#[cfg(test)]
mod tests {
    use super::*;

    fn story(created_at: u64, words: usize) -> StoryEntry {
        StoryEntry {
            id: format!("w-{created_at}"),
            title: String::new(),
            text: String::new(),
            created_at,
            updated_at: created_at,
            word_count: words,
            elapsed_seconds: 0,
        }
    }

    /// UTC midnight.
    fn at(year: i32, month: u32, day: u32) -> u64 {
        (days_from_civil(year, month, day) * 86_400) as u64
    }

    /// Unix seconds for a local wall-clock time at a fixed UTC offset.
    fn local_at(year: i32, month: u32, day: u32, hour: u32, minute: u32, offset: i32) -> u64 {
        (days_from_civil(year, month, day) * 86_400 + hour as i64 * 3_600 + minute as i64 * 60
            - offset as i64) as u64
    }

    fn utc(_: u64) -> i32 {
        0
    }

    #[test]
    fn date_and_time_formatting_uses_the_offset() {
        let noon = at(2026, 5, 12) + 12 * 3_600 + 5 * 60;
        assert_eq!(format_date(noon, 0), "12 May 2026");
        assert_eq!(format_date_time(noon, 0), "12 May 2026, 12:05");
        assert_eq!(format_date(noon, 3_600), "12 May 2026");
        assert_eq!(format_date_time(noon, 3_600), "12 May 2026, 13:05");
        assert_eq!(format_month(2026, 5), "May 2026");
    }

    #[test]
    fn totals_and_months_cover_a_year_boundary() {
        let stories = vec![
            story(at(2025, 12, 31), 10),
            story(at(2026, 1, 1), 20),
            story(at(2026, 1, 2), 30),
            story(at(2026, 2, 15), 40),
        ];
        let stats = Stats::compute(&stories, at(2026, 2, 15), utc);

        assert_eq!(stats.total_stories, 4);
        assert_eq!(stats.total_words, 100);
        assert_eq!(stats.months.len(), 3);
        assert_eq!(
            stats.months[0],
            MonthStat {
                year: 2026,
                month: 2,
                label: "Feb 2026".into(),
                stories: 1,
                words: 40,
                // January is the busiest month with 50 words.
                ratio: 0.8,
            }
        );
        assert_eq!(stats.months[1].label, "Jan 2026");
        assert_eq!(stats.months[1].stories, 2);
        assert_eq!(stats.months[1].words, 50);
        assert_eq!(stats.months[2].label, "Dec 2025");
        assert_eq!(stats.months[2].stories, 1);
        assert_eq!(stats.months[2].words, 10);
    }

    #[test]
    fn month_grouping_uses_local_time_near_midnight() {
        // 01:00 UTC on 1 February is still 31 January in New York.
        let stories = vec![story(local_at(2026, 1, 31, 20, 0, -18_000), 5)];
        let stats = Stats::compute(&stories, local_at(2026, 2, 2, 12, 0, -18_000), |_| -18_000);
        assert_eq!(stats.months.len(), 1);
        assert_eq!(stats.months[0].label, "Jan 2026");
        assert_eq!(stats.months[0].stories, 1);
    }

    #[test]
    fn month_grouping_uses_local_time_after_midnight_in_a_positive_offset() {
        // 00:30 local on 1 February is still 31 January in UTC.
        let stories = vec![story(local_at(2026, 2, 1, 0, 30, 7_200), 5)];
        let stats = Stats::compute(&stories, local_at(2026, 2, 2, 12, 0, 7_200), |_| 7_200);
        assert_eq!(stats.months.len(), 1);
        assert_eq!(stats.months[0].label, "Feb 2026");
    }

    #[test]
    fn streak_counts_across_a_year_boundary() {
        let stories = vec![
            story(at(2025, 12, 31), 1),
            story(at(2026, 1, 1), 1),
            story(at(2026, 1, 2), 1),
        ];
        let stats = Stats::compute(&stories, at(2026, 1, 2), utc);
        assert_eq!(stats.streak_days, 3);
    }

    #[test]
    fn streak_uses_local_days_across_a_year_boundary() {
        // Both stories are on 31 December and 1 January local time, even though
        // the UTC instants are late in the evening.
        let stories = vec![
            story(local_at(2025, 12, 31, 23, 30, 3_600), 1),
            story(local_at(2026, 1, 1, 23, 30, 3_600), 1),
        ];
        let stats = Stats::compute(&stories, local_at(2026, 1, 2, 12, 0, 3_600), |_| 3_600);
        assert_eq!(stats.streak_days, 2);
        assert_eq!(stats.months.len(), 2);
        assert_eq!(stats.months[0].label, "Jan 2026");
        assert_eq!(stats.months[1].label, "Dec 2025");
    }

    #[test]
    fn dst_offsets_are_applied_per_timestamp() {
        // Europe: 29 March 2026, 01:00 UTC is the spring-forward instant.
        let transition = at(2026, 3, 29) + 3_600;
        let offset_at = |unix: u64| if unix < transition { 3_600 } else { 7_200 };
        let before = transition - 1_800;
        let after = transition + 1_800;

        assert_eq!(format_date_time(before, 3_600), "29 Mar 2026, 01:30");
        assert_eq!(format_date_time(after, 7_200), "29 Mar 2026, 03:30");

        let stories = vec![story(before, 2), story(after, 3)];
        let stats = Stats::compute(&stories, transition + 86_400, offset_at);
        assert_eq!(stats.months.len(), 1);
        assert_eq!(stats.months[0].label, "Mar 2026");
        assert_eq!(stats.months[0].stories, 2);
        assert_eq!(stats.months[0].words, 5);
        assert_eq!(stats.streak_days, 1);
    }

    #[test]
    fn streak_stays_alive_through_today_after_yesterdays_story() {
        let stories = vec![story(at(2026, 1, 1), 1), story(at(2026, 1, 2), 1)];
        let stats = Stats::compute(&stories, at(2026, 1, 3), utc);
        assert_eq!(stats.streak_days, 2);

        // Two days later the streak is over.
        let stats = Stats::compute(&stories, at(2026, 1, 4), utc);
        assert_eq!(stats.streak_days, 0);
    }

    #[test]
    fn multiple_stories_on_one_day_count_once() {
        let stories = vec![
            story(at(2026, 3, 1), 1),
            story(at(2026, 3, 1) + 3_600, 2),
            story(at(2026, 3, 2), 3),
        ];
        let stats = Stats::compute(&stories, at(2026, 3, 2), utc);
        assert_eq!(stats.streak_days, 2);
        assert_eq!(stats.months[0].stories, 3);
        assert_eq!(stats.months[0].words, 6);
    }

    #[test]
    fn empty_store_has_zeroed_statistics() {
        let stats = Stats::compute(&[], at(2026, 5, 12), utc);
        assert_eq!(stats.total_stories, 0);
        assert_eq!(stats.total_words, 0);
        assert!(stats.months.is_empty());
        assert_eq!(stats.streak_days, 0);
    }

    #[test]
    fn statistics_match_the_saved_story_records() {
        let stories = vec![
            story(at(2025, 11, 2), 12),
            story(at(2025, 12, 20), 30),
            story(at(2026, 1, 5), 7),
        ];
        let stats = Stats::compute(&stories, at(2026, 1, 5), utc);
        assert_eq!(stats.total_stories, stories.len());
        assert_eq!(
            stats.total_words,
            stories.iter().map(|story| story.word_count).sum::<usize>()
        );
        assert_eq!(
            stats
                .months
                .iter()
                .map(|month| month.stories)
                .sum::<usize>(),
            stories.len()
        );
        assert_eq!(
            stats.months.iter().map(|month| month.words).sum::<usize>(),
            stats.total_words
        );
    }
}
