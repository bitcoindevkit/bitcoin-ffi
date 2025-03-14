macro_rules! impl_from_core_type {
    ($ffi_type:ty, $core_type:ty) => {
        impl From<$core_type> for $ffi_type {
            fn from(core_type: $core_type) -> Self {
                Self(core_type)
            }
        }
    };
}

macro_rules! impl_from_ffi_type {
    ($ffi_type:ty, $core_type:ty) => {
        impl From<$ffi_type> for $core_type {
            fn from(ffi_type: $ffi_type) -> Self {
                ffi_type.0
            }
        }
    };
}

#[macro_export]
macro_rules! define_custom_string_type {
    ($ffi_type:ident) => {
        uniffi::custom_type!($ffi_type, String, {
            remote,
            lower: |value: $ffi_type| value.to_string(),
            try_lift: |value: String| value.parse()
                .map_err(|_| uniffi::deps::anyhow::Error::msg(
                    format!("Failed to parse {} from string", stringify!($ffi_type))
                )),
        });
    };
}
