macro_rules! impl_from_core_type {
    ($ffi_type:ident, $core_type:ident) => {
        impl From<$core_type> for $ffi_type {
            fn from(core_type: $core_type) -> Self {
                $ffi_type(core_type)
            }
        }
    };
}

macro_rules! impl_from_ffi_type {
    ($ffi_type:ident, $core_type:ident) => {
        impl From<$ffi_type> for $core_type {
            fn from(ffi_type: $ffi_type) -> Self {
                ffi_type.0
            }
        }
    };
}

