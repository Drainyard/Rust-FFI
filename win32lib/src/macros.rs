/// Creates a macro that can make a C-style enum with defined values
#[macro_export]
macro_rules! ENUM {
    {enum $name:ident { $($variant:ident = $value:expr,)+ }} => {
        pub type $name = u32;
        $(pub const $variant: $name = $value;)+
    };
}

#[macro_export]
macro_rules! opaque {
    ($name:ident, $inner:ident) => {
        #[repr(C)] pub struct $inner { private: [u8; 0] }
        pub type $name = *mut $inner;
    }
}

#[macro_export]
macro_rules! STRUCT {
    ($(#[$attrs:meta])* struct $name:ident {
        $($field:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] #[derive(Copy)] $(#[$attrs])*
        pub struct $name {
            $(pub $field: $ftype,)+
        }

        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }

        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { std::mem::zeroed() } }
        }
    );
}

// #[macro_export]
// macro_rules! UNION {
//     ($(#[$attrs:meta])* union $name:ident {
//         [$stype:ty; $ssize:expr],
//         $($variant:ident $variant_mut:ident: $ftype:ty,)+
//     }) => (
//         #[repr(C)] $(#[$attrs])*
//         pub struct $name([$stype; $ssize]);

//         impl Copy for $name{}
//         impl Clone for $name {
//             #[inline]
//             fn clone(&self) -> $name { *self }
//         }

//         impl $name {$(
//             #[inline]
//             pub fn $variant(&self) -> $ftype {
//                 &*(self as *const _ as *const $ftype)
//             }

//             #[inline]
//             pub fn $variant_mut(&mut self) -> &mut $ftype {
//                 &mut *(self as *mut _ as *mut $ftype)
//             }
//         )+}
//     );
// }

#[macro_export]
macro_rules! UNION {
    ($(#[$attrs:meta])* union $name:ident {
        $($field:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])*
        pub union $name {
            $(pub $field: $ftype,)+
        }

        impl Copy for $name{}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
    );
}


#[macro_export]
macro_rules! DEFINE_GUID {
    (
        $name:ident, $l:expr, $w1:expr, $w2:expr,
        $b1:expr, $b2:expr, $b3:expr, $b4:expr, $b5:expr, $b6:expr, $b7:expr, $b8:expr
    ) => {
        pub const $name: $crate::shared::guiddef::GUID = $crate::shared::guiddef::GUID {
            Data1: $l,
            Data2: $w1,
            Data3: $w2,
            Data4: [$b1,$b2, $b3, $b4, $b5, $b6, $b7, $b8],
        };
    }
}
