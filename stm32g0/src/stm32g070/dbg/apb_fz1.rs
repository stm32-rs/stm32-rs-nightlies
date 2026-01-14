///Register `APB_FZ1` reader
pub type R = crate::R<APB_FZ1rs>;
///Register `APB_FZ1` writer
pub type W = crate::W<APB_FZ1rs>;
///Field `DBG_TIMER2_STOP` reader - Debug Timer 2 stopped when Core is halted
pub type DBG_TIMER2_STOP_R = crate::BitReader;
///Field `DBG_TIMER2_STOP` writer - Debug Timer 2 stopped when Core is halted
pub type DBG_TIMER2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM3_STOP` reader - TIM3 counter stopped when core is halted
pub type DBG_TIM3_STOP_R = crate::BitReader;
///Field `DBG_TIM3_STOP` writer - TIM3 counter stopped when core is halted
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIMER6_STOP` reader - Debug Timer 6 stopped when Core is halted
pub type DBG_TIMER6_STOP_R = crate::BitReader;
///Field `DBG_TIMER6_STOP` writer - Debug Timer 6 stopped when Core is halted
pub type DBG_TIMER6_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM7_STOP` reader - TIM7 counter stopped when core is halted
pub type DBG_TIM7_STOP_R = crate::BitReader;
///Field `DBG_TIM7_STOP` writer - TIM7 counter stopped when core is halted
pub type DBG_TIM7_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_RTC_STOP` reader - Debug RTC stopped when Core is halted
pub type DBG_RTC_STOP_R = crate::BitReader;
///Field `DBG_RTC_STOP` writer - Debug RTC stopped when Core is halted
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_WWDG_STOP` reader - Debug Window Wachdog stopped when Core is halted
pub type DBG_WWDG_STOP_R = crate::BitReader;
///Field `DBG_WWDG_STOP` writer - Debug Window Wachdog stopped when Core is halted
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_IWDG_STOP` reader - Debug Independent Wachdog stopped when Core is halted
pub type DBG_IWDG_STOP_R = crate::BitReader;
///Field `DBG_IWDG_STOP` writer - Debug Independent Wachdog stopped when Core is halted
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout mode stopped when core is halted
pub type DBG_I2C1_STOP_R = crate::BitReader;
///Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout mode stopped when core is halted
pub type DBG_I2C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM2_STOP` reader - Clocking of LPTIMER2 counter when the core is halted
pub type DBG_LPTIM2_STOP_R = crate::BitReader;
///Field `DBG_LPTIM2_STOP` writer - Clocking of LPTIMER2 counter when the core is halted
pub type DBG_LPTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM1_STOP` reader - Clocking of LPTIMER1 counter when the core is halted
pub type DBG_LPTIM1_STOP_R = crate::BitReader;
///Field `DBG_LPTIM1_STOP` writer - Clocking of LPTIMER1 counter when the core is halted
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Debug Timer 2 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer2_stop(&self) -> DBG_TIMER2_STOP_R {
        DBG_TIMER2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Debug Timer 6 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer6_stop(&self) -> DBG_TIMER6_STOP_R {
        DBG_TIMER6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 10 - Debug RTC stopped when Core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Debug Window Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Debug Independent Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 30 - Clocking of LPTIMER2 counter when the core is halted
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Clocking of LPTIMER1 counter when the core is halted
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_FZ1")
            .field("dbg_timer2_stop", &self.dbg_timer2_stop())
            .field("dbg_tim3_stop", &self.dbg_tim3_stop())
            .field("dbg_timer6_stop", &self.dbg_timer6_stop())
            .field("dbg_tim7_stop", &self.dbg_tim7_stop())
            .field("dbg_rtc_stop", &self.dbg_rtc_stop())
            .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .field("dbg_i2c1_stop", &self.dbg_i2c1_stop())
            .field("dbg_lptim2_stop", &self.dbg_lptim2_stop())
            .field("dbg_lptim1_stop", &self.dbg_lptim1_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - Debug Timer 2 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer2_stop(&mut self) -> DBG_TIMER2_STOP_W<'_, APB_FZ1rs> {
        DBG_TIMER2_STOP_W::new(self, 0)
    }
    ///Bit 1 - TIM3 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<'_, APB_FZ1rs> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    ///Bit 4 - Debug Timer 6 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_timer6_stop(&mut self) -> DBG_TIMER6_STOP_W<'_, APB_FZ1rs> {
        DBG_TIMER6_STOP_W::new(self, 4)
    }
    ///Bit 5 - TIM7 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<'_, APB_FZ1rs> {
        DBG_TIM7_STOP_W::new(self, 5)
    }
    ///Bit 10 - Debug RTC stopped when Core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<'_, APB_FZ1rs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    ///Bit 11 - Debug Window Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<'_, APB_FZ1rs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    ///Bit 12 - Debug Independent Wachdog stopped when Core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<'_, APB_FZ1rs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    ///Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<'_, APB_FZ1rs> {
        DBG_I2C1_STOP_W::new(self, 21)
    }
    ///Bit 30 - Clocking of LPTIMER2 counter when the core is halted
    #[inline(always)]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<'_, APB_FZ1rs> {
        DBG_LPTIM2_STOP_W::new(self, 30)
    }
    ///Bit 31 - Clocking of LPTIMER1 counter when the core is halted
    #[inline(always)]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<'_, APB_FZ1rs> {
        DBG_LPTIM1_STOP_W::new(self, 31)
    }
}
/**DBG APB freeze register 1

You can [`read`](crate::Reg::read) this register and get [`apb_fz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_fz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DBG:APB_FZ1)*/
pub struct APB_FZ1rs;
impl crate::RegisterSpec for APB_FZ1rs {
    type Ux = u32;
}
///`read()` method returns [`apb_fz1::R`](R) reader structure
impl crate::Readable for APB_FZ1rs {}
///`write(|w| ..)` method takes [`apb_fz1::W`](W) writer structure
impl crate::Writable for APB_FZ1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB_FZ1 to value 0
impl crate::Resettable for APB_FZ1rs {}
