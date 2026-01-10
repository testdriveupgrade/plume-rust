//
// plume
//
// Copyright (c) 2024 renderbag and contributors. All rights reserved.
// Licensed under the MIT license. See LICENSE file for details.
//

#![allow(dead_code, unused_variables)]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Mutex;

pub mod d3d12 {
    pub type DescriptorHeap = u64;
    pub type Resource = u64;
    pub type CommandQueue = u64;
    pub type CommandAllocator = u64;
    pub type GraphicsCommandList = u64;
    pub type GraphicsCommandList1 = u64;
    pub type GraphicsCommandList4 = u64;
    pub type GraphicsCommandList9 = u64;
    pub type Fence = u64;
    pub type PipelineState = u64;
    pub type RootSignature = u64;
    pub type StateObject = u64;
    pub type StateObjectProperties = u64;
    pub type Device8 = u64;

    pub type PrimitiveTopology = u32;
    pub const D3D_PRIMITIVE_TOPOLOGY_UNDEFINED: PrimitiveTopology = 0;

    #[derive(Clone, Copy, Default)]
    pub struct CpuDescriptorHandle;
    #[derive(Clone, Copy, Default)]
    pub struct GpuDescriptorHandle;
    #[derive(Clone, Copy, Default)]
    pub struct ShaderResourceViewDesc;
    #[derive(Clone, Copy, Default)]
    pub struct UnorderedAccessViewDesc;
    #[derive(Clone, Copy, Default)]
    pub struct SamplerDesc;

    pub type DescriptorHeapType = u32;
    pub type ResourceStates = u32;
    pub const D3D12_RESOURCE_STATE_COMMON: ResourceStates = 0;
    pub const D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: u32 = 0;

    pub type ShaderModel = u32;
}

pub mod dxgi {
    pub type Format = u32;
    pub type SwapChain3 = u64;
    pub type Factory4 = u64;
    pub type Adapter1 = u64;

    pub const DXGI_FORMAT_UNKNOWN: Format = 0;
}

pub mod d3d12ma {
    pub type Allocation = u64;
    pub type Allocator = u64;
    pub type Pool = u64;
}

pub type Handle = u64;

#[derive(Clone, Copy, Default)]
pub struct RenderBufferDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderTextureDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderTextureViewDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderDescriptorSetDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderPipelineLayoutDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderComputePipelineDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderGraphicsPipelineDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderRaytracingPipelineDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderSwapChainDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderFramebufferDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderPoolDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderSamplerDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderBufferBarrier;
#[derive(Clone, Copy, Default)]
pub struct RenderTextureBarrier;
#[derive(Clone, Copy, Default)]
pub struct RenderTextureCopyLocation;
#[derive(Clone, Copy, Default)]
pub struct RenderBufferReference;
#[derive(Clone, Copy, Default)]
pub struct RenderRange;
#[derive(Clone, Copy, Default)]
pub struct RenderViewport;
#[derive(Clone, Copy, Default)]
pub struct RenderRect;
#[derive(Clone, Copy, Default)]
pub struct RenderColor;
#[derive(Clone, Copy, Default)]
pub struct RenderBottomLevelASBuildInfo;
#[derive(Clone, Copy, Default)]
pub struct RenderBottomLevelASMesh;
#[derive(Clone, Copy, Default)]
pub struct RenderTopLevelASBuildInfo;
#[derive(Clone, Copy, Default)]
pub struct RenderTopLevelASInstance;
#[derive(Clone, Copy, Default)]
pub struct RenderShaderBindingGroupsInfo;
#[derive(Clone, Copy, Default)]
pub struct RenderShaderBindingGroups;
#[derive(Clone, Copy, Default)]
pub struct RenderShaderBindingTableInfo;
#[derive(Clone, Copy, Default)]
pub struct RenderDeviceCapabilities;
#[derive(Clone, Copy, Default)]
pub struct RenderDeviceDescription;
#[derive(Clone, Copy, Default)]
pub struct RenderInterfaceCapabilities;
#[derive(Clone, Copy, Default)]
pub struct RenderPipelineProgram;
#[derive(Clone, Copy, Default)]
pub struct RenderIndexBufferView;
#[derive(Clone, Copy, Default)]
pub struct RenderVertexBufferView;
#[derive(Clone, Copy, Default)]
pub struct RenderInputSlot;
#[derive(Clone, Copy, Default)]
pub struct RenderResolveMode;
#[derive(Clone, Copy, Default)]
pub struct RenderWindow;
#[derive(Clone, Copy, Default)]
pub struct RenderShaderFormat;
#[derive(Clone, Copy, Default)]
pub struct RenderSampleCounts;
#[derive(Clone, Copy, Default)]
pub struct RenderPushConstantRange;

#[derive(Clone, Copy, Default)]
pub enum RenderBarrierStage {
    #[default]
    None,
}

pub type RenderBarrierStages = RenderBarrierStage;

#[derive(Clone, Copy, Default)]
pub enum RenderTextureLayout {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Default)]
pub enum RenderAccelerationStructureType {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Default)]
pub enum RenderCommandListType {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Default)]
pub enum RenderDescriptorRangeType {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Default)]
pub enum RenderTextureViewDimension {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Default)]
pub enum RenderBorderColor {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Default)]
pub enum RenderShaderVisibility {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Default)]
pub enum RenderRootDescriptorType {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Default)]
pub enum RenderFormat {
    #[default]
    Unknown,
}

pub struct RenderBuffer;
pub struct RenderBufferStructuredView;
pub struct RenderBufferFormattedView;
pub struct RenderTexture;
pub struct RenderTextureView;
pub struct RenderAccelerationStructure;
pub struct RenderDescriptorSet;
pub struct RenderPipelineLayout;
pub struct RenderPipeline;
pub struct RenderSampler;
pub struct RenderShader;
pub struct RenderSwapChain;
pub struct RenderFramebuffer;
pub struct RenderQueryPool;
pub struct RenderCommandList;
pub struct RenderCommandFence;
pub struct RenderCommandSemaphore;
pub struct RenderCommandQueue;
pub struct RenderPool;
pub struct RenderDevice;
pub struct RenderInterface;

const SHADER_DESCRIPTOR_HEAP_SIZE: u32 = 65536;
const SAMPLER_DESCRIPTOR_HEAP_SIZE: u32 = 1024;
const TARGET_DESCRIPTOR_HEAP_SIZE: u32 = 16384;

fn utf8_to_utf16(value: &str) -> Vec<u16> {
    value.encode_utf16().collect()
}

fn utf16_to_utf8(value: &[u16]) -> String {
    String::from_utf16_lossy(value)
}

fn round_up_u32(value: u32, power_of_2_alignment: u32) -> u32 {
    (value + power_of_2_alignment - 1) & !(power_of_2_alignment - 1)
}

fn round_up_u64(value: u64, power_of_2_alignment: u64) -> u64 {
    (value + power_of_2_alignment - 1) & !(power_of_2_alignment - 1)
}

fn to_dxgi(format: RenderFormat) -> dxgi::Format {
    todo!("map RenderFormat to DXGI format")
}

pub struct D3D12DescriptorHeapAllocator {
    pub heap: d3d12::DescriptorHeap,
    pub heap_size: u32,
    pub free_size: u32,
    pub device: *mut D3D12Device,
    pub cpu_descriptor_handle: d3d12::CpuDescriptorHandle,
    pub gpu_descriptor_handle: d3d12::GpuDescriptorHandle,
    pub descriptor_handle_increment: u32,
    pub offset_free_block_map: BTreeMap<u32, FreeBlock>,
    pub size_free_block_map: BTreeMap<u32, Vec<u32>>,
    pub allocation_mutex: Mutex<()>,
}

impl D3D12DescriptorHeapAllocator {
    pub const INVALID_OFFSET: u32 = 0xFFFFFFFF;

    pub fn new(device: *mut D3D12Device, heap_size: u32, heap_type: d3d12::DescriptorHeapType) -> Self {
        Self {
            heap: 0,
            heap_size,
            free_size: 0,
            device,
            cpu_descriptor_handle: d3d12::CpuDescriptorHandle::default(),
            gpu_descriptor_handle: d3d12::GpuDescriptorHandle::default(),
            descriptor_handle_increment: 0,
            offset_free_block_map: BTreeMap::new(),
            size_free_block_map: BTreeMap::new(),
            allocation_mutex: Mutex::new(()),
        }
    }

    pub fn add_free_block(&mut self, offset: u32, size: u32) {
        todo!("add free block")
    }

    pub fn allocate(&mut self, size: u32) -> u32 {
        todo!("allocate descriptor heap range")
    }

    pub fn free(&mut self, offset: u32, size: u32) {
        todo!("free descriptor heap range")
    }

    pub fn get_cpu_handle_at(&self, index: u32) -> d3d12::CpuDescriptorHandle {
        todo!("get CPU descriptor handle")
    }

    pub fn get_gpu_handle_at(&self, index: u32) -> d3d12::GpuDescriptorHandle {
        todo!("get GPU descriptor handle")
    }
}

pub struct FreeBlock {
    pub size: u32,
}

impl FreeBlock {
    pub fn new(size: u32) -> Self {
        Self { size }
    }
}

pub struct D3D12DescriptorSet {
    pub device: *mut D3D12Device,
    pub view_allocation: HeapAllocation,
    pub sampler_allocation: HeapAllocation,
    pub descriptor_types: Vec<RenderDescriptorRangeType>,
    pub descriptor_heap_indices: Vec<u32>,
    pub descriptor_type_max_index: u32,
}

#[derive(Clone, Copy, Default)]
pub struct HeapAllocation {
    pub offset: u32,
    pub count: u32,
}

impl D3D12DescriptorSet {
    pub fn new(device: *mut D3D12Device, desc: &RenderDescriptorSetDesc) -> Self {
        Self {
            device,
            view_allocation: HeapAllocation::default(),
            sampler_allocation: HeapAllocation::default(),
            descriptor_types: Vec::new(),
            descriptor_heap_indices: Vec::new(),
            descriptor_type_max_index: 0,
        }
    }

    pub fn set_buffer(
        &mut self,
        descriptor_index: u32,
        buffer: *const RenderBuffer,
        buffer_size: u64,
        buffer_structured_view: *const RenderBufferStructuredView,
        buffer_formatted_view: *const RenderBufferFormattedView,
    ) {
        todo!("set D3D12 descriptor buffer")
    }

    pub fn set_texture(
        &mut self,
        descriptor_index: u32,
        texture: *const RenderTexture,
        texture_layout: RenderTextureLayout,
        texture_view: *const RenderTextureView,
    ) {
        todo!("set D3D12 descriptor texture")
    }

    pub fn set_sampler(&mut self, descriptor_index: u32, sampler: *const RenderSampler) {
        todo!("set D3D12 descriptor sampler")
    }

    pub fn set_acceleration_structure(
        &mut self,
        descriptor_index: u32,
        acceleration_structure: *const RenderAccelerationStructure,
    ) {
        todo!("set D3D12 descriptor acceleration structure")
    }

    pub fn set_srv(
        &mut self,
        descriptor_index: u32,
        resource: d3d12::Resource,
        view_desc: *const d3d12::ShaderResourceViewDesc,
    ) {
        todo!("set D3D12 SRV")
    }

    pub fn set_uav(
        &mut self,
        descriptor_index: u32,
        resource: d3d12::Resource,
        view_desc: *const d3d12::UnorderedAccessViewDesc,
    ) {
        todo!("set D3D12 UAV")
    }

    pub fn set_cbv(&mut self, descriptor_index: u32, resource: d3d12::Resource, buffer_size: u64) {
        todo!("set D3D12 CBV")
    }
}

pub struct D3D12SwapChain {
    pub desc: RenderSwapChainDesc,
    pub d3d: dxgi::SwapChain3,
    pub waitable_object: Handle,
    pub command_queue: *mut D3D12CommandQueue,
    pub textures: Vec<D3D12Texture>,
    pub native_format: dxgi::Format,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub vsync_enabled: bool,
    pub swap_chain_flags: u32,
}

impl D3D12SwapChain {
    pub fn new(command_queue: *mut D3D12CommandQueue, desc: RenderSwapChainDesc) -> Self {
        Self {
            desc,
            d3d: 0,
            waitable_object: 0,
            command_queue,
            textures: Vec::new(),
            native_format: dxgi::DXGI_FORMAT_UNKNOWN,
            width: 0,
            height: 0,
            refresh_rate: 0,
            vsync_enabled: true,
            swap_chain_flags: 0,
        }
    }

    pub fn present(
        &mut self,
        texture_index: u32,
        wait_semaphores: *mut *mut RenderCommandSemaphore,
        wait_semaphore_count: u32,
    ) -> bool {
        todo!("present D3D12 swapchain")
    }

    pub fn wait(&mut self) {
        todo!("wait D3D12 swapchain")
    }

    pub fn resize(&mut self) -> bool {
        todo!("resize D3D12 swapchain")
    }

    pub fn needs_resize(&self) -> bool {
        todo!("check D3D12 swapchain resize")
    }

    pub fn set_vsync_enabled(&mut self, vsync_enabled: bool) {
        self.vsync_enabled = vsync_enabled;
    }

    pub fn is_vsync_enabled(&self) -> bool {
        self.vsync_enabled
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_texture(&mut self, texture_index: u32) -> *mut RenderTexture {
        todo!("get D3D12 swapchain texture")
    }

    pub fn get_texture_count(&self) -> u32 {
        self.textures.len() as u32
    }

    pub fn acquire_texture(
        &mut self,
        signal_semaphore: *mut RenderCommandSemaphore,
        texture_index: &mut u32,
    ) -> bool {
        todo!("acquire D3D12 swapchain texture")
    }

    pub fn get_window(&self) -> RenderWindow {
        todo!("get D3D12 swapchain window")
    }

    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }

    pub fn get_refresh_rate(&self) -> u32 {
        self.refresh_rate
    }

    pub fn get_window_size(&self, dst_width: &mut u32, dst_height: &mut u32) {
        *dst_width = self.width;
        *dst_height = self.height;
    }

    pub fn set_textures(&mut self) {
        todo!("set D3D12 swapchain textures")
    }
}

pub struct D3D12Framebuffer {
    pub device: *mut D3D12Device,
    pub width: u32,
    pub height: u32,
    pub color_targets: Vec<*const D3D12Texture>,
    pub depth_target: *const D3D12Texture,
    pub color_handles: Vec<d3d12::CpuDescriptorHandle>,
    pub color_target_allocator_offsets: Vec<u32>,
    pub depth_handle: d3d12::CpuDescriptorHandle,
    pub depth_target_allocator_offset: u32,
}

impl D3D12Framebuffer {
    pub fn new(device: *mut D3D12Device, desc: &RenderFramebufferDesc) -> Self {
        Self {
            device,
            width: 0,
            height: 0,
            color_targets: Vec::new(),
            depth_target: std::ptr::null(),
            color_handles: Vec::new(),
            color_target_allocator_offsets: Vec::new(),
            depth_handle: d3d12::CpuDescriptorHandle::default(),
            depth_target_allocator_offset: D3D12DescriptorHeapAllocator::INVALID_OFFSET,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn create_render_target_heap(&mut self, texture: *const D3D12Texture, texture_view: *const D3D12TextureView) {
        todo!("create D3D12 render target heap")
    }

    pub fn create_depth_stencil_heap(
        &mut self,
        texture: *const D3D12Texture,
        texture_view: *const D3D12TextureView,
        read_only: bool,
    ) {
        todo!("create D3D12 depth stencil heap")
    }

    pub fn release_target_heap(&mut self) {
        todo!("release D3D12 target heap")
    }
}

pub struct D3D12QueryPool {
    pub device: *mut D3D12Device,
    pub d3d: Handle,
    pub results: Vec<u64>,
    pub readback_buffer: Option<Box<RenderBuffer>>,
}

impl D3D12QueryPool {
    pub fn new(device: *mut D3D12Device, query_count: u32) -> Self {
        Self {
            device,
            d3d: 0,
            results: vec![0; query_count as usize],
            readback_buffer: None,
        }
    }

    pub fn query_results(&mut self) {
        todo!("query D3D12 timestamps")
    }

    pub fn get_results(&self) -> &[u64] {
        &self.results
    }

    pub fn get_count(&self) -> u32 {
        self.results.len() as u32
    }
}

pub struct D3D12CommandList {
    pub d3d: d3d12::GraphicsCommandList,
    pub d3d_v1: d3d12::GraphicsCommandList1,
    pub d3d_v4: d3d12::GraphicsCommandList4,
    pub d3d_v9: d3d12::GraphicsCommandList9,
    pub command_allocator: d3d12::CommandAllocator,
    pub queue: *mut D3D12CommandQueue,
    pub target_framebuffer: *const D3D12Framebuffer,
    pub target_framebuffer_sample_positions_set: bool,
    pub open: bool,
    pub active_compute_pipeline_layout: *const D3D12PipelineLayout,
    pub active_graphics_pipeline_layout: *const D3D12PipelineLayout,
    pub active_graphics_pipeline: *const D3D12GraphicsPipeline,
    pub descriptor_heaps_set: bool,
    pub active_topology: d3d12::PrimitiveTopology,
    pub active_stencil_ref: u32,
    pub active_sample_positions: bool,
}

impl D3D12CommandList {
    pub fn new(queue: *mut D3D12CommandQueue) -> Self {
        Self {
            d3d: 0,
            d3d_v1: 0,
            d3d_v4: 0,
            d3d_v9: 0,
            command_allocator: 0,
            queue,
            target_framebuffer: std::ptr::null(),
            target_framebuffer_sample_positions_set: false,
            open: false,
            active_compute_pipeline_layout: std::ptr::null(),
            active_graphics_pipeline_layout: std::ptr::null(),
            active_graphics_pipeline: std::ptr::null(),
            descriptor_heaps_set: false,
            active_topology: d3d12::D3D_PRIMITIVE_TOPOLOGY_UNDEFINED,
            active_stencil_ref: 0,
            active_sample_positions: false,
        }
    }

    pub fn begin(&mut self) {
        todo!("begin D3D12 command list")
    }

    pub fn end(&mut self) {
        todo!("end D3D12 command list")
    }

    pub fn barriers(
        &mut self,
        stages: RenderBarrierStages,
        buffer_barriers: *const RenderBufferBarrier,
        buffer_barriers_count: u32,
        texture_barriers: *const RenderTextureBarrier,
        texture_barriers_count: u32,
    ) {
        todo!("record D3D12 barriers")
    }

    pub fn dispatch(&mut self, thread_group_count_x: u32, thread_group_count_y: u32, thread_group_count_z: u32) {
        todo!("dispatch D3D12 compute")
    }

    pub fn trace_rays(
        &mut self,
        width: u32,
        height: u32,
        depth: u32,
        shader_binding_table: RenderBufferReference,
        shader_binding_groups_info: &RenderShaderBindingGroupsInfo,
    ) {
        todo!("trace D3D12 rays")
    }

    pub fn draw_instanced(
        &mut self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    ) {
        todo!("draw D3D12 instanced")
    }

    pub fn draw_indexed_instanced(
        &mut self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    ) {
        todo!("draw D3D12 indexed instanced")
    }

    pub fn set_pipeline(&mut self, pipeline: *const RenderPipeline) {
        todo!("set D3D12 pipeline")
    }

    pub fn set_compute_pipeline_layout(&mut self, pipeline_layout: *const RenderPipelineLayout) {
        todo!("set D3D12 compute pipeline layout")
    }

    pub fn set_compute_push_constants(&mut self, range_index: u32, data: *const u8, offset: u32, size: u32) {
        todo!("set D3D12 compute push constants")
    }

    pub fn set_compute_descriptor_set(&mut self, descriptor_set: *mut RenderDescriptorSet, set_index: u32) {
        todo!("set D3D12 compute descriptor set")
    }

    pub fn set_graphics_pipeline_layout(&mut self, pipeline_layout: *const RenderPipelineLayout) {
        todo!("set D3D12 graphics pipeline layout")
    }

    pub fn set_graphics_push_constants(&mut self, range_index: u32, data: *const u8, offset: u32, size: u32) {
        todo!("set D3D12 graphics push constants")
    }

    pub fn set_graphics_descriptor_set(&mut self, descriptor_set: *mut RenderDescriptorSet, set_index: u32) {
        todo!("set D3D12 graphics descriptor set")
    }

    pub fn set_graphics_root_descriptor(&mut self, buffer_reference: RenderBufferReference, root_descriptor_index: u32) {
        todo!("set D3D12 graphics root descriptor")
    }

    pub fn set_raytracing_pipeline_layout(&mut self, pipeline_layout: *const RenderPipelineLayout) {
        todo!("set D3D12 raytracing pipeline layout")
    }

    pub fn set_raytracing_push_constants(&mut self, range_index: u32, data: *const u8, offset: u32, size: u32) {
        todo!("set D3D12 raytracing push constants")
    }

    pub fn set_raytracing_descriptor_set(&mut self, descriptor_set: *mut RenderDescriptorSet, set_index: u32) {
        todo!("set D3D12 raytracing descriptor set")
    }

    pub fn set_index_buffer(&mut self, view: *const RenderIndexBufferView) {
        todo!("set D3D12 index buffer")
    }

    pub fn set_vertex_buffers(
        &mut self,
        start_slot: u32,
        views: *const RenderVertexBufferView,
        view_count: u32,
        input_slots: *const RenderInputSlot,
    ) {
        todo!("set D3D12 vertex buffers")
    }

    pub fn set_viewports(&mut self, viewports: *const RenderViewport, count: u32) {
        todo!("set D3D12 viewports")
    }

    pub fn set_scissors(&mut self, scissor_rects: *const RenderRect, count: u32) {
        todo!("set D3D12 scissors")
    }

    pub fn set_framebuffer(&mut self, framebuffer: *const RenderFramebuffer) {
        todo!("set D3D12 framebuffer")
    }

    pub fn set_depth_bias(&mut self, depth_bias: f32, depth_bias_clamp: f32, slope_scaled_depth_bias: f32) {
        todo!("set D3D12 depth bias")
    }

    pub fn clear_color(
        &mut self,
        attachment_index: u32,
        color_value: RenderColor,
        clear_rects: *const RenderRect,
        clear_rects_count: u32,
    ) {
        todo!("clear D3D12 color")
    }

    pub fn clear_depth_stencil(
        &mut self,
        clear_depth: bool,
        clear_stencil: bool,
        depth_value: f32,
        stencil_value: u32,
        clear_rects: *const RenderRect,
        clear_rects_count: u32,
    ) {
        todo!("clear D3D12 depth stencil")
    }

    pub fn copy_buffer_region(&mut self, dst_buffer: RenderBufferReference, src_buffer: RenderBufferReference, size: u64) {
        todo!("copy D3D12 buffer region")
    }

    pub fn copy_texture_region(
        &mut self,
        dst_location: RenderTextureCopyLocation,
        src_location: RenderTextureCopyLocation,
        dst_x: u32,
        dst_y: u32,
        dst_z: u32,
        src_box: *const RenderBox,
    ) {
        todo!("copy D3D12 texture region")
    }

    pub fn copy_buffer(&mut self, dst_buffer: *const RenderBuffer, src_buffer: *const RenderBuffer) {
        todo!("copy D3D12 buffer")
    }

    pub fn copy_texture(&mut self, dst_texture: *const RenderTexture, src_texture: *const RenderTexture) {
        todo!("copy D3D12 texture")
    }

    pub fn resolve_texture(&mut self, dst_texture: *const RenderTexture, src_texture: *const RenderTexture) {
        todo!("resolve D3D12 texture")
    }

    pub fn resolve_texture_region(
        &mut self,
        dst_texture: *const RenderTexture,
        dst_x: u32,
        dst_y: u32,
        src_texture: *const RenderTexture,
        src_rect: *const RenderRect,
        resolve_mode: RenderResolveMode,
    ) {
        todo!("resolve D3D12 texture region")
    }

    pub fn build_bottom_level_as(
        &mut self,
        dst_acceleration_structure: *const RenderAccelerationStructure,
        scratch_buffer: RenderBufferReference,
        build_info: &RenderBottomLevelASBuildInfo,
    ) {
        todo!("build D3D12 bottom level AS")
    }

    pub fn build_top_level_as(
        &mut self,
        dst_acceleration_structure: *const RenderAccelerationStructure,
        scratch_buffer: RenderBufferReference,
        instances_buffer: RenderBufferReference,
        build_info: &RenderTopLevelASBuildInfo,
    ) {
        todo!("build D3D12 top level AS")
    }

    pub fn discard_texture(&mut self, texture: *const RenderTexture) {
        todo!("discard D3D12 texture")
    }

    pub fn reset_query_pool(&mut self, query_pool: *const RenderQueryPool, query_first_index: u32, query_count: u32) {
        todo!("reset D3D12 query pool")
    }

    pub fn write_timestamp(&mut self, query_pool: *const RenderQueryPool, query_index: u32) {
        todo!("write D3D12 timestamp")
    }

    pub fn check_descriptor_heaps(&mut self) {
        todo!("check D3D12 descriptor heaps")
    }

    pub fn notify_descriptor_heap_was_changed_externally(&mut self) {
        todo!("notify descriptor heap changed externally")
    }

    pub fn check_topology(&mut self) {
        todo!("check D3D12 topology")
    }

    pub fn check_stencil_ref(&mut self) {
        todo!("check D3D12 stencil ref")
    }

    pub fn check_framebuffer_sample_positions(&mut self) {
        todo!("check D3D12 framebuffer sample positions")
    }

    pub fn set_sample_positions(&mut self, texture: *const RenderTexture) {
        todo!("set D3D12 sample positions")
    }

    pub fn reset_sample_positions(&mut self) {
        todo!("reset D3D12 sample positions")
    }

    pub fn set_descriptor_set(
        &mut self,
        active_pipeline_layout: *const D3D12PipelineLayout,
        descriptor_set: *mut RenderDescriptorSet,
        set_index: u32,
        set_compute: bool,
    ) {
        todo!("set D3D12 descriptor set")
    }

    pub fn set_root_descriptor_table(
        &mut self,
        heap_allocator: *mut D3D12DescriptorHeapAllocator,
        heap_allocation: &HeapAllocation,
        root_index: u32,
        set_compute: bool,
    ) {
        todo!("set D3D12 root descriptor table")
    }

    pub fn set_root_descriptor(
        &mut self,
        active_pipeline_layout: *const D3D12PipelineLayout,
        buffer_reference: RenderBufferReference,
        set_index: u32,
        set_compute: bool,
    ) {
        todo!("set D3D12 root descriptor")
    }
}

pub struct D3D12CommandFence {
    pub d3d: d3d12::Fence,
    pub device: *mut D3D12Device,
    pub fence_event: Handle,
    pub fence_value: u64,
}

impl D3D12CommandFence {
    pub fn new(device: *mut D3D12Device) -> Self {
        Self {
            d3d: 0,
            device,
            fence_event: 0,
            fence_value: 0,
        }
    }
}

pub struct D3D12CommandSemaphore {
    pub d3d: d3d12::Fence,
    pub device: *mut D3D12Device,
    pub semaphore_value: u64,
}

impl D3D12CommandSemaphore {
    pub fn new(device: *mut D3D12Device) -> Self {
        Self {
            d3d: 0,
            device,
            semaphore_value: 0,
        }
    }
}

pub struct D3D12CommandQueue {
    pub d3d: d3d12::CommandQueue,
    pub device: *mut D3D12Device,
    pub queue_type: RenderCommandListType,
}

impl D3D12CommandQueue {
    pub fn new(device: *mut D3D12Device, queue_type: RenderCommandListType) -> Self {
        Self {
            d3d: 0,
            device,
            queue_type,
        }
    }

    pub fn create_command_list(&mut self) -> Box<D3D12CommandList> {
        todo!("create D3D12 command list")
    }

    pub fn create_swap_chain(&mut self, desc: &RenderSwapChainDesc) -> Box<D3D12SwapChain> {
        todo!("create D3D12 swapchain")
    }

    pub fn execute_command_lists(
        &mut self,
        command_lists: *const *const RenderCommandList,
        command_list_count: u32,
        wait_semaphores: *mut *mut RenderCommandSemaphore,
        wait_semaphore_count: u32,
        signal_semaphores: *mut *mut RenderCommandSemaphore,
        signal_semaphore_count: u32,
        signal_fence: *mut RenderCommandFence,
    ) {
        todo!("execute D3D12 command lists")
    }

    pub fn wait_for_command_fence(&mut self, fence: *mut RenderCommandFence) {
        todo!("wait for D3D12 command fence")
    }
}

pub struct D3D12Buffer {
    pub d3d: d3d12::Resource,
    pub resource_states: d3d12::ResourceStates,
    pub device: *mut D3D12Device,
    pub allocation: d3d12ma::Allocation,
    pub pool: *mut D3D12Pool,
    pub desc: RenderBufferDesc,
}

impl D3D12Buffer {
    pub fn new(device: *mut D3D12Device, pool: *mut D3D12Pool, desc: RenderBufferDesc) -> Self {
        Self {
            d3d: 0,
            resource_states: d3d12::D3D12_RESOURCE_STATE_COMMON,
            device,
            allocation: 0,
            pool,
            desc,
        }
    }

    pub fn map(&mut self, subresource: u32, read_range: Option<&RenderRange>) -> *mut u8 {
        todo!("map D3D12 buffer")
    }

    pub fn unmap(&mut self, subresource: u32, written_range: Option<&RenderRange>) {
        todo!("unmap D3D12 buffer")
    }

    pub fn create_buffer_formatted_view(&self, format: RenderFormat) -> Box<RenderBufferFormattedView> {
        todo!("create D3D12 buffer formatted view")
    }

    pub fn set_name(&mut self, name: &str) {
        todo!("set D3D12 buffer debug name")
    }

    pub fn get_device_address(&self) -> u64 {
        todo!("get D3D12 buffer device address")
    }
}

pub struct D3D12BufferFormattedView {
    pub format: RenderFormat,
    pub buffer: *mut D3D12Buffer,
}

impl D3D12BufferFormattedView {
    pub fn new(buffer: *mut D3D12Buffer, format: RenderFormat) -> Self {
        Self { format, buffer }
    }
}

pub struct D3D12Texture {
    pub d3d: d3d12::Resource,
    pub resource_states: d3d12::ResourceStates,
    pub layout: RenderTextureLayout,
    pub device: *mut D3D12Device,
    pub allocation: d3d12ma::Allocation,
    pub pool: *mut D3D12Pool,
    pub desc: RenderTextureDesc,
}

impl D3D12Texture {
    pub fn new(device: *mut D3D12Device, pool: *mut D3D12Pool, desc: RenderTextureDesc) -> Self {
        Self {
            d3d: 0,
            resource_states: d3d12::D3D12_RESOURCE_STATE_COMMON,
            layout: RenderTextureLayout::Unknown,
            device,
            allocation: 0,
            pool,
            desc,
        }
    }

    pub fn create_texture_view(&self, desc: &RenderTextureViewDesc) -> Box<D3D12TextureView> {
        todo!("create D3D12 texture view")
    }

    pub fn set_name(&mut self, name: &str) {
        todo!("set D3D12 texture debug name")
    }
}

pub struct D3D12TextureView {
    pub format: dxgi::Format,
    pub texture: *const D3D12Texture,
    pub desc: RenderTextureViewDesc,
    pub dimension: RenderTextureViewDimension,
    pub mip_levels: u32,
    pub mip_slice: u32,
    pub array_size: u32,
    pub array_index: u32,
    pub shader4_component_mapping: u32,
}

impl D3D12TextureView {
    pub fn new(texture: *const D3D12Texture, desc: RenderTextureViewDesc) -> Self {
        Self {
            format: dxgi::DXGI_FORMAT_UNKNOWN,
            texture,
            desc,
            dimension: RenderTextureViewDimension::Unknown,
            mip_levels: 0,
            mip_slice: 0,
            array_size: 0,
            array_index: 0,
            shader4_component_mapping: d3d12::D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
        }
    }
}

pub struct D3D12AccelerationStructure {
    pub device: *mut D3D12Device,
    pub buffer: *const D3D12Buffer,
    pub offset: u64,
    pub size: u64,
    pub structure_type: RenderAccelerationStructureType,
}

impl D3D12AccelerationStructure {
    pub fn new(device: *mut D3D12Device, desc: &RenderAccelerationStructureDesc) -> Self {
        Self {
            device,
            buffer: std::ptr::null(),
            offset: 0,
            size: 0,
            structure_type: RenderAccelerationStructureType::Unknown,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct RenderAccelerationStructureDesc;

pub struct D3D12Pool {
    pub d3d: d3d12ma::Pool,
    pub device: *mut D3D12Device,
    pub desc: RenderPoolDesc,
}

impl D3D12Pool {
    pub fn new(device: *mut D3D12Device, desc: &RenderPoolDesc, gpu_upload_heap_fallback: bool) -> Self {
        Self { d3d: 0, device, desc: *desc }
    }

    pub fn create_buffer(&mut self, desc: &RenderBufferDesc) -> Box<D3D12Buffer> {
        todo!("create D3D12 pool buffer")
    }

    pub fn create_texture(&mut self, desc: &RenderTextureDesc) -> Box<D3D12Texture> {
        todo!("create D3D12 pool texture")
    }
}

pub struct D3D12Shader {
    pub d3d: Vec<u8>,
    pub entry_point_name: String,
    pub device: *mut D3D12Device,
    pub format: RenderShaderFormat,
}

impl D3D12Shader {
    pub fn new(device: *mut D3D12Device, data: *const u8, size: u64, entry_point_name: &str, format: RenderShaderFormat) -> Self {
        Self {
            d3d: Vec::new(),
            entry_point_name: entry_point_name.to_string(),
            device,
            format,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        todo!("set D3D12 shader debug name")
    }
}

pub struct D3D12Sampler {
    pub sampler_desc: d3d12::SamplerDesc,
    pub device: *mut D3D12Device,
    pub border_color: RenderBorderColor,
    pub shader_visibility: RenderShaderVisibility,
}

impl D3D12Sampler {
    pub fn new(device: *mut D3D12Device, desc: &RenderSamplerDesc) -> Self {
        Self {
            sampler_desc: d3d12::SamplerDesc::default(),
            device,
            border_color: RenderBorderColor::Unknown,
            shader_visibility: RenderShaderVisibility::Unknown,
        }
    }
}

pub struct D3D12Pipeline {
    pub device: *mut D3D12Device,
    pub pipeline_type: D3D12PipelineType,
}

#[derive(Clone, Copy, Default)]
pub enum D3D12PipelineType {
    #[default]
    Unknown,
    Compute,
    Graphics,
    Raytracing,
}

impl D3D12Pipeline {
    pub fn new(device: *mut D3D12Device, pipeline_type: D3D12PipelineType) -> Self {
        Self { device, pipeline_type }
    }
}

pub struct D3D12ComputePipeline {
    pub base: D3D12Pipeline,
    pub d3d: d3d12::PipelineState,
}

impl D3D12ComputePipeline {
    pub fn new(device: *mut D3D12Device, desc: &RenderComputePipelineDesc) -> Self {
        Self {
            base: D3D12Pipeline::new(device, D3D12PipelineType::Compute),
            d3d: 0,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        todo!("set D3D12 compute pipeline debug name")
    }

    pub fn get_program(&self, name: &str) -> RenderPipelineProgram {
        todo!("get D3D12 compute pipeline program")
    }
}

pub struct D3D12GraphicsPipeline {
    pub base: D3D12Pipeline,
    pub d3d: d3d12::PipelineState,
    pub input_slots: Vec<RenderInputSlot>,
    pub topology: d3d12::PrimitiveTopology,
    pub stencil_ref: u32,
}

impl D3D12GraphicsPipeline {
    pub fn new(device: *mut D3D12Device, desc: &RenderGraphicsPipelineDesc) -> Self {
        Self {
            base: D3D12Pipeline::new(device, D3D12PipelineType::Graphics),
            d3d: 0,
            input_slots: Vec::new(),
            topology: d3d12::D3D_PRIMITIVE_TOPOLOGY_UNDEFINED,
            stencil_ref: 0,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        todo!("set D3D12 graphics pipeline debug name")
    }

    pub fn get_program(&self, name: &str) -> RenderPipelineProgram {
        todo!("get D3D12 graphics pipeline program")
    }
}

pub struct D3D12RaytracingPipeline {
    pub base: D3D12Pipeline,
    pub state_object: d3d12::StateObject,
    pub state_object_properties: d3d12::StateObjectProperties,
    pub program_shader_identifiers: Vec<*mut std::ffi::c_void>,
    pub name_program_map: HashMap<String, RenderPipelineProgram>,
    pub pipeline_layout: *const D3D12PipelineLayout,
}

impl D3D12RaytracingPipeline {
    pub fn new(
        device: *mut D3D12Device,
        desc: &RenderRaytracingPipelineDesc,
        previous_pipeline: Option<&RenderPipeline>,
    ) -> Self {
        Self {
            base: D3D12Pipeline::new(device, D3D12PipelineType::Raytracing),
            state_object: 0,
            state_object_properties: 0,
            program_shader_identifiers: Vec::new(),
            name_program_map: HashMap::new(),
            pipeline_layout: std::ptr::null(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        todo!("set D3D12 raytracing pipeline debug name")
    }

    pub fn get_program(&self, name: &str) -> RenderPipelineProgram {
        todo!("get D3D12 raytracing pipeline program")
    }
}

pub struct D3D12PipelineLayout {
    pub root_signature: d3d12::RootSignature,
    pub device: *mut D3D12Device,
    pub push_constant_ranges: Vec<RenderPushConstantRange>,
    pub set_view_root_indices: Vec<u32>,
    pub set_sampler_root_indices: Vec<u32>,
    pub root_descriptor_root_indices_and_types: Vec<(u32, RenderRootDescriptorType)>,
    pub set_count: u32,
    pub root_count: u32,
}

impl D3D12PipelineLayout {
    pub fn new(device: *mut D3D12Device, desc: &RenderPipelineLayoutDesc) -> Self {
        Self {
            root_signature: 0,
            device,
            push_constant_ranges: Vec::new(),
            set_view_root_indices: Vec::new(),
            set_sampler_root_indices: Vec::new(),
            root_descriptor_root_indices_and_types: Vec::new(),
            set_count: 0,
            root_count: 0,
        }
    }
}

pub struct D3D12Device {
    pub d3d: d3d12::Device8,
    pub render_interface: *mut D3D12Interface,
    pub adapter: dxgi::Adapter1,
    pub allocator: d3d12ma::Allocator,
    pub shader_model: d3d12::ShaderModel,
    pub rt_dummy_global_pipeline_layout: Option<Box<RenderPipelineLayout>>,
    pub rt_dummy_local_pipeline_layout: Option<Box<RenderPipelineLayout>>,
    pub view_heap_allocator: Option<Box<D3D12DescriptorHeapAllocator>>,
    pub sampler_heap_allocator: Option<Box<D3D12DescriptorHeapAllocator>>,
    pub color_target_heap_allocator: Option<Box<D3D12DescriptorHeapAllocator>>,
    pub depth_target_heap_allocator: Option<Box<D3D12DescriptorHeapAllocator>>,
    pub custom_upload_pool: Option<Box<D3D12Pool>>,
    pub capabilities: RenderDeviceCapabilities,
    pub description: RenderDeviceDescription,
    pub timestamp_frequency: u64,
    pub gpu_upload_heap_fallback: bool,
}

impl D3D12Device {
    pub fn new(render_interface: *mut D3D12Interface, preferred_device_name: &str) -> Self {
        Self {
            d3d: 0,
            render_interface,
            adapter: 0,
            allocator: 0,
            shader_model: 0,
            rt_dummy_global_pipeline_layout: None,
            rt_dummy_local_pipeline_layout: None,
            view_heap_allocator: None,
            sampler_heap_allocator: None,
            color_target_heap_allocator: None,
            depth_target_heap_allocator: None,
            custom_upload_pool: None,
            capabilities: RenderDeviceCapabilities::default(),
            description: RenderDeviceDescription::default(),
            timestamp_frequency: 1,
            gpu_upload_heap_fallback: false,
        }
    }

    pub fn create_descriptor_set(&mut self, desc: &RenderDescriptorSetDesc) -> Box<D3D12DescriptorSet> {
        todo!("create D3D12 descriptor set")
    }

    pub fn create_shader(
        &mut self,
        data: *const u8,
        size: u64,
        entry_point_name: &str,
        format: RenderShaderFormat,
    ) -> Box<D3D12Shader> {
        todo!("create D3D12 shader")
    }

    pub fn create_sampler(&mut self, desc: &RenderSamplerDesc) -> Box<D3D12Sampler> {
        todo!("create D3D12 sampler")
    }

    pub fn create_compute_pipeline(&mut self, desc: &RenderComputePipelineDesc) -> Box<D3D12ComputePipeline> {
        todo!("create D3D12 compute pipeline")
    }

    pub fn create_graphics_pipeline(&mut self, desc: &RenderGraphicsPipelineDesc) -> Box<D3D12GraphicsPipeline> {
        todo!("create D3D12 graphics pipeline")
    }

    pub fn create_raytracing_pipeline(
        &mut self,
        desc: &RenderRaytracingPipelineDesc,
        previous_pipeline: Option<&RenderPipeline>,
    ) -> Box<D3D12RaytracingPipeline> {
        todo!("create D3D12 raytracing pipeline")
    }

    pub fn create_command_queue(&mut self, queue_type: RenderCommandListType) -> Box<D3D12CommandQueue> {
        todo!("create D3D12 command queue")
    }

    pub fn create_buffer(&mut self, desc: &RenderBufferDesc) -> Box<D3D12Buffer> {
        todo!("create D3D12 buffer")
    }

    pub fn create_texture(&mut self, desc: &RenderTextureDesc) -> Box<D3D12Texture> {
        todo!("create D3D12 texture")
    }

    pub fn create_acceleration_structure(
        &mut self,
        desc: &RenderAccelerationStructureDesc,
    ) -> Box<D3D12AccelerationStructure> {
        todo!("create D3D12 acceleration structure")
    }

    pub fn create_pool(&mut self, desc: &RenderPoolDesc) -> Box<D3D12Pool> {
        todo!("create D3D12 pool")
    }

    pub fn create_pipeline_layout(&mut self, desc: &RenderPipelineLayoutDesc) -> Box<D3D12PipelineLayout> {
        todo!("create D3D12 pipeline layout")
    }

    pub fn create_command_fence(&mut self) -> Box<D3D12CommandFence> {
        todo!("create D3D12 command fence")
    }

    pub fn create_command_semaphore(&mut self) -> Box<D3D12CommandSemaphore> {
        todo!("create D3D12 command semaphore")
    }

    pub fn create_framebuffer(&mut self, desc: &RenderFramebufferDesc) -> Box<D3D12Framebuffer> {
        todo!("create D3D12 framebuffer")
    }

    pub fn create_query_pool(&mut self, query_count: u32) -> Box<D3D12QueryPool> {
        todo!("create D3D12 query pool")
    }

    pub fn set_bottom_level_as_build_info(
        &mut self,
        build_info: &mut RenderBottomLevelASBuildInfo,
        meshes: *const RenderBottomLevelASMesh,
        mesh_count: u32,
        prefer_fast_build: bool,
        prefer_fast_trace: bool,
    ) {
        todo!("set D3D12 bottom level AS build info")
    }

    pub fn set_top_level_as_build_info(
        &mut self,
        build_info: &mut RenderTopLevelASBuildInfo,
        instances: *const RenderTopLevelASInstance,
        instance_count: u32,
        prefer_fast_build: bool,
        prefer_fast_trace: bool,
    ) {
        todo!("set D3D12 top level AS build info")
    }

    pub fn set_shader_binding_table_info(
        &mut self,
        table_info: &mut RenderShaderBindingTableInfo,
        groups: &RenderShaderBindingGroups,
        pipeline: *const RenderPipeline,
        descriptor_sets: *mut *mut RenderDescriptorSet,
        descriptor_set_count: u32,
    ) {
        todo!("set D3D12 shader binding table info")
    }

    pub fn get_capabilities(&self) -> RenderDeviceCapabilities {
        self.capabilities
    }

    pub fn get_description(&self) -> RenderDeviceDescription {
        self.description
    }

    pub fn get_sample_counts_supported(&self, format: RenderFormat) -> RenderSampleCounts {
        todo!("get D3D12 sample counts")
    }

    pub fn release(&mut self) {
        todo!("release D3D12 device")
    }

    pub fn is_valid(&self) -> bool {
        self.d3d != 0
    }

    pub fn begin_capture(&mut self) -> bool {
        todo!("begin D3D12 capture")
    }

    pub fn end_capture(&mut self) -> bool {
        todo!("end D3D12 capture")
    }
}

pub struct D3D12Interface {
    pub dxgi_factory: dxgi::Factory4,
    pub capabilities: RenderInterfaceCapabilities,
    pub device_names: Vec<String>,
    pub allow_tearing: bool,
}

impl D3D12Interface {
    pub fn new() -> Self {
        Self {
            dxgi_factory: 0,
            capabilities: RenderInterfaceCapabilities::default(),
            device_names: Vec::new(),
            allow_tearing: false,
        }
    }

    pub fn create_device(&mut self, preferred_device_name: &str) -> Box<D3D12Device> {
        todo!("create D3D12 device")
    }

    pub fn get_capabilities(&self) -> RenderInterfaceCapabilities {
        self.capabilities
    }

    pub fn get_device_names(&self) -> &[String] {
        &self.device_names
    }

    pub fn is_valid(&self) -> bool {
        self.dxgi_factory != 0
    }
}

#[derive(Clone, Copy, Default)]
pub struct RenderBox;
