use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Optional_f32 {
    pub value: f32,
}

impl From<Option<f32>> for Optional_f32 {
    fn from(option: Option<f32>) -> Self {
        match option {
            Some(value) => Self { value },
            None => Self::NONE,
        }
    }
}

impl Optional_f32 {
    pub const NONE_VALUE: f32 = f32::MIN;
    pub const NONE: Self = Self::new(Self::NONE_VALUE);

    #[inline(always)]
    pub const fn new(value: f32) -> Self {
        Self { value }
    }

    #[inline(always)]
    pub fn has_value(self) -> bool {
        self.value != Self::NONE_VALUE
    }

    #[inline(always)]
    pub fn value_or(self, fallback: f32) -> f32 {
        if self.has_value() {
            self.value
        } else {
            fallback
        }
    }
}
