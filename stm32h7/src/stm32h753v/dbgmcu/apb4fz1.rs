#[doc = "Register `APB4FZ1` reader"]
pub type R = crate::R<APB4FZ1rs>;
#[doc = "Register `APB4FZ1` writer"]
pub type W = crate::W<APB4FZ1rs>;
#[doc = "Field `DBG_I2C4` reader - I2C4 SMBUS timeout stop in debug"]
pub type DBG_I2C4_R = crate::BitReader;
#[doc = "Field `DBG_I2C4` writer - I2C4 SMBUS timeout stop in debug"]
pub type DBG_I2C4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM2` reader - LPTIM2 stop in debug"]
pub type DBG_LPTIM2_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM2` writer - LPTIM2 stop in debug"]
pub type DBG_LPTIM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM3` reader - LPTIM2 stop in debug"]
pub type DBG_LPTIM3_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM3` writer - LPTIM2 stop in debug"]
pub type DBG_LPTIM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM4` reader - LPTIM4 stop in debug"]
pub type DBG_LPTIM4_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM4` writer - LPTIM4 stop in debug"]
pub type DBG_LPTIM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM5` reader - LPTIM5 stop in debug"]
pub type DBG_LPTIM5_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM5` writer - LPTIM5 stop in debug"]
pub type DBG_LPTIM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC` reader - RTC stop in debug"]
pub type DBG_RTC_R = crate::BitReader;
#[doc = "Field `DBG_RTC` writer - RTC stop in debug"]
pub type DBG_RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WDGLSD1` reader - Independent watchdog for D1 stop in debug"]
pub type DBG_WDGLSD1_R = crate::BitReader;
#[doc = "Field `DBG_WDGLSD1` writer - Independent watchdog for D1 stop in debug"]
pub type DBG_WDGLSD1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c4(&self) -> DBG_I2C4_R {
        DBG_I2C4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim2(&self) -> DBG_LPTIM2_R {
        DBG_LPTIM2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim3(&self) -> DBG_LPTIM3_R {
        DBG_LPTIM3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim4(&self) -> DBG_LPTIM4_R {
        DBG_LPTIM4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim5(&self) -> DBG_LPTIM5_R {
        DBG_LPTIM5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC stop in debug"]
    #[inline(always)]
    pub fn dbg_rtc(&self) -> DBG_RTC_R {
        DBG_RTC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Independent watchdog for D1 stop in debug"]
    #[inline(always)]
    pub fn dbg_wdglsd1(&self) -> DBG_WDGLSD1_R {
        DBG_WDGLSD1_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c4(&mut self) -> DBG_I2C4_W<APB4FZ1rs> {
        DBG_I2C4_W::new(self, 7)
    }
    #[doc = "Bit 9 - LPTIM2 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim2(&mut self) -> DBG_LPTIM2_W<APB4FZ1rs> {
        DBG_LPTIM2_W::new(self, 9)
    }
    #[doc = "Bit 10 - LPTIM2 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim3(&mut self) -> DBG_LPTIM3_W<APB4FZ1rs> {
        DBG_LPTIM3_W::new(self, 10)
    }
    #[doc = "Bit 11 - LPTIM4 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim4(&mut self) -> DBG_LPTIM4_W<APB4FZ1rs> {
        DBG_LPTIM4_W::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM5 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim5(&mut self) -> DBG_LPTIM5_W<APB4FZ1rs> {
        DBG_LPTIM5_W::new(self, 12)
    }
    #[doc = "Bit 16 - RTC stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc(&mut self) -> DBG_RTC_W<APB4FZ1rs> {
        DBG_RTC_W::new(self, 16)
    }
    #[doc = "Bit 18 - Independent watchdog for D1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wdglsd1(&mut self) -> DBG_WDGLSD1_W<APB4FZ1rs> {
        DBG_WDGLSD1_W::new(self, 18)
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
