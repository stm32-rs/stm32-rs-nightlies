#[doc = "Register `APB3FZR` reader"]
pub type R = crate::R<APB3FZRrs>;
#[doc = "Register `APB3FZR` writer"]
pub type W = crate::W<APB3FZRrs>;
#[doc = "Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout stop in debug"]
pub type DBG_I2C3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout stop in debug"]
pub type DBG_I2C3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C4_STOP` reader - I2C4 SMBUS timeout stop in debug"]
pub type DBG_I2C4_STOP_R = crate::BitReader;
#[doc = "Field `DBG_I2C4_STOP` writer - I2C4 SMBUS timeout stop in debug"]
pub type DBG_I2C4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM1_STOP` reader - LPTIM1 stop in debug"]
pub type DBG_LPTIM1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM1_STOP` writer - LPTIM1 stop in debug"]
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM3_STOP` reader - LPTIM3 stop in debug"]
pub type DBG_LPTIM3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM3_STOP` writer - LPTIM3 stop in debug"]
pub type DBG_LPTIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM4_STOP` reader - LPTIM4 stop in debug"]
pub type DBG_LPTIM4_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM4_STOP` writer - LPTIM4 stop in debug"]
pub type DBG_LPTIM4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM5_STOP` reader - LPTIM5 stop in debug"]
pub type DBG_LPTIM5_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM5_STOP` writer - LPTIM5 stop in debug"]
pub type DBG_LPTIM5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM6_STOP` reader - LPTIM6 stop in debug"]
pub type DBG_LPTIM6_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM6_STOP` writer - LPTIM6 stop in debug"]
pub type DBG_LPTIM6_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_STOP` reader - RTC stop in debug"]
pub type DBG_RTC_STOP_R = crate::BitReader;
#[doc = "Field `DBG_RTC_STOP` writer - RTC stop in debug"]
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c4_stop(&self) -> DBG_I2C4_STOP_R {
        DBG_I2C4_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LPTIM3 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim3_stop(&self) -> DBG_LPTIM3_STOP_R {
        DBG_LPTIM3_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LPTIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim4_stop(&self) -> DBG_LPTIM4_STOP_R {
        DBG_LPTIM4_STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPTIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim5_stop(&self) -> DBG_LPTIM5_STOP_R {
        DBG_LPTIM5_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPTIM6 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim6_stop(&self) -> DBG_LPTIM6_STOP_R {
        DBG_LPTIM6_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - RTC stop in debug"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<APB3FZRrs> {
        DBG_I2C3_STOP_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c4_stop(&mut self) -> DBG_I2C4_STOP_W<APB3FZRrs> {
        DBG_I2C4_STOP_W::new(self, 11)
    }
    #[doc = "Bit 17 - LPTIM1 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<APB3FZRrs> {
        DBG_LPTIM1_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - LPTIM3 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim3_stop(&mut self) -> DBG_LPTIM3_STOP_W<APB3FZRrs> {
        DBG_LPTIM3_STOP_W::new(self, 18)
    }
    #[doc = "Bit 19 - LPTIM4 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim4_stop(&mut self) -> DBG_LPTIM4_STOP_W<APB3FZRrs> {
        DBG_LPTIM4_STOP_W::new(self, 19)
    }
    #[doc = "Bit 20 - LPTIM5 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim5_stop(&mut self) -> DBG_LPTIM5_STOP_W<APB3FZRrs> {
        DBG_LPTIM5_STOP_W::new(self, 20)
    }
    #[doc = "Bit 21 - LPTIM6 stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim6_stop(&mut self) -> DBG_LPTIM6_STOP_W<APB3FZRrs> {
        DBG_LPTIM6_STOP_W::new(self, 21)
    }
    #[doc = "Bit 30 - RTC stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<APB3FZRrs> {
        DBG_RTC_STOP_W::new(self, 30)
    }
}
#[doc = "DBGMCU APB3 peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3fzr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3fzr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3FZRrs;
impl crate::RegisterSpec for APB3FZRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3fzr::R`](R) reader structure"]
impl crate::Readable for APB3FZRrs {}
#[doc = "`write(|w| ..)` method takes [`apb3fzr::W`](W) writer structure"]
impl crate::Writable for APB3FZRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB3FZR to value 0"]
impl crate::Resettable for APB3FZRrs {
    const RESET_VALUE: u32 = 0;
}
