#[doc = "Register `EXTI_RPR3` reader"]
pub type R = crate::R<EXTI_RPR3rs>;
#[doc = "Register `EXTI_RPR3` writer"]
pub type W = crate::W<EXTI_RPR3rs>;
#[doc = "Field `RPIF65` reader - RPIF65"]
pub type RPIF65_R = crate::BitReader;
#[doc = "Field `RPIF65` writer - RPIF65"]
pub type RPIF65_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF66` reader - RPIF66"]
pub type RPIF66_R = crate::BitReader;
#[doc = "Field `RPIF66` writer - RPIF66"]
pub type RPIF66_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF68` reader - RPIF68"]
pub type RPIF68_R = crate::BitReader;
#[doc = "Field `RPIF68` writer - RPIF68"]
pub type RPIF68_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF73` reader - RPIF73"]
pub type RPIF73_R = crate::BitReader;
#[doc = "Field `RPIF73` writer - RPIF73"]
pub type RPIF73_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPIF74` reader - RPIF74"]
pub type RPIF74_R = crate::BitReader;
#[doc = "Field `RPIF74` writer - RPIF74"]
pub type RPIF74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - RPIF65"]
    #[inline(always)]
    pub fn rpif65(&self) -> RPIF65_R {
        RPIF65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RPIF66"]
    #[inline(always)]
    pub fn rpif66(&self) -> RPIF66_R {
        RPIF66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RPIF68"]
    #[inline(always)]
    pub fn rpif68(&self) -> RPIF68_R {
        RPIF68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - RPIF73"]
    #[inline(always)]
    pub fn rpif73(&self) -> RPIF73_R {
        RPIF73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RPIF74"]
    #[inline(always)]
    pub fn rpif74(&self) -> RPIF74_R {
        RPIF74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RPIF65"]
    #[inline(always)]
    #[must_use]
    pub fn rpif65(&mut self) -> RPIF65_W<EXTI_RPR3rs> {
        RPIF65_W::new(self, 1)
    }
    #[doc = "Bit 2 - RPIF66"]
    #[inline(always)]
    #[must_use]
    pub fn rpif66(&mut self) -> RPIF66_W<EXTI_RPR3rs> {
        RPIF66_W::new(self, 2)
    }
    #[doc = "Bit 4 - RPIF68"]
    #[inline(always)]
    #[must_use]
    pub fn rpif68(&mut self) -> RPIF68_W<EXTI_RPR3rs> {
        RPIF68_W::new(self, 4)
    }
    #[doc = "Bit 9 - RPIF73"]
    #[inline(always)]
    #[must_use]
    pub fn rpif73(&mut self) -> RPIF73_W<EXTI_RPR3rs> {
        RPIF73_W::new(self, 9)
    }
    #[doc = "Bit 10 - RPIF74"]
    #[inline(always)]
    #[must_use]
    pub fn rpif74(&mut self) -> RPIF74_W<EXTI_RPR3rs> {
        RPIF74_W::new(self, 10)
    }
}
#[doc = "Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_rpr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_rpr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_RPR3rs;
impl crate::RegisterSpec for EXTI_RPR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_rpr3::R`](R) reader structure"]
impl crate::Readable for EXTI_RPR3rs {}
#[doc = "`write(|w| ..)` method takes [`exti_rpr3::W`](W) writer structure"]
impl crate::Writable for EXTI_RPR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_RPR3 to value 0"]
impl crate::Resettable for EXTI_RPR3rs {
    const RESET_VALUE: u32 = 0;
}
