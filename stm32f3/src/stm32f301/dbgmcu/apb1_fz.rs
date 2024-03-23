#[doc = "Register `APB1_FZ` reader"]
pub type R = crate::R<APB1_FZrs>;
#[doc = "Register `APB1_FZ` writer"]
pub type W = crate::W<APB1_FZrs>;
#[doc = "Field `DBG_TIM2_STOP` reader - Debug Timer 2 stopped when Core is halted"]
pub type DBG_TIM2_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM2_STOP` writer - Debug Timer 2 stopped when Core is halted"]
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3_STOP` reader - Debug Timer 3 stopped when Core is halted"]
pub type DBG_TIM3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM3_STOP` writer - Debug Timer 3 stopped when Core is halted"]
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM4_STOP` reader - Debug Timer 4 stopped when Core is halted"]
pub type DBG_TIM4_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM4_STOP` writer - Debug Timer 4 stopped when Core is halted"]
pub type DBG_TIM4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM5_STOP` reader - Debug Timer 5 stopped when Core is halted"]
pub type DBG_TIM5_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM5_STOP` writer - Debug Timer 5 stopped when Core is halted"]
pub type DBG_TIM5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM6_STOP` reader - Debug Timer 6 stopped when Core is halted"]
pub type DBG_TIM6_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM6_STOP` writer - Debug Timer 6 stopped when Core is halted"]
pub type DBG_TIM6_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM7_STOP` reader - Debug Timer 7 stopped when Core is halted"]
pub type DBG_TIM7_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM7_STOP` writer - Debug Timer 7 stopped when Core is halted"]
pub type DBG_TIM7_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM12_STOP` reader - Debug Timer 12 stopped when Core is halted"]
pub type DBG_TIM12_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM12_STOP` writer - Debug Timer 12 stopped when Core is halted"]
pub type DBG_TIM12_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM13_STOP` reader - Debug Timer 13 stopped when Core is halted"]
pub type DBG_TIM13_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM13_STOP` writer - Debug Timer 13 stopped when Core is halted"]
pub type DBG_TIM13_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIMER14_STOP` reader - Debug Timer 14 stopped when Core is halted"]
pub type DBG_TIMER14_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIMER14_STOP` writer - Debug Timer 14 stopped when Core is halted"]
pub type DBG_TIMER14_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM18_STOP` reader - Debug Timer 18 stopped when Core is halted"]
pub type DBG_TIM18_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM18_STOP` writer - Debug Timer 18 stopped when Core is halted"]
pub type DBG_TIM18_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_R = crate::BitReader;
#[doc = "Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted"]
pub type DBG_WWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted"]
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted"]
pub type DBG_IWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted"]
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` reader - SMBUS timeout mode stopped when Core is halted"]
pub type I2C1_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` writer - SMBUS timeout mode stopped when Core is halted"]
pub type I2C1_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` reader - SMBUS timeout mode stopped when Core is halted"]
pub type I2C2_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` writer - SMBUS timeout mode stopped when Core is halted"]
pub type I2C2_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_CAN_STOP` reader - Debug CAN stopped when core is halted"]
pub type DBG_CAN_STOP_R = crate::BitReader;
#[doc = "Field `DBG_CAN_STOP` writer - Debug CAN stopped when core is halted"]
pub type DBG_CAN_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Timer 3 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Timer 4 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Timer 5 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Debug Timer 7 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug Timer 12 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim12_stop(&self) -> DBG_TIM12_STOP_R {
        DBG_TIM12_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debug Timer 13 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim13_stop(&self) -> DBG_TIM13_STOP_R {
        DBG_TIM13_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debug Timer 14 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer14_stop(&self) -> DBG_TIMER14_STOP_R {
        DBG_TIMER14_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Debug Timer 18 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_tim18_stop(&self) -> DBG_TIM18_STOP_R {
        DBG_TIM18_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    pub fn i2c1_smbus_timeout(&self) -> I2C1_SMBUS_TIMEOUT_R {
        I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    pub fn i2c2_smbus_timeout(&self) -> I2C2_SMBUS_TIMEOUT_R {
        I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - Debug CAN stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_can_stop(&self) -> DBG_CAN_STOP_R {
        DBG_CAN_STOP_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<APB1_FZrs> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Timer 3 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<APB1_FZrs> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Debug Timer 4 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<APB1_FZrs> {
        DBG_TIM4_STOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Debug Timer 5 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<APB1_FZrs> {
        DBG_TIM5_STOP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Debug Timer 6 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<APB1_FZrs> {
        DBG_TIM6_STOP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Debug Timer 7 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<APB1_FZrs> {
        DBG_TIM7_STOP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Debug Timer 12 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim12_stop(&mut self) -> DBG_TIM12_STOP_W<APB1_FZrs> {
        DBG_TIM12_STOP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Debug Timer 13 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim13_stop(&mut self) -> DBG_TIM13_STOP_W<APB1_FZrs> {
        DBG_TIM13_STOP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Debug Timer 14 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer14_stop(&mut self) -> DBG_TIMER14_STOP_W<APB1_FZrs> {
        DBG_TIMER14_STOP_W::new(self, 8)
    }
    #[doc = "Bit 9 - Debug Timer 18 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim18_stop(&mut self) -> DBG_TIM18_STOP_W<APB1_FZrs> {
        DBG_TIM18_STOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Debug RTC stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<APB1_FZrs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<APB1_FZrs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<APB1_FZrs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    #[doc = "Bit 21 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_smbus_timeout(&mut self) -> I2C1_SMBUS_TIMEOUT_W<APB1_FZrs> {
        I2C1_SMBUS_TIMEOUT_W::new(self, 21)
    }
    #[doc = "Bit 22 - SMBUS timeout mode stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_smbus_timeout(&mut self) -> I2C2_SMBUS_TIMEOUT_W<APB1_FZrs> {
        I2C2_SMBUS_TIMEOUT_W::new(self, 22)
    }
    #[doc = "Bit 25 - Debug CAN stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_can_stop(&mut self) -> DBG_CAN_STOP_W<APB1_FZrs> {
        DBG_CAN_STOP_W::new(self, 25)
    }
}
#[doc = "APB Low Freeze Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1_fz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1_fz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1_FZrs;
impl crate::RegisterSpec for APB1_FZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1_fz::R`](R) reader structure"]
impl crate::Readable for APB1_FZrs {}
#[doc = "`write(|w| ..)` method takes [`apb1_fz::W`](W) writer structure"]
impl crate::Writable for APB1_FZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1_FZ to value 0"]
impl crate::Resettable for APB1_FZrs {
    const RESET_VALUE: u32 = 0;
}
