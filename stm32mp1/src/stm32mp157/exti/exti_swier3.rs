#[doc = "Register `EXTI_SWIER3` reader"]
pub type R = crate::R<EXTI_SWIER3rs>;
#[doc = "Register `EXTI_SWIER3` writer"]
pub type W = crate::W<EXTI_SWIER3rs>;
#[doc = "Field `SWI65` reader - SWI65"]
pub type SWI65_R = crate::BitReader;
#[doc = "Field `SWI65` writer - SWI65"]
pub type SWI65_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI66` reader - SWI66"]
pub type SWI66_R = crate::BitReader;
#[doc = "Field `SWI66` writer - SWI66"]
pub type SWI66_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI68` reader - SWI68"]
pub type SWI68_R = crate::BitReader;
#[doc = "Field `SWI68` writer - SWI68"]
pub type SWI68_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI73` reader - SWI73"]
pub type SWI73_R = crate::BitReader;
#[doc = "Field `SWI73` writer - SWI73"]
pub type SWI73_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI74` reader - SWI74"]
pub type SWI74_R = crate::BitReader;
#[doc = "Field `SWI74` writer - SWI74"]
pub type SWI74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SWI65"]
    #[inline(always)]
    pub fn swi65(&self) -> SWI65_R {
        SWI65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWI66"]
    #[inline(always)]
    pub fn swi66(&self) -> SWI66_R {
        SWI66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SWI68"]
    #[inline(always)]
    pub fn swi68(&self) -> SWI68_R {
        SWI68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - SWI73"]
    #[inline(always)]
    pub fn swi73(&self) -> SWI73_R {
        SWI73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SWI74"]
    #[inline(always)]
    pub fn swi74(&self) -> SWI74_R {
        SWI74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SWI65"]
    #[inline(always)]
    #[must_use]
    pub fn swi65(&mut self) -> SWI65_W<EXTI_SWIER3rs> {
        SWI65_W::new(self, 1)
    }
    #[doc = "Bit 2 - SWI66"]
    #[inline(always)]
    #[must_use]
    pub fn swi66(&mut self) -> SWI66_W<EXTI_SWIER3rs> {
        SWI66_W::new(self, 2)
    }
    #[doc = "Bit 4 - SWI68"]
    #[inline(always)]
    #[must_use]
    pub fn swi68(&mut self) -> SWI68_W<EXTI_SWIER3rs> {
        SWI68_W::new(self, 4)
    }
    #[doc = "Bit 9 - SWI73"]
    #[inline(always)]
    #[must_use]
    pub fn swi73(&mut self) -> SWI73_W<EXTI_SWIER3rs> {
        SWI73_W::new(self, 9)
    }
    #[doc = "Bit 10 - SWI74"]
    #[inline(always)]
    #[must_use]
    pub fn swi74(&mut self) -> SWI74_W<EXTI_SWIER3rs> {
        SWI74_W::new(self, 10)
    }
}
#[doc = "Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_swier3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_swier3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_SWIER3rs;
impl crate::RegisterSpec for EXTI_SWIER3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_swier3::R`](R) reader structure"]
impl crate::Readable for EXTI_SWIER3rs {}
#[doc = "`write(|w| ..)` method takes [`exti_swier3::W`](W) writer structure"]
impl crate::Writable for EXTI_SWIER3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_SWIER3 to value 0"]
impl crate::Resettable for EXTI_SWIER3rs {
    const RESET_VALUE: u32 = 0;
}
