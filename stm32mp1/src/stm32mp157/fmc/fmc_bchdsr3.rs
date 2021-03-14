#[doc = "Reader of register FMC_BCHDSR3"]
pub type R = crate::R<u32, super::FMC_BCHDSR3>;
#[doc = "Reader of field `EBP5`"]
pub type EBP5_R = crate::R<u16, u16>;
#[doc = "Reader of field `EBP6`"]
pub type EBP6_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - EBP5"]
    #[inline(always)]
    pub fn ebp5(&self) -> EBP5_R {
        EBP5_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - EBP6"]
    #[inline(always)]
    pub fn ebp6(&self) -> EBP6_R {
        EBP6_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
