use crate::shared::windef::{ UINT };

STRUCT! { struct DXGI_SAMPLE_DESC {
  Count: UINT,
  Quality: UINT,
}}

STRUCT! { struct DXGI_RATIONAL {
  Numerator: UINT,
  Denominator: UINT,
}}
