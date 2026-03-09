use std::{io, ops::Deref};

use jni::{
    JNIEnv, JavaVM,
    objects::{GlobalRef, JClass, JObject, JString, JValue},
    sys::jlong,
};
use once_cell::sync::OnceCell;

use crate::{DEFAULT_CANCEL_LABEL, DEFAULT_OK_LABEL, DEFAULT_TITLE, InputBox};

use super::Backend;

static GLOBAL: OnceCell<(JavaVM, GlobalRef, GlobalRef)> = OnceCell::new();

/// Android backend for InputBox.
///
/// This backend uses JNI to call into the Android InputBox AAR library. The AAR
/// provides a native Android dialog implementation.
///
/// # Setup
///
/// To use this backend, you need to:
///
/// 1. Add the `inputbox-android` AAR to your Android project.
/// 2. Call [`Android::set_android_activity`] to initialize the backend with the
///    Android actitivty.
/// 3. Use `System.loadLibrary` to load your native library containing this
///    crate before showing any dialogs.
///
/// # Limitations
///
/// - `width` and `height` options are ignored.
///
/// # Defaults
///
/// - `title`: `DEFAULT_TITLE`
/// - `prompt`: empty
/// - `cancel_label`: `DEFAULT_CANCEL_LABEL`
/// - `ok_label`: `DEFAULT_OK_LABEL`
#[derive(Debug, Clone, Default)]
pub struct Android {
    _priv: (),
}

impl Android {
    /// Creates a new Android backend.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_android_activity(env: &mut JNIEnv, activity: &JObject) -> jni::errors::Result<()> {
        let java_vm = env.get_java_vm()?;
        let java_class = env.find_class("moe/mivik/inputbox/InputBox")?;
        let java_class = env.new_global_ref(java_class)?;
        let activity = env.new_global_ref(activity)?;
        let _ = GLOBAL.set((java_vm, java_class, activity));
        Ok(())
    }

    fn show_dialog(
        &self,
        input: &InputBox,
        callback: Box<dyn FnOnce(io::Result<Option<String>>) + Send>,
    ) -> jni::errors::Result<()> {
        let (vm, java_class, activity) = GLOBAL
            .get()
            .expect("Android activity not set. Call Android::set_android_activity first.");
        let mut env = vm.attach_current_thread()?;

        let java_class: &JClass = (java_class.deref()).into();

        let title = env.new_string(input.title.as_deref().unwrap_or(DEFAULT_TITLE))?;
        let prompt = input
            .prompt
            .as_ref()
            .map(|it| env.new_string(it))
            .transpose()?
            .map_or_else(JObject::null, |it| it.into());
        let default = env.new_string(&input.default)?;
        let ok_label = env.new_string(input.ok_label.as_deref().unwrap_or(DEFAULT_OK_LABEL))?;
        let cancel_label = env.new_string(
            input
                .cancel_label
                .as_deref()
                .unwrap_or(DEFAULT_CANCEL_LABEL),
        )?;
        let mode = env.new_string(input.mode.as_str())?;

        env.call_static_method(
            java_class,
            "showInput",
            "(JLandroid/app/Activity;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;ZZ)V",
            &[
                JValue::Long(Box::into_raw(Box::new(callback)) as _),
                activity.into(),
                (&title).into(),
                (&prompt).into(),
                (&default).into(),
                (&ok_label).into(),
                (&cancel_label).into(),
                (&mode).into(),
                input.auto_wrap.into(),
                input.scroll_to_end.into(),
            ],
        )?;

        Ok(())
    }
}

impl Backend for Android {
    fn execute_async(
        &self,
        input: &InputBox,
        callback: Box<dyn FnOnce(io::Result<Option<String>>) + Send>,
    ) -> io::Result<()> {
        self.show_dialog(input, callback).map_err(io::Error::other)
    }
}

#[unsafe(export_name = "Java_moe_mivik_inputbox_InputBox_inputCallback")]
extern "system" fn input_callback(mut env: JNIEnv, _class: JClass, callback: jlong, text: JString) {
    let text: Option<String> = if text.is_null() {
        None
    } else {
        env.get_string(&text).ok().map(|s| s.into())
    };
    let callback = unsafe {
        Box::from_raw(callback as *mut Box<dyn FnOnce(io::Result<Option<String>>) + Send>)
    };
    callback(Ok(text));
}
