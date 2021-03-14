#[doc = "Reader of register FMC_BCHDSR4"]
pub type R = crate::R<u32, super::FMC_BCHDSR4>;
#[doc = "Reader of field `EBP7`"]
pub type EBP7_R = crate::R<u16, u16>;
#[doc = "Reader of field `EBP8`"]
pub type EBP8_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - EBP7"]
    #[inline(always)]
    pub fn ebp7(&self) -> EBP7_R {
        EBP7_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - EBP8"]
    #[inline(always)]
    pub fn ebp8(&self) -> EBP8_R {
        EBP8_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
