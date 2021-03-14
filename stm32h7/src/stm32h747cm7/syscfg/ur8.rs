#[doc = "Reader of register UR8"]
pub type R = crate::R<u32, super::UR8>;
#[doc = "Reader of field `MEPAD_2`"]
pub type MEPAD_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `MESAD_2`"]
pub type MESAD_2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Mass erase protected area disabled for bank 2"]
    #[inline(always)]
    pub fn mepad_2(&self) -> MEPAD_2_R {
        MEPAD_2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Mass erase secured area disabled for bank 2"]
    #[inline(always)]
    pub fn mesad_2(&self) -> MESAD_2_R {
        MESAD_2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
