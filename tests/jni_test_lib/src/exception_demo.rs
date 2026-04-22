use jni::sys::{jclass, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_samples_jni_exceptions_JniThrowExample_throwViaThrow(
    env: *mut JNIEnv,
    _class: jclass,
    throwable: jobject,
) {
    unsafe {
        ((*(*env)).v24.Throw)(env, throwable);
    }
}
