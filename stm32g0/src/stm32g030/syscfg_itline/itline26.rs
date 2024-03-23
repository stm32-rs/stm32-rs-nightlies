#[doc = "Register `ITLINE26` reader"]
pub type R = crate::R<ITLINE26rs>;
#[doc = "Field `SPI2` reader - SPI2"]
pub type SPI2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 26 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline26::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE26rs;
impl crate::RegisterSpec for ITLINE26rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline26::R`](R) reader structure"]
impl crate::Readable for ITLINE26rs {}
#[doc = "`reset()` method sets ITLINE26 to value 0"]
impl crate::Resettable for ITLINE26rs {
    const RESET_VALUE: u32 = 0;
}
