#[doc = "Reader of register FMC_BCHDSR2"]
pub type R = crate::R<u32, super::FMC_BCHDSR2>;
#[doc = "Reader of field `EBP3`"]
pub type EBP3_R = crate::R<u16, u16>;
#[doc = "Reader of field `EBP4`"]
pub type EBP4_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - EBP3"]
    #[inline(always)]
    pub fn ebp3(&self) -> EBP3_R {
        EBP3_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - EBP4"]
    #[inline(always)]
    pub fn ebp4(&self) -> EBP4_R {
        EBP4_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
