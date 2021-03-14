#[doc = "Reader of register UR4"]
pub type R = crate::R<u32, super::UR4>;
#[doc = "Reader of field `MEPAD_1`"]
pub type MEPAD_1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 16 - Mass Erase Protected Area Disabled for bank 1"]
    #[inline(always)]
    pub fn mepad_1(&self) -> MEPAD_1_R {
        MEPAD_1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
