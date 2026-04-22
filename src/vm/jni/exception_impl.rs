use crate::vm::jni::java_thread::JavaThread;
use jni_sys::{jboolean, jint, jthrowable, JNIEnv, JNI_ERR, JNI_FALSE, JNI_OK};
use std::ptr::null_mut;

pub(super) extern "system" fn exception_check(_env: *mut JNIEnv) -> jboolean {
    JNI_FALSE // todo: implement me
}

pub(super) extern "system" fn exception_occurred(_env: *mut JNIEnv) -> jthrowable {
    null_mut() as jthrowable // todo: implement me
}

pub(super) extern "system" fn throw(_env: *mut JNIEnv, obj: jthrowable) -> jint {
    if obj.is_null() {
        return JNI_ERR;
    }
    JavaThread::set_pending_exception(obj as i32);
    JNI_OK
}
