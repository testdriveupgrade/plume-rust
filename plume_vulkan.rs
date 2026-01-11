//
// plume
//
// Copyright (c) 2024 renderbag and contributors. All rights reserved.
// Licensed under the MIT license. See LICENSE file for details.
//

#![allow(dead_code, unused_variables)]

use std::collections::{HashMap, HashSet};
use std::ptr;

#[cfg(target_os = "windows")]
const VK_USE_PLATFORM_WIN32_KHR: bool = true;
#[cfg(target_os = "android")]
const VK_USE_PLATFORM_ANDROID_KHR: bool = true;
#[cfg(target_os = "linux")]
const VK_USE_PLATFORM_XLIB_KHR: bool = true;
#[cfg(target_os = "macos")]
const VK_USE_PLATFORM_METAL_EXT: bool = true;

const VK_ENABLE_BETA_EXTENSIONS: bool = true;

pub mod vk {
    #[derive(Clone, Copy, Default)]
    pub struct ApplicationInfo;
    #[derive(Clone, Copy, Default)]
    pub struct PhysicalDeviceProperties;
    #[derive(Clone, Copy, Default)]
    pub struct PhysicalDeviceRayTracingPipelinePropertiesKHR;
    #[derive(Clone, Copy, Default)]
    pub struct PhysicalDeviceSampleLocationsPropertiesEXT;
    #[derive(Clone, Copy, Default)]
    pub struct ImageSubresourceRange;
    #[derive(Clone, Copy, Default)]
    pub struct SwapchainCreateInfoKHR;
    #[derive(Clone, Copy, Default)]
    pub struct SurfaceFormatKHR;
    #[derive(Clone, Copy, Default)]
    pub struct DescriptorSetLayoutBinding;
    #[derive(Clone, Copy, Default)]
    pub struct PushConstantRange;
    #[derive(Clone, Copy, Default)]
    pub struct DescriptorBufferInfo;
    #[derive(Clone, Copy, Default)]
    pub struct DescriptorImageInfo;

    pub type AccelerationStructureKHR = u64;
    pub type Buffer = u64;
    pub type BufferView = u64;
    pub type CommandBuffer = u64;
    pub type CommandPool = u64;
    pub type CompositeAlphaFlagBitsKHR = u32;
    pub type DescriptorPool = u64;
    pub type DescriptorSet = u64;
    pub type DescriptorSetLayout = u64;
    pub type DescriptorType = u32;
    pub type Device = u64;
    pub type Fence = u64;
    pub type Format = u32;
    pub type Framebuffer = u64;
    pub type Image = u64;
    pub type ImageView = u64;
    pub type Instance = u64;
    pub type PhysicalDevice = u64;
    pub type Pipeline = u64;
    pub type PipelineLayout = u64;
    pub type PresentModeKHR = u32;
    pub type Queue = u64;
    pub type QueryPool = u64;
    pub type RenderPass = u64;
    pub type SampleCountFlagBits = u32;
    pub type Sampler = u64;
    pub type Semaphore = u64;
    pub type ShaderModule = u64;
    pub type SurfaceKHR = u64;
    pub type SwapchainKHR = u64;
}

pub type VmaAllocation = u64;
pub type VmaAllocator = u64;
pub type VmaPool = u64;
#[derive(Clone, Copy, Default)]
pub struct VmaAllocationInfo;

pub struct RenderBuffer;
pub struct RenderBufferFormattedView;
pub struct RenderTexture;
pub struct RenderTextureView;
pub struct RenderAccelerationStructure;
pub struct RenderPipelineLayout;
pub struct RenderShader;
pub struct RenderSampler;
pub struct RenderPipeline;
pub struct RenderDescriptorSet;
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

#[derive(Clone, Copy, Default)]
pub struct RenderBufferDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderBufferBarrier;
#[derive(Clone, Copy, Default)]
pub struct RenderBufferReference;
#[derive(Clone, Copy, Default)]
pub struct RenderBufferStructuredView;
#[derive(Clone, Copy, Default)]
pub struct RenderBufferFormattedViewDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderRange;
#[derive(Clone, Copy, Default)]
pub struct RenderTextureDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderTextureViewDesc;
#[derive(Clone, Copy, Default)]
pub struct RenderTextureBarrier;
#[derive(Clone, Copy, Default)]
pub struct RenderTextureCopyLocation;
#[derive(Clone, Copy, Default)]
pub struct RenderAccelerationStructureDesc;
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
pub struct RenderSamplerDesc;
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

pub struct VulkanCommandQueue;
pub struct VulkanDevice;
pub struct VulkanInterface;
pub struct VulkanPool;
pub struct VulkanQueue;

pub struct VulkanBuffer {
    pub vk: vk::Buffer,
    pub device: *mut VulkanDevice,
    pub pool: *mut VulkanPool,
    pub allocation: VmaAllocation,
    pub allocation_info: VmaAllocationInfo,
    pub desc: RenderBufferDesc,
    pub barrier_stages: RenderBarrierStages,
}

impl VulkanBuffer {
    pub fn new(device: *mut VulkanDevice, pool: *mut VulkanPool, desc: RenderBufferDesc) -> Self {
        Self {
            vk: 0,
            device,
            pool,
            allocation: 0,
            allocation_info: VmaAllocationInfo::default(),
            desc,
            barrier_stages: RenderBarrierStage::None,
        }
    }

    pub fn map(&mut self, subresource: u32, read_range: Option<&RenderRange>) -> *mut u8 {
        todo!("map Vulkan buffer")
    }

    pub fn unmap(&mut self, subresource: u32, written_range: Option<&RenderRange>) {
        todo!("unmap Vulkan buffer")
    }

    pub fn create_buffer_formatted_view(
        &self,
        format: RenderFormat,
    ) -> Box<VulkanBufferFormattedView> {
        todo!("create Vulkan buffer formatted view")
    }

    pub fn set_name(&mut self, name: &str) {
        todo!("set Vulkan buffer debug name")
    }

    pub fn get_device_address(&self) -> u64 {
        todo!("get Vulkan buffer device address")
    }
}

pub struct VulkanBufferFormattedView {
    pub vk: vk::BufferView,
    pub buffer: *mut VulkanBuffer,
}

impl VulkanBufferFormattedView {
    pub fn new(buffer: *mut VulkanBuffer, format: RenderFormat) -> Self {
        Self { vk: 0, buffer }
    }
}

pub struct VulkanTexture {
    pub vk: vk::Image,
    pub image_view: vk::ImageView,
    pub image_format: vk::Format,
    pub image_subresource_range: vk::ImageSubresourceRange,
    pub device: *mut VulkanDevice,
    pub pool: *mut VulkanPool,
    pub allocation: VmaAllocation,
    pub allocation_info: VmaAllocationInfo,
    pub texture_layout: RenderTextureLayout,
    pub barrier_stages: RenderBarrierStages,
    pub ownership: bool,
    pub desc: RenderTextureDesc,
}

impl VulkanTexture {
    pub fn new(device: *mut VulkanDevice, pool: *mut VulkanPool, desc: RenderTextureDesc) -> Self {
        Self {
            vk: 0,
            image_view: 0,
            image_format: 0,
            image_subresource_range: vk::ImageSubresourceRange::default(),
            device,
            pool,
            allocation: 0,
            allocation_info: VmaAllocationInfo::default(),
            texture_layout: RenderTextureLayout::Unknown,
            barrier_stages: RenderBarrierStage::None,
            ownership: false,
            desc,
        }
    }

    pub fn new_from_image(device: *mut VulkanDevice, image: vk::Image) -> Self {
        Self {
            vk: image,
            image_view: 0,
            image_format: 0,
            image_subresource_range: vk::ImageSubresourceRange::default(),
            device,
            pool: ptr::null_mut(),
            allocation: 0,
            allocation_info: VmaAllocationInfo::default(),
            texture_layout: RenderTextureLayout::Unknown,
            barrier_stages: RenderBarrierStage::None,
            ownership: false,
            desc: RenderTextureDesc::default(),
        }
    }

    pub fn create_image_view(&mut self, format: vk::Format) {
        todo!("create Vulkan image view")
    }

    pub fn create_texture_view(&self, desc: &RenderTextureViewDesc) -> Box<VulkanTextureView> {
        todo!("create Vulkan texture view")
    }

    pub fn set_name(&mut self, name: &str) {
        todo!("set Vulkan texture debug name")
    }

    pub fn fill_subresource_range(&mut self) {
        todo!("fill Vulkan image subresource range")
    }
}

pub struct VulkanTextureView {
    pub vk: vk::ImageView,
    pub texture: *const VulkanTexture,
    pub desc: RenderTextureViewDesc,
}

impl VulkanTextureView {
    pub fn new(texture: *const VulkanTexture, desc: RenderTextureViewDesc) -> Self {
        Self { vk: 0, texture, desc }
    }
}

pub struct VulkanAccelerationStructure {
    pub vk: vk::AccelerationStructureKHR,
    pub device: *mut VulkanDevice,
    pub structure_type: RenderAccelerationStructureType,
}

impl VulkanAccelerationStructure {
    pub fn new(device: *mut VulkanDevice, desc: &RenderAccelerationStructureDesc) -> Self {
        Self {
            vk: 0,
            device,
            structure_type: RenderAccelerationStructureType::Unknown,
        }
    }
}

pub struct VulkanDescriptorSetLayout {
    pub vk: vk::DescriptorSetLayout,
    pub set_bindings: Vec<vk::DescriptorSetLayoutBinding>,
    pub descriptor_index_bases: Vec<u32>,
    pub descriptor_binding_indices: Vec<u32>,
    pub device: *mut VulkanDevice,
}

impl VulkanDescriptorSetLayout {
    pub fn new(device: *mut VulkanDevice, desc: &RenderDescriptorSetDesc) -> Self {
        Self {
            vk: 0,
            set_bindings: Vec::new(),
            descriptor_index_bases: Vec::new(),
            descriptor_binding_indices: Vec::new(),
            device,
        }
    }
}

pub struct VulkanPipelineLayout {
    pub vk: vk::PipelineLayout,
    pub push_constant_ranges: Vec<vk::PushConstantRange>,
    pub descriptor_set_layouts: Vec<*mut VulkanDescriptorSetLayout>,
    pub device: *mut VulkanDevice,
}

impl VulkanPipelineLayout {
    pub fn new(device: *mut VulkanDevice, desc: &RenderPipelineLayoutDesc) -> Self {
        Self {
            vk: 0,
            push_constant_ranges: Vec::new(),
            descriptor_set_layouts: Vec::new(),
            device,
        }
    }
}

pub struct VulkanShader {
    pub vk: vk::ShaderModule,
    pub entry_point_name: String,
    pub device: *mut VulkanDevice,
    pub format: RenderShaderFormat,
}

impl VulkanShader {
    pub fn new(
        device: *mut VulkanDevice,
        data: *const u8,
        size: u64,
        entry_point_name: &str,
        format: RenderShaderFormat,
    ) -> Self {
        Self {
            vk: 0,
            entry_point_name: entry_point_name.to_string(),
            device,
            format,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        todo!("set Vulkan shader debug name")
    }
}

pub struct VulkanSampler {
    pub vk: vk::Sampler,
    pub device: *mut VulkanDevice,
}

impl VulkanSampler {
    pub fn new(device: *mut VulkanDevice, desc: &RenderSamplerDesc) -> Self {
        Self { vk: 0, device }
    }
}

pub struct VulkanPipeline {
    pub device: *mut VulkanDevice,
    pub pipeline_type: VulkanPipelineType,
}

#[derive(Clone, Copy, Default)]
pub enum VulkanPipelineType {
    #[default]
    Unknown,
    Compute,
    Graphics,
    Raytracing,
}

impl VulkanPipeline {
    pub fn new(device: *mut VulkanDevice, pipeline_type: VulkanPipelineType) -> Self {
        Self {
            device,
            pipeline_type,
        }
    }
}

pub struct VulkanComputePipeline {
    pub base: VulkanPipeline,
    pub vk: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
}

impl VulkanComputePipeline {
    pub fn new(device: *mut VulkanDevice, desc: &RenderComputePipelineDesc) -> Self {
        Self {
            base: VulkanPipeline::new(device, VulkanPipelineType::Compute),
            vk: 0,
            pipeline_layout: 0,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        todo!("set Vulkan compute pipeline debug name")
    }

    pub fn get_program(&self, name: &str) -> RenderPipelineProgram {
        todo!("get Vulkan compute pipeline program")
    }
}

pub struct VulkanGraphicsPipeline {
    pub base: VulkanPipeline,
    pub vk: vk::Pipeline,
    pub render_pass: vk::RenderPass,
}

impl VulkanGraphicsPipeline {
    pub fn new(device: *mut VulkanDevice, desc: &RenderGraphicsPipelineDesc) -> Self {
        Self {
            base: VulkanPipeline::new(device, VulkanPipelineType::Graphics),
            vk: 0,
            render_pass: 0,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        todo!("set Vulkan graphics pipeline debug name")
    }

    pub fn get_program(&self, name: &str) -> RenderPipelineProgram {
        todo!("get Vulkan graphics pipeline program")
    }

    pub fn create_render_pass(
        device: *mut VulkanDevice,
        render_target_format: *const vk::Format,
        render_target_count: u32,
        depth_target_format: vk::Format,
        sample_count: vk::SampleCountFlagBits,
    ) -> vk::RenderPass {
        todo!("create Vulkan render pass")
    }
}

pub struct VulkanRaytracingPipeline {
    pub base: VulkanPipeline,
    pub vk: vk::Pipeline,
    pub name_program_map: HashMap<String, RenderPipelineProgram>,
    pub group_count: u32,
    pub descriptor_set_count: u32,
}

impl VulkanRaytracingPipeline {
    pub fn new(
        device: *mut VulkanDevice,
        desc: &RenderRaytracingPipelineDesc,
        previous_pipeline: Option<&RenderPipeline>,
    ) -> Self {
        Self {
            base: VulkanPipeline::new(device, VulkanPipelineType::Raytracing),
            vk: 0,
            name_program_map: HashMap::new(),
            group_count: 0,
            descriptor_set_count: 0,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        todo!("set Vulkan raytracing pipeline debug name")
    }

    pub fn get_program(&self, name: &str) -> RenderPipelineProgram {
        todo!("get Vulkan raytracing pipeline program")
    }
}

pub struct VulkanDescriptorSet {
    pub vk: vk::DescriptorSet,
    pub set_layout: *mut VulkanDescriptorSetLayout,
    pub descriptor_pool: vk::DescriptorPool,
    pub device: *mut VulkanDevice,
}

impl VulkanDescriptorSet {
    pub fn new(device: *mut VulkanDevice, desc: &RenderDescriptorSetDesc) -> Self {
        Self {
            vk: 0,
            set_layout: ptr::null_mut(),
            descriptor_pool: 0,
            device,
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
        todo!("set Vulkan descriptor buffer")
    }

    pub fn set_texture(
        &mut self,
        descriptor_index: u32,
        texture: *const RenderTexture,
        texture_layout: RenderTextureLayout,
        texture_view: *const RenderTextureView,
    ) {
        todo!("set Vulkan descriptor texture")
    }

    pub fn set_sampler(&mut self, descriptor_index: u32, sampler: *const RenderSampler) {
        todo!("set Vulkan descriptor sampler")
    }

    pub fn set_acceleration_structure(
        &mut self,
        descriptor_index: u32,
        acceleration_structure: *const RenderAccelerationStructure,
    ) {
        todo!("set Vulkan descriptor acceleration structure")
    }

    pub fn set_descriptor(
        &mut self,
        descriptor_index: u32,
        buffer_info: *const vk::DescriptorBufferInfo,
        image_info: *const vk::DescriptorImageInfo,
        texel_buffer_view: *const vk::BufferView,
        p_next: *mut std::ffi::c_void,
    ) {
        todo!("set Vulkan descriptor")
    }

    pub fn create_descriptor_pool(
        device: *mut VulkanDevice,
        type_counts: &HashMap<vk::DescriptorType, u32>,
        last_range_is_boundless: bool,
    ) -> vk::DescriptorPool {
        todo!("create Vulkan descriptor pool")
    }
}

pub struct VulkanSwapChain {
    pub desc: RenderSwapChainDesc,
    pub vk: vk::SwapchainKHR,
    pub command_queue: *mut VulkanCommandQueue,
    pub surface: vk::SurfaceKHR,
    pub present_count: u64,
    pub width: u32,
    pub height: u32,
    pub create_info: vk::SwapchainCreateInfoKHR,
    pub picked_surface_format: vk::SurfaceFormatKHR,
    pub created_present_mode: vk::PresentModeKHR,
    pub required_present_mode: vk::PresentModeKHR,
    pub picked_alpha_flag: vk::CompositeAlphaFlagBitsKHR,
    pub textures: Vec<VulkanTexture>,
    pub current_present_id: u64,
    pub immediate_present_mode_supported: bool,
    pub mailbox_present_mode_supported: bool,
}

impl VulkanSwapChain {
    pub fn new(command_queue: *mut VulkanCommandQueue, desc: RenderSwapChainDesc) -> Self {
        Self {
            desc,
            vk: 0,
            command_queue,
            surface: 0,
            present_count: 0,
            width: 0,
            height: 0,
            create_info: vk::SwapchainCreateInfoKHR::default(),
            picked_surface_format: vk::SurfaceFormatKHR::default(),
            created_present_mode: 0,
            required_present_mode: 0,
            picked_alpha_flag: 0,
            textures: Vec::new(),
            current_present_id: 0,
            immediate_present_mode_supported: false,
            mailbox_present_mode_supported: false,
        }
    }

    pub fn present(
        &mut self,
        texture_index: u32,
        wait_semaphores: *mut *mut RenderCommandSemaphore,
        wait_semaphore_count: u32,
    ) -> bool {
        todo!("present Vulkan swapchain")
    }

    pub fn wait(&mut self) {
        todo!("wait Vulkan swapchain")
    }

    pub fn resize(&mut self) -> bool {
        todo!("resize Vulkan swapchain")
    }

    pub fn needs_resize(&self) -> bool {
        todo!("check Vulkan swapchain resize")
    }

    pub fn set_vsync_enabled(&mut self, vsync_enabled: bool) {
        todo!("set Vulkan swapchain vsync")
    }

    pub fn is_vsync_enabled(&self) -> bool {
        todo!("get Vulkan swapchain vsync")
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_texture(&mut self, texture_index: u32) -> *mut RenderTexture {
        todo!("get Vulkan swapchain texture")
    }

    pub fn get_texture_count(&self) -> u32 {
        self.textures.len() as u32
    }

    pub fn acquire_texture(
        &mut self,
        signal_semaphore: *mut RenderCommandSemaphore,
        texture_index: &mut u32,
    ) -> bool {
        todo!("acquire Vulkan swapchain texture")
    }

    pub fn get_window(&self) -> RenderWindow {
        todo!("get Vulkan swapchain window")
    }

    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }

    pub fn get_refresh_rate(&self) -> u32 {
        todo!("get Vulkan swapchain refresh rate")
    }

    pub fn get_window_size(&self, dst_width: &mut u32, dst_height: &mut u32) {
        *dst_width = self.width;
        *dst_height = self.height;
    }

    pub fn release_swap_chain(&mut self) {
        todo!("release Vulkan swapchain")
    }

    pub fn release_image_views(&mut self) {
        todo!("release Vulkan swapchain image views")
    }
}

pub struct VulkanFramebuffer {
    pub device: *mut VulkanDevice,
    pub vk: vk::Framebuffer,
    pub render_pass: vk::RenderPass,
    pub color_attachments: Vec<*const VulkanTexture>,
    pub depth_attachment: *const VulkanTexture,
    pub depth_attachment_view: Option<Box<VulkanTextureView>>,
    pub depth_attachment_read_only: bool,
    pub width: u32,
    pub height: u32,
}

impl VulkanFramebuffer {
    pub fn new(device: *mut VulkanDevice, desc: &RenderFramebufferDesc) -> Self {
        Self {
            device,
            vk: 0,
            render_pass: 0,
            color_attachments: Vec::new(),
            depth_attachment: ptr::null(),
            depth_attachment_view: None,
            depth_attachment_read_only: false,
            width: 0,
            height: 0,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn contains(&self, attachment: *const VulkanTexture) -> bool {
        self.color_attachments.iter().any(|&ptr| ptr == attachment) || self.depth_attachment == attachment
    }
}

pub struct VulkanQueryPool {
    pub device: *mut VulkanDevice,
    pub results: Vec<u64>,
    pub vk: vk::QueryPool,
}

impl VulkanQueryPool {
    pub fn new(device: *mut VulkanDevice, query_count: u32) -> Self {
        Self {
            device,
            results: vec![0; query_count as usize],
            vk: 0,
        }
    }

    pub fn query_results(&mut self) {
        todo!("query Vulkan timestamps")
    }

    pub fn get_results(&self) -> &[u64] {
        &self.results
    }

    pub fn get_count(&self) -> u32 {
        self.results.len() as u32
    }
}

pub struct VulkanCommandList {
    pub vk: vk::CommandBuffer,
    pub command_pool: vk::CommandPool,
    pub queue: *mut VulkanCommandQueue,
    pub target_framebuffer: *const VulkanFramebuffer,
    pub active_compute_pipeline_layout: *const VulkanPipelineLayout,
    pub active_graphics_pipeline_layout: *const VulkanPipelineLayout,
    pub active_raytracing_pipeline_layout: *const VulkanPipelineLayout,
    pub active_render_pass: vk::RenderPass,
}

impl VulkanCommandList {
    pub fn new(queue: *mut VulkanCommandQueue) -> Self {
        Self {
            vk: 0,
            command_pool: 0,
            queue,
            target_framebuffer: ptr::null(),
            active_compute_pipeline_layout: ptr::null(),
            active_graphics_pipeline_layout: ptr::null(),
            active_raytracing_pipeline_layout: ptr::null(),
            active_render_pass: 0,
        }
    }

    pub fn begin(&mut self) {
        todo!("begin Vulkan command list")
    }

    pub fn end(&mut self) {
        todo!("end Vulkan command list")
    }

    pub fn barriers(
        &mut self,
        stages: RenderBarrierStages,
        buffer_barriers: *const RenderBufferBarrier,
        buffer_barriers_count: u32,
        texture_barriers: *const RenderTextureBarrier,
        texture_barriers_count: u32,
    ) {
        todo!("record Vulkan barriers")
    }

    pub fn dispatch(&mut self, thread_group_count_x: u32, thread_group_count_y: u32, thread_group_count_z: u32) {
        todo!("dispatch Vulkan compute")
    }

    pub fn trace_rays(
        &mut self,
        width: u32,
        height: u32,
        depth: u32,
        shader_binding_table: RenderBufferReference,
        shader_binding_groups_info: &RenderShaderBindingGroupsInfo,
    ) {
        todo!("trace Vulkan rays")
    }

    pub fn draw_instanced(
        &mut self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    ) {
        todo!("draw Vulkan instanced")
    }

    pub fn draw_indexed_instanced(
        &mut self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    ) {
        todo!("draw Vulkan indexed instanced")
    }

    pub fn set_pipeline(&mut self, pipeline: *const RenderPipeline) {
        todo!("set Vulkan pipeline")
    }

    pub fn set_compute_pipeline_layout(&mut self, pipeline_layout: *const RenderPipelineLayout) {
        todo!("set Vulkan compute pipeline layout")
    }

    pub fn set_compute_push_constants(&mut self, range_index: u32, data: *const u8, offset: u32, size: u32) {
        todo!("set Vulkan compute push constants")
    }

    pub fn set_compute_descriptor_set(&mut self, descriptor_set: *mut RenderDescriptorSet, set_index: u32) {
        todo!("set Vulkan compute descriptor set")
    }

    pub fn set_graphics_pipeline_layout(&mut self, pipeline_layout: *const RenderPipelineLayout) {
        todo!("set Vulkan graphics pipeline layout")
    }

    pub fn set_graphics_push_constants(&mut self, range_index: u32, data: *const u8, offset: u32, size: u32) {
        todo!("set Vulkan graphics push constants")
    }

    pub fn set_graphics_descriptor_set(&mut self, descriptor_set: *mut RenderDescriptorSet, set_index: u32) {
        todo!("set Vulkan graphics descriptor set")
    }

    pub fn set_graphics_root_descriptor(&mut self, buffer_reference: RenderBufferReference, root_descriptor_index: u32) {
        todo!("set Vulkan graphics root descriptor")
    }

    pub fn set_raytracing_pipeline_layout(&mut self, pipeline_layout: *const RenderPipelineLayout) {
        todo!("set Vulkan raytracing pipeline layout")
    }

    pub fn set_raytracing_push_constants(&mut self, range_index: u32, data: *const u8, offset: u32, size: u32) {
        todo!("set Vulkan raytracing push constants")
    }

    pub fn set_raytracing_descriptor_set(&mut self, descriptor_set: *mut RenderDescriptorSet, set_index: u32) {
        todo!("set Vulkan raytracing descriptor set")
    }

    pub fn set_index_buffer(&mut self, view: *const RenderIndexBufferView) {
        todo!("set Vulkan index buffer")
    }

    pub fn set_vertex_buffers(
        &mut self,
        start_slot: u32,
        views: *const RenderVertexBufferView,
        view_count: u32,
        input_slots: *const RenderInputSlot,
    ) {
        todo!("set Vulkan vertex buffers")
    }

    pub fn set_viewports(&mut self, viewports: *const RenderViewport, count: u32) {
        todo!("set Vulkan viewports")
    }

    pub fn set_scissors(&mut self, scissor_rects: *const RenderRect, count: u32) {
        todo!("set Vulkan scissors")
    }

    pub fn set_framebuffer(&mut self, framebuffer: *const RenderFramebuffer) {
        todo!("set Vulkan framebuffer")
    }

    pub fn set_depth_bias(&mut self, depth_bias: f32, depth_bias_clamp: f32, slope_scaled_depth_bias: f32) {
        todo!("set Vulkan depth bias")
    }

    pub fn clear_color(
        &mut self,
        attachment_index: u32,
        color_value: RenderColor,
        clear_rects: *const RenderRect,
        clear_rects_count: u32,
    ) {
        todo!("clear Vulkan color")
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
        todo!("clear Vulkan depth stencil")
    }

    pub fn copy_buffer_region(
        &mut self,
        dst_buffer: RenderBufferReference,
        src_buffer: RenderBufferReference,
        size: u64,
    ) {
        todo!("copy Vulkan buffer region")
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
        todo!("copy Vulkan texture region")
    }

    pub fn copy_buffer(&mut self, dst_buffer: *const RenderBuffer, src_buffer: *const RenderBuffer) {
        todo!("copy Vulkan buffer")
    }

    pub fn copy_texture(&mut self, dst_texture: *const RenderTexture, src_texture: *const RenderTexture) {
        todo!("copy Vulkan texture")
    }

    pub fn resolve_texture(&mut self, dst_texture: *const RenderTexture, src_texture: *const RenderTexture) {
        todo!("resolve Vulkan texture")
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
        todo!("resolve Vulkan texture region")
    }

    pub fn build_bottom_level_as(
        &mut self,
        dst_acceleration_structure: *const RenderAccelerationStructure,
        scratch_buffer: RenderBufferReference,
        build_info: &RenderBottomLevelASBuildInfo,
    ) {
        todo!("build Vulkan bottom level AS")
    }

    pub fn build_top_level_as(
        &mut self,
        dst_acceleration_structure: *const RenderAccelerationStructure,
        scratch_buffer: RenderBufferReference,
        instances_buffer: RenderBufferReference,
        build_info: &RenderTopLevelASBuildInfo,
    ) {
        todo!("build Vulkan top level AS")
    }

    pub fn discard_texture(&mut self, texture: *const RenderTexture) {
        todo!("discard Vulkan texture")
    }

    pub fn reset_query_pool(&mut self, query_pool: *const RenderQueryPool, query_first_index: u32, query_count: u32) {
        todo!("reset Vulkan query pool")
    }

    pub fn write_timestamp(&mut self, query_pool: *const RenderQueryPool, query_index: u32) {
        todo!("write Vulkan timestamp")
    }

    pub fn check_active_render_pass(&mut self) {
        todo!("check Vulkan active render pass")
    }

    pub fn end_active_render_pass(&mut self) {
        todo!("end Vulkan active render pass")
    }

    pub fn set_descriptor_set(
        &mut self,
        bind_point: u32,
        pipeline_layout: *const VulkanPipelineLayout,
        descriptor_set: *const RenderDescriptorSet,
        set_index: u32,
    ) {
        todo!("set Vulkan descriptor set")
    }
}

pub struct VulkanCommandFence {
    pub vk: vk::Fence,
    pub device: *mut VulkanDevice,
}

impl VulkanCommandFence {
    pub fn new(device: *mut VulkanDevice) -> Self {
        Self { vk: 0, device }
    }
}

pub struct VulkanCommandSemaphore {
    pub vk: vk::Semaphore,
    pub device: *mut VulkanDevice,
}

impl VulkanCommandSemaphore {
    pub fn new(device: *mut VulkanDevice) -> Self {
        Self { vk: 0, device }
    }
}

pub struct VulkanCommandQueue {
    pub queue: *mut VulkanQueue,
    pub device: *mut VulkanDevice,
    pub family_index: u32,
    pub queue_index: u32,
    pub swap_chains: HashSet<*mut VulkanSwapChain>,
    pub command_list_type: RenderCommandListType,
}

impl VulkanCommandQueue {
    pub fn new(device: *mut VulkanDevice, command_list_type: RenderCommandListType) -> Self {
        Self {
            queue: ptr::null_mut(),
            device,
            family_index: 0,
            queue_index: 0,
            swap_chains: HashSet::new(),
            command_list_type,
        }
    }

    pub fn create_command_list(&mut self) -> Box<VulkanCommandList> {
        todo!("create Vulkan command list")
    }

    pub fn create_swap_chain(&mut self, desc: &RenderSwapChainDesc) -> Box<VulkanSwapChain> {
        todo!("create Vulkan swapchain")
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
        todo!("execute Vulkan command lists")
    }

    pub fn wait_for_command_fence(&mut self, fence: *mut RenderCommandFence) {
        todo!("wait for Vulkan command fence")
    }
}

pub struct VulkanPool {
    pub vk: VmaPool,
    pub device: *mut VulkanDevice,
}

impl VulkanPool {
    pub fn new(device: *mut VulkanDevice, desc: &RenderPoolDesc) -> Self {
        Self { vk: 0, device }
    }

    pub fn create_buffer(&mut self, desc: &RenderBufferDesc) -> Box<VulkanBuffer> {
        todo!("create Vulkan pool buffer")
    }

    pub fn create_texture(&mut self, desc: &RenderTextureDesc) -> Box<VulkanTexture> {
        todo!("create Vulkan pool texture")
    }
}

pub struct VulkanQueue {
    pub vk: vk::Queue,
    pub mutex: std::sync::Mutex<()>,
    pub virtual_queues: HashSet<*const VulkanCommandQueue>,
}

impl VulkanQueue {
    pub fn new(vk: vk::Queue) -> Self {
        Self {
            vk,
            mutex: std::sync::Mutex::new(()),
            virtual_queues: HashSet::new(),
        }
    }
}

pub struct VulkanQueueFamily {
    pub queues: Vec<VulkanQueue>,
}

impl VulkanQueueFamily {
    pub fn add(&mut self, virtual_queue: *mut VulkanCommandQueue) {
        todo!("add Vulkan virtual queue")
    }

    pub fn remove(&mut self, virtual_queue: *mut VulkanCommandQueue) {
        todo!("remove Vulkan virtual queue")
    }
}

pub struct VulkanDevice {
    pub vk: vk::Device,
    pub render_interface: *mut VulkanInterface,
    pub physical_device: vk::PhysicalDevice,
    pub physical_device_properties: vk::PhysicalDeviceProperties,
    pub allocator: VmaAllocator,
    pub queue_family_indices: [u32; 3],
    pub queue_families: Vec<VulkanQueueFamily>,
    pub capabilities: RenderDeviceCapabilities,
    pub description: RenderDeviceDescription,
    pub rt_pipeline_properties: vk::PhysicalDeviceRayTracingPipelinePropertiesKHR,
    pub sample_location_properties: vk::PhysicalDeviceSampleLocationsPropertiesEXT,
    pub null_buffer: Option<Box<RenderBuffer>>,
    pub load_store_op_none_supported: bool,
    pub null_descriptor_supported: bool,
}

impl VulkanDevice {
    pub fn new(render_interface: *mut VulkanInterface, preferred_device_name: &str) -> Self {
        Self {
            vk: 0,
            render_interface,
            physical_device: 0,
            physical_device_properties: vk::PhysicalDeviceProperties::default(),
            allocator: 0,
            queue_family_indices: [0; 3],
            queue_families: Vec::new(),
            capabilities: RenderDeviceCapabilities::default(),
            description: RenderDeviceDescription::default(),
            rt_pipeline_properties: vk::PhysicalDeviceRayTracingPipelinePropertiesKHR::default(),
            sample_location_properties: vk::PhysicalDeviceSampleLocationsPropertiesEXT::default(),
            null_buffer: None,
            load_store_op_none_supported: false,
            null_descriptor_supported: false,
        }
    }

    pub fn create_descriptor_set(&mut self, desc: &RenderDescriptorSetDesc) -> Box<VulkanDescriptorSet> {
        todo!("create Vulkan descriptor set")
    }

    pub fn create_shader(
        &mut self,
        data: *const u8,
        size: u64,
        entry_point_name: &str,
        format: RenderShaderFormat,
    ) -> Box<VulkanShader> {
        todo!("create Vulkan shader")
    }

    pub fn create_sampler(&mut self, desc: &RenderSamplerDesc) -> Box<VulkanSampler> {
        todo!("create Vulkan sampler")
    }

    pub fn create_compute_pipeline(&mut self, desc: &RenderComputePipelineDesc) -> Box<VulkanComputePipeline> {
        todo!("create Vulkan compute pipeline")
    }

    pub fn create_graphics_pipeline(&mut self, desc: &RenderGraphicsPipelineDesc) -> Box<VulkanGraphicsPipeline> {
        todo!("create Vulkan graphics pipeline")
    }

    pub fn create_raytracing_pipeline(
        &mut self,
        desc: &RenderRaytracingPipelineDesc,
        previous_pipeline: Option<&RenderPipeline>,
    ) -> Box<VulkanRaytracingPipeline> {
        todo!("create Vulkan raytracing pipeline")
    }

    pub fn create_command_queue(&mut self, command_list_type: RenderCommandListType) -> Box<VulkanCommandQueue> {
        todo!("create Vulkan command queue")
    }

    pub fn create_buffer(&mut self, desc: &RenderBufferDesc) -> Box<VulkanBuffer> {
        todo!("create Vulkan buffer")
    }

    pub fn create_texture(&mut self, desc: &RenderTextureDesc) -> Box<VulkanTexture> {
        todo!("create Vulkan texture")
    }

    pub fn create_acceleration_structure(&mut self, desc: &RenderAccelerationStructureDesc) -> Box<VulkanAccelerationStructure> {
        todo!("create Vulkan acceleration structure")
    }

    pub fn create_pool(&mut self, desc: &RenderPoolDesc) -> Box<VulkanPool> {
        todo!("create Vulkan pool")
    }

    pub fn create_pipeline_layout(&mut self, desc: &RenderPipelineLayoutDesc) -> Box<VulkanPipelineLayout> {
        todo!("create Vulkan pipeline layout")
    }

    pub fn create_command_fence(&mut self) -> Box<VulkanCommandFence> {
        todo!("create Vulkan command fence")
    }

    pub fn create_command_semaphore(&mut self) -> Box<VulkanCommandSemaphore> {
        todo!("create Vulkan command semaphore")
    }

    pub fn create_framebuffer(&mut self, desc: &RenderFramebufferDesc) -> Box<VulkanFramebuffer> {
        todo!("create Vulkan framebuffer")
    }

    pub fn create_query_pool(&mut self, query_count: u32) -> Box<VulkanQueryPool> {
        todo!("create Vulkan query pool")
    }

    pub fn set_bottom_level_as_build_info(
        &mut self,
        build_info: &mut RenderBottomLevelASBuildInfo,
        meshes: *const RenderBottomLevelASMesh,
        mesh_count: u32,
        prefer_fast_build: bool,
        prefer_fast_trace: bool,
    ) {
        todo!("set Vulkan bottom level AS build info")
    }

    pub fn set_top_level_as_build_info(
        &mut self,
        build_info: &mut RenderTopLevelASBuildInfo,
        instances: *const RenderTopLevelASInstance,
        instance_count: u32,
        prefer_fast_build: bool,
        prefer_fast_trace: bool,
    ) {
        todo!("set Vulkan top level AS build info")
    }

    pub fn set_shader_binding_table_info(
        &mut self,
        table_info: &mut RenderShaderBindingTableInfo,
        groups: &RenderShaderBindingGroups,
        pipeline: *const RenderPipeline,
        descriptor_sets: *mut *mut RenderDescriptorSet,
        descriptor_set_count: u32,
    ) {
        todo!("set Vulkan shader binding table info")
    }

    pub fn get_capabilities(&self) -> RenderDeviceCapabilities {
        self.capabilities
    }

    pub fn get_description(&self) -> RenderDeviceDescription {
        self.description
    }

    pub fn get_sample_counts_supported(&self, format: RenderFormat) -> RenderSampleCounts {
        todo!("get Vulkan sample counts")
    }

    pub fn release(&mut self) {
        todo!("release Vulkan device")
    }

    pub fn is_valid(&self) -> bool {
        self.vk != 0
    }

    pub fn begin_capture(&mut self) -> bool {
        todo!("begin Vulkan capture")
    }

    pub fn end_capture(&mut self) -> bool {
        todo!("end Vulkan capture")
    }
}

pub struct VulkanInterface {
    pub instance: vk::Instance,
    pub app_info: vk::ApplicationInfo,
    pub capabilities: RenderInterfaceCapabilities,
    pub device_names: Vec<String>,
}

impl VulkanInterface {
    pub fn new() -> Self {
        Self {
            instance: 0,
            app_info: vk::ApplicationInfo::default(),
            capabilities: RenderInterfaceCapabilities::default(),
            device_names: Vec::new(),
        }
    }

    pub fn create_device(&mut self, preferred_device_name: &str) -> Box<VulkanDevice> {
        todo!("create Vulkan device")
    }

    pub fn get_capabilities(&self) -> RenderInterfaceCapabilities {
        self.capabilities
    }

    pub fn get_device_names(&self) -> &[String] {
        &self.device_names
    }

    pub fn is_valid(&self) -> bool {
        self.instance != 0
    }
}

#[derive(Clone, Copy, Default)]
pub struct RenderFormat;

#[derive(Clone, Copy, Default)]
pub struct RenderPoolDesc;

#[derive(Clone, Copy, Default)]
pub struct RenderBox;
