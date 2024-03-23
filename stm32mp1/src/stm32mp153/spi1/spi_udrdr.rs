#[doc = "Register `SPI_UDRDR` reader"]
pub type R = crate::R<SPI_UDRDRrs>;
#[doc = "Register `SPI_UDRDR` writer"]
pub type W = crate::W<SPI_UDRDRrs>;
#[doc = "Field `UDRDR` reader - UDRDR"]
pub type UDRDR_R = crate::FieldReader<u32>;
#[doc = "Field `UDRDR` writer - UDRDR"]
pub type UDRDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - UDRDR"]
    #[inline(always)]
    pub fn udrdr(&self) -> UDRDR_R {
        UDRDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - UDRDR"]
    #[inline(always)]
    #[must_use]
    pub fn udrdr(&mut self) -> UDRDR_W<SPI_UDRDRrs> {
        UDRDR_W::new(self, 0)
    }
}
#[doc = "SPI underrun data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_udrdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_udrdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_UDRDRrs;
impl crate::RegisterSpec for SPI_UDRDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_udrdr::R`](R) reader structure"]
impl crate::Readable for SPI_UDRDRrs {}
#[doc = "`write(|w| ..)` method takes [`spi_udrdr::W`](W) writer structure"]
impl crate::Writable for SPI_UDRDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_UDRDR to value 0"]
impl crate::Resettable for SPI_UDRDRrs {
    const RESET_VALUE: u32 = 0;
}
