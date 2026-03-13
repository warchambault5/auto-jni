use std::path::Path;

/// Call a static Java method, caching the method ID in a `OnceCell`.
#[macro_export]
macro_rules! call_static {
    ($path:tt, $method:tt, $sig:tt, $args:expr, $ret:expr) => {{
        use auto_jni::once_cell::sync::OnceCell;
        use auto_jni::jni::objects::{JClass, JStaticMethodID};
        use crate::java;
        static FNPTR: OnceCell<JStaticMethodID> = OnceCell::new();
        static CLASS: OnceCell<JClass> = OnceCell::new();
        let mut env = java();
        let fnptr = FNPTR.get_or_init(|| env.get_static_method_id($path, $method, $sig).unwrap());
        let class = CLASS.get_or_init(|| env.find_class($path).unwrap());
        unsafe { env.call_static_method_unchecked(class, fnptr, $ret, $args).unwrap() }
    }};
}

/// Call an instance Java method, caching the method ID in a `OnceCell`.
#[macro_export]
macro_rules! call {
    ($obj:expr, $path:tt, $method:tt, $sig:tt, $args:expr, $ret:expr) => {{
        use auto_jni::once_cell::sync::OnceCell;
        use auto_jni::jni::objects::JMethodID;
        use crate::java;
        static FNPTR: OnceCell<JMethodID> = OnceCell::new();
        let mut env = java();
        let fnptr = FNPTR.get_or_init(|| {
            let class = env.find_class($path).unwrap();
            env.get_method_id(class, $method, $sig).unwrap()
        });
        unsafe { env.call_method_unchecked($obj, fnptr, $ret, $args).unwrap() }
    }};
}

/// Construct a Java object, caching the constructor ID in a `OnceCell`.
/// Returns a `GlobalRef`.
#[macro_export]
macro_rules! create {
    ($path:tt, $sig:tt, $args:expr) => {{
        use auto_jni::once_cell::sync::OnceCell;
        use auto_jni::jni::objects::{JClass, JMethodID};
        use crate::java;
        static FNPTR: OnceCell<JMethodID> = OnceCell::new();
        static CLASS: OnceCell<JClass> = OnceCell::new();
        let mut env = java();
        let class = CLASS.get_or_init(|| env.find_class($path).unwrap());
        let fnptr = FNPTR.get_or_init(|| env.get_method_id(class, "<init>", $sig).unwrap());
        let obj = unsafe { env.new_object_unchecked(class, *fnptr, $args).unwrap() };
        env.new_global_ref(obj).unwrap()
    }};
}

// /// Generate a Rust source file with JNI bindings for the given Java classes.
// ///
// /// Call this from your `build.rs`:
// /// ```no_run
// /// auto_jni::generate_bindings_file(
// ///     vec!["com.example.MyClass"],
// ///     Some("path/to/classes".into()),
// ///     &std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("bindings.rs"),
// ///     Some(vec!["-Djava.class.path=path/to/classes".into()]),
// /// ).unwrap();
// /// ```
// pub fn generate_bindings_file(
//     classes: Vec<&str>,
//     class_path: Option<String>,
//     output_path: &Path,
//     jvm_options: Option<Vec<String>>,
// ) -> std::io::Result<()> {
//     let mut file = File::create(output_path)?;

//     write_header(&mut file, jvm_options)?;

//     for class in classes {
//         let bindings = parse_javap_output(class, class_path.clone());
//         write_class(&mut file, class, bindings)?;
//     }

//     Ok(())
// }