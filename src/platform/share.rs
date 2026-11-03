//! Android share-sheet bridge.
//!
//! Builds an `ACTION_SEND` intent with `text/plain`, wraps it in
//! `Intent.createChooser`, and starts it from the activity context. The desktop
//! preview reports a friendly error instead of touching JNI, so the editor can
//! keep the draft and show a note.

/// Open the Android share sheet with `text`. Returns an error when the sheet
/// could not be launched; callers keep the draft in that case.
pub fn share_text(text: &str) -> Result<(), String> {
    #[cfg(target_os = "android")]
    {
        android::share(text)
    }
    #[cfg(not(target_os = "android"))]
    {
        let _ = text;
        Err("Sharing is only available in the Android app. Your draft is still saved.".into())
    }
}

#[cfg(target_os = "android")]
mod android {
    use jni::JNIEnv;
    use jni::JavaVM;
    use jni::errors::Error as JniError;
    use jni::objects::{JObject, JValue};

    pub fn share(text: &str) -> Result<(), String> {
        let context = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(context.vm().cast()) }
            .map_err(|error| format!("could not access the Android Java VM: {error}"))?;
        let mut env = vm
            .attach_current_thread()
            .map_err(|error| format!("could not attach the share thread: {error}"))?;

        // The activity owns this local reference; never delete it from here.
        let activity = unsafe { JObject::from_raw(context.context().cast()) };
        let result = share_inner(&mut env, &activity, text);
        std::mem::forget(activity);
        if result.is_err() && env.exception_check().unwrap_or(false) {
            // Print the Java stack trace to logcat before clearing it.
            let _ = env.exception_describe();
        }
        clear_exception(&mut env);
        result.map_err(|error| format!("could not open the share sheet: {error}"))
    }

    fn share_inner(env: &mut JNIEnv, activity: &JObject, text: &str) -> Result<(), JniError> {
        let action = env.new_string("android.intent.action.SEND")?;
        let intent = env.new_object(
            "android/content/Intent",
            "(Ljava/lang/String;)V",
            &[JValue::Object(&JObject::from(action))],
        )?;

        let mime = env.new_string("text/plain")?;
        env.call_method(
            &intent,
            "setType",
            "(Ljava/lang/String;)Landroid/content/Intent;",
            &[JValue::Object(&JObject::from(mime))],
        )?;

        let extra_name = env.new_string("android.intent.extra.TEXT")?;
        let body = env.new_string(text)?;
        env.call_method(
            &intent,
            "putExtra",
            "(Ljava/lang/String;Ljava/lang/CharSequence;)Landroid/content/Intent;",
            &[
                JValue::Object(&JObject::from(extra_name)),
                JValue::Object(&JObject::from(body)),
            ],
        )?;

        let chooser_title = env.new_string("Share your story")?;
        let chooser = env
            .call_static_method(
                "android/content/Intent",
                "createChooser",
                "(Landroid/content/Intent;Ljava/lang/CharSequence;)Landroid/content/Intent;",
                &[
                    JValue::Object(&intent),
                    JValue::Object(&JObject::from(chooser_title)),
                ],
            )?
            .l()?;

        // The JNI bridge runs on Slint's native event-loop thread, and the
        // context handed to us by android-activity is not always the Activity
        // itself. `FLAG_ACTIVITY_NEW_TASK` makes `startActivity` valid from any
        // context; without it Android throws AndroidRuntimeException.
        const FLAG_ACTIVITY_NEW_TASK: i32 = 0x1000_0000;
        env.call_method(
            &chooser,
            "addFlags",
            "(I)Landroid/content/Intent;",
            &[JValue::Int(FLAG_ACTIVITY_NEW_TASK)],
        )?;

        env.call_method(
            activity,
            "startActivity",
            "(Landroid/content/Intent;)V",
            &[JValue::Object(&chooser)],
        )?;
        Ok(())
    }

    fn clear_exception(env: &mut JNIEnv) {
        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_os = "android"))]
    #[test]
    fn desktop_share_reports_android_only_and_mentions_the_draft() {
        let error = share_text("Hallo.").expect_err("desktop cannot share");
        assert!(error.contains("Android"));
        assert!(error.contains("draft"));
    }
}
