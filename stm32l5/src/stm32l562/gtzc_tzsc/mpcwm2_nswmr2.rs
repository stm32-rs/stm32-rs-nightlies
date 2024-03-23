#[doc = "Register `MPCWM2_NSWMR2` reader"]
pub type R = crate::R<MPCWM2_NSWMR2rs>;
#[doc = "Register `MPCWM2_NSWMR2` writer"]
pub type W = crate::W<MPCWM2_NSWMR2rs>;
#[doc = "Field `NSWM2STRT` reader - NSWM2STRT"]
pub type NSWM2STRT_R = crate::FieldReader<u16>;
#[doc = "Field `NSWM2STRT` writer - NSWM2STRT"]
pub type NSWM2STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NSWM2LGTH` reader - NSWM2LGTH"]
pub type NSWM2LGTH_R = crate::FieldReader<u16>;
#[doc = "Field `NSWM2LGTH` writer - NSWM2LGTH"]
pub type NSWM2LGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:10 - NSWM2STRT"]
    #[inline(always)]
    pub fn nswm2strt(&self) -> NSWM2STRT_R {
        NSWM2STRT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - NSWM2LGTH"]
    #[inline(always)]
    pub fn nswm2lgth(&self) -> NSWM2LGTH_R {
        NSWM2LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - NSWM2STRT"]
    #[inline(always)]
    #[must_use]
    pub fn nswm2strt(&mut self) -> NSWM2STRT_W<MPCWM2_NSWMR2rs> {
        NSWM2STRT_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - NSWM2LGTH"]
    #[inline(always)]
    #[must_use]
    pub fn nswm2lgth(&mut self) -> NSWM2LGTH_W<MPCWM2_NSWMR2rs> {
        NSWM2LGTH_W::new(self, 16)
    }
}
#[doc = "TZSC external memory non-secure watermark register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm2_nswmr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm2_nswmr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPCWM2_NSWMR2rs;
impl crate::RegisterSpec for MPCWM2_NSWMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpcwm2_nswmr2::R`](R) reader structure"]
impl crate::Readable for MPCWM2_NSWMR2rs {}
#[doc = "`write(|w| ..)` method takes [`mpcwm2_nswmr2::W`](W) writer structure"]
impl crate::Writable for MPCWM2_NSWMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPCWM2_NSWMR2 to value 0"]
impl crate::Resettable for MPCWM2_NSWMR2rs {
    const RESET_VALUE: u32 = 0;
}
