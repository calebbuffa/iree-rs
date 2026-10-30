#include "wrapper.h"

iree_status_t iree_native_hal_device_queue_barrier(
    iree_hal_device_t *device, iree_hal_queue_affinity_t queue_affinity,
    const iree_hal_semaphore_list_t *wait_semaphore_list,
    const iree_hal_semaphore_list_t *signal_semaphore_list,
    iree_hal_execute_flags_t flags) {
  return iree_hal_device_queue_barrier(device, queue_affinity,
                                       *wait_semaphore_list,
                                       *signal_semaphore_list, flags);
}

iree_status_t iree_native_hal_device_queue_execute(
    iree_hal_device_t *device, iree_hal_queue_affinity_t queue_affinity,
    const iree_hal_semaphore_list_t *wait_semaphore_list,
    const iree_hal_semaphore_list_t *signal_semaphore_list,
    iree_hal_command_buffer_t *command_buffer,
    const iree_hal_buffer_binding_table_t *binding_table,
    iree_hal_execute_flags_t flags) {
  return iree_hal_device_queue_execute(
      device, queue_affinity, *wait_semaphore_list, *signal_semaphore_list,
      command_buffer, *binding_table, flags);
}
