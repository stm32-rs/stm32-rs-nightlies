#[doc = "Reader of register UR10"]
pub type R = crate::R<u32, super::UR10>;
#[doc = "Reader of field `PA_END_2`"]
pub type PA_END_2_R = crate::R<u16, u16>;
#[doc = "Reader of field `SA_BEG_2`"]
pub type SA_BEG_2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Protected area end address for bank 2"]
    #[inline(always)]
    pub fn pa_end_2(&self) -> PA_END_2_R {
        PA_END_2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Secured area start address for bank 2"]
    #[inline(always)]
    pub fn sa_beg_2(&self) -> SA_BEG_2_R {
        SA_BEG_2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
