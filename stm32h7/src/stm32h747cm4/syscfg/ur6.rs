#[doc = "Reader of register UR6"]
pub type R = crate::R<u32, super::UR6>;
#[doc = "Reader of field `PA_BEG_1`"]
pub type PA_BEG_1_R = crate::R<u16, u16>;
#[doc = "Reader of field `PA_END_1`"]
pub type PA_END_1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Protected area start address for bank 1"]
    #[inline(always)]
    pub fn pa_beg_1(&self) -> PA_BEG_1_R {
        PA_BEG_1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Protected area end address for bank 1"]
    #[inline(always)]
    pub fn pa_end_1(&self) -> PA_END_1_R {
        PA_END_1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
