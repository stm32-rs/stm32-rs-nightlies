#[doc = "Register `ADF_DFLT0ISR` reader"]
pub type R = crate::R<ADF_DFLT0ISRrs>;
#[doc = "Register `ADF_DFLT0ISR` writer"]
pub type W = crate::W<ADF_DFLT0ISRrs>;
#[doc = "Field `FTHF` reader - RXFIFO threshold flag"]
pub type FTHF_R = crate::BitReader;
#[doc = "Field `DOVRF` reader - Data overflow flag"]
pub type DOVRF_R = crate::BitReader;
#[doc = "Field `DOVRF` writer - Data overflow flag"]
pub type DOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEF` reader - RXFIFO not empty flag"]
pub type RXNEF_R = crate::BitReader;
#[doc = "Field `SATF` reader - Saturation detection flag"]
pub type SATF_R = crate::BitReader;
#[doc = "Field `SATF` writer - Saturation detection flag"]
pub type SATF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKABF` reader - Clock absence detection flag"]
pub type CKABF_R = crate::BitReader;
#[doc = "Field `CKABF` writer - Clock absence detection flag"]
pub type CKABF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFOVRF` reader - Reshape filter overrun detection flag"]
pub type RFOVRF_R = crate::BitReader;
#[doc = "Field `RFOVRF` writer - Reshape filter overrun detection flag"]
pub type RFOVRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDDETF` reader - Sound activity detection flag"]
pub type SDDETF_R = crate::BitReader;
#[doc = "Field `SDDETF` writer - Sound activity detection flag"]
pub type SDDETF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDLVLF` reader - Sound level value ready flag"]
pub type SDLVLF_R = crate::BitReader;
#[doc = "Field `SDLVLF` writer - Sound level value ready flag"]
pub type SDLVLF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXFIFO threshold flag"]
    #[inline(always)]
    pub fn fthf(&self) -> FTHF_R {
        FTHF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data overflow flag"]
    #[inline(always)]
    pub fn dovrf(&self) -> DOVRF_R {
        DOVRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - RXFIFO not empty flag"]
    #[inline(always)]
    pub fn rxnef(&self) -> RXNEF_R {
        RXNEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Saturation detection flag"]
    #[inline(always)]
    pub fn satf(&self) -> SATF_R {
        SATF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock absence detection flag"]
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reshape filter overrun detection flag"]
    #[inline(always)]
    pub fn rfovrf(&self) -> RFOVRF_R {
        RFOVRF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Sound activity detection flag"]
    #[inline(always)]
    pub fn sddetf(&self) -> SDDETF_R {
        SDDETF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Sound level value ready flag"]
    #[inline(always)]
    pub fn sdlvlf(&self) -> SDLVLF_R {
        SDLVLF_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Data overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn dovrf(&mut self) -> DOVRF_W<ADF_DFLT0ISRrs> {
        DOVRF_W::new(self, 1)
    }
    #[doc = "Bit 9 - Saturation detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn satf(&mut self) -> SATF_W<ADF_DFLT0ISRrs> {
        SATF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock absence detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn ckabf(&mut self) -> CKABF_W<ADF_DFLT0ISRrs> {
        CKABF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Reshape filter overrun detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn rfovrf(&mut self) -> RFOVRF_W<ADF_DFLT0ISRrs> {
        RFOVRF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Sound activity detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn sddetf(&mut self) -> SDDETF_W<ADF_DFLT0ISRrs> {
        SDDETF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Sound level value ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn sdlvlf(&mut self) -> SDLVLF_W<ADF_DFLT0ISRrs> {
        SDLVLF_W::new(self, 13)
    }
}
#[doc = "ADF DFLT0 interrupt status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adf_dflt0isr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adf_dflt0isr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADF_DFLT0ISRrs;
impl crate::RegisterSpec for ADF_DFLT0ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adf_dflt0isr::R`](R) reader structure"]
impl crate::Readable for ADF_DFLT0ISRrs {}
#[doc = "`write(|w| ..)` method takes [`adf_dflt0isr::W`](W) writer structure"]
impl crate::Writable for ADF_DFLT0ISRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADF_DFLT0ISR to value 0"]
impl crate::Resettable for ADF_DFLT0ISRrs {
    const RESET_VALUE: u32 = 0;
}
