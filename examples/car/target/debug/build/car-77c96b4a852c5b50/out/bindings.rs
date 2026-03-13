use auto_jni::jni::objects::{JObject, GlobalRef};
use auto_jni::jni::objects::{JValue, JObjectArray};
use auto_jni::jni::signature::{Primitive, ReturnType};
use auto_jni::jni::{InitArgsBuilder, JNIEnv, JNIVersion, JavaVM};
use auto_jni::lazy_static::lazy_static;
use auto_jni::errors::JNIError;
use auto_jni::{call, call_static, create};

lazy_static! { static ref JAVA: JavaVM = create_jvm(); }

fn create_jvm() -> JavaVM {
    let jvm_args = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        .option("-Djava.class.path=../java/src")
        .build().unwrap();
    JavaVM::new(jvm_args).unwrap()
}

pub fn java() -> JNIEnv<'static> {
    JAVA.attach_current_thread_permanently().unwrap()
}

pub struct com_example_Car {
    inner: GlobalRef,
}

impl<'a> com_example_Car {
    pub fn inner(&self) -> &GlobalRef {
        &self.inner
    }
}

