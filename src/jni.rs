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
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_divide(
    mut env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    division_result(&mut env, math::divide(a, b))
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_remainder(
    mut env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    division_result(&mut env, math::remainder(a, b))
}

fn division_result(env: &mut JNIEnv, result: Option<i32>) -> jint {
    match result {
        Some(value) => value,
        None => {
            // JNI callers observe the pending exception, not this placeholder value.
            let _ = env.throw_new("java/lang/ArithmeticException", "/ by zero");
            0
        }
    }
}
