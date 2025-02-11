// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod nexus {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_SERVER_CODE: u64 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_SERVER_CODE: u64 = 1;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_SERVER_CODE: [ServerCode; 2] = [
  ServerCode::SUCCESS,
  ServerCode::FAILED,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ServerCode(pub u64);
#[allow(non_upper_case_globals)]
impl ServerCode {
  pub const SUCCESS: Self = Self(0);
  pub const FAILED: Self = Self(1);

  pub const ENUM_MIN: u64 = 0;
  pub const ENUM_MAX: u64 = 1;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::SUCCESS,
    Self::FAILED,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::SUCCESS => Some("SUCCESS"),
      Self::FAILED => Some("FAILED"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for ServerCode {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for ServerCode {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u64>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for ServerCode {
    type Output = ServerCode;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u64>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for ServerCode {
  type Scalar = u64;
  #[inline]
  fn to_little_endian(self) -> u64 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u64) -> Self {
    let b = u64::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for ServerCode {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u64::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ServerCode {}
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_PACKET_TYPE: u16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_PACKET_TYPE: u16 = 5;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_PACKET_TYPE: [PacketType; 6] = [
  PacketType::CG_LOGIN_REQ,
  PacketType::GC_LOGIN_RES,
  PacketType::GC_SPAWN_ACTOR_NOTI,
  PacketType::CG_CHANGE_MOVE_DIRECTION_REQ,
  PacketType::GC_CHANGE_MOVE_DIRECTION_RES,
  PacketType::GC_CHANGE_ACTOR_DIRECTION_NOTI,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PacketType(pub u16);
#[allow(non_upper_case_globals)]
impl PacketType {
  pub const CG_LOGIN_REQ: Self = Self(0);
  pub const GC_LOGIN_RES: Self = Self(1);
  pub const GC_SPAWN_ACTOR_NOTI: Self = Self(2);
  pub const CG_CHANGE_MOVE_DIRECTION_REQ: Self = Self(3);
  pub const GC_CHANGE_MOVE_DIRECTION_RES: Self = Self(4);
  pub const GC_CHANGE_ACTOR_DIRECTION_NOTI: Self = Self(5);

  pub const ENUM_MIN: u16 = 0;
  pub const ENUM_MAX: u16 = 5;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::CG_LOGIN_REQ,
    Self::GC_LOGIN_RES,
    Self::GC_SPAWN_ACTOR_NOTI,
    Self::CG_CHANGE_MOVE_DIRECTION_REQ,
    Self::GC_CHANGE_MOVE_DIRECTION_RES,
    Self::GC_CHANGE_ACTOR_DIRECTION_NOTI,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::CG_LOGIN_REQ => Some("CG_LOGIN_REQ"),
      Self::GC_LOGIN_RES => Some("GC_LOGIN_RES"),
      Self::GC_SPAWN_ACTOR_NOTI => Some("GC_SPAWN_ACTOR_NOTI"),
      Self::CG_CHANGE_MOVE_DIRECTION_REQ => Some("CG_CHANGE_MOVE_DIRECTION_REQ"),
      Self::GC_CHANGE_MOVE_DIRECTION_RES => Some("GC_CHANGE_MOVE_DIRECTION_RES"),
      Self::GC_CHANGE_ACTOR_DIRECTION_NOTI => Some("GC_CHANGE_ACTOR_DIRECTION_NOTI"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for PacketType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for PacketType {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u16>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for PacketType {
    type Output = PacketType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u16>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for PacketType {
  type Scalar = u16;
  #[inline]
  fn to_little_endian(self) -> u16 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u16) -> Self {
    let b = u16::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for PacketType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for PacketType {}
// struct Color, aligned to 1
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Color(pub [u8; 4]);
impl Default for Color { 
  fn default() -> Self { 
    Self([0; 4])
  }
}
impl core::fmt::Debug for Color {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("Color")
      .field("r", &self.r())
      .field("g", &self.g())
      .field("b", &self.b())
      .field("a", &self.a())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Color {}
impl<'a> flatbuffers::Follow<'a> for Color {
  type Inner = &'a Color;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Color>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Color {
  type Inner = &'a Color;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Color>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Color {
    type Output = Color;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const Color as *const u8, <Self as flatbuffers::Push>::size());
        dst.copy_from_slice(src);
    }
    #[inline]
    fn alignment() -> flatbuffers::PushAlignment {
        flatbuffers::PushAlignment::new(1)
    }
}

impl<'a> flatbuffers::Verifiable for Color {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> Color {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    r: i8,
    g: i8,
    b: i8,
    a: i8,
  ) -> Self {
    let mut s = Self([0; 4]);
    s.set_r(r);
    s.set_g(g);
    s.set_b(b);
    s.set_a(a);
    s
  }

  pub fn r(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<<i8 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<i8 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_r(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<i8 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn g(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<<i8 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[1..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<i8 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_g(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[1..].as_mut_ptr(),
        core::mem::size_of::<<i8 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn b(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<<i8 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[2..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<i8 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_b(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[2..].as_mut_ptr(),
        core::mem::size_of::<<i8 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn a(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<<i8 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[3..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<i8 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_a(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[3..].as_mut_ptr(),
        core::mem::size_of::<<i8 as EndianScalar>::Scalar>(),
      );
    }
  }

}

// struct Vec2, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec2(pub [u8; 8]);
impl Default for Vec2 { 
  fn default() -> Self { 
    Self([0; 8])
  }
}
impl core::fmt::Debug for Vec2 {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("Vec2")
      .field("x", &self.x())
      .field("y", &self.y())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Vec2 {}
impl<'a> flatbuffers::Follow<'a> for Vec2 {
  type Inner = &'a Vec2;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a Vec2>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Vec2 {
  type Inner = &'a Vec2;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<Vec2>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for Vec2 {
    type Output = Vec2;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const Vec2 as *const u8, <Self as flatbuffers::Push>::size());
        dst.copy_from_slice(src);
    }
    #[inline]
    fn alignment() -> flatbuffers::PushAlignment {
        flatbuffers::PushAlignment::new(4)
    }
}

impl<'a> flatbuffers::Verifiable for Vec2 {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> Vec2 {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    x: f32,
    y: f32,
  ) -> Self {
    let mut s = Self([0; 8]);
    s.set_x(x);
    s.set_y(y);
    s
  }

  pub fn x(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_x(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn y(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_y(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

}

pub enum CGLoginReqOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct CGLoginReq<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for CGLoginReq<'a> {
  type Inner = CGLoginReq<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> CGLoginReq<'a> {
  pub const VT_NAME: flatbuffers::VOffsetT = 4;
  pub const VT_COLOR: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    CGLoginReq { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args CGLoginReqArgs<'args>
  ) -> flatbuffers::WIPOffset<CGLoginReq<'bldr>> {
    let mut builder = CGLoginReqBuilder::new(_fbb);
    if let Some(x) = args.color { builder.add_color(x); }
    if let Some(x) = args.name { builder.add_name(x); }
    builder.finish()
  }


  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(CGLoginReq::VT_NAME, None)}
  }
  #[inline]
  pub fn color(&self) -> Option<&'a Color> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Color>(CGLoginReq::VT_COLOR, None)}
  }
}

impl flatbuffers::Verifiable for CGLoginReq<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_field::<Color>("color", Self::VT_COLOR, false)?
     .finish();
    Ok(())
  }
}
pub struct CGLoginReqArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub color: Option<&'a Color>,
}
impl<'a> Default for CGLoginReqArgs<'a> {
  #[inline]
  fn default() -> Self {
    CGLoginReqArgs {
      name: None,
      color: None,
    }
  }
}

pub struct CGLoginReqBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> CGLoginReqBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(CGLoginReq::VT_NAME, name);
  }
  #[inline]
  pub fn add_color(&mut self, color: &Color) {
    self.fbb_.push_slot_always::<&Color>(CGLoginReq::VT_COLOR, color);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> CGLoginReqBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    CGLoginReqBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<CGLoginReq<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for CGLoginReq<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("CGLoginReq");
      ds.field("name", &self.name());
      ds.field("color", &self.color());
      ds.finish()
  }
}
pub enum GCLoginResOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct GCLoginRes<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for GCLoginRes<'a> {
  type Inner = GCLoginRes<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> GCLoginRes<'a> {
  pub const VT_ACTOR_IDX: flatbuffers::VOffsetT = 4;
  pub const VT_RESULT: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    GCLoginRes { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args GCLoginResArgs
  ) -> flatbuffers::WIPOffset<GCLoginRes<'bldr>> {
    let mut builder = GCLoginResBuilder::new(_fbb);
    builder.add_result(args.result);
    builder.add_actor_idx(args.actor_idx);
    builder.finish()
  }


  #[inline]
  pub fn actor_idx(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(GCLoginRes::VT_ACTOR_IDX, Some(0)).unwrap()}
  }
  #[inline]
  pub fn result(&self) -> ServerCode {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<ServerCode>(GCLoginRes::VT_RESULT, Some(ServerCode::SUCCESS)).unwrap()}
  }
}

impl flatbuffers::Verifiable for GCLoginRes<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("actor_idx", Self::VT_ACTOR_IDX, false)?
     .visit_field::<ServerCode>("result", Self::VT_RESULT, false)?
     .finish();
    Ok(())
  }
}
pub struct GCLoginResArgs {
    pub actor_idx: u64,
    pub result: ServerCode,
}
impl<'a> Default for GCLoginResArgs {
  #[inline]
  fn default() -> Self {
    GCLoginResArgs {
      actor_idx: 0,
      result: ServerCode::SUCCESS,
    }
  }
}

pub struct GCLoginResBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> GCLoginResBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_actor_idx(&mut self, actor_idx: u64) {
    self.fbb_.push_slot::<u64>(GCLoginRes::VT_ACTOR_IDX, actor_idx, 0);
  }
  #[inline]
  pub fn add_result(&mut self, result: ServerCode) {
    self.fbb_.push_slot::<ServerCode>(GCLoginRes::VT_RESULT, result, ServerCode::SUCCESS);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> GCLoginResBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    GCLoginResBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<GCLoginRes<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for GCLoginRes<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("GCLoginRes");
      ds.field("actor_idx", &self.actor_idx());
      ds.field("result", &self.result());
      ds.finish()
  }
}
pub enum GCSpawnActorNotiOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct GCSpawnActorNoti<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for GCSpawnActorNoti<'a> {
  type Inner = GCSpawnActorNoti<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> GCSpawnActorNoti<'a> {
  pub const VT_ACTOR_IDX: flatbuffers::VOffsetT = 4;
  pub const VT_NAME: flatbuffers::VOffsetT = 6;
  pub const VT_COLOR: flatbuffers::VOffsetT = 8;
  pub const VT_SPEED: flatbuffers::VOffsetT = 10;
  pub const VT_POSITION: flatbuffers::VOffsetT = 12;
  pub const VT_DIRECTION: flatbuffers::VOffsetT = 14;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    GCSpawnActorNoti { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args GCSpawnActorNotiArgs<'args>
  ) -> flatbuffers::WIPOffset<GCSpawnActorNoti<'bldr>> {
    let mut builder = GCSpawnActorNotiBuilder::new(_fbb);
    builder.add_actor_idx(args.actor_idx);
    if let Some(x) = args.direction { builder.add_direction(x); }
    if let Some(x) = args.position { builder.add_position(x); }
    builder.add_speed(args.speed);
    if let Some(x) = args.color { builder.add_color(x); }
    if let Some(x) = args.name { builder.add_name(x); }
    builder.finish()
  }


  #[inline]
  pub fn actor_idx(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(GCSpawnActorNoti::VT_ACTOR_IDX, Some(0)).unwrap()}
  }
  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(GCSpawnActorNoti::VT_NAME, None)}
  }
  #[inline]
  pub fn color(&self) -> Option<&'a Color> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Color>(GCSpawnActorNoti::VT_COLOR, None)}
  }
  #[inline]
  pub fn speed(&self) -> f32 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<f32>(GCSpawnActorNoti::VT_SPEED, Some(0.0)).unwrap()}
  }
  #[inline]
  pub fn position(&self) -> Option<&'a Vec2> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Vec2>(GCSpawnActorNoti::VT_POSITION, None)}
  }
  #[inline]
  pub fn direction(&self) -> Option<&'a Vec2> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Vec2>(GCSpawnActorNoti::VT_DIRECTION, None)}
  }
}

impl flatbuffers::Verifiable for GCSpawnActorNoti<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("actor_idx", Self::VT_ACTOR_IDX, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_field::<Color>("color", Self::VT_COLOR, false)?
     .visit_field::<f32>("speed", Self::VT_SPEED, false)?
     .visit_field::<Vec2>("position", Self::VT_POSITION, false)?
     .visit_field::<Vec2>("direction", Self::VT_DIRECTION, false)?
     .finish();
    Ok(())
  }
}
pub struct GCSpawnActorNotiArgs<'a> {
    pub actor_idx: u64,
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub color: Option<&'a Color>,
    pub speed: f32,
    pub position: Option<&'a Vec2>,
    pub direction: Option<&'a Vec2>,
}
impl<'a> Default for GCSpawnActorNotiArgs<'a> {
  #[inline]
  fn default() -> Self {
    GCSpawnActorNotiArgs {
      actor_idx: 0,
      name: None,
      color: None,
      speed: 0.0,
      position: None,
      direction: None,
    }
  }
}

pub struct GCSpawnActorNotiBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> GCSpawnActorNotiBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_actor_idx(&mut self, actor_idx: u64) {
    self.fbb_.push_slot::<u64>(GCSpawnActorNoti::VT_ACTOR_IDX, actor_idx, 0);
  }
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(GCSpawnActorNoti::VT_NAME, name);
  }
  #[inline]
  pub fn add_color(&mut self, color: &Color) {
    self.fbb_.push_slot_always::<&Color>(GCSpawnActorNoti::VT_COLOR, color);
  }
  #[inline]
  pub fn add_speed(&mut self, speed: f32) {
    self.fbb_.push_slot::<f32>(GCSpawnActorNoti::VT_SPEED, speed, 0.0);
  }
  #[inline]
  pub fn add_position(&mut self, position: &Vec2) {
    self.fbb_.push_slot_always::<&Vec2>(GCSpawnActorNoti::VT_POSITION, position);
  }
  #[inline]
  pub fn add_direction(&mut self, direction: &Vec2) {
    self.fbb_.push_slot_always::<&Vec2>(GCSpawnActorNoti::VT_DIRECTION, direction);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> GCSpawnActorNotiBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    GCSpawnActorNotiBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<GCSpawnActorNoti<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for GCSpawnActorNoti<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("GCSpawnActorNoti");
      ds.field("actor_idx", &self.actor_idx());
      ds.field("name", &self.name());
      ds.field("color", &self.color());
      ds.field("speed", &self.speed());
      ds.field("position", &self.position());
      ds.field("direction", &self.direction());
      ds.finish()
  }
}
pub enum CGChangeMoveDirectionReqOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct CGChangeMoveDirectionReq<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for CGChangeMoveDirectionReq<'a> {
  type Inner = CGChangeMoveDirectionReq<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> CGChangeMoveDirectionReq<'a> {
  pub const VT_DIRECTION: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    CGChangeMoveDirectionReq { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args CGChangeMoveDirectionReqArgs<'args>
  ) -> flatbuffers::WIPOffset<CGChangeMoveDirectionReq<'bldr>> {
    let mut builder = CGChangeMoveDirectionReqBuilder::new(_fbb);
    if let Some(x) = args.direction { builder.add_direction(x); }
    builder.finish()
  }


  #[inline]
  pub fn direction(&self) -> Option<&'a Vec2> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Vec2>(CGChangeMoveDirectionReq::VT_DIRECTION, None)}
  }
}

impl flatbuffers::Verifiable for CGChangeMoveDirectionReq<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Vec2>("direction", Self::VT_DIRECTION, false)?
     .finish();
    Ok(())
  }
}
pub struct CGChangeMoveDirectionReqArgs<'a> {
    pub direction: Option<&'a Vec2>,
}
impl<'a> Default for CGChangeMoveDirectionReqArgs<'a> {
  #[inline]
  fn default() -> Self {
    CGChangeMoveDirectionReqArgs {
      direction: None,
    }
  }
}

pub struct CGChangeMoveDirectionReqBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> CGChangeMoveDirectionReqBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_direction(&mut self, direction: &Vec2) {
    self.fbb_.push_slot_always::<&Vec2>(CGChangeMoveDirectionReq::VT_DIRECTION, direction);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> CGChangeMoveDirectionReqBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    CGChangeMoveDirectionReqBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<CGChangeMoveDirectionReq<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for CGChangeMoveDirectionReq<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("CGChangeMoveDirectionReq");
      ds.field("direction", &self.direction());
      ds.finish()
  }
}
pub enum GCChangeMoveDirectionResOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct GCChangeMoveDirectionRes<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for GCChangeMoveDirectionRes<'a> {
  type Inner = GCChangeMoveDirectionRes<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> GCChangeMoveDirectionRes<'a> {
  pub const VT_RESULT: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    GCChangeMoveDirectionRes { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args GCChangeMoveDirectionResArgs
  ) -> flatbuffers::WIPOffset<GCChangeMoveDirectionRes<'bldr>> {
    let mut builder = GCChangeMoveDirectionResBuilder::new(_fbb);
    builder.add_result(args.result);
    builder.finish()
  }


  #[inline]
  pub fn result(&self) -> ServerCode {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<ServerCode>(GCChangeMoveDirectionRes::VT_RESULT, Some(ServerCode::SUCCESS)).unwrap()}
  }
}

impl flatbuffers::Verifiable for GCChangeMoveDirectionRes<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<ServerCode>("result", Self::VT_RESULT, false)?
     .finish();
    Ok(())
  }
}
pub struct GCChangeMoveDirectionResArgs {
    pub result: ServerCode,
}
impl<'a> Default for GCChangeMoveDirectionResArgs {
  #[inline]
  fn default() -> Self {
    GCChangeMoveDirectionResArgs {
      result: ServerCode::SUCCESS,
    }
  }
}

pub struct GCChangeMoveDirectionResBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> GCChangeMoveDirectionResBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_result(&mut self, result: ServerCode) {
    self.fbb_.push_slot::<ServerCode>(GCChangeMoveDirectionRes::VT_RESULT, result, ServerCode::SUCCESS);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> GCChangeMoveDirectionResBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    GCChangeMoveDirectionResBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<GCChangeMoveDirectionRes<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for GCChangeMoveDirectionRes<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("GCChangeMoveDirectionRes");
      ds.field("result", &self.result());
      ds.finish()
  }
}
pub enum GCChangeActorDirectionNotiOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct GCChangeActorDirectionNoti<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for GCChangeActorDirectionNoti<'a> {
  type Inner = GCChangeActorDirectionNoti<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> GCChangeActorDirectionNoti<'a> {
  pub const VT_ACTOR_IDX: flatbuffers::VOffsetT = 4;
  pub const VT_DIRECTION: flatbuffers::VOffsetT = 6;
  pub const VT_POSITION: flatbuffers::VOffsetT = 8;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    GCChangeActorDirectionNoti { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args GCChangeActorDirectionNotiArgs<'args>
  ) -> flatbuffers::WIPOffset<GCChangeActorDirectionNoti<'bldr>> {
    let mut builder = GCChangeActorDirectionNotiBuilder::new(_fbb);
    builder.add_actor_idx(args.actor_idx);
    if let Some(x) = args.position { builder.add_position(x); }
    if let Some(x) = args.direction { builder.add_direction(x); }
    builder.finish()
  }


  #[inline]
  pub fn actor_idx(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(GCChangeActorDirectionNoti::VT_ACTOR_IDX, Some(0)).unwrap()}
  }
  #[inline]
  pub fn direction(&self) -> Option<&'a Vec2> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Vec2>(GCChangeActorDirectionNoti::VT_DIRECTION, None)}
  }
  #[inline]
  pub fn position(&self) -> Option<&'a Vec2> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Vec2>(GCChangeActorDirectionNoti::VT_POSITION, None)}
  }
}

impl flatbuffers::Verifiable for GCChangeActorDirectionNoti<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("actor_idx", Self::VT_ACTOR_IDX, false)?
     .visit_field::<Vec2>("direction", Self::VT_DIRECTION, false)?
     .visit_field::<Vec2>("position", Self::VT_POSITION, false)?
     .finish();
    Ok(())
  }
}
pub struct GCChangeActorDirectionNotiArgs<'a> {
    pub actor_idx: u64,
    pub direction: Option<&'a Vec2>,
    pub position: Option<&'a Vec2>,
}
impl<'a> Default for GCChangeActorDirectionNotiArgs<'a> {
  #[inline]
  fn default() -> Self {
    GCChangeActorDirectionNotiArgs {
      actor_idx: 0,
      direction: None,
      position: None,
    }
  }
}

pub struct GCChangeActorDirectionNotiBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> GCChangeActorDirectionNotiBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_actor_idx(&mut self, actor_idx: u64) {
    self.fbb_.push_slot::<u64>(GCChangeActorDirectionNoti::VT_ACTOR_IDX, actor_idx, 0);
  }
  #[inline]
  pub fn add_direction(&mut self, direction: &Vec2) {
    self.fbb_.push_slot_always::<&Vec2>(GCChangeActorDirectionNoti::VT_DIRECTION, direction);
  }
  #[inline]
  pub fn add_position(&mut self, position: &Vec2) {
    self.fbb_.push_slot_always::<&Vec2>(GCChangeActorDirectionNoti::VT_POSITION, position);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> GCChangeActorDirectionNotiBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    GCChangeActorDirectionNotiBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<GCChangeActorDirectionNoti<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for GCChangeActorDirectionNoti<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("GCChangeActorDirectionNoti");
      ds.field("actor_idx", &self.actor_idx());
      ds.field("direction", &self.direction());
      ds.field("position", &self.position());
      ds.finish()
  }
}
}  // pub mod Nexus

