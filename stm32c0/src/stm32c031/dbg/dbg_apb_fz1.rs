#[doc = "Register `DBG_APB_FZ1` reader"]
pub type R = crate::R<DBG_APB_FZ1rs>;
#[doc = "Register `DBG_APB_FZ1` writer"]
pub type W = crate::W<DBG_APB_FZ1rs>;
#[doc = "Field `DBG_TIM3_STOP` reader - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
pub type DBG_TIM3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM3_STOP` writer - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_STOP` reader - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
pub type DBG_RTC_STOP_R = crate::BitReader;
#[doc = "Field `DBG_RTC_STOP` writer - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
pub type DBG_WWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` reader - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
pub type DBG_IWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` reader - SMBUS timeout when core is halted"]
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` writer - SMBUS timeout when core is halted"]
pub type DBG_I2C1_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBUS timeout when core is halted"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clocking of TIM3 counter when the core is halted This bit enables/disables the clock to the counter of TIM3 when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<DBG_APB_FZ1rs> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    #[doc = "Bit 10 - Clocking of RTC counter when the core is halted This bit enables/disables the clock to the counter of RTC when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<DBG_APB_FZ1rs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clocking of WWDG counter when the core is halted This bit enables/disables the clock to the counter of WWDG when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<DBG_APB_FZ1rs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clocking of IWDG counter when the core is halted This bit enables/disables the clock to the counter of IWDG when the core is halted:"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<DBG_APB_FZ1rs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    #[doc = "Bit 21 - SMBUS timeout when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W<DBG_APB_FZ1rs> {
        DBG_I2C1_SMBUS_TIMEOUT_W::new(self, 21)
    }
}
#[doc = "DBG APB freeze register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_apb_fz1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_apb_fz1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_APB_FZ1rs;
impl crate::RegisterSpec for DBG_APB_FZ1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_apb_fz1::R`](R) reader structure"]
impl crate::Readable for DBG_APB_FZ1rs {}
#[doc = "`write(|w| ..)` method takes [`dbg_apb_fz1::W`](W) writer structure"]
impl crate::Writable for DBG_APB_FZ1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_APB_FZ1 to value 0"]
impl crate::Resettable for DBG_APB_FZ1rs {
    const RESET_VALUE: u32 = 0;
}
