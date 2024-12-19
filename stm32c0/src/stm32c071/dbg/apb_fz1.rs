///Register `APB_FZ1` reader
pub type R = crate::R<APB_FZ1rs>;
///Register `APB_FZ1` writer
pub type W = crate::W<APB_FZ1rs>;
///Field `DBG_TIM2_STOP` reader - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted: This bit is only available on STM32C071xx. On the other devices, it is reserved.
pub type DBG_TIM2_STOP_R = crate::BitReader;
///Field `DBG_TIM2_STOP` writer - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted: This bit is only available on STM32C071xx. On the other devices, it is reserved.
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM3_STOP` reader - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
pub type DBG_TIM3_STOP_R = crate::BitReader;
///Field `DBG_TIM3_STOP` writer - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_RTC_STOP` reader - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
pub type DBG_RTC_STOP_R = crate::BitReader;
///Field `DBG_RTC_STOP` writer - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_WWDG_STOP` reader - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
pub type DBG_WWDG_STOP_R = crate::BitReader;
///Field `DBG_WWDG_STOP` writer - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_IWDG_STOP` reader - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
pub type DBG_IWDG_STOP_R = crate::BitReader;
///Field `DBG_IWDG_STOP` writer - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C1_SMBUS_TIMEOUT` reader - SMBUS timeout when core is halted
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::BitReader;
///Field `DBG_I2C1_SMBUS_TIMEOUT` writer - SMBUS timeout when core is halted
pub type DBG_I2C1_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted: This bit is only available on STM32C071xx. On the other devices, it is reserved.
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - SMBUS timeout when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_FZ1")
            .field("dbg_tim2_stop", &self.dbg_tim2_stop())
            .field("dbg_tim3_stop", &self.dbg_tim3_stop())
            .field("dbg_rtc_stop", &self.dbg_rtc_stop())
            .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .field("dbg_i2c1_smbus_timeout", &self.dbg_i2c1_smbus_timeout())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clocking of TIM2 counter when the core is halted This bit enables/disables the clock to the counter of TIM2 when the core is halted: This bit is only available on STM32C071xx. On the other devices, it is reserved.
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<APB_FZ1rs> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    ///Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<APB_FZ1rs> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    ///Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<APB_FZ1rs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    ///Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<APB_FZ1rs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    ///Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<APB_FZ1rs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    ///Bit 21 - SMBUS timeout when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W<APB_FZ1rs> {
        DBG_I2C1_SMBUS_TIMEOUT_W::new(self, 21)
    }
}
/**DBG APB freeze register 1

You can [`read`](crate::Reg::read) this register and get [`apb_fz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_fz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#DBG:APB_FZ1)*/
pub struct APB_FZ1rs;
impl crate::RegisterSpec for APB_FZ1rs {
    type Ux = u32;
}
///`read()` method returns [`apb_fz1::R`](R) reader structure
impl crate::Readable for APB_FZ1rs {}
///`write(|w| ..)` method takes [`apb_fz1::W`](W) writer structure
impl crate::Writable for APB_FZ1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APB_FZ1 to value 0
impl crate::Resettable for APB_FZ1rs {
    const RESET_VALUE: u32 = 0;
}
