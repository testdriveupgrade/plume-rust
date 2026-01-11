//
// plume
//
// Copyright (c) 2024 renderbag and contributors. All rights reserved.
// Licensed under the MIT license. See LICENSE file for details.
//

#![allow(dead_code, unused_variables)]

#[cfg(target_os = "windows")]
pub type RenderWindow = *mut std::ffi::c_void;
#[cfg(target_os = "android")]
pub type RenderWindow = *mut std::ffi::c_void;
#[cfg(all(target_os = "linux", not(feature = "plume_sdl_vulkan_enabled")))]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct RenderWindow {
    pub display: *mut std::ffi::c_void,
    pub window: u64,
}
#[cfg(target_os = "macos")]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct RenderWindow {
    pub window: *mut std::ffi::c_void,
    pub view: *mut std::ffi::c_void,
}
#[cfg(feature = "plume_sdl_vulkan_enabled")]
pub type RenderWindow = *mut std::ffi::c_void;

pub struct RenderBuffer;
pub struct RenderDescriptorSet;
pub struct RenderPipeline;
pub struct RenderPipelineLayout;
pub struct RenderSampler;
pub struct RenderShader;
pub struct RenderTexture;
pub struct RenderTextureView;
pub struct RenderQueryPool;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RenderDeviceVendor {
    Unknown = 0x0,
    Amd = 0x1002,
    Nvidia = 0x10DE,
    Intel = 0x8086,
    Apple = 0x106B,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderFormat {
    Unknown,
    R32G32B32A32Typeless,
    R32G32B32A32Float,
    R32G32B32A32Uint,
    R32G32B32A32Sint,
    R32G32B32Typeless,
    R32G32B32Float,
    R32G32B32Uint,
    R32G32B32Sint,
    R16G16B16A16Typeless,
    R16G16B16A16Float,
    R16G16B16A16Unorm,
    R16G16B16A16Uint,
    R16G16B16A16Snorm,
    R16G16B16A16Sint,
    R32G32Typeless,
    R32G32Float,
    R32G32Uint,
    R32G32Sint,
    R8G8B8A8Typeless,
    R8G8B8A8Unorm,
    R8G8B8A8Uint,
    R8G8B8A8Snorm,
    R8G8B8A8Sint,
    B8G8R8A8Unorm,
    R16G16Typeless,
    R16G16Float,
    R16G16Unorm,
    R16G16Uint,
    R16G16Snorm,
    R16G16Sint,
    R32Typeless,
    D32Float,
    D32FloatS8Uint,
    R32Float,
    R32Uint,
    R32Sint,
    R8G8Typeless,
    R8G8Unorm,
    R8G8Uint,
    R8G8Snorm,
    R8G8Sint,
    R16Typeless,
    R16Float,
    D16Unorm,
    R16Unorm,
    R16Uint,
    R16Snorm,
    R16Sint,
    R8Typeless,
    R8Unorm,
    R8Uint,
    R8Snorm,
    R8Sint,
    BC1Typeless,
    BC1Unorm,
    BC1UnormSrgb,
    BC2Typeless,
    BC2Unorm,
    BC2UnormSrgb,
    BC3Typeless,
    BC3Unorm,
    BC3UnormSrgb,
    BC4Typeless,
    BC4Unorm,
    BC4Snorm,
    BC5Typeless,
    BC5Unorm,
    BC5Snorm,
    BC6HTypeless,
    BC6HUf16,
    BC6HSf16,
    BC7Typeless,
    BC7Unorm,
    BC7UnormSrgb,
    Max,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderTextureDimension {
    Unknown,
    Texture1D,
    Texture2D,
    Texture3D,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderTextureViewDimension {
    Unknown,
    Texture1D,
    Texture2D,
    Texture3D,
    TextureCube,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderCommandListType {
    Unknown,
    Direct,
    Compute,
    Copy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderPrimitiveTopology {
    Unknown,
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderCullMode {
    Unknown,
    None,
    Front,
    Back,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderFrontFace {
    Unknown,
    Clockwise,
    CounterClockwise,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderComparisonFunction {
    Unknown,
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderStencilOp {
    Unknown,
    Keep,
    Zero,
    Replace,
    IncrementAndClamp,
    DecrementAndClamp,
    Invert,
    IncrementAndWrap,
    DecrementAndWrap,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderInputSlotClassification {
    Unknown,
    PerVertexData,
    PerInstanceData,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderBlend {
    Unknown,
    Zero,
    One,
    SrcColor,
    InvSrcColor,
    SrcAlpha,
    InvSrcAlpha,
    DestAlpha,
    InvDestAlpha,
    DestColor,
    InvDestColor,
    SrcAlphaSat,
    BlendFactor,
    InvBlendFactor,
    Src1Color,
    InvSrc1Color,
    Src1Alpha,
    InvSrc1Alpha,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderBlendOperation {
    Unknown,
    Add,
    Subtract,
    RevSubtract,
    Min,
    Max,
}

pub mod RenderColorWriteEnable {
    pub type Bits = u8;
    pub const UNKNOWN: Bits = 0x0;
    pub const RED: Bits = 0x1;
    pub const GREEN: Bits = 0x2;
    pub const BLUE: Bits = 0x4;
    pub const ALPHA: Bits = 0x8;
    pub const ALL: Bits = RED | GREEN | BLUE | ALPHA;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderLogicOperation {
    Unknown,
    Clear,
    Set,
    Copy,
    CopyInverted,
    Noop,
    Invert,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Equiv,
    AndReverse,
    AndInverted,
    OrReverse,
    OrInverted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderFilter {
    Unknown,
    Nearest,
    Linear,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderMipmapMode {
    Unknown,
    Nearest,
    Linear,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderTextureAddressMode {
    Unknown,
    Wrap,
    Mirror,
    Clamp,
    Border,
    MirrorOnce,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderBorderColor {
    Unknown,
    TransparentBlack,
    OpaqueBlack,
    OpaqueWhite,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderShaderVisibility {
    Unknown,
    All,
    Vertex,
    Geometry,
    Pixel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderDescriptorRangeType {
    Unknown,
    ConstantBuffer,
    FormattedBuffer,
    ReadWriteFormattedBuffer,
    Texture,
    ReadWriteTexture,
    Sampler,
    StructuredBuffer,
    ReadWriteStructuredBuffer,
    ByteAddressBuffer,
    ReadWriteByteAddressBuffer,
    AccelerationStructure,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderRootDescriptorType {
    Unknown,
    ConstantBuffer,
    ShaderResource,
    UnorderedAccess,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderHeapType {
    Unknown,
    Default,
    Upload,
    Readback,
    GpuUpload,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderTextureArrangement {
    Unknown,
    RowMajor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderShaderFormat {
    Unknown,
    Dxil,
    Spirv,
    Metal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderRaytracingPipelineLibrarySymbolType {
    Unknown,
    Raygen,
    Miss,
    ClosestHit,
    AnyHit,
    Intersection,
    Callable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderAccelerationStructureType {
    Unknown,
    TopLevel,
    BottomLevel,
}

pub mod RenderShaderStageFlag {
    pub type Bits = u32;
    pub const NONE: Bits = 0x0;
    pub const VERTEX: Bits = 1 << 0;
    pub const GEOMETRY: Bits = 1 << 1;
    pub const PIXEL: Bits = 1 << 2;
    pub const COMPUTE: Bits = 1 << 3;
    pub const RAYGEN: Bits = 1 << 4;
    pub const ANY_HIT: Bits = 1 << 5;
    pub const CLOSEST_HIT: Bits = 1 << 6;
    pub const MISS: Bits = 1 << 7;
    pub const INTERSECTION: Bits = 1 << 8;
    pub const CALLABLE: Bits = 1 << 9;
}

pub type RenderShaderStageFlags = u32;

pub mod RenderBufferFlag {
    pub type Bits = u32;
    pub const NONE: Bits = 0x0;
    pub const VERTEX: Bits = 1 << 0;
    pub const INDEX: Bits = 1 << 1;
    pub const STORAGE: Bits = 1 << 2;
    pub const CONSTANT: Bits = 1 << 3;
    pub const FORMATTED: Bits = 1 << 4;
    pub const ACCELERATION_STRUCTURE: Bits = 1 << 5;
    pub const ACCELERATION_STRUCTURE_INPUT: Bits = 1 << 6;
    pub const ACCELERATION_STRUCTURE_SCRATCH: Bits = 1 << 7;
    pub const SHADER_BINDING_TABLE: Bits = 1 << 8;
    pub const UNORDERED_ACCESS: Bits = 1 << 9;
    pub const DEVICE_ADDRESSABLE: Bits = 1 << 10;
}

pub type RenderBufferFlags = u32;

pub mod RenderTextureFlag {
    pub type Bits = u32;
    pub const NONE: Bits = 0x0;
    pub const RENDER_TARGET: Bits = 1 << 0;
    pub const DEPTH_TARGET: Bits = 1 << 1;
    pub const STORAGE: Bits = 1 << 2;
    pub const UNORDERED_ACCESS: Bits = 1 << 3;
    pub const CUBE: Bits = 1 << 4;
}

pub type RenderTextureFlags = u32;

pub mod RenderBarrierStage {
    pub type Bits = u32;
    pub const NONE: Bits = 0x0;
    pub const GRAPHICS: Bits = 1 << 0;
    pub const COMPUTE: Bits = 1 << 1;
    pub const COPY: Bits = 1 << 2;
    pub const GRAPHICS_AND_COMPUTE: Bits = GRAPHICS | COMPUTE;
    pub const ALL: Bits = GRAPHICS | COMPUTE | COPY;
}

pub type RenderBarrierStages = u32;

pub mod RenderBufferAccess {
    pub type Bits = u32;
    pub const NONE: Bits = 0x0;
    pub const READ: Bits = 1 << 0;
    pub const WRITE: Bits = 1 << 1;
}

pub type RenderBufferAccessBits = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderTextureLayout {
    Unknown,
    General,
    ShaderRead,
    ColorWrite,
    DepthWrite,
    DepthRead,
    CopySource,
    CopyDest,
    ResolveSource,
    ResolveDest,
    Present,
}

pub mod RenderSampleCount {
    pub type Bits = u32;
    pub const COUNT_0: Bits = 0x0;
    pub const COUNT_1: Bits = 0x1;
    pub const COUNT_2: Bits = 0x2;
    pub const COUNT_4: Bits = 0x4;
    pub const COUNT_8: Bits = 0x8;
    pub const COUNT_16: Bits = 0x10;
    pub const COUNT_32: Bits = 0x20;
    pub const COUNT_64: Bits = 0x40;
    pub const COUNT_MAX: Bits = COUNT_64;
}

pub type RenderSampleCounts = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderDeviceType {
    Unknown,
    Integrated,
    Discrete,
    Virtual,
    Cpu,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderResolveMode {
    Min,
    Max,
    Average,
}

pub const fn render_format_size(format: RenderFormat) -> u32 {
    match format {
        RenderFormat::R32G32B32A32Typeless
        | RenderFormat::R32G32B32A32Float
        | RenderFormat::R32G32B32A32Uint
        | RenderFormat::R32G32B32A32Sint => 16,
        RenderFormat::R32G32B32Typeless
        | RenderFormat::R32G32B32Float
        | RenderFormat::R32G32B32Uint
        | RenderFormat::R32G32B32Sint => 12,
        RenderFormat::R16G16B16A16Typeless
        | RenderFormat::R16G16B16A16Float
        | RenderFormat::R16G16B16A16Unorm
        | RenderFormat::R16G16B16A16Uint
        | RenderFormat::R16G16B16A16Snorm
        | RenderFormat::R16G16B16A16Sint
        | RenderFormat::R32G32Typeless
        | RenderFormat::R32G32Float
        | RenderFormat::R32G32Uint
        | RenderFormat::R32G32Sint
        | RenderFormat::D32FloatS8Uint => 8,
        RenderFormat::R8G8B8A8Typeless
        | RenderFormat::R8G8B8A8Unorm
        | RenderFormat::R8G8B8A8Uint
        | RenderFormat::R8G8B8A8Snorm
        | RenderFormat::R8G8B8A8Sint
        | RenderFormat::B8G8R8A8Unorm
        | RenderFormat::R16G16Typeless
        | RenderFormat::R16G16Float
        | RenderFormat::R16G16Unorm
        | RenderFormat::R16G16Uint
        | RenderFormat::R16G16Snorm
        | RenderFormat::R16G16Sint
        | RenderFormat::R32Typeless
        | RenderFormat::D32Float
        | RenderFormat::R32Float
        | RenderFormat::R32Uint
        | RenderFormat::R32Sint => 4,
        RenderFormat::R8G8Typeless
        | RenderFormat::R8G8Unorm
        | RenderFormat::R8G8Uint
        | RenderFormat::R8G8Snorm
        | RenderFormat::R8G8Sint
        | RenderFormat::R16Typeless
        | RenderFormat::R16Float
        | RenderFormat::D16Unorm
        | RenderFormat::R16Unorm
        | RenderFormat::R16Uint
        | RenderFormat::R16Snorm
        | RenderFormat::R16Sint => 2,
        RenderFormat::R8Typeless
        | RenderFormat::R8Unorm
        | RenderFormat::R8Uint
        | RenderFormat::R8Snorm
        | RenderFormat::R8Sint => 1,
        RenderFormat::BC1Unorm
        | RenderFormat::BC1UnormSrgb
        | RenderFormat::BC1Typeless
        | RenderFormat::BC4Unorm
        | RenderFormat::BC4Snorm
        | RenderFormat::BC4Typeless => 8,
        RenderFormat::BC2Unorm
        | RenderFormat::BC2UnormSrgb
        | RenderFormat::BC2Typeless
        | RenderFormat::BC3Unorm
        | RenderFormat::BC3UnormSrgb
        | RenderFormat::BC3Typeless
        | RenderFormat::BC5Unorm
        | RenderFormat::BC5Snorm
        | RenderFormat::BC6HUf16
        | RenderFormat::BC6HSf16
        | RenderFormat::BC7Unorm
        | RenderFormat::BC7UnormSrgb => 16,
        _ => 1,
    }
}

pub const fn render_format_block_width(format: RenderFormat) -> u32 {
    match format {
        RenderFormat::BC1Typeless
        | RenderFormat::BC1Unorm
        | RenderFormat::BC1UnormSrgb
        | RenderFormat::BC2Typeless
        | RenderFormat::BC2Unorm
        | RenderFormat::BC2UnormSrgb
        | RenderFormat::BC3Typeless
        | RenderFormat::BC3Unorm
        | RenderFormat::BC3UnormSrgb
        | RenderFormat::BC4Typeless
        | RenderFormat::BC4Unorm
        | RenderFormat::BC4Snorm
        | RenderFormat::BC5Typeless
        | RenderFormat::BC5Unorm
        | RenderFormat::BC5Snorm
        | RenderFormat::BC6HTypeless
        | RenderFormat::BC6HUf16
        | RenderFormat::BC6HSf16
        | RenderFormat::BC7Typeless
        | RenderFormat::BC7Unorm
        | RenderFormat::BC7UnormSrgb => 4,
        _ => 1,
    }
}

pub const fn render_format_is_depth(format: RenderFormat) -> bool {
    matches!(format, RenderFormat::D16Unorm | RenderFormat::D32Float | RenderFormat::D32FloatS8Uint)
}

pub const fn render_format_is_stencil(format: RenderFormat) -> bool {
    matches!(format, RenderFormat::D32FloatS8Uint)
}

pub const fn render_texture_dimension_to_view(dimension: RenderTextureDimension) -> RenderTextureViewDimension {
    match dimension {
        RenderTextureDimension::Unknown => RenderTextureViewDimension::Unknown,
        RenderTextureDimension::Texture1D => RenderTextureViewDimension::Texture1D,
        RenderTextureDimension::Texture2D => RenderTextureViewDimension::Texture2D,
        RenderTextureDimension::Texture3D => RenderTextureViewDimension::Texture3D,
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl RenderColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderAffineTransform {
    pub m: [[f32; 4]; 3],
}

impl RenderAffineTransform {
    pub fn identity() -> Self {
        let mut transform = Self::default();
        transform.m[0][0] = 1.0;
        transform.m[1][1] = 1.0;
        transform.m[2][2] = 1.0;
        transform
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderDepth {
    pub depth: f32,
}

impl RenderDepth {
    pub fn new(depth: f32) -> Self {
        Self { depth }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderMultisamplingLocation {
    pub x: i8,
    pub y: i8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderMultisampling {
    pub sample_count: RenderSampleCounts,
    pub sample_locations: [RenderMultisamplingLocation; 16],
    pub sample_locations_enabled: bool,
}

impl Default for RenderMultisampling {
    fn default() -> Self {
        Self {
            sample_count: RenderSampleCount::COUNT_1,
            sample_locations: [RenderMultisamplingLocation::default(); 16],
            sample_locations_enabled: false,
        }
    }
}

impl RenderMultisampling {
    pub fn new(sample_count: RenderSampleCounts) -> Self {
        Self {
            sample_count,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderBufferReference {
    pub buffer: *const RenderBuffer,
    pub offset: u64,
}

impl RenderBufferReference {
    pub fn new(buffer: *const RenderBuffer, offset: u64) -> Self {
        Self { buffer, offset }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderBufferBarrier {
    pub buffer: *mut RenderBuffer,
    pub access_bits: RenderBufferAccessBits,
}

impl RenderBufferBarrier {
    pub fn new(buffer: *mut RenderBuffer, access_bits: RenderBufferAccessBits) -> Self {
        Self { buffer, access_bits }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderBufferStructuredView {
    pub structure_byte_stride: u32,
    pub first_element: u32,
}

impl RenderBufferStructuredView {
    pub fn new(structure_byte_stride: u32, first_element: u32) -> Self {
        Self {
            structure_byte_stride,
            first_element,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderTextureBarrier {
    pub texture: *mut RenderTexture,
    pub layout: RenderTextureLayout,
}

impl RenderTextureBarrier {
    pub fn new(texture: *mut RenderTexture, layout: RenderTextureLayout) -> Self {
        Self { texture, layout }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderClearValueData {
    Color(RenderColor),
    Depth(RenderDepth),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderClearValue {
    pub format: RenderFormat,
    pub data: RenderClearValueData,
}

impl RenderClearValue {
    pub fn color(color: RenderColor, format: RenderFormat) -> Self {
        Self {
            format,
            data: RenderClearValueData::Color(color),
        }
    }

    pub fn depth(depth: RenderDepth, format: RenderFormat) -> Self {
        Self {
            format,
            data: RenderClearValueData::Depth(depth),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderBufferDesc {
    pub size: u64,
    pub heap_type: RenderHeapType,
    pub flags: RenderBufferFlags,
    pub committed: bool,
}

impl RenderBufferDesc {
    pub fn default_buffer(size: u64, flags: RenderBufferFlags) -> Self {
        Self {
            size,
            heap_type: RenderHeapType::Default,
            flags,
            committed: false,
        }
    }

    pub fn upload_buffer(size: u64, flags: RenderBufferFlags) -> Self {
        Self {
            size,
            heap_type: RenderHeapType::Upload,
            flags,
            committed: false,
        }
    }

    pub fn readback_buffer(size: u64, flags: RenderBufferFlags) -> Self {
        Self {
            size,
            heap_type: RenderHeapType::Readback,
            flags,
            committed: false,
        }
    }

    pub fn vertex_buffer(size: u64, heap_type: RenderHeapType, flags: RenderBufferFlags) -> Self {
        Self {
            size,
            heap_type,
            flags: flags | RenderBufferFlag::VERTEX,
            committed: false,
        }
    }

    pub fn index_buffer(size: u64, heap_type: RenderHeapType, flags: RenderBufferFlags) -> Self {
        Self {
            size,
            heap_type,
            flags: flags | RenderBufferFlag::INDEX,
            committed: false,
        }
    }

    pub fn acceleration_structure_buffer(size: u64) -> Self {
        Self {
            size,
            heap_type: RenderHeapType::Default,
            flags: RenderBufferFlag::ACCELERATION_STRUCTURE,
            committed: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderTextureDesc {
    pub dimension: RenderTextureDimension,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32,
    pub array_size: u32,
    pub multisampling: RenderMultisampling,
    pub format: RenderFormat,
    pub texture_arrangement: RenderTextureArrangement,
    pub optimized_clear_value: Option<RenderClearValue>,
    pub flags: RenderTextureFlags,
    pub committed: bool,
}

impl RenderTextureDesc {
    pub fn texture(
        dimension: RenderTextureDimension,
        width: u32,
        height: u32,
        depth: u32,
        mip_levels: u32,
        array_size: u32,
        format: RenderFormat,
        flags: RenderTextureFlags,
    ) -> Self {
        Self {
            dimension,
            width,
            height,
            depth,
            mip_levels,
            array_size,
            format,
            flags,
            ..Default::default()
        }
    }

    pub fn texture_1d(width: u32, mip_levels: u32, format: RenderFormat, flags: RenderTextureFlags) -> Self {
        Self::texture(RenderTextureDimension::Texture1D, width, 1, 1, mip_levels, 1, format, flags)
    }

    pub fn texture_2d(width: u32, height: u32, mip_levels: u32, format: RenderFormat, flags: RenderTextureFlags) -> Self {
        Self::texture(RenderTextureDimension::Texture2D, width, height, 1, mip_levels, 1, format, flags)
    }

    pub fn texture_3d(width: u32, height: u32, depth: u32, mip_levels: u32, format: RenderFormat, flags: RenderTextureFlags) -> Self {
        Self::texture(RenderTextureDimension::Texture3D, width, height, depth, mip_levels, 1, format, flags)
    }

    pub fn color_target(
        width: u32,
        height: u32,
        format: RenderFormat,
        multisampling: RenderMultisampling,
        optimized_clear_value: Option<RenderClearValue>,
        flags: RenderTextureFlags,
    ) -> Self {
        Self {
            committed: true,
            dimension: RenderTextureDimension::Texture2D,
            width,
            height,
            depth: 1,
            mip_levels: 1,
            array_size: 1,
            format,
            multisampling,
            flags: flags | RenderTextureFlag::RENDER_TARGET,
            optimized_clear_value,
            ..Default::default()
        }
    }

    pub fn depth_target(
        width: u32,
        height: u32,
        format: RenderFormat,
        multisampling: RenderMultisampling,
        optimized_clear_value: Option<RenderClearValue>,
        flags: RenderTextureFlags,
    ) -> Self {
        Self {
            committed: true,
            dimension: RenderTextureDimension::Texture2D,
            width,
            height,
            depth: 1,
            mip_levels: 1,
            array_size: 1,
            format,
            multisampling,
            flags: flags | RenderTextureFlag::DEPTH_TARGET,
            optimized_clear_value,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RenderSwizzle {
    Identity = 0,
    Zero = 1,
    One = 2,
    R = 3,
    G = 4,
    B = 5,
    A = 6,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderComponentMapping {
    pub r: RenderSwizzle,
    pub g: RenderSwizzle,
    pub b: RenderSwizzle,
    pub a: RenderSwizzle,
}

impl RenderComponentMapping {
    pub fn new(r: RenderSwizzle, g: RenderSwizzle, b: RenderSwizzle, a: RenderSwizzle) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderTextureViewDesc {
    pub format: RenderFormat,
    pub dimension: RenderTextureViewDimension,
    pub mip_levels: u32,
    pub mip_slice: u32,
    pub array_size: u32,
    pub array_index: u32,
    pub component_mapping: RenderComponentMapping,
}

impl Default for RenderTextureViewDesc {
    fn default() -> Self {
        Self {
            format: RenderFormat::Unknown,
            dimension: RenderTextureViewDimension::Unknown,
            mip_levels: u32::MAX,
            mip_slice: 0,
            array_size: u32::MAX,
            array_index: 0,
            component_mapping: RenderComponentMapping::default(),
        }
    }
}

impl RenderTextureViewDesc {
    pub fn texture_1d(format: RenderFormat) -> Self {
        Self {
            format,
            dimension: RenderTextureViewDimension::Texture1D,
            ..Default::default()
        }
    }

    pub fn texture_2d(format: RenderFormat) -> Self {
        Self {
            format,
            dimension: RenderTextureViewDimension::Texture2D,
            ..Default::default()
        }
    }

    pub fn texture_3d(format: RenderFormat) -> Self {
        Self {
            format,
            dimension: RenderTextureViewDimension::Texture3D,
            ..Default::default()
        }
    }

    pub fn texture_cube(format: RenderFormat) -> Self {
        Self {
            format,
            dimension: RenderTextureViewDimension::TextureCube,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderAccelerationStructureDesc {
    pub structure_type: RenderAccelerationStructureType,
    pub buffer: RenderBufferReference,
    pub size: u64,
}

impl RenderAccelerationStructureDesc {
    pub fn new(structure_type: RenderAccelerationStructureType, buffer: RenderBufferReference, size: u64) -> Self {
        Self {
            structure_type,
            buffer,
            size,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderTextureCopyType {
    Unknown,
    Subresource,
    PlacedFootprint,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RenderTextureCopyData {
    PlacedFootprint {
        format: RenderFormat,
        width: u32,
        height: u32,
        depth: u32,
        row_width: u32,
        offset: u64,
    },
    Subresource {
        mip_level: u32,
        array_index: u32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderTextureCopyLocation {
    pub texture: *const RenderTexture,
    pub buffer: *const RenderBuffer,
    pub copy_type: RenderTextureCopyType,
    pub data: RenderTextureCopyData,
}

impl RenderTextureCopyLocation {
    pub fn placed_footprint(
        buffer: *const RenderBuffer,
        format: RenderFormat,
        width: u32,
        height: u32,
        depth: u32,
        row_width: u32,
        offset: u64,
    ) -> Self {
        Self {
            texture: std::ptr::null(),
            buffer,
            copy_type: RenderTextureCopyType::PlacedFootprint,
            data: RenderTextureCopyData::PlacedFootprint {
                format,
                width,
                height,
                depth,
                row_width,
                offset,
            },
        }
    }

    pub fn subresource(texture: *const RenderTexture, mip_level: u32, array_index: u32) -> Self {
        Self {
            texture,
            buffer: std::ptr::null(),
            copy_type: RenderTextureCopyType::Subresource,
            data: RenderTextureCopyData::Subresource {
                mip_level,
                array_index,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderPoolDesc {
    pub heap_type: RenderHeapType,
    pub min_block_count: u32,
    pub max_block_count: u32,
    pub use_linear_algorithm: bool,
    pub allow_only_buffers: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderSwapChainDesc {
    pub render_window: RenderWindow,
    pub format: RenderFormat,
    pub texture_count: u32,
    pub enable_present_wait: bool,
    pub max_frame_latency: u32,
}

impl RenderSwapChainDesc {
    pub fn new(
        render_window: RenderWindow,
        format: RenderFormat,
        texture_count: u32,
        enable_present_wait: bool,
        max_frame_latency: u32,
    ) -> Self {
        Self {
            render_window,
            format,
            texture_count,
            enable_present_wait,
            max_frame_latency,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderInputSlot {
    pub index: u32,
    pub stride: u32,
    pub classification: RenderInputSlotClassification,
}

impl RenderInputSlot {
    pub fn new(index: u32, stride: u32, classification: RenderInputSlotClassification) -> Self {
        Self {
            index,
            stride,
            classification,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderInputElement {
    pub semantic_name: *const std::ffi::c_char,
    pub semantic_index: u32,
    pub location: u32,
    pub format: RenderFormat,
    pub slot_index: u32,
    pub aligned_byte_offset: u32,
}

impl RenderInputElement {
    pub fn new(
        semantic_name: *const std::ffi::c_char,
        semantic_index: u32,
        location: u32,
        format: RenderFormat,
        slot_index: u32,
        aligned_byte_offset: u32,
    ) -> Self {
        Self {
            semantic_name,
            semantic_index,
            location,
            format,
            slot_index,
            aligned_byte_offset,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderBlendDesc {
    pub blend_enabled: bool,
    pub src_blend: RenderBlend,
    pub dst_blend: RenderBlend,
    pub blend_op: RenderBlendOperation,
    pub src_blend_alpha: RenderBlend,
    pub dst_blend_alpha: RenderBlend,
    pub blend_op_alpha: RenderBlendOperation,
    pub render_target_write_mask: u8,
}

impl RenderBlendDesc {
    pub fn copy() -> Self {
        Self {
            src_blend: RenderBlend::One,
            dst_blend: RenderBlend::Zero,
            blend_op: RenderBlendOperation::Add,
            src_blend_alpha: RenderBlend::One,
            dst_blend_alpha: RenderBlend::Zero,
            blend_op_alpha: RenderBlendOperation::Add,
            render_target_write_mask: RenderColorWriteEnable::ALL,
            ..Default::default()
        }
    }

    pub fn alpha_blend() -> Self {
        Self {
            blend_enabled: true,
            src_blend: RenderBlend::SrcAlpha,
            dst_blend: RenderBlend::InvSrcAlpha,
            blend_op: RenderBlendOperation::Add,
            src_blend_alpha: RenderBlend::One,
            dst_blend_alpha: RenderBlend::InvSrcAlpha,
            blend_op_alpha: RenderBlendOperation::Add,
            render_target_write_mask: RenderColorWriteEnable::ALL,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderStencilFaceDesc {
    pub pass_op: RenderStencilOp,
    pub fail_op: RenderStencilOp,
    pub depth_fail_op: RenderStencilOp,
    pub compare_function: RenderComparisonFunction,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderSpecConstant {
    pub index: u32,
    pub value: u32,
}

impl RenderSpecConstant {
    pub fn new(index: u32, value: u32) -> Self {
        Self { index, value }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderComputePipelineDesc {
    pub pipeline_layout: *const RenderPipelineLayout,
    pub compute_shader: *const RenderShader,
    pub spec_constants: *const RenderSpecConstant,
    pub spec_constants_count: u32,
    pub thread_group_size_x: u32,
    pub thread_group_size_y: u32,
    pub thread_group_size_z: u32,
}

impl RenderComputePipelineDesc {
    pub fn new(
        pipeline_layout: *const RenderPipelineLayout,
        compute_shader: *const RenderShader,
        thread_group_size_x: u32,
        thread_group_size_y: u32,
        thread_group_size_z: u32,
    ) -> Self {
        Self {
            pipeline_layout,
            compute_shader,
            thread_group_size_x,
            thread_group_size_y,
            thread_group_size_z,
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderGraphicsPipelineDesc {
    pub pipeline_layout: *const RenderPipelineLayout,
    pub vertex_shader: *const RenderShader,
    pub geometry_shader: *const RenderShader,
    pub pixel_shader: *const RenderShader,
    pub depth_function: RenderComparisonFunction,
    pub depth_clip_enabled: bool,
    pub depth_bias: i32,
    pub depth_bias_clamp: f32,
    pub slope_scaled_depth_bias: f32,
    pub dynamic_depth_bias_enabled: bool,
    pub depth_enabled: bool,
    pub depth_write_enabled: bool,
    pub stencil_enabled: bool,
    pub stencil_read_mask: u32,
    pub stencil_write_mask: u32,
    pub stencil_reference: u32,
    pub stencil_front_face: RenderStencilFaceDesc,
    pub stencil_back_face: RenderStencilFaceDesc,
    pub multisampling: RenderMultisampling,
    pub alpha_to_coverage_enabled: bool,
    pub primitive_topology: RenderPrimitiveTopology,
    pub cull_mode: RenderCullMode,
    pub front_face: RenderFrontFace,
    pub render_target_format: [RenderFormat; RenderGraphicsPipelineDesc::MAX_RENDER_TARGETS],
    pub render_target_blend: [RenderBlendDesc; RenderGraphicsPipelineDesc::MAX_RENDER_TARGETS],
    pub render_target_count: u32,
    pub logic_op_enabled: bool,
    pub logic_op: RenderLogicOperation,
    pub depth_target_format: RenderFormat,
    pub input_slots: *const RenderInputSlot,
    pub input_slots_count: u32,
    pub input_elements: *const RenderInputElement,
    pub input_elements_count: u32,
    pub spec_constants: *const RenderSpecConstant,
    pub spec_constants_count: u32,
}

impl RenderGraphicsPipelineDesc {
    pub const MAX_RENDER_TARGETS: usize = 8;
}

impl Default for RenderGraphicsPipelineDesc {
    fn default() -> Self {
        Self {
            pipeline_layout: std::ptr::null(),
            vertex_shader: std::ptr::null(),
            geometry_shader: std::ptr::null(),
            pixel_shader: std::ptr::null(),
            depth_function: RenderComparisonFunction::Never,
            depth_clip_enabled: false,
            depth_bias: 0,
            depth_bias_clamp: 0.0,
            slope_scaled_depth_bias: 0.0,
            dynamic_depth_bias_enabled: false,
            depth_enabled: false,
            depth_write_enabled: false,
            stencil_enabled: false,
            stencil_read_mask: 0xFFFFFFFF,
            stencil_write_mask: 0xFFFFFFFF,
            stencil_reference: 0,
            stencil_front_face: RenderStencilFaceDesc::default(),
            stencil_back_face: RenderStencilFaceDesc::default(),
            multisampling: RenderMultisampling::default(),
            alpha_to_coverage_enabled: false,
            primitive_topology: RenderPrimitiveTopology::TriangleList,
            cull_mode: RenderCullMode::None,
            front_face: RenderFrontFace::Clockwise,
            render_target_format: [RenderFormat::Unknown; RenderGraphicsPipelineDesc::MAX_RENDER_TARGETS],
            render_target_blend: [RenderBlendDesc::default(); RenderGraphicsPipelineDesc::MAX_RENDER_TARGETS],
            render_target_count: 0,
            logic_op_enabled: false,
            logic_op: RenderLogicOperation::Noop,
            depth_target_format: RenderFormat::Unknown,
            input_slots: std::ptr::null(),
            input_slots_count: 0,
            input_elements: std::ptr::null(),
            input_elements_count: 0,
            spec_constants: std::ptr::null(),
            spec_constants_count: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderRaytracingPipelineLibrarySymbol {
    pub import_name: *const std::ffi::c_char,
    pub symbol_type: RenderRaytracingPipelineLibrarySymbolType,
    pub export_name: *const std::ffi::c_char,
    pub spec_constants: *const RenderSpecConstant,
    pub spec_constants_count: u32,
}

impl RenderRaytracingPipelineLibrarySymbol {
    pub fn new(
        import_name: *const std::ffi::c_char,
        symbol_type: RenderRaytracingPipelineLibrarySymbolType,
        export_name: *const std::ffi::c_char,
        spec_constants: *const RenderSpecConstant,
        spec_constants_count: u32,
    ) -> Self {
        Self {
            import_name,
            symbol_type,
            export_name,
            spec_constants,
            spec_constants_count,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderRaytracingPipelineLibrary {
    pub shader: *const RenderShader,
    pub symbols: *const RenderRaytracingPipelineLibrarySymbol,
    pub symbols_count: u32,
}

impl RenderRaytracingPipelineLibrary {
    pub fn new(
        shader: *const RenderShader,
        symbols: *const RenderRaytracingPipelineLibrarySymbol,
        symbols_count: u32,
    ) -> Self {
        Self {
            shader,
            symbols,
            symbols_count,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderRaytracingPipelineHitGroup {
    pub hit_group_name: *const std::ffi::c_char,
    pub closest_hit_name: *const std::ffi::c_char,
    pub any_hit_name: *const std::ffi::c_char,
    pub intersection_name: *const std::ffi::c_char,
}

impl RenderRaytracingPipelineHitGroup {
    pub fn new(
        hit_group_name: *const std::ffi::c_char,
        closest_hit_name: *const std::ffi::c_char,
        any_hit_name: *const std::ffi::c_char,
        intersection_name: *const std::ffi::c_char,
    ) -> Self {
        Self {
            hit_group_name,
            closest_hit_name,
            any_hit_name,
            intersection_name,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderRaytracingPipelineDesc {
    pub libraries: *const RenderRaytracingPipelineLibrary,
    pub libraries_count: u32,
    pub hit_groups: *const RenderRaytracingPipelineHitGroup,
    pub hit_groups_count: u32,
    pub pipeline_layout: *const RenderPipelineLayout,
    pub max_payload_size: u32,
    pub max_attribute_size: u32,
    pub max_recursion_depth: u32,
    pub state_update_enabled: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderPipelineProgram {
    pub program_index: u32,
}

impl RenderPipelineProgram {
    pub fn new(program_index: u32) -> Self {
        Self { program_index }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderSamplerDesc {
    pub min_filter: RenderFilter,
    pub mag_filter: RenderFilter,
    pub mipmap_mode: RenderMipmapMode,
    pub address_u: RenderTextureAddressMode,
    pub address_v: RenderTextureAddressMode,
    pub address_w: RenderTextureAddressMode,
    pub mip_lod_bias: f32,
    pub max_anisotropy: u32,
    pub anisotropy_enabled: bool,
    pub comparison_func: RenderComparisonFunction,
    pub comparison_enabled: bool,
    pub border_color: RenderBorderColor,
    pub min_lod: f32,
    pub max_lod: f32,
    pub shader_visibility: RenderShaderVisibility,
}

impl Default for RenderSamplerDesc {
    fn default() -> Self {
        Self {
            min_filter: RenderFilter::Linear,
            mag_filter: RenderFilter::Linear,
            mipmap_mode: RenderMipmapMode::Linear,
            address_u: RenderTextureAddressMode::Wrap,
            address_v: RenderTextureAddressMode::Wrap,
            address_w: RenderTextureAddressMode::Wrap,
            mip_lod_bias: 0.0,
            max_anisotropy: 16,
            anisotropy_enabled: false,
            comparison_func: RenderComparisonFunction::Never,
            comparison_enabled: false,
            border_color: RenderBorderColor::OpaqueBlack,
            min_lod: 0.0,
            max_lod: f32::MAX,
            shader_visibility: RenderShaderVisibility::All,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderDescriptorRange {
    pub range_type: RenderDescriptorRangeType,
    pub count: u32,
    pub binding: u32,
    pub immutable_sampler: *const *const RenderSampler,
}

impl RenderDescriptorRange {
    pub fn new(
        range_type: RenderDescriptorRangeType,
        binding: u32,
        count: u32,
        immutable_sampler: *const *const RenderSampler,
    ) -> Self {
        Self {
            range_type,
            count,
            binding,
            immutable_sampler,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderDescriptorSetDesc {
    pub descriptor_ranges: *const RenderDescriptorRange,
    pub descriptor_ranges_count: u32,
    pub last_range_is_boundless: bool,
    pub boundless_range_size: u32,
}

impl RenderDescriptorSetDesc {
    pub fn new(
        descriptor_ranges: *const RenderDescriptorRange,
        descriptor_ranges_count: u32,
        last_range_is_boundless: bool,
        boundless_range_size: u32,
    ) -> Self {
        Self {
            descriptor_ranges,
            descriptor_ranges_count,
            last_range_is_boundless,
            boundless_range_size,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderPushConstantRange {
    pub binding: u32,
    pub set: u32,
    pub offset: u32,
    pub size: u32,
    pub stage_flags: RenderShaderStageFlags,
}

impl RenderPushConstantRange {
    pub fn new(binding: u32, set: u32, offset: u32, size: u32, stage_flags: RenderShaderStageFlags) -> Self {
        Self {
            binding,
            set,
            offset,
            size,
            stage_flags,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderRootDescriptorDesc {
    pub shader_register: u32,
    pub register_space: u32,
    pub descriptor_type: RenderRootDescriptorType,
}

impl RenderRootDescriptorDesc {
    pub fn new(shader_register: u32, register_space: u32, descriptor_type: RenderRootDescriptorType) -> Self {
        Self {
            shader_register,
            register_space,
            descriptor_type,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderPipelineLayoutDesc {
    pub push_constant_ranges: *const RenderPushConstantRange,
    pub push_constant_ranges_count: u32,
    pub descriptor_set_descs: *const RenderDescriptorSetDesc,
    pub descriptor_set_descs_count: u32,
    pub root_descriptor_descs: *const RenderRootDescriptorDesc,
    pub root_descriptor_descs_count: u32,
    pub is_local: bool,
    pub allow_input_layout: bool,
}

impl RenderPipelineLayoutDesc {
    pub fn new(
        push_constant_ranges: *const RenderPushConstantRange,
        push_constant_ranges_count: u32,
        descriptor_set_descs: *const RenderDescriptorSetDesc,
        descriptor_set_descs_count: u32,
        is_local: bool,
        allow_input_layout: bool,
    ) -> Self {
        Self {
            push_constant_ranges,
            push_constant_ranges_count,
            descriptor_set_descs,
            descriptor_set_descs_count,
            root_descriptor_descs: std::ptr::null(),
            root_descriptor_descs_count: 0,
            is_local,
            allow_input_layout,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderIndexBufferView {
    pub buffer: RenderBufferReference,
    pub size: u32,
    pub format: RenderFormat,
}

impl RenderIndexBufferView {
    pub fn new(buffer: RenderBufferReference, size: u32, format: RenderFormat) -> Self {
        Self { buffer, size, format }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderVertexBufferView {
    pub buffer: RenderBufferReference,
    pub size: u32,
}

impl RenderVertexBufferView {
    pub fn new(buffer: RenderBufferReference, size: u32) -> Self {
        Self { buffer, size }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl RenderViewport {
    pub fn new(x: f32, y: f32, width: f32, height: f32, min_depth: f32, max_depth: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            min_depth,
            max_depth,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl RenderRect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderBox {
    pub left: i32,
    pub top: i32,
    pub front: i32,
    pub right: i32,
    pub bottom: i32,
    pub back: i32,
}

impl RenderBox {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32, front: i32, back: i32) -> Self {
        Self {
            left,
            top,
            front,
            right,
            bottom,
            back,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderRange {
    pub begin: u64,
    pub end: u64,
}

impl RenderRange {
    pub fn new(begin: u64, end: u64) -> Self {
        Self { begin, end }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderFramebufferDesc {
    pub color_attachments: *const *const RenderTexture,
    pub color_attachment_views: *const *const RenderTextureView,
    pub color_attachments_count: u32,
    pub depth_attachment: *const RenderTexture,
    pub depth_attachment_view: *const RenderTextureView,
    pub depth_attachment_read_only: bool,
}

impl RenderFramebufferDesc {
    pub fn new(
        color_attachments: *const *const RenderTexture,
        color_attachments_count: u32,
        depth_attachment: *const RenderTexture,
        depth_attachment_read_only: bool,
    ) -> Self {
        Self {
            color_attachments,
            color_attachment_views: std::ptr::null(),
            color_attachments_count,
            depth_attachment,
            depth_attachment_view: std::ptr::null(),
            depth_attachment_read_only,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderBottomLevelASMesh {
    pub index_buffer: RenderBufferReference,
    pub vertex_buffer: RenderBufferReference,
    pub index_format: RenderFormat,
    pub vertex_format: RenderFormat,
    pub index_count: u32,
    pub vertex_count: u32,
    pub vertex_stride: u32,
    pub is_opaque: bool,
}

impl RenderBottomLevelASMesh {
    pub fn new(
        index_buffer: RenderBufferReference,
        vertex_buffer: RenderBufferReference,
        index_format: RenderFormat,
        vertex_format: RenderFormat,
        index_count: u32,
        vertex_count: u32,
        vertex_stride: u32,
        is_opaque: bool,
    ) -> Self {
        Self {
            index_buffer,
            vertex_buffer,
            index_format,
            vertex_format,
            index_count,
            vertex_count,
            vertex_stride,
            is_opaque,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderBottomLevelASBuildInfo {
    pub mesh_count: u32,
    pub primitive_count: u32,
    pub prefer_fast_build: bool,
    pub prefer_fast_trace: bool,
    pub scratch_size: u64,
    pub acceleration_structure_size: u64,
    pub build_data: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderTopLevelASInstance {
    pub bottom_level_as: RenderBufferReference,
    pub instance_id: u32,
    pub instance_mask: u32,
    pub instance_contribution_to_hit_group_index: u32,
    pub cull_disable: bool,
    pub transform: RenderAffineTransform,
}

impl RenderTopLevelASInstance {
    pub fn new(
        bottom_level_as: RenderBufferReference,
        instance_id: u32,
        instance_mask: u32,
        instance_contribution_to_hit_group_index: u32,
        cull_disable: bool,
        transform: RenderAffineTransform,
    ) -> Self {
        Self {
            bottom_level_as,
            instance_id,
            instance_mask,
            instance_contribution_to_hit_group_index,
            cull_disable,
            transform,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderTopLevelASBuildInfo {
    pub instances_buffer_data: Vec<u8>,
    pub instance_count: u32,
    pub prefer_fast_build: bool,
    pub prefer_fast_trace: bool,
    pub scratch_size: u64,
    pub acceleration_structure_size: u64,
    pub build_data: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderShaderBindingGroup {
    pub pipeline_programs: *const RenderPipelineProgram,
    pub pipeline_programs_count: u32,
}

impl RenderShaderBindingGroup {
    pub fn new(pipeline_programs: *const RenderPipelineProgram, pipeline_programs_count: u32) -> Self {
        Self {
            pipeline_programs,
            pipeline_programs_count,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderShaderBindingGroups {
    pub ray_gen: RenderShaderBindingGroup,
    pub miss: RenderShaderBindingGroup,
    pub hit_group: RenderShaderBindingGroup,
    pub callable: RenderShaderBindingGroup,
}

impl RenderShaderBindingGroups {
    pub fn new(
        ray_gen: RenderShaderBindingGroup,
        miss: RenderShaderBindingGroup,
        hit_group: RenderShaderBindingGroup,
        callable: RenderShaderBindingGroup,
    ) -> Self {
        Self {
            ray_gen,
            miss,
            hit_group,
            callable,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderShaderBindingGroupInfo {
    pub offset: u64,
    pub size: u64,
    pub stride: u32,
    pub start_index: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderShaderBindingGroupsInfo {
    pub ray_gen: RenderShaderBindingGroupInfo,
    pub miss: RenderShaderBindingGroupInfo,
    pub hit_group: RenderShaderBindingGroupInfo,
    pub callable: RenderShaderBindingGroupInfo,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderShaderBindingTableInfo {
    pub table_buffer_data: Vec<u8>,
    pub groups: RenderShaderBindingGroupsInfo,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderDeviceDescription {
    pub name: String,
    pub device_type: RenderDeviceType,
    pub vendor: RenderDeviceVendor,
    pub driver_version: u64,
    pub dedicated_video_memory: u64,
}

impl Default for RenderDeviceDescription {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            device_type: RenderDeviceType::Unknown,
            vendor: RenderDeviceVendor::Unknown,
            driver_version: 0,
            dedicated_video_memory: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderDeviceCapabilities {
    pub geometry_shader: bool,
    pub raytracing: bool,
    pub raytracing_state_update: bool,
    pub sample_locations: bool,
    pub resolve_region: bool,
    pub resolve_modes: bool,
    pub descriptor_indexing: bool,
    pub scalar_block_layout: bool,
    pub buffer_device_address: bool,
    pub sampler_mirror_clamp_to_edge: bool,
    pub present_wait: bool,
    pub display_timing: bool,
    pub max_texture_size: u64,
    pub prefer_hdr: bool,
    pub triangle_fan: bool,
    pub dynamic_depth_bias: bool,
    pub uma: bool,
    pub gpu_upload_heap: bool,
    pub query_pools: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderInterfaceCapabilities {
    pub shader_format: RenderShaderFormat,
}
