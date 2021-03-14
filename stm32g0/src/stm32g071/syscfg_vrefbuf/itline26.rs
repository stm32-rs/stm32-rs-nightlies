#[doc = "Reader of register ITLINE26"]
pub type R = crate::R<u32, super::ITLINE26>;
#[doc = "Reader of field `SPI2`"]
pub type SPI2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 0x01) != 0)
    }
}
