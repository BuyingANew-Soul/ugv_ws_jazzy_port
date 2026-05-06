// generated from rosidl_typesupport_introspection_c/resource/idl__type_support.c.em
// with input from ugv_interface:srv/MapSave.idl
// generated code does not contain a copyright notice

#include <stddef.h>
#include "ugv_interface/srv/detail/map_save__rosidl_typesupport_introspection_c.h"
#include "ugv_interface/msg/rosidl_typesupport_introspection_c__visibility_control.h"
#include "rosidl_typesupport_introspection_c/field_types.h"
#include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/message_introspection.h"
#include "ugv_interface/srv/detail/map_save__functions.h"
#include "ugv_interface/srv/detail/map_save__struct.h"


// Include directives for member types
// Member `mapname`
#include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ugv_interface__srv__MapSave_Request__init(message_memory);
}

void ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_fini_function(void * message_memory)
{
  ugv_interface__srv__MapSave_Request__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_message_member_array[1] = {
  {
    "mapname",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ugv_interface__srv__MapSave_Request, mapname),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_message_members = {
  "ugv_interface__srv",  // message namespace
  "MapSave_Request",  // message name
  1,  // number of fields
  sizeof(ugv_interface__srv__MapSave_Request),
  false,  // has_any_key_member_
  ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_message_member_array,  // message members
  ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_init_function,  // function to initialize message memory (memory has to be allocated)
  ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_message_type_support_handle = {
  0,
  &ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_message_members,
  get_message_typesupport_handle_function,
  &ugv_interface__srv__MapSave_Request__get_type_hash,
  &ugv_interface__srv__MapSave_Request__get_type_description,
  &ugv_interface__srv__MapSave_Request__get_type_description_sources,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ugv_interface
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ugv_interface, srv, MapSave_Request)() {
  if (!ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_message_type_support_handle.typesupport_identifier) {
    ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "ugv_interface/srv/detail/map_save__rosidl_typesupport_introspection_c.h"
// already included above
// #include "ugv_interface/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "ugv_interface/srv/detail/map_save__functions.h"
// already included above
// #include "ugv_interface/srv/detail/map_save__struct.h"


// Include directives for member types
// Member `response`
// already included above
// #include "rosidl_runtime_c/string_functions.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ugv_interface__srv__MapSave_Response__init(message_memory);
}

void ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_fini_function(void * message_memory)
{
  ugv_interface__srv__MapSave_Response__fini(message_memory);
}

static rosidl_typesupport_introspection_c__MessageMember ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_message_member_array[1] = {
  {
    "response",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_STRING,  // type
    0,  // upper bound of string
    NULL,  // members of sub message
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ugv_interface__srv__MapSave_Response, response),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_message_members = {
  "ugv_interface__srv",  // message namespace
  "MapSave_Response",  // message name
  1,  // number of fields
  sizeof(ugv_interface__srv__MapSave_Response),
  false,  // has_any_key_member_
  ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_message_member_array,  // message members
  ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_init_function,  // function to initialize message memory (memory has to be allocated)
  ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_message_type_support_handle = {
  0,
  &ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_message_members,
  get_message_typesupport_handle_function,
  &ugv_interface__srv__MapSave_Response__get_type_hash,
  &ugv_interface__srv__MapSave_Response__get_type_description,
  &ugv_interface__srv__MapSave_Response__get_type_description_sources,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ugv_interface
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ugv_interface, srv, MapSave_Response)() {
  if (!ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_message_type_support_handle.typesupport_identifier) {
    ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

// already included above
// #include <stddef.h>
// already included above
// #include "ugv_interface/srv/detail/map_save__rosidl_typesupport_introspection_c.h"
// already included above
// #include "ugv_interface/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "rosidl_typesupport_introspection_c/field_types.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
// already included above
// #include "rosidl_typesupport_introspection_c/message_introspection.h"
// already included above
// #include "ugv_interface/srv/detail/map_save__functions.h"
// already included above
// #include "ugv_interface/srv/detail/map_save__struct.h"


// Include directives for member types
// Member `info`
#include "service_msgs/msg/service_event_info.h"
// Member `info`
#include "service_msgs/msg/detail/service_event_info__rosidl_typesupport_introspection_c.h"
// Member `request`
// Member `response`
#include "ugv_interface/srv/map_save.h"
// Member `request`
// Member `response`
// already included above
// #include "ugv_interface/srv/detail/map_save__rosidl_typesupport_introspection_c.h"

#ifdef __cplusplus
extern "C"
{
#endif

void ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_init_function(
  void * message_memory, enum rosidl_runtime_c__message_initialization _init)
{
  // TODO(karsten1987): initializers are not yet implemented for typesupport c
  // see https://github.com/ros2/ros2/issues/397
  (void) _init;
  ugv_interface__srv__MapSave_Event__init(message_memory);
}

void ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_fini_function(void * message_memory)
{
  ugv_interface__srv__MapSave_Event__fini(message_memory);
}

size_t ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__size_function__MapSave_Event__request(
  const void * untyped_member)
{
  const ugv_interface__srv__MapSave_Request__Sequence * member =
    (const ugv_interface__srv__MapSave_Request__Sequence *)(untyped_member);
  return member->size;
}

const void * ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__get_const_function__MapSave_Event__request(
  const void * untyped_member, size_t index)
{
  const ugv_interface__srv__MapSave_Request__Sequence * member =
    (const ugv_interface__srv__MapSave_Request__Sequence *)(untyped_member);
  return &member->data[index];
}

void * ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__get_function__MapSave_Event__request(
  void * untyped_member, size_t index)
{
  ugv_interface__srv__MapSave_Request__Sequence * member =
    (ugv_interface__srv__MapSave_Request__Sequence *)(untyped_member);
  return &member->data[index];
}

void ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__fetch_function__MapSave_Event__request(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const ugv_interface__srv__MapSave_Request * item =
    ((const ugv_interface__srv__MapSave_Request *)
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__get_const_function__MapSave_Event__request(untyped_member, index));
  ugv_interface__srv__MapSave_Request * value =
    (ugv_interface__srv__MapSave_Request *)(untyped_value);
  *value = *item;
}

void ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__assign_function__MapSave_Event__request(
  void * untyped_member, size_t index, const void * untyped_value)
{
  ugv_interface__srv__MapSave_Request * item =
    ((ugv_interface__srv__MapSave_Request *)
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__get_function__MapSave_Event__request(untyped_member, index));
  const ugv_interface__srv__MapSave_Request * value =
    (const ugv_interface__srv__MapSave_Request *)(untyped_value);
  *item = *value;
}

bool ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__resize_function__MapSave_Event__request(
  void * untyped_member, size_t size)
{
  ugv_interface__srv__MapSave_Request__Sequence * member =
    (ugv_interface__srv__MapSave_Request__Sequence *)(untyped_member);
  ugv_interface__srv__MapSave_Request__Sequence__fini(member);
  return ugv_interface__srv__MapSave_Request__Sequence__init(member, size);
}

size_t ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__size_function__MapSave_Event__response(
  const void * untyped_member)
{
  const ugv_interface__srv__MapSave_Response__Sequence * member =
    (const ugv_interface__srv__MapSave_Response__Sequence *)(untyped_member);
  return member->size;
}

const void * ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__get_const_function__MapSave_Event__response(
  const void * untyped_member, size_t index)
{
  const ugv_interface__srv__MapSave_Response__Sequence * member =
    (const ugv_interface__srv__MapSave_Response__Sequence *)(untyped_member);
  return &member->data[index];
}

void * ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__get_function__MapSave_Event__response(
  void * untyped_member, size_t index)
{
  ugv_interface__srv__MapSave_Response__Sequence * member =
    (ugv_interface__srv__MapSave_Response__Sequence *)(untyped_member);
  return &member->data[index];
}

void ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__fetch_function__MapSave_Event__response(
  const void * untyped_member, size_t index, void * untyped_value)
{
  const ugv_interface__srv__MapSave_Response * item =
    ((const ugv_interface__srv__MapSave_Response *)
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__get_const_function__MapSave_Event__response(untyped_member, index));
  ugv_interface__srv__MapSave_Response * value =
    (ugv_interface__srv__MapSave_Response *)(untyped_value);
  *value = *item;
}

void ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__assign_function__MapSave_Event__response(
  void * untyped_member, size_t index, const void * untyped_value)
{
  ugv_interface__srv__MapSave_Response * item =
    ((ugv_interface__srv__MapSave_Response *)
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__get_function__MapSave_Event__response(untyped_member, index));
  const ugv_interface__srv__MapSave_Response * value =
    (const ugv_interface__srv__MapSave_Response *)(untyped_value);
  *item = *value;
}

bool ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__resize_function__MapSave_Event__response(
  void * untyped_member, size_t size)
{
  ugv_interface__srv__MapSave_Response__Sequence * member =
    (ugv_interface__srv__MapSave_Response__Sequence *)(untyped_member);
  ugv_interface__srv__MapSave_Response__Sequence__fini(member);
  return ugv_interface__srv__MapSave_Response__Sequence__init(member, size);
}

static rosidl_typesupport_introspection_c__MessageMember ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_message_member_array[3] = {
  {
    "info",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is key
    false,  // is array
    0,  // array size
    false,  // is upper bound
    offsetof(ugv_interface__srv__MapSave_Event, info),  // bytes offset in struct
    NULL,  // default value
    NULL,  // size() function pointer
    NULL,  // get_const(index) function pointer
    NULL,  // get(index) function pointer
    NULL,  // fetch(index, &value) function pointer
    NULL,  // assign(index, value) function pointer
    NULL  // resize(index) function pointer
  },
  {
    "request",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is key
    true,  // is array
    1,  // array size
    true,  // is upper bound
    offsetof(ugv_interface__srv__MapSave_Event, request),  // bytes offset in struct
    NULL,  // default value
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__size_function__MapSave_Event__request,  // size() function pointer
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__get_const_function__MapSave_Event__request,  // get_const(index) function pointer
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__get_function__MapSave_Event__request,  // get(index) function pointer
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__fetch_function__MapSave_Event__request,  // fetch(index, &value) function pointer
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__assign_function__MapSave_Event__request,  // assign(index, value) function pointer
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__resize_function__MapSave_Event__request  // resize(index) function pointer
  },
  {
    "response",  // name
    rosidl_typesupport_introspection_c__ROS_TYPE_MESSAGE,  // type
    0,  // upper bound of string
    NULL,  // members of sub message (initialized later)
    false,  // is key
    true,  // is array
    1,  // array size
    true,  // is upper bound
    offsetof(ugv_interface__srv__MapSave_Event, response),  // bytes offset in struct
    NULL,  // default value
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__size_function__MapSave_Event__response,  // size() function pointer
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__get_const_function__MapSave_Event__response,  // get_const(index) function pointer
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__get_function__MapSave_Event__response,  // get(index) function pointer
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__fetch_function__MapSave_Event__response,  // fetch(index, &value) function pointer
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__assign_function__MapSave_Event__response,  // assign(index, value) function pointer
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__resize_function__MapSave_Event__response  // resize(index) function pointer
  }
};

static const rosidl_typesupport_introspection_c__MessageMembers ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_message_members = {
  "ugv_interface__srv",  // message namespace
  "MapSave_Event",  // message name
  3,  // number of fields
  sizeof(ugv_interface__srv__MapSave_Event),
  false,  // has_any_key_member_
  ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_message_member_array,  // message members
  ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_init_function,  // function to initialize message memory (memory has to be allocated)
  ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_fini_function  // function to terminate message instance (will not free memory)
};

// this is not const since it must be initialized on first access
// since C does not allow non-integral compile-time constants
static rosidl_message_type_support_t ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_message_type_support_handle = {
  0,
  &ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_message_members,
  get_message_typesupport_handle_function,
  &ugv_interface__srv__MapSave_Event__get_type_hash,
  &ugv_interface__srv__MapSave_Event__get_type_description,
  &ugv_interface__srv__MapSave_Event__get_type_description_sources,
};

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ugv_interface
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ugv_interface, srv, MapSave_Event)() {
  ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_message_member_array[0].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, service_msgs, msg, ServiceEventInfo)();
  ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_message_member_array[1].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ugv_interface, srv, MapSave_Request)();
  ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_message_member_array[2].members_ =
    ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ugv_interface, srv, MapSave_Response)();
  if (!ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_message_type_support_handle.typesupport_identifier) {
    ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_message_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  return &ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_message_type_support_handle;
}
#ifdef __cplusplus
}
#endif

#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "ugv_interface/msg/rosidl_typesupport_introspection_c__visibility_control.h"
// already included above
// #include "ugv_interface/srv/detail/map_save__rosidl_typesupport_introspection_c.h"
// already included above
// #include "rosidl_typesupport_introspection_c/identifier.h"
#include "rosidl_typesupport_introspection_c/service_introspection.h"

// this is intentionally not const to allow initialization later to prevent an initialization race
static rosidl_typesupport_introspection_c__ServiceMembers ugv_interface__srv__detail__map_save__rosidl_typesupport_introspection_c__MapSave_service_members = {
  "ugv_interface__srv",  // service namespace
  "MapSave",  // service name
  // the following fields are initialized below on first access
  NULL,  // request message
  // ugv_interface__srv__detail__map_save__rosidl_typesupport_introspection_c__MapSave_Request_message_type_support_handle,
  NULL,  // response message
  // ugv_interface__srv__detail__map_save__rosidl_typesupport_introspection_c__MapSave_Response_message_type_support_handle
  NULL  // event_message
  // ugv_interface__srv__detail__map_save__rosidl_typesupport_introspection_c__MapSave_Response_message_type_support_handle
};


static rosidl_service_type_support_t ugv_interface__srv__detail__map_save__rosidl_typesupport_introspection_c__MapSave_service_type_support_handle = {
  0,
  &ugv_interface__srv__detail__map_save__rosidl_typesupport_introspection_c__MapSave_service_members,
  get_service_typesupport_handle_function,
  &ugv_interface__srv__MapSave_Request__rosidl_typesupport_introspection_c__MapSave_Request_message_type_support_handle,
  &ugv_interface__srv__MapSave_Response__rosidl_typesupport_introspection_c__MapSave_Response_message_type_support_handle,
  &ugv_interface__srv__MapSave_Event__rosidl_typesupport_introspection_c__MapSave_Event_message_type_support_handle,
  ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_CREATE_EVENT_MESSAGE_SYMBOL_NAME(
    rosidl_typesupport_c,
    ugv_interface,
    srv,
    MapSave
  ),
  ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_DESTROY_EVENT_MESSAGE_SYMBOL_NAME(
    rosidl_typesupport_c,
    ugv_interface,
    srv,
    MapSave
  ),
  &ugv_interface__srv__MapSave__get_type_hash,
  &ugv_interface__srv__MapSave__get_type_description,
  &ugv_interface__srv__MapSave__get_type_description_sources,
};

// Forward declaration of message type support functions for service members
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ugv_interface, srv, MapSave_Request)(void);

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ugv_interface, srv, MapSave_Response)(void);

const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ugv_interface, srv, MapSave_Event)(void);

ROSIDL_TYPESUPPORT_INTROSPECTION_C_EXPORT_ugv_interface
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ugv_interface, srv, MapSave)(void) {
  if (!ugv_interface__srv__detail__map_save__rosidl_typesupport_introspection_c__MapSave_service_type_support_handle.typesupport_identifier) {
    ugv_interface__srv__detail__map_save__rosidl_typesupport_introspection_c__MapSave_service_type_support_handle.typesupport_identifier =
      rosidl_typesupport_introspection_c__identifier;
  }
  rosidl_typesupport_introspection_c__ServiceMembers * service_members =
    (rosidl_typesupport_introspection_c__ServiceMembers *)ugv_interface__srv__detail__map_save__rosidl_typesupport_introspection_c__MapSave_service_type_support_handle.data;

  if (!service_members->request_members_) {
    service_members->request_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ugv_interface, srv, MapSave_Request)()->data;
  }
  if (!service_members->response_members_) {
    service_members->response_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ugv_interface, srv, MapSave_Response)()->data;
  }
  if (!service_members->event_members_) {
    service_members->event_members_ =
      (const rosidl_typesupport_introspection_c__MessageMembers *)
      ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_c, ugv_interface, srv, MapSave_Event)()->data;
  }

  return &ugv_interface__srv__detail__map_save__rosidl_typesupport_introspection_c__MapSave_service_type_support_handle;
}
