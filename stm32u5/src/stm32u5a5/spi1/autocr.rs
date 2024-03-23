#[doc = "Register `AUTOCR` reader"]
pub type R = crate::R<AUTOCRrs>;
#[doc = "Register `AUTOCR` writer"]
pub type W = crate::W<AUTOCRrs>;
#[doc = "Field `TRIGSEL` reader - TRIGSEL"]
pub type TRIGSEL_R = crate::FieldReader;
#[doc = "Field `TRIGSEL` writer - TRIGSEL"]
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIGPOL` reader - TRIGPOL"]
pub type TRIGPOL_R = crate::BitReader;
#[doc = "Field `TRIGPOL` writer - TRIGPOL"]
pub type TRIGPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGEN` reader - TRIGEN"]
pub type TRIGEN_R = crate::BitReader;
#[doc = "Field `TRIGEN` writer - TRIGEN"]
pub type TRIGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:19 - TRIGSEL"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TRIGPOL"]
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TRIGEN"]
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:19 - TRIGSEL"]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<AUTOCRrs> {
        TRIGSEL_W::new(self, 16)
    }
    #[doc = "Bit 20 - TRIGPOL"]
    #[inline(always)]
    #[must_use]
    pub fn trigpol(&mut self) -> TRIGPOL_W<AUTOCRrs> {
        TRIGPOL_W::new(self, 20)
    }
    #[doc = "Bit 21 - TRIGEN"]
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<AUTOCRrs> {
        TRIGEN_W::new(self, 21)
    }
}
#[doc = "SPI autonomous mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUTOCRrs;
impl crate::RegisterSpec for AUTOCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocr::R`](R) reader structure"]
impl crate::Readable for AUTOCRrs {}
#[doc = "`write(|w| ..)` method takes [`autocr::W`](W) writer structure"]
impl crate::Writable for AUTOCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOCR to value 0"]
impl crate::Resettable for AUTOCRrs {
    const RESET_VALUE: u32 = 0;
}
