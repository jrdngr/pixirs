#[macro_export]
macro_rules! js_get {
    ($js_ref:expr, $var:expr) => {
        let result = js! {
            const js_result = @{$js_ref.js_ref()};
            return js_result.$var;
        };
        result.try_into().unwrap()
    }
}

#[macro_export]
macro_rules! js_set {
    ($js_ref:expr, $var:expr, $value:expr) => {
        js! { @(no_return)
            const this = @{js_ref.js_ref()};
            this.$var = $value;
        };
    }
}