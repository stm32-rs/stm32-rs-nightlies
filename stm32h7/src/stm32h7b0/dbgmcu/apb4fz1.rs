///Register `APB4FZ1` reader
pub type R = crate::R<APB4FZ1rs>;
///Register `APB4FZ1` writer
pub type W = crate::W<APB4FZ1rs>;
///Field `I2C4` reader - I2C4 SMBUS timeout stop in debug
pub type I2C4_R = crate::BitReader;
///Field `I2C4` writer - I2C4 SMBUS timeout stop in debug
pub type I2C4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM2` reader - LPTIM2 stop in debug
pub type LPTIM2_R = crate::BitReader;
///Field `LPTIM2` writer - LPTIM2 stop in debug
pub type LPTIM2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM3` reader - LPTIM3 stop in debug
pub type LPTIM3_R = crate::BitReader;
///Field `LPTIM3` writer - LPTIM3 stop in debug
pub type LPTIM3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTC` reader - RTC stop in debug
pub type RTC_R = crate::BitReader;
///Field `RTC` writer - RTC stop in debug
pub type RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGLSCD` reader - LS watchdog for CPU domain stop in debug
pub type WDGLSCD_R = crate::BitReader;
///Field `WDGLSCD` writer - LS watchdog for CPU domain stop in debug
pub type WDGLSCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 7 - I2C4 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn i2c4(&self) -> I2C4_R {
        I2C4_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 stop in debug
    #[inline(always)]
    pub fn lptim3(&self) -> LPTIM3_R {
        LPTIM3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - RTC stop in debug
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - LS watchdog for CPU domain stop in debug
    #[inline(always)]
    pub fn wdglscd(&self) -> WDGLSCD_R {
        WDGLSCD_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4FZ1")
            .field("i2c4", &self.i2c4())
            .field("lptim2", &self.lptim2())
            .field("lptim3", &self.lptim3())
            .field("rtc", &self.rtc())
            .field("wdglscd", &self.wdglscd())
            .finish()
    }
}
impl W {
    ///Bit 7 - I2C4 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn i2c4(&mut self) -> I2C4_W<'_, APB4FZ1rs> {
        I2C4_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 stop in debug
    #[inline(always)]
    pub fn lptim2(&mut self) -> LPTIM2_W<'_, APB4FZ1rs> {
        LPTIM2_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 stop in debug
    #[inline(always)]
    pub fn lptim3(&mut self) -> LPTIM3_W<'_, APB4FZ1rs> {
        LPTIM3_W::new(self, 10)
    }
    ///Bit 16 - RTC stop in debug
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<'_, APB4FZ1rs> {
        RTC_W::new(self, 16)
    }
    ///Bit 18 - LS watchdog for CPU domain stop in debug
    #[inline(always)]
    pub fn wdglscd(&mut self) -> WDGLSCD_W<'_, APB4FZ1rs> {
        WDGLSCD_W::new(self, 18)
    }
}
/**DBGMCU APB4 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb4fz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#DBGMCU:APB4FZ1)*/
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
