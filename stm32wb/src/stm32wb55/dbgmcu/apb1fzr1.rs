#[doc = "Register `APB1FZR1` reader"]
pub type R = crate::R<APB1FZR1rs>;
#[doc = "Register `APB1FZR1` writer"]
pub type W = crate::W<APB1FZR1rs>;
#[doc = "Field `DBG_TIMER2_STOP` reader - Debug Timer 2 stopped when Core is halted"]
pub type DBG_TIMER2_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIMER2_STOP` writer - Debug Timer 2 stopped when Core is halted"]
pub type DBG_TIMER2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_STOP` reader - RTC counter stopped when core is halted"]
pub type DBG_RTC_STOP_R = crate::BitReader;
#[doc = "Field `DBG_RTC_STOP` writer - RTC counter stopped when core is halted"]
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - WWDG counter stopped when core is halted"]
pub type DBG_WWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - WWDG counter stopped when core is halted"]
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` reader - IWDG counter stopped when core is halted"]
pub type DBG_IWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - IWDG counter stopped when core is halted"]
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C1_STOP` reader - Debug I2C1 SMBUS timeout stopped when Core is halted"]
pub type DBG_I2C1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_I2C1_STOP` writer - Debug I2C1 SMBUS timeout stopped when Core is halted"]
pub type DBG_I2C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C3_STOP` reader - Debug I2C3 SMBUS timeout stopped when core is halted"]
pub type DBG_I2C3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_I2C3_STOP` writer - Debug I2C3 SMBUS timeout stopped when core is halted"]
pub type DBG_I2C3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM1_STOP` reader - Debug LPTIM1 stopped when Core is halted"]
pub type DBG_LPTIM1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM1_STOP` writer - Debug LPTIM1 stopped when Core is halted"]
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_timer2_stop(&self) -> DBG_TIMER2_STOP_R {
        DBG_TIMER2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - RTC counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IWDG counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - Debug I2C1 SMBUS timeout stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Debug I2C3 SMBUS timeout stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Debug LPTIM1 stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Timer 2 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_timer2_stop(&mut self) -> DBG_TIMER2_STOP_W<APB1FZR1rs> {
        DBG_TIMER2_STOP_W::new(self, 0)
    }
    #[doc = "Bit 10 - RTC counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<APB1FZR1rs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    #[doc = "Bit 11 - WWDG counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<APB1FZR1rs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    #[doc = "Bit 12 - IWDG counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<APB1FZR1rs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    #[doc = "Bit 21 - Debug I2C1 SMBUS timeout stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<APB1FZR1rs> {
        DBG_I2C1_STOP_W::new(self, 21)
    }
    #[doc = "Bit 23 - Debug I2C3 SMBUS timeout stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<APB1FZR1rs> {
        DBG_I2C3_STOP_W::new(self, 23)
    }
    #[doc = "Bit 31 - Debug LPTIM1 stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<APB1FZR1rs> {
        DBG_LPTIM1_STOP_W::new(self, 31)
    }
}
#[doc = "APB1 Low Freeze Register CPU1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1fzr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1fzr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1FZR1rs;
impl crate::RegisterSpec for APB1FZR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1fzr1::R`](R) reader structure"]
impl crate::Readable for APB1FZR1rs {}
#[doc = "`write(|w| ..)` method takes [`apb1fzr1::W`](W) writer structure"]
impl crate::Writable for APB1FZR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1FZR1 to value 0"]
impl crate::Resettable for APB1FZR1rs {
    const RESET_VALUE: u32 = 0;
}
