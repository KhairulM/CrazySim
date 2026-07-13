#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__FullState() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__FullState__init(msg: *mut FullState) -> bool;
    fn crazyflie_interfaces__msg__FullState__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<FullState>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__FullState__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<FullState>);
    fn crazyflie_interfaces__msg__FullState__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<FullState>, out_seq: *mut rosidl_runtime_rs::Sequence<FullState>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__FullState
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FullState {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pose: geometry_msgs::msg::rmw::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub twist: geometry_msgs::msg::rmw::Twist,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acc: geometry_msgs::msg::rmw::Vector3,

}



impl Default for FullState {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__FullState__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__FullState__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for FullState {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__FullState__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__FullState__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__FullState__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for FullState {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for FullState where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/FullState";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__FullState() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__LogDataGeneric() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__LogDataGeneric__init(msg: *mut LogDataGeneric) -> bool;
    fn crazyflie_interfaces__msg__LogDataGeneric__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LogDataGeneric>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__LogDataGeneric__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LogDataGeneric>);
    fn crazyflie_interfaces__msg__LogDataGeneric__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LogDataGeneric>, out_seq: *mut rosidl_runtime_rs::Sequence<LogDataGeneric>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__LogDataGeneric
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LogDataGeneric {
    /// Header including the ROS 2 timestamp when the log data was received
    pub header: std_msgs::msg::rmw::Header,

    /// on-board timestamp from the STM32 (in ms)
    pub timestamp: u32,

    /// converted values, in the order as specified for the log block
    pub values: rosidl_runtime_rs::Sequence<f32>,

}



impl Default for LogDataGeneric {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__LogDataGeneric__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__LogDataGeneric__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LogDataGeneric {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__LogDataGeneric__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__LogDataGeneric__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__LogDataGeneric__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LogDataGeneric {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LogDataGeneric where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/LogDataGeneric";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__LogDataGeneric() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__Hover() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__Hover__init(msg: *mut Hover) -> bool;
    fn crazyflie_interfaces__msg__Hover__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Hover>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__Hover__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Hover>);
    fn crazyflie_interfaces__msg__Hover__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Hover>, out_seq: *mut rosidl_runtime_rs::Sequence<Hover>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__Hover
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Hover {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub vx: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub vy: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_rate: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z_distance: f32,

}



impl Default for Hover {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__Hover__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__Hover__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Hover {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Hover__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Hover__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Hover__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Hover {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Hover where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/Hover";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__Hover() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__LogBlock() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__LogBlock__init(msg: *mut LogBlock) -> bool;
    fn crazyflie_interfaces__msg__LogBlock__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LogBlock>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__LogBlock__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LogBlock>);
    fn crazyflie_interfaces__msg__LogBlock__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LogBlock>, out_seq: *mut rosidl_runtime_rs::Sequence<LogBlock>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__LogBlock
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LogBlock {

    // This member is not documented.
    #[allow(missing_docs)]
    pub topic_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub frequency: i16,


    // This member is not documented.
    #[allow(missing_docs)]
    pub variables: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for LogBlock {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__LogBlock__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__LogBlock__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LogBlock {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__LogBlock__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__LogBlock__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__LogBlock__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LogBlock {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LogBlock where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/LogBlock";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__LogBlock() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__Position() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__Position__init(msg: *mut Position) -> bool;
    fn crazyflie_interfaces__msg__Position__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Position>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__Position__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Position>);
    fn crazyflie_interfaces__msg__Position__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Position>, out_seq: *mut rosidl_runtime_rs::Sequence<Position>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__Position
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Position {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f32,

}



impl Default for Position {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__Position__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__Position__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Position {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Position__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Position__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__Position__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Position {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Position where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/Position";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__Position() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__TrajectoryPolynomialPiece() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__TrajectoryPolynomialPiece__init(msg: *mut TrajectoryPolynomialPiece) -> bool;
    fn crazyflie_interfaces__msg__TrajectoryPolynomialPiece__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryPolynomialPiece>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__TrajectoryPolynomialPiece__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<TrajectoryPolynomialPiece>);
    fn crazyflie_interfaces__msg__TrajectoryPolynomialPiece__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<TrajectoryPolynomialPiece>, out_seq: *mut rosidl_runtime_rs::Sequence<TrajectoryPolynomialPiece>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__TrajectoryPolynomialPiece
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TrajectoryPolynomialPiece {

    // This member is not documented.
    #[allow(missing_docs)]
    pub poly_x: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub poly_y: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub poly_z: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub poly_yaw: rosidl_runtime_rs::Sequence<f32>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub duration: builtin_interfaces::msg::rmw::Duration,

}



impl Default for TrajectoryPolynomialPiece {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__TrajectoryPolynomialPiece__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__TrajectoryPolynomialPiece__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for TrajectoryPolynomialPiece {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__TrajectoryPolynomialPiece__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__TrajectoryPolynomialPiece__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__TrajectoryPolynomialPiece__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for TrajectoryPolynomialPiece {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for TrajectoryPolynomialPiece where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/TrajectoryPolynomialPiece";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__TrajectoryPolynomialPiece() }
  }
}


#[link(name = "crazyflie_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__VelocityWorld() -> *const std::ffi::c_void;
}

#[link(name = "crazyflie_interfaces__rosidl_generator_c")]
extern "C" {
    fn crazyflie_interfaces__msg__VelocityWorld__init(msg: *mut VelocityWorld) -> bool;
    fn crazyflie_interfaces__msg__VelocityWorld__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<VelocityWorld>, size: usize) -> bool;
    fn crazyflie_interfaces__msg__VelocityWorld__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<VelocityWorld>);
    fn crazyflie_interfaces__msg__VelocityWorld__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<VelocityWorld>, out_seq: *mut rosidl_runtime_rs::Sequence<VelocityWorld>) -> bool;
}

// Corresponds to crazyflie_interfaces__msg__VelocityWorld
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VelocityWorld {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub vel: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_rate: f32,

}



impl Default for VelocityWorld {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !crazyflie_interfaces__msg__VelocityWorld__init(&mut msg as *mut _) {
        panic!("Call to crazyflie_interfaces__msg__VelocityWorld__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for VelocityWorld {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__VelocityWorld__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__VelocityWorld__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { crazyflie_interfaces__msg__VelocityWorld__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for VelocityWorld {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for VelocityWorld where Self: Sized {
  const TYPE_NAME: &'static str = "crazyflie_interfaces/msg/VelocityWorld";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__crazyflie_interfaces__msg__VelocityWorld() }
  }
}


