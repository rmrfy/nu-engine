use jni::objects::{JClass, JIntArray};
use jni::sys::{jint, jintArray};
use jni::JNIEnv;

use crate::math;
use crate::sort;

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

// Java declarations: public static native int[] sortAscending(int[] values);
//                    public static native int[] sortDescending(int[] values);
#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_sortAscending(
    mut env: JNIEnv,
    _class: JClass,
    values: JIntArray,
) -> jintArray {
    sort_array(&mut env, values, sort::ascending)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_sortDescending(
    mut env: JNIEnv,
    _class: JClass,
    values: JIntArray,
) -> jintArray {
    sort_array(&mut env, values, sort::descending)
}

// Copy the input so sorting does not modify the caller's array.
fn sort_array(env: &mut JNIEnv, values: JIntArray, sort: fn(&mut [i32])) -> jintArray {
    if values.is_null() {
        let _ = env.throw_new("java/lang/NullPointerException", "values must not be null");
        return std::ptr::null_mut();
    }

    let result = (|| -> jni::errors::Result<jintArray> {
        let length = env.get_array_length(&values)?;
        let mut elements = vec![0; length as usize];
        env.get_int_array_region(&values, 0, &mut elements)?;
        sort(&mut elements);
        let output = env.new_int_array(length)?;
        env.set_int_array_region(&output, 0, &elements)?;
        Ok(output.into_raw())
    })();

    match result {
        Ok(output) => output,
        Err(error) => {
            if !env.exception_check().unwrap_or(true) {
                let _ = env.throw_new("java/lang/RuntimeException", error.to_string());
            }
            std::ptr::null_mut()
        }
    }
}
