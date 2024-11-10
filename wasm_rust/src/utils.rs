pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


// QUE: 如何理解这里的宏语法?
// QUE：宏如果定义在这个文件中，应该如何在其他的文件中引入并使用？
// #[macro_export]
// macro_rules! log {
//     ($($t:tt)*) => {
//         web_sys::console::log_1(&format!($($t)*).into());
//     };
// }