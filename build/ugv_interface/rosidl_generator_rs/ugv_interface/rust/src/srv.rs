#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to ugv_interface__srv__MapSave_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MapSave_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub mapname: std::string::String,

}



impl Default for MapSave_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MapSave_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MapSave_Request {
  type RmwMsg = super::srv::rmw::MapSave_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mapname: msg.mapname.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        mapname: msg.mapname.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      mapname: msg.mapname.to_string(),
    }
  }
}


// Corresponds to ugv_interface__srv__MapSave_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MapSave_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub response: std::string::String,

}



impl Default for MapSave_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MapSave_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MapSave_Response {
  type RmwMsg = super::srv::rmw::MapSave_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: msg.response.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        response: msg.response.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      response: msg.response.to_string(),
    }
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


