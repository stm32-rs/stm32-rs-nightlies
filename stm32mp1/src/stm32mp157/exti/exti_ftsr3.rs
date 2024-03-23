#[doc = "Register `EXTI_FTSR3` reader"]
pub type R = crate::R<EXTI_FTSR3rs>;
#[doc = "Register `EXTI_FTSR3` writer"]
pub type W = crate::W<EXTI_FTSR3rs>;
#[doc = "Field `FT65` reader - FT65"]
pub type FT65_R = crate::BitReader;
#[doc = "Field `FT65` writer - FT65"]
pub type FT65_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT66` reader - FT66"]
pub type FT66_R = crate::BitReader;
#[doc = "Field `FT66` writer - FT66"]
pub type FT66_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT68` reader - FT68"]
pub type FT68_R = crate::BitReader;
#[doc = "Field `FT68` writer - FT68"]
pub type FT68_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT73` reader - FT73"]
pub type FT73_R = crate::BitReader;
#[doc = "Field `FT73` writer - FT73"]
pub type FT73_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT74` reader - FT74"]
pub type FT74_R = crate::BitReader;
#[doc = "Field `FT74` writer - FT74"]
pub type FT74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - FT65"]
    #[inline(always)]
    pub fn ft65(&self) -> FT65_R {
        FT65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FT66"]
    #[inline(always)]
    pub fn ft66(&self) -> FT66_R {
        FT66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FT68"]
    #[inline(always)]
    pub fn ft68(&self) -> FT68_R {
        FT68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - FT73"]
    #[inline(always)]
    pub fn ft73(&self) -> FT73_R {
        FT73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FT74"]
    #[inline(always)]
    pub fn ft74(&self) -> FT74_R {
        FT74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FT65"]
    #[inline(always)]
    #[must_use]
    pub fn ft65(&mut self) -> FT65_W<EXTI_FTSR3rs> {
        FT65_W::new(self, 1)
    }
    #[doc = "Bit 2 - FT66"]
    #[inline(always)]
    #[must_use]
    pub fn ft66(&mut self) -> FT66_W<EXTI_FTSR3rs> {
        FT66_W::new(self, 2)
    }
    #[doc = "Bit 4 - FT68"]
    #[inline(always)]
    #[must_use]
    pub fn ft68(&mut self) -> FT68_W<EXTI_FTSR3rs> {
        FT68_W::new(self, 4)
    }
    #[doc = "Bit 9 - FT73"]
    #[inline(always)]
    #[must_use]
    pub fn ft73(&mut self) -> FT73_W<EXTI_FTSR3rs> {
        FT73_W::new(self, 9)
    }
    #[doc = "Bit 10 - FT74"]
    #[inline(always)]
    #[must_use]
    pub fn ft74(&mut self) -> FT74_W<EXTI_FTSR3rs> {
        FT74_W::new(self, 10)
    }
}
#[doc = "Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_ftsr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_ftsr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_FTSR3rs;
impl crate::RegisterSpec for EXTI_FTSR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_ftsr3::R`](R) reader structure"]
impl crate::Readable for EXTI_FTSR3rs {}
#[doc = "`write(|w| ..)` method takes [`exti_ftsr3::W`](W) writer structure"]
impl crate::Writable for EXTI_FTSR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_FTSR3 to value 0"]
impl crate::Resettable for EXTI_FTSR3rs {
    const RESET_VALUE: u32 = 0;
}
