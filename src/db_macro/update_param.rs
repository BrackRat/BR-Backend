#[macro_export]
macro_rules! generate_update_params {
    // 递归终止条件：当没有更多的字段需要处理时
    ($db_type:ident, $params:expr;) => {};

    // 处理一个字段，然后递归地处理剩下的
    ($db_type:ident, $params:expr; $field:ident: $value:expr, $($tail:tt)*) => {
        // 如果字段值为Some，添加到参数向量中
        if let Some(value) = $value {
            $params.push($db_type::SetParam::$field(value));
        }

        // 递归处理剩下的字段
        generate_update_params!($db_type, $params; $($tail)*);
    };
}
