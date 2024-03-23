#[doc = "Register `EXTI_RTSR3` reader"]
pub type R = crate::R<EXTI_RTSR3rs>;
#[doc = "Register `EXTI_RTSR3` writer"]
pub type W = crate::W<EXTI_RTSR3rs>;
#[doc = "Field `RT65` reader - RT65"]
pub type RT65_R = crate::BitReader;
#[doc = "Field `RT65` writer - RT65"]
pub type RT65_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT66` reader - RT66"]
pub type RT66_R = crate::BitReader;
#[doc = "Field `RT66` writer - RT66"]
pub type RT66_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT68` reader - RT68"]
pub type RT68_R = crate::BitReader;
#[doc = "Field `RT68` writer - RT68"]
pub type RT68_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT73` reader - RT73"]
pub type RT73_R = crate::BitReader;
#[doc = "Field `RT73` writer - RT73"]
pub type RT73_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT74` reader - RT74"]
pub type RT74_R = crate::BitReader;
#[doc = "Field `RT74` writer - RT74"]
pub type RT74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - RT65"]
    #[inline(always)]
    pub fn rt65(&self) -> RT65_R {
        RT65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RT66"]
    #[inline(always)]
    pub fn rt66(&self) -> RT66_R {
        RT66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RT68"]
    #[inline(always)]
    pub fn rt68(&self) -> RT68_R {
        RT68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - RT73"]
    #[inline(always)]
    pub fn rt73(&self) -> RT73_R {
        RT73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RT74"]
    #[inline(always)]
    pub fn rt74(&self) -> RT74_R {
        RT74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RT65"]
    #[inline(always)]
    #[must_use]
    pub fn rt65(&mut self) -> RT65_W<EXTI_RTSR3rs> {
        RT65_W::new(self, 1)
    }
    #[doc = "Bit 2 - RT66"]
    #[inline(always)]
    #[must_use]
    pub fn rt66(&mut self) -> RT66_W<EXTI_RTSR3rs> {
        RT66_W::new(self, 2)
    }
    #[doc = "Bit 4 - RT68"]
    #[inline(always)]
    #[must_use]
    pub fn rt68(&mut self) -> RT68_W<EXTI_RTSR3rs> {
        RT68_W::new(self, 4)
    }
    #[doc = "Bit 9 - RT73"]
    #[inline(always)]
    #[must_use]
    pub fn rt73(&mut self) -> RT73_W<EXTI_RTSR3rs> {
        RT73_W::new(self, 9)
    }
    #[doc = "Bit 10 - RT74"]
    #[inline(always)]
    #[must_use]
    pub fn rt74(&mut self) -> RT74_W<EXTI_RTSR3rs> {
        RT74_W::new(self, 10)
    }
}
#[doc = "Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rtsr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rtsr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_RTSR3rs;
impl crate::RegisterSpec for EXTI_RTSR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_rtsr3::R`](R) reader structure"]
impl crate::Readable for EXTI_RTSR3rs {}
#[doc = "`write(|w| ..)` method takes [`exti_rtsr3::W`](W) writer structure"]
impl crate::Writable for EXTI_RTSR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_RTSR3 to value 0"]
impl crate::Resettable for EXTI_RTSR3rs {
    const RESET_VALUE: u32 = 0;
}
