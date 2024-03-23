#[doc = "Register `ADF_DFLT0IER` reader"]
pub type R = crate::R<ADF_DFLT0IERrs>;
#[doc = "Register `ADF_DFLT0IER` writer"]
pub type W = crate::W<ADF_DFLT0IERrs>;
#[doc = "Field `FTHIE` reader - RXFIFO threshold interrupt enable"]
pub type FTHIE_R = crate::BitReader;
#[doc = "Field `FTHIE` writer - RXFIFO threshold interrupt enable"]
pub type FTHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOVRIE` reader - Data overflow interrupt enable"]
pub type DOVRIE_R = crate::BitReader;
#[doc = "Field `DOVRIE` writer - Data overflow interrupt enable"]
pub type DOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SATIE` reader - Saturation detection interrupt enable"]
pub type SATIE_R = crate::BitReader;
#[doc = "Field `SATIE` writer - Saturation detection interrupt enable"]
pub type SATIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKABIE` reader - Clock absence detection interrupt enable"]
pub type CKABIE_R = crate::BitReader;
#[doc = "Field `CKABIE` writer - Clock absence detection interrupt enable"]
pub type CKABIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFOVRIE` reader - Reshape filter overrun interrupt enable"]
pub type RFOVRIE_R = crate::BitReader;
#[doc = "Field `RFOVRIE` writer - Reshape filter overrun interrupt enable"]
pub type RFOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDDETIE` reader - Sound activity detection interrupt enable"]
pub type SDDETIE_R = crate::BitReader;
#[doc = "Field `SDDETIE` writer - Sound activity detection interrupt enable"]
pub type SDDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDLVLIE` reader - SAD sound-level value ready enable"]
pub type SDLVLIE_R = crate::BitReader;
#[doc = "Field `SDLVLIE` writer - SAD sound-level value ready enable"]
pub type SDLVLIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXFIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn fthie(&self) -> FTHIE_R {
        FTHIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data overflow interrupt enable"]
    #[inline(always)]
    pub fn dovrie(&self) -> DOVRIE_R {
        DOVRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - Saturation detection interrupt enable"]
    #[inline(always)]
    pub fn satie(&self) -> SATIE_R {
        SATIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock absence detection interrupt enable"]
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reshape filter overrun interrupt enable"]
    #[inline(always)]
    pub fn rfovrie(&self) -> RFOVRIE_R {
        RFOVRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Sound activity detection interrupt enable"]
    #[inline(always)]
    pub fn sddetie(&self) -> SDDETIE_R {
        SDDETIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SAD sound-level value ready enable"]
    #[inline(always)]
    pub fn sdlvlie(&self) -> SDLVLIE_R {
        SDLVLIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXFIFO threshold interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fthie(&mut self) -> FTHIE_W<ADF_DFLT0IERrs> {
        FTHIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dovrie(&mut self) -> DOVRIE_W<ADF_DFLT0IERrs> {
        DOVRIE_W::new(self, 1)
    }
    #[doc = "Bit 9 - Saturation detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn satie(&mut self) -> SATIE_W<ADF_DFLT0IERrs> {
        SATIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock absence detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ckabie(&mut self) -> CKABIE_W<ADF_DFLT0IERrs> {
        CKABIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Reshape filter overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfovrie(&mut self) -> RFOVRIE_W<ADF_DFLT0IERrs> {
        RFOVRIE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Sound activity detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sddetie(&mut self) -> SDDETIE_W<ADF_DFLT0IERrs> {
        SDDETIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - SAD sound-level value ready enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdlvlie(&mut self) -> SDLVLIE_W<ADF_DFLT0IERrs> {
        SDLVLIE_W::new(self, 13)
    }
}
#[doc = "ADF DFLT0 interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adf_dflt0ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adf_dflt0ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADF_DFLT0IERrs;
impl crate::RegisterSpec for ADF_DFLT0IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adf_dflt0ier::R`](R) reader structure"]
impl crate::Readable for ADF_DFLT0IERrs {}
#[doc = "`write(|w| ..)` method takes [`adf_dflt0ier::W`](W) writer structure"]
impl crate::Writable for ADF_DFLT0IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADF_DFLT0IER to value 0"]
impl crate::Resettable for ADF_DFLT0IERrs {
    const RESET_VALUE: u32 = 0;
}
