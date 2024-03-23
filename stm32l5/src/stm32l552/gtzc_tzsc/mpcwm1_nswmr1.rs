#[doc = "Register `MPCWM1_NSWMR1` reader"]
pub type R = crate::R<MPCWM1_NSWMR1rs>;
#[doc = "Register `MPCWM1_NSWMR1` writer"]
pub type W = crate::W<MPCWM1_NSWMR1rs>;
#[doc = "Field `NSWM1STRT` reader - NSWM1STRT"]
pub type NSWM1STRT_R = crate::FieldReader<u16>;
#[doc = "Field `NSWM1STRT` writer - NSWM1STRT"]
pub type NSWM1STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NSWM1LGTH` reader - NSWM1LGTH"]
pub type NSWM1LGTH_R = crate::FieldReader<u16>;
#[doc = "Field `NSWM1LGTH` writer - NSWM1LGTH"]
pub type NSWM1LGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:10 - NSWM1STRT"]
    #[inline(always)]
    pub fn nswm1strt(&self) -> NSWM1STRT_R {
        NSWM1STRT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - NSWM1LGTH"]
    #[inline(always)]
    pub fn nswm1lgth(&self) -> NSWM1LGTH_R {
        NSWM1LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - NSWM1STRT"]
    #[inline(always)]
    #[must_use]
    pub fn nswm1strt(&mut self) -> NSWM1STRT_W<MPCWM1_NSWMR1rs> {
        NSWM1STRT_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - NSWM1LGTH"]
    #[inline(always)]
    #[must_use]
    pub fn nswm1lgth(&mut self) -> NSWM1LGTH_W<MPCWM1_NSWMR1rs> {
        NSWM1LGTH_W::new(self, 16)
    }
}
#[doc = "TZSC external memory non-secure watermark register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpcwm1_nswmr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpcwm1_nswmr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MPCWM1_NSWMR1rs;
impl crate::RegisterSpec for MPCWM1_NSWMR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpcwm1_nswmr1::R`](R) reader structure"]
impl crate::Readable for MPCWM1_NSWMR1rs {}
#[doc = "`write(|w| ..)` method takes [`mpcwm1_nswmr1::W`](W) writer structure"]
impl crate::Writable for MPCWM1_NSWMR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPCWM1_NSWMR1 to value 0"]
impl crate::Resettable for MPCWM1_NSWMR1rs {
    const RESET_VALUE: u32 = 0;
}
