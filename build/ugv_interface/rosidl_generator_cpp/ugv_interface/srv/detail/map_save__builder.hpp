// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from ugv_interface:srv/MapSave.idl
// generated code does not contain a copyright notice

// IWYU pragma: private, include "ugv_interface/srv/map_save.hpp"


#ifndef UGV_INTERFACE__SRV__DETAIL__MAP_SAVE__BUILDER_HPP_
#define UGV_INTERFACE__SRV__DETAIL__MAP_SAVE__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "ugv_interface/srv/detail/map_save__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace ugv_interface
{

namespace srv
{

namespace builder
{

class Init_MapSave_Request_mapname
{
public:
  Init_MapSave_Request_mapname()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ugv_interface::srv::MapSave_Request mapname(::ugv_interface::srv::MapSave_Request::_mapname_type arg)
  {
    msg_.mapname = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ugv_interface::srv::MapSave_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ugv_interface::srv::MapSave_Request>()
{
  return ugv_interface::srv::builder::Init_MapSave_Request_mapname();
}

}  // namespace ugv_interface


namespace ugv_interface
{

namespace srv
{

namespace builder
{

class Init_MapSave_Response_response
{
public:
  Init_MapSave_Response_response()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::ugv_interface::srv::MapSave_Response response(::ugv_interface::srv::MapSave_Response::_response_type arg)
  {
    msg_.response = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ugv_interface::srv::MapSave_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ugv_interface::srv::MapSave_Response>()
{
  return ugv_interface::srv::builder::Init_MapSave_Response_response();
}

}  // namespace ugv_interface


namespace ugv_interface
{

namespace srv
{

namespace builder
{

class Init_MapSave_Event_response
{
public:
  explicit Init_MapSave_Event_response(::ugv_interface::srv::MapSave_Event & msg)
  : msg_(msg)
  {}
  ::ugv_interface::srv::MapSave_Event response(::ugv_interface::srv::MapSave_Event::_response_type arg)
  {
    msg_.response = std::move(arg);
    return std::move(msg_);
  }

private:
  ::ugv_interface::srv::MapSave_Event msg_;
};

class Init_MapSave_Event_request
{
public:
  explicit Init_MapSave_Event_request(::ugv_interface::srv::MapSave_Event & msg)
  : msg_(msg)
  {}
  Init_MapSave_Event_response request(::ugv_interface::srv::MapSave_Event::_request_type arg)
  {
    msg_.request = std::move(arg);
    return Init_MapSave_Event_response(msg_);
  }

private:
  ::ugv_interface::srv::MapSave_Event msg_;
};

class Init_MapSave_Event_info
{
public:
  Init_MapSave_Event_info()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MapSave_Event_request info(::ugv_interface::srv::MapSave_Event::_info_type arg)
  {
    msg_.info = std::move(arg);
    return Init_MapSave_Event_request(msg_);
  }

private:
  ::ugv_interface::srv::MapSave_Event msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::ugv_interface::srv::MapSave_Event>()
{
  return ugv_interface::srv::builder::Init_MapSave_Event_info();
}

}  // namespace ugv_interface

#endif  // UGV_INTERFACE__SRV__DETAIL__MAP_SAVE__BUILDER_HPP_
