#[doc = "Register `ITLINE25` reader"]
pub type R = crate::R<ITLINE25rs>;
#[doc = "Field `SPI1` reader - SPI1"]
pub type SPI1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SPI1"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new((self.bits & 1) != 0)
    }
}
#[doc = "interrupt line 25 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline25::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE25rs;
impl crate::RegisterSpec for ITLINE25rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline25::R`](R) reader structure"]
impl crate::Readable for ITLINE25rs {}
#[doc = "`reset()` method sets ITLINE25 to value 0"]
impl crate::Resettable for ITLINE25rs {
    const RESET_VALUE: u32 = 0;
}
