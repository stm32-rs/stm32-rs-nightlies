#[doc = "Reader of register ITLINE4"]
pub type R = crate::R<u32, super::ITLINE4>;
#[doc = "Reader of field `RCC`"]
pub type RCC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RCC"]
    #[inline(always)]
    pub fn rcc(&self) -> RCC_R {
        RCC_R::new((self.bits & 0x01) != 0)
    }
}
