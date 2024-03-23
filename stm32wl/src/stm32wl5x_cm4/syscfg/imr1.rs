#[doc = "Register `IMR1` reader"]
pub type R = crate::R<IMR1rs>;
#[doc = "Register `IMR1` writer"]
pub type W = crate::W<IMR1rs>;
#[doc = "Field `RTCSTAMPTAMPLSECSSIM` reader - RTCSTAMPTAMPLSECSSIM"]
pub type RTCSTAMPTAMPLSECSSIM_R = crate::BitReader;
#[doc = "Field `RTCSTAMPTAMPLSECSSIM` writer - RTCSTAMPTAMPLSECSSIM"]
pub type RTCSTAMPTAMPLSECSSIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSSRUIM` reader - RTCSSRUIM"]
pub type RTCSSRUIM_R = crate::BitReader;
#[doc = "Field `RTCSSRUIM` writer - RTCSSRUIM"]
pub type RTCSSRUIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI5IM` reader - EXTI5IM"]
pub type EXTI5IM_R = crate::BitReader;
#[doc = "Field `EXTI5IM` writer - EXTI5IM"]
pub type EXTI5IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI6IM` reader - EXTI6IM"]
pub type EXTI6IM_R = crate::BitReader;
#[doc = "Field `EXTI6IM` writer - EXTI6IM"]
pub type EXTI6IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI7IM` reader - EXTI7IM"]
pub type EXTI7IM_R = crate::BitReader;
#[doc = "Field `EXTI7IM` writer - EXTI7IM"]
pub type EXTI7IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI8IM` reader - EXTI8IM"]
pub type EXTI8IM_R = crate::BitReader;
#[doc = "Field `EXTI8IM` writer - EXTI8IM"]
pub type EXTI8IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI9IM` reader - EXTI9IM"]
pub type EXTI9IM_R = crate::BitReader;
#[doc = "Field `EXTI9IM` writer - EXTI9IM"]
pub type EXTI9IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI10IM` reader - EXTI10IM"]
pub type EXTI10IM_R = crate::BitReader;
#[doc = "Field `EXTI10IM` writer - EXTI10IM"]
pub type EXTI10IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI11IM` reader - EXTI11IM"]
pub type EXTI11IM_R = crate::BitReader;
#[doc = "Field `EXTI11IM` writer - EXTI11IM"]
pub type EXTI11IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI12IM` reader - EXTI12IM"]
pub type EXTI12IM_R = crate::BitReader;
#[doc = "Field `EXTI12IM` writer - EXTI12IM"]
pub type EXTI12IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI13IM` reader - EXTI13IM"]
pub type EXTI13IM_R = crate::BitReader;
#[doc = "Field `EXTI13IM` writer - EXTI13IM"]
pub type EXTI13IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI14IM` reader - EXTI14IM"]
pub type EXTI14IM_R = crate::BitReader;
#[doc = "Field `EXTI14IM` writer - EXTI14IM"]
pub type EXTI14IM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI15IM` reader - EXTI15IM"]
pub type EXTI15IM_R = crate::BitReader;
#[doc = "Field `EXTI15IM` writer - EXTI15IM"]
pub type EXTI15IM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTCSTAMPTAMPLSECSSIM"]
    #[inline(always)]
    pub fn rtcstamptamplsecssim(&self) -> RTCSTAMPTAMPLSECSSIM_R {
        RTCSTAMPTAMPLSECSSIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - RTCSSRUIM"]
    #[inline(always)]
    pub fn rtcssruim(&self) -> RTCSSRUIM_R {
        RTCSSRUIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 21 - EXTI5IM"]
    #[inline(always)]
    pub fn exti5im(&self) -> EXTI5IM_R {
        EXTI5IM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EXTI6IM"]
    #[inline(always)]
    pub fn exti6im(&self) -> EXTI6IM_R {
        EXTI6IM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EXTI7IM"]
    #[inline(always)]
    pub fn exti7im(&self) -> EXTI7IM_R {
        EXTI7IM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - EXTI8IM"]
    #[inline(always)]
    pub fn exti8im(&self) -> EXTI8IM_R {
        EXTI8IM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - EXTI9IM"]
    #[inline(always)]
    pub fn exti9im(&self) -> EXTI9IM_R {
        EXTI9IM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EXTI10IM"]
    #[inline(always)]
    pub fn exti10im(&self) -> EXTI10IM_R {
        EXTI10IM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - EXTI11IM"]
    #[inline(always)]
    pub fn exti11im(&self) -> EXTI11IM_R {
        EXTI11IM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - EXTI12IM"]
    #[inline(always)]
    pub fn exti12im(&self) -> EXTI12IM_R {
        EXTI12IM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EXTI13IM"]
    #[inline(always)]
    pub fn exti13im(&self) -> EXTI13IM_R {
        EXTI13IM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EXTI14IM"]
    #[inline(always)]
    pub fn exti14im(&self) -> EXTI14IM_R {
        EXTI14IM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - EXTI15IM"]
    #[inline(always)]
    pub fn exti15im(&self) -> EXTI15IM_R {
        EXTI15IM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTCSTAMPTAMPLSECSSIM"]
    #[inline(always)]
    #[must_use]
    pub fn rtcstamptamplsecssim(&mut self) -> RTCSTAMPTAMPLSECSSIM_W<IMR1rs> {
        RTCSTAMPTAMPLSECSSIM_W::new(self, 0)
    }
    #[doc = "Bit 2 - RTCSSRUIM"]
    #[inline(always)]
    #[must_use]
    pub fn rtcssruim(&mut self) -> RTCSSRUIM_W<IMR1rs> {
        RTCSSRUIM_W::new(self, 2)
    }
    #[doc = "Bit 21 - EXTI5IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti5im(&mut self) -> EXTI5IM_W<IMR1rs> {
        EXTI5IM_W::new(self, 21)
    }
    #[doc = "Bit 22 - EXTI6IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti6im(&mut self) -> EXTI6IM_W<IMR1rs> {
        EXTI6IM_W::new(self, 22)
    }
    #[doc = "Bit 23 - EXTI7IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti7im(&mut self) -> EXTI7IM_W<IMR1rs> {
        EXTI7IM_W::new(self, 23)
    }
    #[doc = "Bit 24 - EXTI8IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti8im(&mut self) -> EXTI8IM_W<IMR1rs> {
        EXTI8IM_W::new(self, 24)
    }
    #[doc = "Bit 25 - EXTI9IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti9im(&mut self) -> EXTI9IM_W<IMR1rs> {
        EXTI9IM_W::new(self, 25)
    }
    #[doc = "Bit 26 - EXTI10IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti10im(&mut self) -> EXTI10IM_W<IMR1rs> {
        EXTI10IM_W::new(self, 26)
    }
    #[doc = "Bit 27 - EXTI11IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti11im(&mut self) -> EXTI11IM_W<IMR1rs> {
        EXTI11IM_W::new(self, 27)
    }
    #[doc = "Bit 28 - EXTI12IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti12im(&mut self) -> EXTI12IM_W<IMR1rs> {
        EXTI12IM_W::new(self, 28)
    }
    #[doc = "Bit 29 - EXTI13IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti13im(&mut self) -> EXTI13IM_W<IMR1rs> {
        EXTI13IM_W::new(self, 29)
    }
    #[doc = "Bit 30 - EXTI14IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti14im(&mut self) -> EXTI14IM_W<IMR1rs> {
        EXTI14IM_W::new(self, 30)
    }
    #[doc = "Bit 31 - EXTI15IM"]
    #[inline(always)]
    #[must_use]
    pub fn exti15im(&mut self) -> EXTI15IM_W<IMR1rs> {
        EXTI15IM_W::new(self, 31)
    }
}
#[doc = "SYSCFG CPU1 interrupt mask register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR1rs;
impl crate::RegisterSpec for IMR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr1::R`](R) reader structure"]
impl crate::Readable for IMR1rs {}
#[doc = "`write(|w| ..)` method takes [`imr1::W`](W) writer structure"]
impl crate::Writable for IMR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR1 to value 0"]
impl crate::Resettable for IMR1rs {
    const RESET_VALUE: u32 = 0;
}
