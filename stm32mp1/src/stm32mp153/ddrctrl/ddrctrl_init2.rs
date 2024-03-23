#[doc = "Register `DDRCTRL_INIT2` reader"]
pub type R = crate::R<DDRCTRL_INIT2rs>;
#[doc = "Register `DDRCTRL_INIT2` writer"]
pub type W = crate::W<DDRCTRL_INIT2rs>;
#[doc = "Field `MIN_STABLE_CLOCK_X1` reader - MIN_STABLE_CLOCK_X1"]
pub type MIN_STABLE_CLOCK_X1_R = crate::FieldReader;
#[doc = "Field `MIN_STABLE_CLOCK_X1` writer - MIN_STABLE_CLOCK_X1"]
pub type MIN_STABLE_CLOCK_X1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IDLE_AFTER_RESET_X32` reader - IDLE_AFTER_RESET_X32"]
pub type IDLE_AFTER_RESET_X32_R = crate::FieldReader;
#[doc = "Field `IDLE_AFTER_RESET_X32` writer - IDLE_AFTER_RESET_X32"]
pub type IDLE_AFTER_RESET_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - MIN_STABLE_CLOCK_X1"]
    #[inline(always)]
    pub fn min_stable_clock_x1(&self) -> MIN_STABLE_CLOCK_X1_R {
        MIN_STABLE_CLOCK_X1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - IDLE_AFTER_RESET_X32"]
    #[inline(always)]
    pub fn idle_after_reset_x32(&self) -> IDLE_AFTER_RESET_X32_R {
        IDLE_AFTER_RESET_X32_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - MIN_STABLE_CLOCK_X1"]
    #[inline(always)]
    #[must_use]
    pub fn min_stable_clock_x1(&mut self) -> MIN_STABLE_CLOCK_X1_W<DDRCTRL_INIT2rs> {
        MIN_STABLE_CLOCK_X1_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - IDLE_AFTER_RESET_X32"]
    #[inline(always)]
    #[must_use]
    pub fn idle_after_reset_x32(&mut self) -> IDLE_AFTER_RESET_X32_W<DDRCTRL_INIT2rs> {
        IDLE_AFTER_RESET_X32_W::new(self, 8)
    }
}
#[doc = "DDRCTRL SDRAM initialization register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_init2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_init2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_INIT2rs;
impl crate::RegisterSpec for DDRCTRL_INIT2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_init2::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT2rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_init2::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_INIT2 to value 0x0d05"]
impl crate::Resettable for DDRCTRL_INIT2rs {
    const RESET_VALUE: u32 = 0x0d05;
}
