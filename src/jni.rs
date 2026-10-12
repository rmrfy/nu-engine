use jni::objects::JClass;
use jni::sys::jint;
use jni::JNIEnv;

use crate::math;

pub const SECRET_KEY: &str = "932dca8e20171d160da4bc7324e9d56194b14d952e7e8b7a53372d1069d1c319";

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
