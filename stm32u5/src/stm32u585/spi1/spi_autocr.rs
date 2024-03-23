#[doc = "Register `SPI_AUTOCR` reader"]
pub type R = crate::R<SPI_AUTOCRrs>;
#[doc = "Register `SPI_AUTOCR` writer"]
pub type W = crate::W<SPI_AUTOCRrs>;
#[doc = "Field `TRIGSEL` reader - trigger selection (refer ). ... Note: these bits can be written only when SPE = 0."]
pub type TRIGSEL_R = crate::FieldReader;
#[doc = "Field `TRIGSEL` writer - trigger selection (refer ). ... Note: these bits can be written only when SPE = 0."]
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIGPOL` reader - trigger polarity Note: This bit can be written only when SPE = 0."]
pub type TRIGPOL_R = crate::BitReader;
#[doc = "Field `TRIGPOL` writer - trigger polarity Note: This bit can be written only when SPE = 0."]
pub type TRIGPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGEN` reader - trigger of CSTART control enable Note: if user can't prevent trigger event during write, the TRIGEN has to be changed when SPI is disabled"]
pub type TRIGEN_R = crate::BitReader;
#[doc = "Field `TRIGEN` writer - trigger of CSTART control enable Note: if user can't prevent trigger event during write, the TRIGEN has to be changed when SPI is disabled"]
pub type TRIGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:19 - trigger selection (refer ). ... Note: these bits can be written only when SPE = 0."]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - trigger polarity Note: This bit can be written only when SPE = 0."]
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - trigger of CSTART control enable Note: if user can't prevent trigger event during write, the TRIGEN has to be changed when SPI is disabled"]
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:19 - trigger selection (refer ). ... Note: these bits can be written only when SPE = 0."]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<SPI_AUTOCRrs> {
        TRIGSEL_W::new(self, 16)
    }
    #[doc = "Bit 20 - trigger polarity Note: This bit can be written only when SPE = 0."]
    #[inline(always)]
    #[must_use]
    pub fn trigpol(&mut self) -> TRIGPOL_W<SPI_AUTOCRrs> {
        TRIGPOL_W::new(self, 20)
    }
    #[doc = "Bit 21 - trigger of CSTART control enable Note: if user can't prevent trigger event during write, the TRIGEN has to be changed when SPI is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<SPI_AUTOCRrs> {
        TRIGEN_W::new(self, 21)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_autocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_autocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_AUTOCRrs;
impl crate::RegisterSpec for SPI_AUTOCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_autocr::R`](R) reader structure"]
impl crate::Readable for SPI_AUTOCRrs {}
#[doc = "`write(|w| ..)` method takes [`spi_autocr::W`](W) writer structure"]
impl crate::Writable for SPI_AUTOCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_AUTOCR to value 0"]
impl crate::Resettable for SPI_AUTOCRrs {
    const RESET_VALUE: u32 = 0;
}
