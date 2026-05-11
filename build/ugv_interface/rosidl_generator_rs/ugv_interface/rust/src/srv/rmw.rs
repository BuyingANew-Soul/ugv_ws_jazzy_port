#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__srv__MapSave_Request() -> *const std::ffi::c_void;
}

#[link(name = "ugv_interface__rosidl_generator_c")]
extern "C" {
    fn ugv_interface__srv__MapSave_Request__init(msg: *mut MapSave_Request) -> bool;
    fn ugv_interface__srv__MapSave_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MapSave_Request>, size: usize) -> bool;
    fn ugv_interface__srv__MapSave_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MapSave_Request>);
    fn ugv_interface__srv__MapSave_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MapSave_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MapSave_Request>) -> bool;
}

// Corresponds to ugv_interface__srv__MapSave_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MapSave_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mapname: rosidl_runtime_rs::String,

}



impl Default for MapSave_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ugv_interface__srv__MapSave_Request__init(&mut msg as *mut _) {
        panic!("Call to ugv_interface__srv__MapSave_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MapSave_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__srv__MapSave_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__srv__MapSave_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__srv__MapSave_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MapSave_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MapSave_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ugv_interface/srv/MapSave_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__srv__MapSave_Request() }
  }
}


#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__srv__MapSave_Response() -> *const std::ffi::c_void;
}

#[link(name = "ugv_interface__rosidl_generator_c")]
extern "C" {
    fn ugv_interface__srv__MapSave_Response__init(msg: *mut MapSave_Response) -> bool;
    fn ugv_interface__srv__MapSave_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MapSave_Response>, size: usize) -> bool;
    fn ugv_interface__srv__MapSave_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MapSave_Response>);
    fn ugv_interface__srv__MapSave_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MapSave_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MapSave_Response>) -> bool;
}

// Corresponds to ugv_interface__srv__MapSave_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MapSave_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: rosidl_runtime_rs::String,

}



impl Default for MapSave_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ugv_interface__srv__MapSave_Response__init(&mut msg as *mut _) {
        panic!("Call to ugv_interface__srv__MapSave_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MapSave_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__srv__MapSave_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__srv__MapSave_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__srv__MapSave_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MapSave_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MapSave_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ugv_interface/srv/MapSave_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__srv__MapSave_Response() }
  }
}






#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ugv_interface__srv__MapSave() -> *const std::ffi::c_void;
}

// Corresponds to ugv_interface__srv__MapSave
#[allow(missing_docs, non_camel_case_types)]
pub struct MapSave;

impl rosidl_runtime_rs::Service for MapSave {
    type Request = MapSave_Request;
    type Response = MapSave_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ugv_interface__srv__MapSave() }
    }
}


