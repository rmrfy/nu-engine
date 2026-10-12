use jni::objects::JClass;
use jni::sys::jint;
use jni::JNIEnv;

use crate::math;

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
