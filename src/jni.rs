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

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_multiply(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    math::multiply(a, b)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_abs(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
) -> jint {
    math::abs(a)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_min(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    math::min(a, b)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_max(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    math::max(a, b)
}
