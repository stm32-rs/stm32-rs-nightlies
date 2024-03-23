#[doc = "Register `SPI_CRCPOLY` reader"]
pub type R = crate::R<SPI_CRCPOLYrs>;
#[doc = "Register `SPI_CRCPOLY` writer"]
pub type W = crate::W<SPI_CRCPOLYrs>;
#[doc = "Field `CRCPOLY` reader - CRCPOLY"]
pub type CRCPOLY_R = crate::FieldReader<u32>;
#[doc = "Field `CRCPOLY` writer - CRCPOLY"]
pub type CRCPOLY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRCPOLY"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRCPOLY"]
    #[inline(always)]
    #[must_use]
    pub fn crcpoly(&mut self) -> CRCPOLY_W<SPI_CRCPOLYrs> {
        CRCPOLY_W::new(self, 0)
    }
}
#[doc = "SPI polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_crcpoly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_crcpoly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CRCPOLYrs;
impl crate::RegisterSpec for SPI_CRCPOLYrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_crcpoly::R`](R) reader structure"]
impl crate::Readable for SPI_CRCPOLYrs {}
#[doc = "`write(|w| ..)` method takes [`spi_crcpoly::W`](W) writer structure"]
impl crate::Writable for SPI_CRCPOLYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_CRCPOLY to value 0x0107"]
impl crate::Resettable for SPI_CRCPOLYrs {
    const RESET_VALUE: u32 = 0x0107;
}
