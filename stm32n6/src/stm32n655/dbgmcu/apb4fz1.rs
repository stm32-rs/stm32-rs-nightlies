///Register `APB4FZ1` reader
pub type R = crate::R<APB4FZ1rs>;
///Register `APB4FZ1` writer
pub type W = crate::W<APB4FZ1rs>;
///Field `DBG_I2C4_STOP` reader - I2C4 stop in debug
pub type DBG_I2C4_STOP_R = crate::BitReader;
///Field `DBG_I2C4_STOP` writer - I2C4 stop in debug
pub type DBG_I2C4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM2_STOP` reader - LPTIM2 stop in debug
pub type DBG_LPTIM2_STOP_R = crate::BitReader;
///Field `DBG_LPTIM2_STOP` writer - LPTIM2 stop in debug
pub type DBG_LPTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM3_STOP` reader - LPTIM3 stop in debug
pub type DBG_LPTIM3_STOP_R = crate::BitReader;
///Field `DBG_LPTIM3_STOP` writer - LPTIM3 stop in debug
pub type DBG_LPTIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM4_STOP` reader - LPTIM4 stop in debug
pub type DBG_LPTIM4_STOP_R = crate::BitReader;
///Field `DBG_LPTIM4_STOP` writer - LPTIM4 stop in debug
pub type DBG_LPTIM4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM5_STOP` reader - LPTIM5 stop in debug
pub type DBG_LPTIM5_STOP_R = crate::BitReader;
///Field `DBG_LPTIM5_STOP` writer - LPTIM5 stop in debug
pub type DBG_LPTIM5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_RTC_STOP` reader - RTC clock is suspended in debug
pub type DBG_RTC_STOP_R = crate::BitReader;
///Field `DBG_RTC_STOP` writer - RTC clock is suspended in debug
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_IWDG_STOP` reader - WWDG stop in debug
pub type DBG_IWDG_STOP_R = crate::BitReader;
///Field `DBG_IWDG_STOP` writer - WWDG stop in debug
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - I2C4 stop in debug
    #[inline(always)]
    pub fn dbg_i2c4_stop(&self) -> DBG_I2C4_STOP_R {
        DBG_I2C4_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 stop in debug
    #[inline(always)]
    pub fn dbg_lptim3_stop(&self) -> DBG_LPTIM3_STOP_R {
        DBG_LPTIM3_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 stop in debug
    #[inline(always)]
    pub fn dbg_lptim4_stop(&self) -> DBG_LPTIM4_STOP_R {
        DBG_LPTIM4_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 stop in debug
    #[inline(always)]
    pub fn dbg_lptim5_stop(&self) -> DBG_LPTIM5_STOP_R {
        DBG_LPTIM5_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - RTC clock is suspended in debug
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - WWDG stop in debug
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4FZ1")
            .field("dbg_i2c4_stop", &self.dbg_i2c4_stop())
            .field("dbg_lptim2_stop", &self.dbg_lptim2_stop())
            .field("dbg_lptim3_stop", &self.dbg_lptim3_stop())
            .field("dbg_lptim4_stop", &self.dbg_lptim4_stop())
            .field("dbg_lptim5_stop", &self.dbg_lptim5_stop())
            .field("dbg_rtc_stop", &self.dbg_rtc_stop())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .finish()
    }
}
impl W {
    ///Bit 8 - I2C4 stop in debug
    #[inline(always)]
    pub fn dbg_i2c4_stop(&mut self) -> DBG_I2C4_STOP_W<'_, APB4FZ1rs> {
        DBG_I2C4_STOP_W::new(self, 8)
    }
    ///Bit 9 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<'_, APB4FZ1rs> {
        DBG_LPTIM2_STOP_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 stop in debug
    #[inline(always)]
    pub fn dbg_lptim3_stop(&mut self) -> DBG_LPTIM3_STOP_W<'_, APB4FZ1rs> {
        DBG_LPTIM3_STOP_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 stop in debug
    #[inline(always)]
    pub fn dbg_lptim4_stop(&mut self) -> DBG_LPTIM4_STOP_W<'_, APB4FZ1rs> {
        DBG_LPTIM4_STOP_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 stop in debug
    #[inline(always)]
    pub fn dbg_lptim5_stop(&mut self) -> DBG_LPTIM5_STOP_W<'_, APB4FZ1rs> {
        DBG_LPTIM5_STOP_W::new(self, 12)
    }
    ///Bit 16 - RTC clock is suspended in debug
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<'_, APB4FZ1rs> {
        DBG_RTC_STOP_W::new(self, 16)
    }
    ///Bit 18 - WWDG stop in debug
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<'_, APB4FZ1rs> {
        DBG_IWDG_STOP_W::new(self, 18)
    }
}
/**DBGMCU APB4 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb4fz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DBGMCU:APB4FZ1)*/
pub struct APB4FZ1rs;
impl crate::RegisterSpec for APB4FZ1rs {
    type Ux = u32;
}
///`read()` method returns [`apb4fz1::R`](R) reader structure
impl crate::Readable for APB4FZ1rs {}
///`write(|w| ..)` method takes [`apb4fz1::W`](W) writer structure
impl crate::Writable for APB4FZ1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4FZ1 to value 0
impl crate::Resettable for APB4FZ1rs {}
