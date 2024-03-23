#[doc = "Register `APB4FZ1` reader"]
pub type R = crate::R<APB4FZ1rs>;
#[doc = "Register `APB4FZ1` writer"]
pub type W = crate::W<APB4FZ1rs>;
#[doc = "Field `I2C4` reader - I2C4 SMBUS timeout stop in debug"]
pub type I2C4_R = crate::BitReader;
#[doc = "Field `I2C4` writer - I2C4 SMBUS timeout stop in debug"]
pub type I2C4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2` reader - LPTIM2 stop in debug"]
pub type LPTIM2_R = crate::BitReader;
#[doc = "Field `LPTIM2` writer - LPTIM2 stop in debug"]
pub type LPTIM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3` reader - LPTIM3 stop in debug"]
pub type LPTIM3_R = crate::BitReader;
#[doc = "Field `LPTIM3` writer - LPTIM3 stop in debug"]
pub type LPTIM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - RTC stop in debug"]
pub type RTC_R = crate::BitReader;
#[doc = "Field `RTC` writer - RTC stop in debug"]
pub type RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGLSCD` reader - LS watchdog for CPU domain stop in debug"]
pub type WDGLSCD_R = crate::BitReader;
#[doc = "Field `WDGLSCD` writer - LS watchdog for CPU domain stop in debug"]
pub type WDGLSCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c4(&self) -> I2C4_R {
        I2C4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 stop in debug"]
    #[inline(always)]
    pub fn lptim3(&self) -> LPTIM3_R {
        LPTIM3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC stop in debug"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - LS watchdog for CPU domain stop in debug"]
    #[inline(always)]
    pub fn wdglscd(&self) -> WDGLSCD_R {
        WDGLSCD_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn i2c4(&mut self) -> I2C4_W<APB4FZ1rs> {
        I2C4_W::new(self, 7)
    }
    #[doc = "Bit 9 - LPTIM2 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2(&mut self) -> LPTIM2_W<APB4FZ1rs> {
        LPTIM2_W::new(self, 9)
    }
    #[doc = "Bit 10 - LPTIM3 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3(&mut self) -> LPTIM3_W<APB4FZ1rs> {
        LPTIM3_W::new(self, 10)
    }
    #[doc = "Bit 16 - RTC stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<APB4FZ1rs> {
        RTC_W::new(self, 16)
    }
    #[doc = "Bit 18 - LS watchdog for CPU domain stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn wdglscd(&mut self) -> WDGLSCD_W<APB4FZ1rs> {
        WDGLSCD_W::new(self, 18)
    }
}
#[doc = "DBGMCU APB4 peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb4fz1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb4fz1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB4FZ1rs;
impl crate::RegisterSpec for APB4FZ1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb4fz1::R`](R) reader structure"]
impl crate::Readable for APB4FZ1rs {}
#[doc = "`write(|w| ..)` method takes [`apb4fz1::W`](W) writer structure"]
impl crate::Writable for APB4FZ1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB4FZ1 to value 0"]
impl crate::Resettable for APB4FZ1rs {
    const RESET_VALUE: u32 = 0;
}
