#[doc = "Register `TIMINGR` reader"]
pub type R = crate::R<TIMINGRrs>;
#[doc = "Register `TIMINGR` writer"]
pub type W = crate::W<TIMINGRrs>;
#[doc = "Field `SCLL` reader - SCLL"]
pub type SCLL_R = crate::FieldReader;
#[doc = "Field `SCLL` writer - SCLL"]
pub type SCLL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `SCLH` reader - SCLH"]
pub type SCLH_R = crate::FieldReader;
#[doc = "Field `SCLH` writer - SCLH"]
pub type SCLH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `SDADEL` reader - SDADEL"]
pub type SDADEL_R = crate::FieldReader;
#[doc = "Field `SDADEL` writer - SDADEL"]
pub type SDADEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `SCLDEL` reader - SCLDEL"]
pub type SCLDEL_R = crate::FieldReader;
#[doc = "Field `SCLDEL` writer - SCLDEL"]
pub type SCLDEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `PRESC` reader - PRESC"]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - PRESC"]
pub type PRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - SCLL"]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCLH"]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - SDADEL"]
    #[inline(always)]
    pub fn sdadel(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SCLDEL"]
    #[inline(always)]
    pub fn scldel(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PRESC"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCLL"]
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> SCLL_W<TIMINGRrs> {
        SCLL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SCLH"]
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SCLH_W<TIMINGRrs> {
        SCLH_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - SDADEL"]
    #[inline(always)]
    #[must_use]
    pub fn sdadel(&mut self) -> SDADEL_W<TIMINGRrs> {
        SDADEL_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - SCLDEL"]
    #[inline(always)]
    #[must_use]
    pub fn scldel(&mut self) -> SCLDEL_W<TIMINGRrs> {
        SCLDEL_W::new(self, 20)
    }
    #[doc = "Bits 28:31 - PRESC"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<TIMINGRrs> {
        PRESC_W::new(self, 28)
    }
}
#[doc = "Timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timingr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timingr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMINGRrs;
impl crate::RegisterSpec for TIMINGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timingr::R`](R) reader structure"]
impl crate::Readable for TIMINGRrs {}
#[doc = "`write(|w| ..)` method takes [`timingr::W`](W) writer structure"]
impl crate::Writable for TIMINGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMINGR to value 0"]
impl crate::Resettable for TIMINGRrs {
    const RESET_VALUE: u32 = 0;
}
