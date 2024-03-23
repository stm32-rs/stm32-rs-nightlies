#[doc = "Register `APB1LFZR` reader"]
pub type R = crate::R<APB1LFZRrs>;
#[doc = "Register `APB1LFZR` writer"]
pub type W = crate::W<APB1LFZRrs>;
#[doc = "Field `DBG_TIM2_STOP` reader - TIM2 counter stopped when core is halted"]
pub type DBG_TIM2_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM2_STOP` writer - TIM2 counter stopped when core is halted"]
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3_STOP` reader - TIM3 stop in debug"]
pub type DBG_TIM3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM3_STOP` writer - TIM3 stop in debug"]
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM4_STOP` reader - TIM4 stop in debug"]
pub type DBG_TIM4_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM4_STOP` writer - TIM4 stop in debug"]
pub type DBG_TIM4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM5_STOP` reader - TIM5 stop in debug"]
pub type DBG_TIM5_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM5_STOP` writer - TIM5 stop in debug"]
pub type DBG_TIM5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM6_STOP` reader - TIM6 counter stopped when core is halted"]
pub type DBG_TIM6_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM6_STOP` writer - TIM6 counter stopped when core is halted"]
pub type DBG_TIM6_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM7_STOP` reader - TIM7 counter stopped when core is halted"]
pub type DBG_TIM7_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM7_STOP` writer - TIM7 counter stopped when core is halted"]
pub type DBG_TIM7_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_STOP` reader - RTC counter stopped when core is halted"]
pub type DBG_RTC_STOP_R = crate::BitReader;
#[doc = "Field `DBG_RTC_STOP` writer - RTC counter stopped when core is halted"]
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - Window watchdog counter stopped when core is halted"]
pub type DBG_WWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - Window watchdog counter stopped when core is halted"]
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` reader - Independent watchdog counter stopped when core is halted"]
pub type DBG_IWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - Independent watchdog counter stopped when core is halted"]
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout counter stopped when core is halted"]
pub type DBG_I2C1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout counter stopped when core is halted"]
pub type DBG_I2C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout counter stopped when core is halted"]
pub type DBG_I2C2_STOP_R = crate::BitReader;
#[doc = "Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout counter stopped when core is halted"]
pub type DBG_I2C2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout counter stopped when core is halted"]
pub type DBG_I2C3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout counter stopped when core is halted"]
pub type DBG_I2C3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM1_STOP` reader - LPTIM1 counter stopped when core is halted"]
pub type DBG_LPTIM1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM1_STOP` writer - LPTIM1 counter stopped when core is halted"]
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Independent watchdog counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<APB1LFZRrs> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<APB1LFZRrs> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<APB1LFZRrs> {
        DBG_TIM4_STOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<APB1LFZRrs> {
        DBG_TIM5_STOP_W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<APB1LFZRrs> {
        DBG_TIM6_STOP_W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM7 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<APB1LFZRrs> {
        DBG_TIM7_STOP_W::new(self, 5)
    }
    #[doc = "Bit 10 - RTC counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<APB1LFZRrs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Window watchdog counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<APB1LFZRrs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Independent watchdog counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<APB1LFZRrs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<APB1LFZRrs> {
        DBG_I2C1_STOP_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W<APB1LFZRrs> {
        DBG_I2C2_STOP_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<APB1LFZRrs> {
        DBG_I2C3_STOP_W::new(self, 23)
    }
    #[doc = "Bit 31 - LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<APB1LFZRrs> {
        DBG_LPTIM1_STOP_W::new(self, 31)
    }
}
#[doc = "Debug MCU APB1 freeze register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1lfzr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1lfzr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LFZRrs;
impl crate::RegisterSpec for APB1LFZRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lfzr::R`](R) reader structure"]
impl crate::Readable for APB1LFZRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1lfzr::W`](W) writer structure"]
impl crate::Writable for APB1LFZRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1LFZR to value 0"]
impl crate::Resettable for APB1LFZRrs {
    const RESET_VALUE: u32 = 0;
}
