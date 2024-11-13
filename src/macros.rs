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

macro_rules! impl_string_custom_typedef {
    ($ffi_type:ident) => {
        impl UniffiCustomTypeConverter for $ffi_type {
            type Builtin = String;
            fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
                Ok(val.parse::<$ffi_type>()?)
            }

            fn from_custom(obj: Self) -> Self::Builtin {
                obj.to_string()
            }
        }
    };
}
