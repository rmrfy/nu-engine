use jni::objects::{JClass, JIntArray};
use jni::sys::{jint, jintArray};
use jni::JNIEnv;

use crate::math;
use crate::sort;

// Return a sorted copy, leaving the caller's Java array unchanged.
fn sort_array(mut env: JNIEnv, input: JIntArray, algorithm: fn(&mut [i32])) -> jintArray {
    let result = (|| -> jni::errors::Result<jintArray> {
        let length = env.get_array_length(&input)?;
        let mut values = vec![0; length as usize];
        env.get_int_array_region(&input, 0, &mut values)?;
        algorithm(&mut values);
        let output = env.new_int_array(length)?;
        env.set_int_array_region(&output, 0, &values)?;
        Ok(output.into_raw())
    })();

    match result {
        Ok(output) => output,
        Err(error) => {
            // Preserve JVM exceptions (including allocation failures).
            if !env.exception_check().unwrap_or(true) {
                let _ = env.throw_new("java/lang/RuntimeException", error.to_string());
            }
            std::ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_bubbleSort(
    env: JNIEnv,
    _class: JClass,
    values: JIntArray,
) -> jintArray {
    sort_array(env, values, sort::bubble_sort)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_insertionSort(
    env: JNIEnv,
    _class: JClass,
    values: JIntArray,
) -> jintArray {
    sort_array(env, values, sort::insertion_sort)
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_io_rmrfy_nu_NuEngine_selectionSort(
    env: JNIEnv,
    _class: JClass,
    values: JIntArray,
) -> jintArray {
    sort_array(env, values, sort::selection_sort)
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
