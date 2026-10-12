use jni::objects::JClass;
use jni::sys::{jint, jlong};
use jni::JNIEnv;

use crate::math;
use crate::utils;

/// Implements `public static native long currentTimeMillis()` on `io.rmrfy.nu.NuEngine`.
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_currentTimeMillis(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    utils::current_time_millis()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_add(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    math::add(a, b)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_subtract(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    math::subtract(a, b)
}
