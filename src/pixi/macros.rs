#[macro_export]
macro_rules! js_get {
    ($js_ref:expr, $var:expr) => {{
        let result = js! {
            const me = @{$js_ref.js_ref()};
            return me.$var;
        };
        result.try_into().unwrap()
    }};
}

#[macro_export]
macro_rules! js_set {
    ($js_ref:expr, $var:expr, $value:expr) => {{
        js! { @(no_return)
            const me = @{$js_ref.js_ref()};
            me.$var = @{$value};
        }};
    };
}

#[macro_export]
macro_rules! js_function {
    ($js_ref:expr, $func:expr, $($params:expr),*) => {{
        js! { @(no_return)
            const me = @{$js_ref.js_ref()};
            me.$func($(@{$params}, )*);
        }
    }};
}