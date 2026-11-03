//! The phone's current UTC offset, including DST.
//!
//! Android exposes the system timezone through `java.util.TimeZone`; asking it
//! for the offset **at a given instant** is what makes dates around a DST
//! change correct. The desktop preview uses the host clock through `chrono`.
//! Everything else in the app takes the offset as a parameter, so date and
//! statistics tests stay deterministic.

/// UTC offset in seconds east of UTC at `unix_seconds`.
///
/// Returns 0 if the platform cannot report an offset; callers then fall back to
/// UTC rather than failing to draw the screen.
pub fn offset_seconds(unix_seconds: u64) -> i32 {
    #[cfg(target_os = "android")]
    {
        android::offset_seconds(unix_seconds)
    }
    #[cfg(not(target_os = "android"))]
    {
        desktop::offset_seconds(unix_seconds)
    }
}

#[cfg(target_os = "android")]
mod android {
    use jni::JNIEnv;
    use jni::JavaVM;
    use jni::errors::Error as JniError;
    use jni::objects::JValue;

    pub fn offset_seconds(unix_seconds: u64) -> i32 {
        let context = ndk_context::android_context();
        let vm = match unsafe { JavaVM::from_raw(context.vm().cast()) } {
            Ok(vm) => vm,
            Err(error) => {
                eprintln!("[timezone] could not access the Java VM: {error}");
                return 0;
            }
        };
        let mut env = match vm.attach_current_thread() {
            Ok(env) => env,
            Err(error) => {
                eprintln!("[timezone] could not attach the timezone thread: {error}");
                return 0;
            }
        };

        let result = offset_inner(&mut env, unix_seconds);
        if result.is_err() && env.exception_check().unwrap_or(false) {
            let _ = env.exception_describe();
        }
        if env.exception_check().unwrap_or(false) {
            let _ = env.exception_clear();
        }
        match result {
            Ok(offset) => offset,
            Err(error) => {
                eprintln!("[timezone] could not read the local UTC offset: {error}");
                0
            }
        }
    }

    fn offset_inner(env: &mut JNIEnv, unix_seconds: u64) -> Result<i32, JniError> {
        let zone = env
            .call_static_method(
                "java/util/TimeZone",
                "getDefault",
                "()Ljava/util/TimeZone;",
                &[],
            )?
            .l()?;
        let millis = (unix_seconds as i64).saturating_mul(1_000);
        let offset = env
            .call_method(&zone, "getOffset", "(J)I", &[JValue::Long(millis)])?
            .i()?;
        // java.util.TimeZone.getOffset(long) reports milliseconds, while the
        // rest of the app works in seconds. Dividing here keeps the JNI unit
        // conversion in one place.
        Ok(super::millis_to_seconds(offset))
    }
}

#[cfg(not(target_os = "android"))]
mod desktop {
    use chrono::{Local, TimeZone};

    pub fn offset_seconds(unix_seconds: u64) -> i32 {
        let seconds = unix_seconds as i64;
        match Local.timestamp_opt(seconds, 0).single() {
            Some(local) => local.offset().local_minus_utc(),
            None => 0,
        }
    }
}

/// Convert `java.util.TimeZone.getOffset(long)` milliseconds to seconds.
/// Only Android calls this outside tests.
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
fn millis_to_seconds(offset_millis: i32) -> i32 {
    offset_millis / 1_000
}

#[cfg(test)]
mod tests {
    use super::millis_to_seconds;

    #[test]
    fn java_offset_milliseconds_become_seconds() {
        assert_eq!(millis_to_seconds(7_200_000), 7_200);
        assert_eq!(millis_to_seconds(-18_000_000), -18_000);
        assert_eq!(millis_to_seconds(0), 0);
    }
}
