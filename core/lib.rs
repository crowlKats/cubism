type Version = u32;

pub const ALIGN_OF_MOC: i32 = 64;
pub const ALIGN_OF_MODEL: i32 = 16;

bitflags::bitflags! {
  pub struct NonDynamicDrawableFlags: u8 {
    const BLEND_ADDITIVE = 1 << 0;
    const BLEND_MULTIPLICATIVE = 1 << 1;
    const IS_DOUBLE_SIDED = 1 << 2;
  }

  pub struct DynamicDrawableFlags: u8 {
    const IS_VISIBLE = 1 << 0;
    const VISIBILITY_DID_CHANGE = 1 << 1;
    const OPACITY_DID_CHANGE = 1 << 2;
    const DRAW_ORDER_DID_CHANGE = 1 << 3;
    const RENDER_ORDER_DID_CHANGE = 1 << 4;
  }
}

#[derive(Copy, Clone, Debug)]
pub enum MocVersion {
  Unknown,
  Version30,
  Version33,
  Version40,
}

#[derive(Copy, Clone, Debug)]
pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

impl Vector2 {
  pub fn new(x: f32, y: f32) -> Self {
    Vector2 { x, y }
  }
}

pub fn get_version() -> Version {
  unsafe { cubism_core_sys::csmGetVersion() }
}

pub fn get_latest_moc_version() -> MocVersion {
  match unsafe { cubism_core_sys::csmGetLatestMocVersion() } {
    cubism_core_sys::csmMocVersion_Unknown => MocVersion::Unknown,
    cubism_core_sys::csmMocVersion_30 => MocVersion::Version30,
    cubism_core_sys::csmMocVersion_33 => MocVersion::Version33,
    cubism_core_sys::csmMocVersion_40 => MocVersion::Version40,
    _ => unreachable!(),
  }
}

pub struct Moc {
  inner: *mut cubism_core_sys::csmMoc,
  _data: Box<[u8]>,
  ptr: *mut std::os::raw::c_void,
  size: u32,
}

impl Moc {
  pub fn revive_moc_in_place(mut data: Box<[u8]>) -> Self {
    // TODO: align memory
    let size = data.len() as u32;
    let ptr = data.as_mut_ptr() as *mut std::os::raw::c_void;
    let moc = unsafe { cubism_core_sys::csmReviveMocInPlace(ptr, size) };
    Moc {
      inner: moc,
      _data: data,
      ptr,
      size,
    }
  }

  pub fn version(&self) -> MocVersion {
    match unsafe { cubism_core_sys::csmGetMocVersion(self.ptr, self.size) } {
      cubism_core_sys::csmMocVersion_Unknown => MocVersion::Unknown,
      cubism_core_sys::csmMocVersion_30 => MocVersion::Version30,
      cubism_core_sys::csmMocVersion_33 => MocVersion::Version33,
      cubism_core_sys::csmMocVersion_40 => MocVersion::Version40,
      _ => unreachable!(),
    }
  }

  pub fn get_size_of_model(&self) -> u32 {
    unsafe {
      cubism_core_sys::csmGetSizeofModel(
        self.inner as *const cubism_core_sys::csmMoc,
      )
    }
  }

  pub fn initialize_model_in_place(&self) -> Model {
    let model = unsafe {
      cubism_core_sys::csmInitializeModelInPlace(
        self.inner as *const cubism_core_sys::csmMoc,
        self.ptr,
        self.size,
      )
    };
    Model { inner: model }
  }
}

pub struct Model {
  inner: *mut cubism_core_sys::csmModel,
}

impl Model {
  pub fn update(&self) {
    unsafe {
      cubism_core_sys::csmUpdateModel(self.inner);
    }
  }

  pub fn read_canvas_info(&self) -> (Vector2, Vector2, f32) {
    let mut size = cubism_core_sys::csmVector2 { X: 0.0, Y: 0.0 };
    let mut origin = cubism_core_sys::csmVector2 { X: 0.0, Y: 0.0 };
    let mut ppu = 0.0f32;
    unsafe {
      cubism_core_sys::csmReadCanvasInfo(
        self.inner,
        &mut size,
        &mut origin,
        &mut ppu,
      );
    }

    (
      Vector2::new(size.X, size.Y),
      Vector2::new(origin.X, origin.Y),
      ppu,
    )
  }

  pub fn get_parameter_count(&self) -> Option<u32> {
    let count = unsafe { cubism_core_sys::csmGetParameterCount(self.inner) };

    if count == -1 {
      None
    } else {
      Some(count as u32)
    }
  }

  pub fn get_parameter_ids(&self) {
    unsafe { cubism_core_sys::csmGetParameterIds(self.inner) };
    // TODO
  }

  pub fn get_parameter_min_values(&self) {
    unsafe { cubism_core_sys::csmGetParameterMinimumValues(self.inner) };
    // TODO
  }

  pub fn get_parameter_max_values(&self) {
    unsafe { cubism_core_sys::csmGetParameterMaximumValues(self.inner) };
    // TODO
  }

  pub fn get_parameter_default_values(&self) {
    unsafe { cubism_core_sys::csmGetParameterDefaultValues(self.inner) };
    // TODO
  }

  pub fn get_parameter_values(&self) {
    unsafe { cubism_core_sys::csmGetParameterValues(self.inner) };
    // TODO
  }

  pub fn get_parameter_key_counts(&self) {
    unsafe { cubism_core_sys::csmGetParameterKeyCounts(self.inner) };
    // TODO
  }

  pub fn get_parameter_key_values(&self) {
    unsafe { cubism_core_sys::csmGetParameterKeyValues(self.inner) };
    // TODO
  }

  pub fn get_part_count(&self) -> Option<u32> {
    let count = unsafe { cubism_core_sys::csmGetPartCount(self.inner) };

    if count == -1 {
      None
    } else {
      Some(count as u32)
    }
  }

  pub fn get_part_ids(&self) {
    unsafe { cubism_core_sys::csmGetPartIds(self.inner) };
    // TODO
  }

  pub fn get_part_opacities(&self) {
    unsafe { cubism_core_sys::csmGetPartOpacities(self.inner) };
    // TODO
  }

  pub fn get_part_parent_part_indices(&self) {
    unsafe { cubism_core_sys::csmGetPartParentPartIndices(self.inner) };
    // TODO
  }

  pub fn get_drawable_count(&self) -> Option<u32> {
    let count = unsafe { cubism_core_sys::csmGetDrawableCount(self.inner) };

    if count == -1 {
      None
    } else {
      Some(count as u32)
    }
  }

  pub fn get_drawable_ids(&self) {
    unsafe { cubism_core_sys::csmGetDrawableIds(self.inner) };
    // TODO
  }

  pub fn get_drawable_constant_flags(&self) -> NonDynamicDrawableFlags {
    let flags_pointer: *const u8 =
      unsafe { cubism_core_sys::csmGetDrawableConstantFlags(self.inner) };
    NonDynamicDrawableFlags::from_bits(unsafe { flags_pointer.read() }).unwrap()
  }

  pub fn get_drawable_dynamic_flags(&self) -> DynamicDrawableFlags {
    let flags_pointer =
      unsafe { cubism_core_sys::csmGetDrawableDynamicFlags(self.inner) };
    DynamicDrawableFlags::from_bits(unsafe { flags_pointer.read() }).unwrap()
  }

  pub fn get_drawable_texture_indices(&self) {
    unsafe { cubism_core_sys::csmGetDrawableTextureIndices(self.inner) };
    // TODO
  }

  pub fn get_drawable_draw_orders(&self) {
    unsafe { cubism_core_sys::csmGetDrawableDrawOrders(self.inner) };
    // TODO
  }

  pub fn get_drawable_render_orders(&self) {
    unsafe { cubism_core_sys::csmGetDrawableRenderOrders(self.inner) };
    // TODO
  }

  pub fn get_drawable_opacities(&self) {
    unsafe { cubism_core_sys::csmGetDrawableOpacities(self.inner) };
    // TODO
  }

  pub fn get_drawable_mask_counts(&self) {
    unsafe { cubism_core_sys::csmGetDrawableMaskCounts(self.inner) };
    // TODO
  }

  pub fn get_drawable_masks(&self) {
    unsafe { cubism_core_sys::csmGetDrawableMasks(self.inner) };
    // TODO
  }

  pub fn get_drawable_vertex_counts(&self) {
    unsafe { cubism_core_sys::csmGetDrawableVertexCounts(self.inner) };
    // TODO
  }

  pub fn get_drawable_vertex_positions(&self) {
    unsafe { cubism_core_sys::csmGetDrawableVertexPositions(self.inner) };
    // TODO
  }

  pub fn get_drawable_vertex_uvs(&self) {
    unsafe { cubism_core_sys::csmGetDrawableVertexUvs(self.inner) };
    // TODO
  }

  pub fn get_drawable_index_counts(&self) {
    unsafe { cubism_core_sys::csmGetDrawableIndexCounts(self.inner) };
    // TODO
  }

  pub fn get_drawable_indices(&self) {
    unsafe { cubism_core_sys::csmGetDrawableIndices(self.inner) };
    // TODO
  }

  pub fn reset_drawable_dynamic_flags(&self) {
    unsafe {
      cubism_core_sys::csmResetDrawableDynamicFlags(self.inner);
    }
  }
}

/*
 csmGetLogFunction
 csmSetLogFunction
*/
