#[doc = "Reader of register UR7"]
pub type R = crate::R<u32, super::UR7>;
#[doc = "Reader of field `SA_BEG_1`"]
pub type SA_BEG_1_R = crate::R<u16, u16>;
#[doc = "Reader of field `SA_END_1`"]
pub type SA_END_1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Secured area start address for bank 1"]
    #[inline(always)]
    pub fn sa_beg_1(&self) -> SA_BEG_1_R {
        SA_BEG_1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Secured area end address for bank 1"]
    #[inline(always)]
    pub fn sa_end_1(&self) -> SA_END_1_R {
        SA_END_1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
