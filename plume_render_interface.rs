//
// plume
//
// Copyright (c) 2024 renderbag and contributors. All rights reserved.
// Licensed under the MIT license. See LICENSE file for details.
//

#![allow(dead_code, unused_variables)]

use crate::plume_render_interface_types::*;

pub trait RenderBufferFormattedView {}

pub trait RenderBuffer {
    fn map(&mut self, subresource: u32, read_range: Option<&RenderRange>) -> *mut u8;
    fn unmap(&mut self, subresource: u32, written_range: Option<&RenderRange>);
    fn create_buffer_formatted_view(&self, format: RenderFormat) -> Box<dyn RenderBufferFormattedView>;
    fn set_name(&mut self, name: &str);
    fn get_device_address(&self) -> u64;

    fn at(&self, offset: u64) -> RenderBufferReference {
        RenderBufferReference::new(self as *const dyn RenderBuffer as *const RenderBuffer, offset)
    }
}

pub trait RenderTextureView {}

pub trait RenderTexture {
    fn create_texture_view(&self, desc: &RenderTextureViewDesc) -> Box<dyn RenderTextureView>;
    fn set_name(&mut self, name: &str);
}

pub trait RenderAccelerationStructure {}

pub trait RenderShader {
    fn set_name(&mut self, name: &str);
}

pub trait RenderSampler {}

pub trait RenderPipeline {
    fn set_name(&mut self, name: &str);
    fn get_program(&self, name: &str) -> RenderPipelineProgram;
}

pub trait RenderPipelineLayout {}

pub trait RenderCommandFence {}

pub trait RenderCommandSemaphore {}

pub trait RenderDescriptorSet {
    fn set_buffer(
        &mut self,
        descriptor_index: u32,
        buffer: &dyn RenderBuffer,
        buffer_size: u64,
        buffer_structured_view: Option<&RenderBufferStructuredView>,
        buffer_formatted_view: Option<&dyn RenderBufferFormattedView>,
    );
    fn set_texture(
        &mut self,
        descriptor_index: u32,
        texture: &dyn RenderTexture,
        texture_layout: RenderTextureLayout,
        texture_view: Option<&dyn RenderTextureView>,
    );
    fn set_sampler(&mut self, descriptor_index: u32, sampler: &dyn RenderSampler);
    fn set_acceleration_structure(
        &mut self,
        descriptor_index: u32,
        acceleration_structure: &dyn RenderAccelerationStructure,
    );
}

pub trait RenderSwapChain {
    fn present(
        &mut self,
        texture_index: u32,
        wait_semaphores: &mut [Option<&mut dyn RenderCommandSemaphore>],
    ) -> bool;
    fn wait(&mut self);
    fn resize(&mut self) -> bool;
    fn needs_resize(&self) -> bool;
    fn set_vsync_enabled(&mut self, vsync_enabled: bool);
    fn is_vsync_enabled(&self) -> bool;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_texture(&mut self, texture_index: u32) -> &mut dyn RenderTexture;
    fn get_texture_count(&self) -> u32;
    fn acquire_texture(&mut self, signal_semaphore: &mut dyn RenderCommandSemaphore, texture_index: &mut u32) -> bool;
    fn get_window(&self) -> RenderWindow;
    fn is_empty(&self) -> bool;
    fn get_refresh_rate(&self) -> u32;
}

pub trait RenderFramebuffer {
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
}

pub trait RenderCommandList {
    fn begin(&mut self);
    fn end(&mut self);
    fn barriers(
        &mut self,
        stages: RenderBarrierStages,
        buffer_barriers: Option<&[RenderBufferBarrier]>,
        texture_barriers: Option<&[RenderTextureBarrier]>,
    );
    fn dispatch(&mut self, thread_group_count_x: u32, thread_group_count_y: u32, thread_group_count_z: u32);
    fn trace_rays(
        &mut self,
        width: u32,
        height: u32,
        depth: u32,
        shader_binding_table: RenderBufferReference,
        shader_binding_groups_info: &RenderShaderBindingGroupsInfo,
    );
    fn draw_instanced(
        &mut self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    );
    fn draw_indexed_instanced(
        &mut self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    );
    fn set_pipeline(&mut self, pipeline: &dyn RenderPipeline);
    fn set_compute_pipeline_layout(&mut self, pipeline_layout: &dyn RenderPipelineLayout);
    fn set_compute_push_constants(&mut self, range_index: u32, data: &[u8], offset: u32, size: u32);
    fn set_compute_descriptor_set(&mut self, descriptor_set: &mut dyn RenderDescriptorSet, set_index: u32);
    fn set_graphics_pipeline_layout(&mut self, pipeline_layout: &dyn RenderPipelineLayout);
    fn set_graphics_push_constants(&mut self, range_index: u32, data: &[u8], offset: u32, size: u32);
    fn set_graphics_descriptor_set(&mut self, descriptor_set: &mut dyn RenderDescriptorSet, set_index: u32);
    fn set_graphics_root_descriptor(&mut self, buffer_reference: RenderBufferReference, root_descriptor_index: u32);
    fn set_raytracing_pipeline_layout(&mut self, pipeline_layout: &dyn RenderPipelineLayout);
    fn set_raytracing_push_constants(&mut self, range_index: u32, data: &[u8], offset: u32, size: u32);
    fn set_raytracing_descriptor_set(&mut self, descriptor_set: &mut dyn RenderDescriptorSet, set_index: u32);
    fn set_index_buffer(&mut self, view: &RenderIndexBufferView);
    fn set_vertex_buffers(&mut self, start_slot: u32, views: &[RenderVertexBufferView], input_slots: &[RenderInputSlot]);
    fn set_viewports(&mut self, viewports: &[RenderViewport]);
    fn set_scissors(&mut self, scissor_rects: &[RenderRect]);
    fn set_framebuffer(&mut self, framebuffer: &dyn RenderFramebuffer);
    fn set_depth_bias(&mut self, depth_bias: f32, depth_bias_clamp: f32, slope_scaled_depth_bias: f32);
    fn clear_color(&mut self, attachment_index: u32, color_value: RenderColor, clear_rects: Option<&[RenderRect]>);
    fn clear_depth_stencil(
        &mut self,
        clear_depth: bool,
        clear_stencil: bool,
        depth_value: f32,
        stencil_value: u32,
        clear_rects: Option<&[RenderRect]>,
    );
    fn copy_buffer_region(&mut self, dst_buffer: RenderBufferReference, src_buffer: RenderBufferReference, size: u64);
    fn copy_texture_region(
        &mut self,
        dst_location: &RenderTextureCopyLocation,
        src_location: &RenderTextureCopyLocation,
        dst_x: u32,
        dst_y: u32,
        dst_z: u32,
        src_box: Option<&RenderBox>,
    );
    fn copy_buffer(&mut self, dst_buffer: &dyn RenderBuffer, src_buffer: &dyn RenderBuffer);
    fn copy_texture(&mut self, dst_texture: &dyn RenderTexture, src_texture: &dyn RenderTexture);
    fn resolve_texture(&mut self, dst_texture: &dyn RenderTexture, src_texture: &dyn RenderTexture);
    fn resolve_texture_region(
        &mut self,
        dst_texture: &dyn RenderTexture,
        dst_x: u32,
        dst_y: u32,
        src_texture: &dyn RenderTexture,
        src_rect: Option<&RenderRect>,
        resolve_mode: RenderResolveMode,
    );
    fn build_bottom_level_as(
        &mut self,
        dst_acceleration_structure: &dyn RenderAccelerationStructure,
        scratch_buffer: RenderBufferReference,
        build_info: &RenderBottomLevelASBuildInfo,
    );
    fn build_top_level_as(
        &mut self,
        dst_acceleration_structure: &dyn RenderAccelerationStructure,
        scratch_buffer: RenderBufferReference,
        instances_buffer: RenderBufferReference,
        build_info: &RenderTopLevelASBuildInfo,
    );
    fn discard_texture(&mut self, texture: &dyn RenderTexture);
    fn reset_query_pool(&mut self, query_pool: &RenderQueryPool, query_first_index: u32, query_count: u32);
    fn write_timestamp(&mut self, query_pool: &RenderQueryPool, query_index: u32);

    fn barriers_with_buffer(&mut self, stages: RenderBarrierStages, buffer_barrier: RenderBufferBarrier) {
        self.barriers(stages, Some(std::slice::from_ref(&buffer_barrier)), None);
    }

    fn barriers_with_texture(&mut self, stages: RenderBarrierStages, texture_barrier: RenderTextureBarrier) {
        self.barriers(stages, None, Some(std::slice::from_ref(&texture_barrier)));
    }

    fn barriers_with_both(
        &mut self,
        stages: RenderBarrierStages,
        buffer_barrier: RenderBufferBarrier,
        texture_barrier: RenderTextureBarrier,
    ) {
        self.barriers(
            stages,
            Some(std::slice::from_ref(&buffer_barrier)),
            Some(std::slice::from_ref(&texture_barrier)),
        );
    }

    fn barriers_with_buffers(&mut self, stages: RenderBarrierStages, buffer_barriers: &[RenderBufferBarrier]) {
        self.barriers(stages, Some(buffer_barriers), None);
    }

    fn barriers_with_textures(&mut self, stages: RenderBarrierStages, texture_barriers: &[RenderTextureBarrier]) {
        self.barriers(stages, None, Some(texture_barriers));
    }

    fn barriers_with_slices(
        &mut self,
        stages: RenderBarrierStages,
        buffer_barriers: &[RenderBufferBarrier],
        texture_barriers: &[RenderTextureBarrier],
    ) {
        self.barriers(stages, Some(buffer_barriers), Some(texture_barriers));
    }

    fn set_viewport(&mut self, viewport: RenderViewport) {
        self.set_viewports(std::slice::from_ref(&viewport));
    }

    fn set_scissor(&mut self, scissor_rect: RenderRect) {
        self.set_scissors(std::slice::from_ref(&scissor_rect));
    }

    fn clear_depth(&mut self, clear_depth: bool, depth_value: f32, clear_rects: Option<&[RenderRect]>) {
        self.clear_depth_stencil(clear_depth, false, depth_value, 0, clear_rects);
    }
}

pub trait RenderCommandQueue {
    fn create_command_list(&mut self) -> Box<dyn RenderCommandList>;
    fn create_swap_chain(&mut self, desc: &RenderSwapChainDesc) -> Box<dyn RenderSwapChain>;
    fn execute_command_lists(
        &mut self,
        command_lists: &[&dyn RenderCommandList],
        wait_semaphores: Option<&mut [Option<&mut dyn RenderCommandSemaphore>]>,
        signal_semaphores: Option<&mut [Option<&mut dyn RenderCommandSemaphore>]>,
        signal_fence: Option<&mut dyn RenderCommandFence>,
    );
    fn wait_for_command_fence(&mut self, fence: &mut dyn RenderCommandFence);

    fn execute_command_list(&mut self, command_list: &dyn RenderCommandList, signal_fence: Option<&mut dyn RenderCommandFence>) {
        self.execute_command_lists(&[command_list], None, None, signal_fence);
    }
}

pub trait RenderPool {
    fn create_buffer(&mut self, desc: &RenderBufferDesc) -> Box<dyn RenderBuffer>;
    fn create_texture(&mut self, desc: &RenderTextureDesc) -> Box<dyn RenderTexture>;
}

pub trait RenderQueryPool {
    fn query_results(&mut self);
    fn get_results(&self) -> &[u64];
    fn get_count(&self) -> u32;
}

pub trait RenderDevice {
    fn create_descriptor_set(&mut self, desc: &RenderDescriptorSetDesc) -> Box<dyn RenderDescriptorSet>;
    fn create_shader(&mut self, data: &[u8], entry_point_name: &str, format: RenderShaderFormat) -> Box<dyn RenderShader>;
    fn create_sampler(&mut self, desc: &RenderSamplerDesc) -> Box<dyn RenderSampler>;
    fn create_compute_pipeline(&mut self, desc: &RenderComputePipelineDesc) -> Box<dyn RenderPipeline>;
    fn create_graphics_pipeline(&mut self, desc: &RenderGraphicsPipelineDesc) -> Box<dyn RenderPipeline>;
    fn create_raytracing_pipeline(
        &mut self,
        desc: &RenderRaytracingPipelineDesc,
        previous_pipeline: Option<&dyn RenderPipeline>,
    ) -> Box<dyn RenderPipeline>;
    fn create_command_queue(&mut self, queue_type: RenderCommandListType) -> Box<dyn RenderCommandQueue>;
    fn create_buffer(&mut self, desc: &RenderBufferDesc) -> Box<dyn RenderBuffer>;
    fn create_texture(&mut self, desc: &RenderTextureDesc) -> Box<dyn RenderTexture>;
    fn create_acceleration_structure(
        &mut self,
        desc: &RenderAccelerationStructureDesc,
    ) -> Box<dyn RenderAccelerationStructure>;
    fn create_pool(&mut self, desc: &RenderPoolDesc) -> Box<dyn RenderPool>;
    fn create_pipeline_layout(&mut self, desc: &RenderPipelineLayoutDesc) -> Box<dyn RenderPipelineLayout>;
    fn create_command_fence(&mut self) -> Box<dyn RenderCommandFence>;
    fn create_command_semaphore(&mut self) -> Box<dyn RenderCommandSemaphore>;
    fn create_framebuffer(&mut self, desc: &RenderFramebufferDesc) -> Box<dyn RenderFramebuffer>;
    fn create_query_pool(&mut self, query_count: u32) -> Box<dyn RenderQueryPool>;
    fn set_bottom_level_as_build_info(
        &mut self,
        build_info: &mut RenderBottomLevelASBuildInfo,
        meshes: &[RenderBottomLevelASMesh],
        prefer_fast_build: bool,
        prefer_fast_trace: bool,
    );
    fn set_top_level_as_build_info(
        &mut self,
        build_info: &mut RenderTopLevelASBuildInfo,
        instances: &[RenderTopLevelASInstance],
        prefer_fast_build: bool,
        prefer_fast_trace: bool,
    );
    fn set_shader_binding_table_info(
        &mut self,
        table_info: &mut RenderShaderBindingTableInfo,
        groups: &RenderShaderBindingGroups,
        pipeline: &dyn RenderPipeline,
        descriptor_sets: &mut [&mut dyn RenderDescriptorSet],
    );
    fn get_capabilities(&self) -> RenderDeviceCapabilities;
    fn get_description(&self) -> RenderDeviceDescription;
    fn get_sample_counts_supported(&self, format: RenderFormat) -> RenderSampleCounts;
    fn begin_capture(&mut self) -> bool;
    fn end_capture(&mut self) -> bool;
}

pub trait RenderInterface {
    fn create_device(&mut self, preferred_device_name: &str) -> Box<dyn RenderDevice>;
    fn get_device_names(&self) -> &[String];
    fn get_capabilities(&self) -> RenderInterfaceCapabilities;
}

pub fn render_interface_test(_render_interface: &mut dyn RenderInterface) {
    todo!("RenderInterfaceTest")
}

pub fn test_initialize(_render_interface: &mut dyn RenderInterface, _window: RenderWindow) {
    todo!("TestInitialize")
}

pub fn test_draw() {
    todo!("TestDraw")
}

pub fn test_resize() {
    todo!("TestResize")
}

pub fn test_shutdown() {
    todo!("TestShutdown")
}
