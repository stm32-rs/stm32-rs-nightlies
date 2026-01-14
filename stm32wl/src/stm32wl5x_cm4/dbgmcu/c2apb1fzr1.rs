///Register `C2APB1FZR1` reader
pub type R = crate::R<C2APB1FZR1rs>;
///Register `C2APB1FZR1` writer
pub type W = crate::W<C2APB1FZR1rs>;
///Field `DBG_TIM2_STOP` reader - DBG_TIM2_STOP
pub type DBG_TIM2_STOP_R = crate::BitReader;
///Field `DBG_TIM2_STOP` writer - DBG_TIM2_STOP
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_RTC_STOP` reader - DBG_RTC_STOP
pub type DBG_RTC_STOP_R = crate::BitReader;
///Field `DBG_RTC_STOP` writer - DBG_RTC_STOP
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_IWDG_STOP` reader - DBG_IWDG_STOP
pub type DBG_IWDG_STOP_R = crate::BitReader;
///Field `DBG_IWDG_STOP` writer - DBG_IWDG_STOP
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C1_STOP` reader - DBG_I2C1_STOP
pub type DBG_I2C1_STOP_R = crate::BitReader;
///Field `DBG_I2C1_STOP` writer - DBG_I2C1_STOP
pub type DBG_I2C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C2_STOP` reader - DBG_I2C2_STOP
pub type DBG_I2C2_STOP_R = crate::BitReader;
///Field `DBG_I2C2_STOP` writer - DBG_I2C2_STOP
pub type DBG_I2C2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C3_STOP` reader - DBG_I2C3_STOP
pub type DBG_I2C3_STOP_R = crate::BitReader;
///Field `DBG_I2C3_STOP` writer - DBG_I2C3_STOP
pub type DBG_I2C3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM1_STOP` reader - DBG_LPTIM1_STOP
pub type DBG_LPTIM1_STOP_R = crate::BitReader;
///Field `DBG_LPTIM1_STOP` writer - DBG_LPTIM1_STOP
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DBG_TIM2_STOP
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 10 - DBG_RTC_STOP
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - DBG_IWDG_STOP
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - DBG_I2C1_STOP
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - DBG_I2C2_STOP
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - DBG_I2C3_STOP
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - DBG_LPTIM1_STOP
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB1FZR1")
            .field("dbg_tim2_stop", &self.dbg_tim2_stop())
            .field("dbg_rtc_stop", &self.dbg_rtc_stop())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .field("dbg_i2c1_stop", &self.dbg_i2c1_stop())
            .field("dbg_i2c2_stop", &self.dbg_i2c2_stop())
            .field("dbg_i2c3_stop", &self.dbg_i2c3_stop())
            .field("dbg_lptim1_stop", &self.dbg_lptim1_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - DBG_TIM2_STOP
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<'_, C2APB1FZR1rs> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    ///Bit 10 - DBG_RTC_STOP
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<'_, C2APB1FZR1rs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    ///Bit 12 - DBG_IWDG_STOP
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<'_, C2APB1FZR1rs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    ///Bit 21 - DBG_I2C1_STOP
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<'_, C2APB1FZR1rs> {
        DBG_I2C1_STOP_W::new(self, 21)
    }
    ///Bit 22 - DBG_I2C2_STOP
    #[inline(always)]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W<'_, C2APB1FZR1rs> {
        DBG_I2C2_STOP_W::new(self, 22)
    }
    ///Bit 23 - DBG_I2C3_STOP
    #[inline(always)]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<'_, C2APB1FZR1rs> {
        DBG_I2C3_STOP_W::new(self, 23)
    }
    ///Bit 31 - DBG_LPTIM1_STOP
    #[inline(always)]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<'_, C2APB1FZR1rs> {
        DBG_LPTIM1_STOP_W::new(self, 31)
    }
}
/**DBGMCU CPU2 APB1 Peripheral Freeze Register 1 \[dual core device

You can [`read`](crate::Reg::read) this register and get [`c2apb1fzr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb1fzr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DBGMCU:C2APB1FZR1)*/
pub struct C2APB1FZR1rs;
impl crate::RegisterSpec for C2APB1FZR1rs {
    type Ux = u32;
}
///`read()` method returns [`c2apb1fzr1::R`](R) reader structure
impl crate::Readable for C2APB1FZR1rs {}
///`write(|w| ..)` method takes [`c2apb1fzr1::W`](W) writer structure
impl crate::Writable for C2APB1FZR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2APB1FZR1 to value 0
impl crate::Resettable for C2APB1FZR1rs {}
