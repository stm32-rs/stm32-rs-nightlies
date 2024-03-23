#[doc = "Register `APB1_FZ` reader"]
pub type R = crate::R<APB1_FZrs>;
#[doc = "Register `APB1_FZ` writer"]
pub type W = crate::W<APB1_FZrs>;
#[doc = "Field `DBG_TIM2_STOP` reader - DBG_TIM2_STOP"]
pub type DBG_TIM2_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM2_STOP` writer - DBG_TIM2_STOP"]
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3_STOP` reader - DBG_TIM3 _STOP"]
pub type DBG_TIM3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM3_STOP` writer - DBG_TIM3 _STOP"]
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM4_STOP` reader - DBG_TIM4_STOP"]
pub type DBG_TIM4_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM4_STOP` writer - DBG_TIM4_STOP"]
pub type DBG_TIM4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM5_STOP` reader - DBG_TIM5_STOP"]
pub type DBG_TIM5_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM5_STOP` writer - DBG_TIM5_STOP"]
pub type DBG_TIM5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_Stop` reader - RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_R = crate::BitReader;
#[doc = "Field `DBG_RTC_Stop` writer - RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - DBG_WWDG_STOP"]
pub type DBG_WWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - DBG_WWDG_STOP"]
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` reader - DBG_IWDEG_STOP"]
pub type DBG_IWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - DBG_IWDEG_STOP"]
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` reader - DBG_J2C1_SMBUS_TIMEOUT"]
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` writer - DBG_J2C1_SMBUS_TIMEOUT"]
pub type DBG_I2C1_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` reader - DBG_J2C2_SMBUS_TIMEOUT"]
pub type DBG_I2C2_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` writer - DBG_J2C2_SMBUS_TIMEOUT"]
pub type DBG_I2C2_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C3SMBUS_TIMEOUT` reader - DBG_J2C3SMBUS_TIMEOUT"]
pub type DBG_I2C3SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `DBG_I2C3SMBUS_TIMEOUT` writer - DBG_J2C3SMBUS_TIMEOUT"]
pub type DBG_I2C3SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DBG_TIM2_STOP"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBG_TIM3 _STOP"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DBG_TIM4_STOP"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DBG_TIM5_STOP"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBG_WWDG_STOP"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DBG_IWDEG_STOP"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DBG_J2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DBG_J2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&self) -> DBG_I2C2_SMBUS_TIMEOUT_R {
        DBG_I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DBG_J2C3SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c3smbus_timeout(&self) -> DBG_I2C3SMBUS_TIMEOUT_R {
        DBG_I2C3SMBUS_TIMEOUT_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBG_TIM2_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<APB1_FZrs> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - DBG_TIM3 _STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<APB1_FZrs> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - DBG_TIM4_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<APB1_FZrs> {
        DBG_TIM4_STOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - DBG_TIM5_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<APB1_FZrs> {
        DBG_TIM5_STOP_W::new(self, 3)
    }
    #[doc = "Bit 10 - RTC stopped when Core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<APB1_FZrs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    #[doc = "Bit 11 - DBG_WWDG_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<APB1_FZrs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    #[doc = "Bit 12 - DBG_IWDEG_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<APB1_FZrs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    #[doc = "Bit 21 - DBG_J2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W<APB1_FZrs> {
        DBG_I2C1_SMBUS_TIMEOUT_W::new(self, 21)
    }
    #[doc = "Bit 22 - DBG_J2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c2_smbus_timeout(&mut self) -> DBG_I2C2_SMBUS_TIMEOUT_W<APB1_FZrs> {
        DBG_I2C2_SMBUS_TIMEOUT_W::new(self, 22)
    }
    #[doc = "Bit 23 - DBG_J2C3SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c3smbus_timeout(&mut self) -> DBG_I2C3SMBUS_TIMEOUT_W<APB1_FZrs> {
        DBG_I2C3SMBUS_TIMEOUT_W::new(self, 23)
    }
}
#[doc = "Debug MCU APB1 Freeze registe\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1_fz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1_fz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1_FZrs;
impl crate::RegisterSpec for APB1_FZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1_fz::R`](R) reader structure"]
impl crate::Readable for APB1_FZrs {}
#[doc = "`write(|w| ..)` method takes [`apb1_fz::W`](W) writer structure"]
impl crate::Writable for APB1_FZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1_FZ to value 0"]
impl crate::Resettable for APB1_FZrs {
    const RESET_VALUE: u32 = 0;
}
