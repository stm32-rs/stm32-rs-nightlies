#[doc = "Reader of register ITLINE25"]
pub type R = crate::R<u32, super::ITLINE25>;
#[doc = "Reader of field `SPI1`"]
pub type SPI1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new((self.bits & 0x01) != 0)
    }
}
