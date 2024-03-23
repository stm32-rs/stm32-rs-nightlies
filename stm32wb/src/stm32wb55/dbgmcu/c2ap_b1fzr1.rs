#[doc = "Register `C2AP_B1FZR1` reader"]
pub type R = crate::R<C2AP_B1FZR1rs>;
#[doc = "Register `C2AP_B1FZR1` writer"]
pub type W = crate::W<C2AP_B1FZR1rs>;
#[doc = "Field `DBG_LPTIM2_STOP` reader - LPTIM2 counter stopped when core is halted"]
pub type DBG_LPTIM2_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM2_STOP` writer - LPTIM2 counter stopped when core is halted"]
pub type DBG_LPTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_STOP` reader - RTC counter stopped when core is halted"]
pub type DBG_RTC_STOP_R = crate::BitReader;
#[doc = "Field `DBG_RTC_STOP` writer - RTC counter stopped when core is halted"]
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` reader - IWDG stopped when core is halted"]
pub type DBG_IWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - IWDG stopped when core is halted"]
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout stopped when core is halted"]
pub type DBG_I2C1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout stopped when core is halted"]
pub type DBG_I2C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout stopped when core is halted"]
pub type DBG_I2C3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout stopped when core is halted"]
pub type DBG_I2C3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM1_STOP` reader - LPTIM1 counter stopped when core is halted"]
pub type DBG_LPTIM1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_LPTIM1_STOP` writer - LPTIM1 counter stopped when core is halted"]
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPTIM2 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - RTC counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - IWDG stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<C2AP_B1FZR1rs> {
        DBG_LPTIM2_STOP_W::new(self, 0)
    }
    #[doc = "Bit 10 - RTC counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<C2AP_B1FZR1rs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    #[doc = "Bit 12 - IWDG stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<C2AP_B1FZR1rs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<C2AP_B1FZR1rs> {
        DBG_I2C1_STOP_W::new(self, 21)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<C2AP_B1FZR1rs> {
        DBG_I2C3_STOP_W::new(self, 23)
    }
    #[doc = "Bit 31 - LPTIM1 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<C2AP_B1FZR1rs> {
        DBG_LPTIM1_STOP_W::new(self, 31)
    }
}
#[doc = "APB1 Low Freeze Register CPU2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ap_b1fzr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ap_b1fzr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2AP_B1FZR1rs;
impl crate::RegisterSpec for C2AP_B1FZR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2ap_b1fzr1::R`](R) reader structure"]
impl crate::Readable for C2AP_B1FZR1rs {}
#[doc = "`write(|w| ..)` method takes [`c2ap_b1fzr1::W`](W) writer structure"]
impl crate::Writable for C2AP_B1FZR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2AP_B1FZR1 to value 0"]
impl crate::Resettable for C2AP_B1FZR1rs {
    const RESET_VALUE: u32 = 0;
}
