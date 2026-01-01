///Register `APB7FZR` reader
pub type R = crate::R<APB7FZRrs>;
///Register `APB7FZR` writer
pub type W = crate::W<APB7FZRrs>;
///Field `DBG_I2C3_STOP` reader - I2C3 stop in CPU debug Access can be protected by GTZC_TZSC.I2C3SEC.
pub type DBG_I2C3_STOP_R = crate::BitReader;
///Field `DBG_I2C3_STOP` writer - I2C3 stop in CPU debug Access can be protected by GTZC_TZSC.I2C3SEC.
pub type DBG_I2C3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM1_STOP` reader - LPTIM1 stop in CPU debug Access can be protected by GTZC_TZSC.LPTIM1SEC.
pub type DBG_LPTIM1_STOP_R = crate::BitReader;
///Field `DBG_LPTIM1_STOP` writer - LPTIM1 stop in CPU debug Access can be protected by GTZC_TZSC.LPTIM1SEC.
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_RTC_STOP` reader - RTC stop in CPU debug Access can be protected by GTZC_TZSC.TIM17SEC. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure.
pub type DBG_RTC_STOP_R = crate::BitReader;
///Field `DBG_RTC_STOP` writer - RTC stop in CPU debug Access can be protected by GTZC_TZSC.TIM17SEC. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure.
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 10 - I2C3 stop in CPU debug Access can be protected by GTZC_TZSC.I2C3SEC.
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 17 - LPTIM1 stop in CPU debug Access can be protected by GTZC_TZSC.LPTIM1SEC.
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 30 - RTC stop in CPU debug Access can be protected by GTZC_TZSC.TIM17SEC. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure.
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB7FZR")
            .field("dbg_i2c3_stop", &self.dbg_i2c3_stop())
            .field("dbg_lptim1_stop", &self.dbg_lptim1_stop())
            .field("dbg_rtc_stop", &self.dbg_rtc_stop())
            .finish()
    }
}
impl W {
    ///Bit 10 - I2C3 stop in CPU debug Access can be protected by GTZC_TZSC.I2C3SEC.
    #[inline(always)]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<'_, APB7FZRrs> {
        DBG_I2C3_STOP_W::new(self, 10)
    }
    ///Bit 17 - LPTIM1 stop in CPU debug Access can be protected by GTZC_TZSC.LPTIM1SEC.
    #[inline(always)]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<'_, APB7FZRrs> {
        DBG_LPTIM1_STOP_W::new(self, 17)
    }
    ///Bit 30 - RTC stop in CPU debug Access can be protected by GTZC_TZSC.TIM17SEC. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure.
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<'_, APB7FZRrs> {
        DBG_RTC_STOP_W::new(self, 30)
    }
}
/**DBGMCU APB7 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb7fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb7fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#DBGMCU:APB7FZR)*/
pub struct APB7FZRrs;
impl crate::RegisterSpec for APB7FZRrs {
    type Ux = u32;
}
///`read()` method returns [`apb7fzr::R`](R) reader structure
impl crate::Readable for APB7FZRrs {}
///`write(|w| ..)` method takes [`apb7fzr::W`](W) writer structure
impl crate::Writable for APB7FZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB7FZR to value 0
impl crate::Resettable for APB7FZRrs {}
