#include <stdio.h>

#include "wrapper.h"

#if IREE_RS_VULKAN
#include "iree/hal/drivers/vulkan/allocator.h"
#endif

iree_allocator_t iree_rs_allocator_system(void) {
  return iree_allocator_system();
}
iree_allocator_t iree_rs_allocator_null(void) {
  return iree_allocator_null();
}
iree_status_code_t iree_rs_status_code(iree_status_t status) {
  return iree_status_code(status);
}
void iree_rs_status_free(iree_status_t status) {
  iree_status_free(status);
}
iree_timeout_t iree_rs_infinite_timeout(void) {
  return iree_infinite_timeout();
}
iree_hal_device_create_params_t iree_rs_device_create_params_default(void) {
  return iree_hal_device_create_params_default();
}
iree_status_t iree_rs_proactor_pool_create_default(
    iree_allocator_t host_allocator, iree_async_proactor_pool_t** out_pool) {
  return iree_async_proactor_pool_create(
      iree_numa_node_count(), /*node_ids=*/NULL,
      iree_async_proactor_pool_options_default(), host_allocator, out_pool);
}
iree_async_frontier_tracker_options_t iree_rs_frontier_tracker_options_default(
    void) {
  return iree_async_frontier_tracker_options_default();
}
iree_vm_type_def_t iree_rs_vm_undefined_type_def(void) {
  return iree_vm_make_undefined_type_def();
}
iree_vm_ref_type_t iree_rs_hal_buffer_view_type(void) {
  return iree_hal_buffer_view_type();
}
iree_status_t iree_rs_device_hal_module_create(iree_vm_instance_t* instance,
                                             iree_hal_device_t* device,
                                             iree_allocator_t host_allocator,
                                             iree_vm_module_t** out_module) {
  *out_module = NULL;
  iree_async_frontier_tracker_t* tracker = NULL;
  IREE_RETURN_IF_ERROR(iree_async_frontier_tracker_create(
      iree_async_frontier_tracker_options_default(), host_allocator,
      &tracker));
  iree_hal_device_group_t* group = NULL;
  iree_status_t status = iree_hal_device_group_create_from_device(
      device, tracker, host_allocator, &group);
  iree_async_frontier_tracker_release(tracker);
  if (iree_status_is_ok(status)) {
    status = iree_hal_module_create(
        instance, iree_hal_module_device_policy_default(), group,
        IREE_HAL_MODULE_FLAG_NONE, iree_hal_module_debug_sink_stdio(stderr),
        host_allocator, out_module);
  }
  iree_hal_device_group_release(group);
  return status;
}
iree_host_size_t iree_rs_status_message(iree_status_t status, char* destination,
                                      iree_host_size_t capacity) {
  iree_host_size_t length = 0;
  iree_status_format(status, capacity, destination, &length);
  if (capacity && destination) destination[capacity - 1] = 0;
  iree_status_free(status);
  return length;
}
iree_hal_buffer_t* iree_rs_buffer_allocated_buffer(iree_hal_buffer_t* buffer) {
  return iree_hal_buffer_allocated_buffer(buffer);
}
iree_device_size_t iree_rs_buffer_byte_offset(iree_hal_buffer_t* buffer) {
  return iree_hal_buffer_byte_offset(buffer);
}
iree_device_size_t iree_rs_buffer_byte_length(iree_hal_buffer_t* buffer) {
  return iree_hal_buffer_byte_length(buffer);
}
iree_device_size_t iree_rs_buffer_allocation_size(iree_hal_buffer_t* buffer) {
  return iree_hal_buffer_allocation_size(buffer);
}
void iree_rs_hal_buffer_retain(iree_hal_buffer_t* buffer) {
  iree_hal_buffer_retain(buffer);
}
void iree_rs_hal_buffer_release(iree_hal_buffer_t* buffer) {
  iree_hal_buffer_release(buffer);
}
void iree_rs_hal_buffer_view_retain(iree_hal_buffer_view_t* view) {
  iree_hal_buffer_view_retain(view);
}
void iree_rs_hal_buffer_view_release(iree_hal_buffer_view_t* view) {
  iree_hal_buffer_view_release(view);
}
void iree_rs_hal_device_retain(iree_hal_device_t* device) {
  iree_hal_device_retain(device);
}
void iree_rs_hal_device_release(iree_hal_device_t* device) {
  iree_hal_device_release(device);
}

#if IREE_RS_VULKAN
iree_status_t iree_rs_vulkan_allocate_dedicated_buffer(
    iree_hal_device_t* device, iree_hal_buffer_params_t params,
    iree_device_size_t allocation_size, iree_hal_buffer_t** out_buffer) {
  if (!device || !out_buffer) {
    return iree_make_status(IREE_STATUS_INVALID_ARGUMENT,
                            "device and out_buffer are required");
  }
  *out_buffer = NULL;
  // Exported entry point that rejects non-Vulkan devices before any cast.
  iree_host_size_t property_count = 0;
  IREE_RETURN_IF_ERROR(iree_hal_vulkan_device_query_cooperative_matrix_properties(
      device, /*property_capacity=*/0, &property_count, /*out_properties=*/NULL));
  // Same canonicalization as public iree_hal_allocator_allocate_buffer.
  iree_hal_buffer_params_canonicalize(&params);
  iree_hal_buffer_t* buffer = NULL;
  IREE_RETURN_IF_ERROR(iree_hal_vulkan_allocator_allocate_direct_buffer(
      iree_hal_device_allocator(device), &params, allocation_size, &buffer));
  if (!iree_hal_vulkan_buffer_isa(buffer)) {
    iree_hal_buffer_release(buffer);
    return iree_make_status(
        IREE_STATUS_OUT_OF_RANGE,
        "dedicated allocation of %" PRIdsz " bytes requires a sparse buffer",
        allocation_size);
  }
  *out_buffer = buffer;
  return iree_ok_status();
}
#endif