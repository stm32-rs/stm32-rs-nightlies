#[doc = "Reader of register FMC_BCHDSR1"]
pub type R = crate::R<u32, super::FMC_BCHDSR1>;
#[doc = "Reader of field `EBP1`"]
pub type EBP1_R = crate::R<u16, u16>;
#[doc = "Reader of field `EBP2`"]
pub type EBP2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - EBP1"]
    #[inline(always)]
    pub fn ebp1(&self) -> EBP1_R {
        EBP1_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - EBP2"]
    #[inline(always)]
    pub fn ebp2(&self) -> EBP2_R {
        EBP2_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
