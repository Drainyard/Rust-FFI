use crate::shared::windef::{ UINT, BOOL };

use crate::shared::dxgicommon::{ DXGI_RATIONAL };
use crate::shared::dxgiformat::{ DXGI_FORMAT };


STRUCT! { struct D3DCOLORVALUE {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}}

pub type DXGI_RGBA = D3DCOLORVALUE;

STRUCT! { struct DXGI_RGB {
    Red: f32,
    Green: f32,
    Blue: f32,
}}


ENUM!{enum DXGI_MODE_SCANLINE_ORDER { 
  DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED        = 0,
  DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE        = 1,
  DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST  = 2,
  DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST  = 3,
}}

ENUM!{ enum DXGI_MODE_SCALING {
    DXGI_MODE_SCALING_UNSPECIFIED   = 0,
    DXGI_MODE_SCALING_CENTERED      = 1,
    DXGI_MODE_SCALING_STRETCHED     = 2,
}}

ENUM!{ enum DXGI_MODE_ROTATION {
    DXGI_MODE_ROTATION_UNSPECIFIED  = 0,
    DXGI_MODE_ROTATION_IDENTITY     = 1,
    DXGI_MODE_ROTATION_ROTATE90     = 2,
    DXGI_MODE_ROTATION_ROTATE180    = 3,
    DXGI_MODE_ROTATION_ROTATE270    = 4,
}}

STRUCT! { struct DXGI_MODE_DESC {
    Width: UINT,
    Height: UINT,
    RefreshRate: DXGI_RATIONAL,
    Format: DXGI_FORMAT,
    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    Scaling: DXGI_MODE_SCALING,
}}

STRUCT! { struct DXGI_GAMMA_CONTROL {
    Scale: DXGI_RGB,
    Offset: DXGI_RGB,
    GammaCurve: [DXGI_RGB; 1025],
}}

STRUCT! { struct DXGI_GAMMA_CONTROL_CAPABILITIES {
    ScaleAndOffsetSupported: BOOL,
    MaxConvertedValue: f32,
    MinConvertedValue: f32 ,
    NumGammaControlPoints: UINT,
    ControlPointPositions: [f32; 1025],
}}

