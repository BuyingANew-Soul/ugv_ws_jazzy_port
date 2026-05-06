
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_Goal() -> *const std::ffi::c_void;
}

#[link(name = "ugv_interface__rosidl_generator_c")]
extern "C" {
    fn ugv_interface__action__Behavior_Goal__init(msg: *mut Behavior_Goal) -> bool;
    fn ugv_interface__action__Behavior_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Behavior_Goal>, size: usize) -> bool;
    fn ugv_interface__action__Behavior_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Behavior_Goal>);
    fn ugv_interface__action__Behavior_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Behavior_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Behavior_Goal>) -> bool;
}

// Corresponds to ugv_interface__action__Behavior_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Behavior_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: rosidl_runtime_rs::String,

}



impl Default for Behavior_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ugv_interface__action__Behavior_Goal__init(&mut msg as *mut _) {
        panic!("Call to ugv_interface__action__Behavior_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Behavior_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Behavior_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Behavior_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "ugv_interface/action/Behavior_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_Goal() }
  }
}


#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_Result() -> *const std::ffi::c_void;
}

#[link(name = "ugv_interface__rosidl_generator_c")]
extern "C" {
    fn ugv_interface__action__Behavior_Result__init(msg: *mut Behavior_Result) -> bool;
    fn ugv_interface__action__Behavior_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Behavior_Result>, size: usize) -> bool;
    fn ugv_interface__action__Behavior_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Behavior_Result>);
    fn ugv_interface__action__Behavior_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Behavior_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Behavior_Result>) -> bool;
}

// Corresponds to ugv_interface__action__Behavior_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Behavior_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: bool,

}



impl Default for Behavior_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ugv_interface__action__Behavior_Result__init(&mut msg as *mut _) {
        panic!("Call to ugv_interface__action__Behavior_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Behavior_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Behavior_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Behavior_Result where Self: Sized {
  const TYPE_NAME: &'static str = "ugv_interface/action/Behavior_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_Result() }
  }
}


#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "ugv_interface__rosidl_generator_c")]
extern "C" {
    fn ugv_interface__action__Behavior_Feedback__init(msg: *mut Behavior_Feedback) -> bool;
    fn ugv_interface__action__Behavior_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Behavior_Feedback>, size: usize) -> bool;
    fn ugv_interface__action__Behavior_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Behavior_Feedback>);
    fn ugv_interface__action__Behavior_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Behavior_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Behavior_Feedback>) -> bool;
}

// Corresponds to ugv_interface__action__Behavior_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Behavior_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: bool,

}



impl Default for Behavior_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ugv_interface__action__Behavior_Feedback__init(&mut msg as *mut _) {
        panic!("Call to ugv_interface__action__Behavior_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Behavior_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Behavior_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Behavior_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "ugv_interface/action/Behavior_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_Feedback() }
  }
}


#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "ugv_interface__rosidl_generator_c")]
extern "C" {
    fn ugv_interface__action__Behavior_FeedbackMessage__init(msg: *mut Behavior_FeedbackMessage) -> bool;
    fn ugv_interface__action__Behavior_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Behavior_FeedbackMessage>, size: usize) -> bool;
    fn ugv_interface__action__Behavior_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Behavior_FeedbackMessage>);
    fn ugv_interface__action__Behavior_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Behavior_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Behavior_FeedbackMessage>) -> bool;
}

// Corresponds to ugv_interface__action__Behavior_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Behavior_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Behavior_Feedback,

}



impl Default for Behavior_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ugv_interface__action__Behavior_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to ugv_interface__action__Behavior_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Behavior_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Behavior_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Behavior_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "ugv_interface/action/Behavior_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_FeedbackMessage() }
  }
}




#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "ugv_interface__rosidl_generator_c")]
extern "C" {
    fn ugv_interface__action__Behavior_SendGoal_Request__init(msg: *mut Behavior_SendGoal_Request) -> bool;
    fn ugv_interface__action__Behavior_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Behavior_SendGoal_Request>, size: usize) -> bool;
    fn ugv_interface__action__Behavior_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Behavior_SendGoal_Request>);
    fn ugv_interface__action__Behavior_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Behavior_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Behavior_SendGoal_Request>) -> bool;
}

// Corresponds to ugv_interface__action__Behavior_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Behavior_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Behavior_Goal,

}



impl Default for Behavior_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ugv_interface__action__Behavior_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to ugv_interface__action__Behavior_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Behavior_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Behavior_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Behavior_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ugv_interface/action/Behavior_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_SendGoal_Request() }
  }
}


#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "ugv_interface__rosidl_generator_c")]
extern "C" {
    fn ugv_interface__action__Behavior_SendGoal_Response__init(msg: *mut Behavior_SendGoal_Response) -> bool;
    fn ugv_interface__action__Behavior_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Behavior_SendGoal_Response>, size: usize) -> bool;
    fn ugv_interface__action__Behavior_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Behavior_SendGoal_Response>);
    fn ugv_interface__action__Behavior_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Behavior_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Behavior_SendGoal_Response>) -> bool;
}

// Corresponds to ugv_interface__action__Behavior_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Behavior_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Behavior_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ugv_interface__action__Behavior_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to ugv_interface__action__Behavior_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Behavior_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Behavior_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Behavior_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ugv_interface/action/Behavior_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_SendGoal_Response() }
  }
}


#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "ugv_interface__rosidl_generator_c")]
extern "C" {
    fn ugv_interface__action__Behavior_GetResult_Request__init(msg: *mut Behavior_GetResult_Request) -> bool;
    fn ugv_interface__action__Behavior_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Behavior_GetResult_Request>, size: usize) -> bool;
    fn ugv_interface__action__Behavior_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Behavior_GetResult_Request>);
    fn ugv_interface__action__Behavior_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Behavior_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Behavior_GetResult_Request>) -> bool;
}

// Corresponds to ugv_interface__action__Behavior_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Behavior_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Behavior_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ugv_interface__action__Behavior_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to ugv_interface__action__Behavior_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Behavior_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Behavior_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Behavior_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "ugv_interface/action/Behavior_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_GetResult_Request() }
  }
}


#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "ugv_interface__rosidl_generator_c")]
extern "C" {
    fn ugv_interface__action__Behavior_GetResult_Response__init(msg: *mut Behavior_GetResult_Response) -> bool;
    fn ugv_interface__action__Behavior_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Behavior_GetResult_Response>, size: usize) -> bool;
    fn ugv_interface__action__Behavior_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Behavior_GetResult_Response>);
    fn ugv_interface__action__Behavior_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Behavior_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Behavior_GetResult_Response>) -> bool;
}

// Corresponds to ugv_interface__action__Behavior_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Behavior_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Behavior_Result,

}



impl Default for Behavior_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !ugv_interface__action__Behavior_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to ugv_interface__action__Behavior_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Behavior_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { ugv_interface__action__Behavior_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Behavior_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Behavior_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "ugv_interface/action/Behavior_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__ugv_interface__action__Behavior_GetResult_Response() }
  }
}






#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ugv_interface__action__Behavior_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to ugv_interface__action__Behavior_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Behavior_SendGoal;

impl rosidl_runtime_rs::Service for Behavior_SendGoal {
    type Request = Behavior_SendGoal_Request;
    type Response = Behavior_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ugv_interface__action__Behavior_SendGoal() }
    }
}




#[link(name = "ugv_interface__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__ugv_interface__action__Behavior_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to ugv_interface__action__Behavior_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Behavior_GetResult;

impl rosidl_runtime_rs::Service for Behavior_GetResult {
    type Request = Behavior_GetResult_Request;
    type Response = Behavior_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__ugv_interface__action__Behavior_GetResult() }
    }
}


