#[doc = "Register `EXTI_FPR3` reader"]
pub type R = crate::R<EXTI_FPR3rs>;
#[doc = "Register `EXTI_FPR3` writer"]
pub type W = crate::W<EXTI_FPR3rs>;
#[doc = "Field `FPIF65` reader - FPIF65"]
pub type FPIF65_R = crate::BitReader;
#[doc = "Field `FPIF65` writer - FPIF65"]
pub type FPIF65_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF66` reader - FPIF66"]
pub type FPIF66_R = crate::BitReader;
#[doc = "Field `FPIF66` writer - FPIF66"]
pub type FPIF66_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF68` reader - FPIF68"]
pub type FPIF68_R = crate::BitReader;
#[doc = "Field `FPIF68` writer - FPIF68"]
pub type FPIF68_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF73` reader - FPIF73"]
pub type FPIF73_R = crate::BitReader;
#[doc = "Field `FPIF73` writer - FPIF73"]
pub type FPIF73_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF74` reader - FPIF74"]
pub type FPIF74_R = crate::BitReader;
#[doc = "Field `FPIF74` writer - FPIF74"]
pub type FPIF74_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - FPIF65"]
    #[inline(always)]
    pub fn fpif65(&self) -> FPIF65_R {
        FPIF65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FPIF66"]
    #[inline(always)]
    pub fn fpif66(&self) -> FPIF66_R {
        FPIF66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FPIF68"]
    #[inline(always)]
    pub fn fpif68(&self) -> FPIF68_R {
        FPIF68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - FPIF73"]
    #[inline(always)]
    pub fn fpif73(&self) -> FPIF73_R {
        FPIF73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FPIF74"]
    #[inline(always)]
    pub fn fpif74(&self) -> FPIF74_R {
        FPIF74_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FPIF65"]
    #[inline(always)]
    #[must_use]
    pub fn fpif65(&mut self) -> FPIF65_W<EXTI_FPR3rs> {
        FPIF65_W::new(self, 1)
    }
    #[doc = "Bit 2 - FPIF66"]
    #[inline(always)]
    #[must_use]
    pub fn fpif66(&mut self) -> FPIF66_W<EXTI_FPR3rs> {
        FPIF66_W::new(self, 2)
    }
    #[doc = "Bit 4 - FPIF68"]
    #[inline(always)]
    #[must_use]
    pub fn fpif68(&mut self) -> FPIF68_W<EXTI_FPR3rs> {
        FPIF68_W::new(self, 4)
    }
    #[doc = "Bit 9 - FPIF73"]
    #[inline(always)]
    #[must_use]
    pub fn fpif73(&mut self) -> FPIF73_W<EXTI_FPR3rs> {
        FPIF73_W::new(self, 9)
    }
    #[doc = "Bit 10 - FPIF74"]
    #[inline(always)]
    #[must_use]
    pub fn fpif74(&mut self) -> FPIF74_W<EXTI_FPR3rs> {
        FPIF74_W::new(self, 10)
    }
}
#[doc = "Contains only register bits for configurable events.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_fpr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exti_fpr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_FPR3rs;
impl crate::RegisterSpec for EXTI_FPR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_fpr3::R`](R) reader structure"]
impl crate::Readable for EXTI_FPR3rs {}
#[doc = "`write(|w| ..)` method takes [`exti_fpr3::W`](W) writer structure"]
impl crate::Writable for EXTI_FPR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTI_FPR3 to value 0"]
impl crate::Resettable for EXTI_FPR3rs {
    const RESET_VALUE: u32 = 0;
}
