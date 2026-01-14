///Register `APB4FZ1` reader
pub type R = crate::R<APB4FZ1rs>;
///Register `APB4FZ1` writer
pub type W = crate::W<APB4FZ1rs>;
///Field `DBG_I2C4` reader - I2C4 SMBUS timeout stop in debug
pub type DBG_I2C4_R = crate::BitReader;
///Field `DBG_I2C4` writer - I2C4 SMBUS timeout stop in debug
pub type DBG_I2C4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM2` reader - LPTIM2 stop in debug
pub type DBG_LPTIM2_R = crate::BitReader;
///Field `DBG_LPTIM2` writer - LPTIM2 stop in debug
pub type DBG_LPTIM2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM3` reader - LPTIM2 stop in debug
pub type DBG_LPTIM3_R = crate::BitReader;
///Field `DBG_LPTIM3` writer - LPTIM2 stop in debug
pub type DBG_LPTIM3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM4` reader - LPTIM4 stop in debug
pub type DBG_LPTIM4_R = crate::BitReader;
///Field `DBG_LPTIM4` writer - LPTIM4 stop in debug
pub type DBG_LPTIM4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM5` reader - LPTIM5 stop in debug
pub type DBG_LPTIM5_R = crate::BitReader;
///Field `DBG_LPTIM5` writer - LPTIM5 stop in debug
pub type DBG_LPTIM5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_RTC` reader - RTC stop in debug
pub type DBG_RTC_R = crate::BitReader;
///Field `DBG_RTC` writer - RTC stop in debug
pub type DBG_RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_WDGLSD1` reader - Independent watchdog for D1 stop in debug
pub type DBG_WDGLSD1_R = crate::BitReader;
///Field `DBG_WDGLSD1` writer - Independent watchdog for D1 stop in debug
pub type DBG_WDGLSD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_WDGLSD2` reader - Independent watchdog for D2 stop in debug
pub type DBG_WDGLSD2_R = crate::BitReader;
///Field `DBG_WDGLSD2` writer - Independent watchdog for D2 stop in debug
pub type DBG_WDGLSD2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 7 - I2C4 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c4(&self) -> DBG_I2C4_R {
        DBG_I2C4_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn dbg_lptim2(&self) -> DBG_LPTIM2_R {
        DBG_LPTIM2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn dbg_lptim3(&self) -> DBG_LPTIM3_R {
        DBG_LPTIM3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 stop in debug
    #[inline(always)]
    pub fn dbg_lptim4(&self) -> DBG_LPTIM4_R {
        DBG_LPTIM4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 stop in debug
    #[inline(always)]
    pub fn dbg_lptim5(&self) -> DBG_LPTIM5_R {
        DBG_LPTIM5_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - RTC stop in debug
    #[inline(always)]
    pub fn dbg_rtc(&self) -> DBG_RTC_R {
        DBG_RTC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Independent watchdog for D1 stop in debug
    #[inline(always)]
    pub fn dbg_wdglsd1(&self) -> DBG_WDGLSD1_R {
        DBG_WDGLSD1_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Independent watchdog for D2 stop in debug
    #[inline(always)]
    pub fn dbg_wdglsd2(&self) -> DBG_WDGLSD2_R {
        DBG_WDGLSD2_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4FZ1")
            .field("dbg_i2c4", &self.dbg_i2c4())
            .field("dbg_lptim2", &self.dbg_lptim2())
            .field("dbg_lptim3", &self.dbg_lptim3())
            .field("dbg_lptim4", &self.dbg_lptim4())
            .field("dbg_lptim5", &self.dbg_lptim5())
            .field("dbg_rtc", &self.dbg_rtc())
            .field("dbg_wdglsd1", &self.dbg_wdglsd1())
            .field("dbg_wdglsd2", &self.dbg_wdglsd2())
            .finish()
    }
}
impl W {
    ///Bit 7 - I2C4 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c4(&mut self) -> DBG_I2C4_W<'_, APB4FZ1rs> {
        DBG_I2C4_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn dbg_lptim2(&mut self) -> DBG_LPTIM2_W<'_, APB4FZ1rs> {
        DBG_LPTIM2_W::new(self, 9)
    }
    ///Bit 10 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn dbg_lptim3(&mut self) -> DBG_LPTIM3_W<'_, APB4FZ1rs> {
        DBG_LPTIM3_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 stop in debug
    #[inline(always)]
    pub fn dbg_lptim4(&mut self) -> DBG_LPTIM4_W<'_, APB4FZ1rs> {
        DBG_LPTIM4_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 stop in debug
    #[inline(always)]
    pub fn dbg_lptim5(&mut self) -> DBG_LPTIM5_W<'_, APB4FZ1rs> {
        DBG_LPTIM5_W::new(self, 12)
    }
    ///Bit 16 - RTC stop in debug
    #[inline(always)]
    pub fn dbg_rtc(&mut self) -> DBG_RTC_W<'_, APB4FZ1rs> {
        DBG_RTC_W::new(self, 16)
    }
    ///Bit 18 - Independent watchdog for D1 stop in debug
    #[inline(always)]
    pub fn dbg_wdglsd1(&mut self) -> DBG_WDGLSD1_W<'_, APB4FZ1rs> {
        DBG_WDGLSD1_W::new(self, 18)
    }
    ///Bit 19 - Independent watchdog for D2 stop in debug
    #[inline(always)]
    pub fn dbg_wdglsd2(&mut self) -> DBG_WDGLSD2_W<'_, APB4FZ1rs> {
        DBG_WDGLSD2_W::new(self, 19)
    }
}
/**DBGMCU APB4 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb4fz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM4.html#DBGMCU:APB4FZ1)*/
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
