#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `DBG_SLEEP` reader - DBG_SLEEP"]
pub type DBG_SLEEP_R = crate::BitReader;
#[doc = "Field `DBG_SLEEP` writer - DBG_SLEEP"]
pub type DBG_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP` reader - DBG_STOP"]
pub type DBG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_STOP` writer - DBG_STOP"]
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STANDBY` reader - DBG_STANDBY"]
pub type DBG_STANDBY_R = crate::BitReader;
#[doc = "Field `DBG_STANDBY` writer - DBG_STANDBY"]
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_IOEN` reader - TRACE_IOEN"]
pub type TRACE_IOEN_R = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - TRACE_IOEN"]
pub type TRACE_IOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_MODE` reader - TRACE_MODE"]
pub type TRACE_MODE_R = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - TRACE_MODE"]
pub type TRACE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBG_IWDG_STOP` reader - DBG_IWDG_STOP"]
pub type DBG_IWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - DBG_IWDG_STOP"]
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - DBG_WWDG_STOP"]
pub type DBG_WWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - DBG_WWDG_STOP"]
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM1_STOP` reader - DBG_TIM1_STOP"]
pub type DBG_TIM1_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM1_STOP` writer - DBG_TIM1_STOP"]
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM2_STOP` reader - DBG_TIM2_STOP"]
pub type DBG_TIM2_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM2_STOP` writer - DBG_TIM2_STOP"]
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3_STOP` reader - DBG_TIM3_STOP"]
pub type DBG_TIM3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM3_STOP` writer - DBG_TIM3_STOP"]
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM4_STOP` reader - DBG_TIM4_STOP"]
pub type DBG_TIM4_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM4_STOP` writer - DBG_TIM4_STOP"]
pub type DBG_TIM4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` reader - DBG_I2C1_SMBUS_TIMEOUT"]
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` writer - DBG_I2C1_SMBUS_TIMEOUT"]
pub type DBG_I2C1_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` reader - DBG_I2C2_SMBUS_TIMEOUT"]
pub type DBG_I2C2_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` writer - DBG_I2C2_SMBUS_TIMEOUT"]
pub type DBG_I2C2_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM5_STOP` reader - DBG_TIM5_STOP"]
pub type DBG_TIM5_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM5_STOP` writer - DBG_TIM5_STOP"]
pub type DBG_TIM5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM6_STOP` reader - DBG_TIM6_STOP"]
pub type DBG_TIM6_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM6_STOP` writer - DBG_TIM6_STOP"]
pub type DBG_TIM6_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM7_STOP` reader - DBG_TIM7_STOP"]
pub type DBG_TIM7_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM7_STOP` writer - DBG_TIM7_STOP"]
pub type DBG_TIM7_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM15_STOP` reader - TIM15 counter stopped when core is halted"]
pub type DBG_TIM15_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM15_STOP` writer - TIM15 counter stopped when core is halted"]
pub type DBG_TIM15_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16_STOP` reader - TIM16 counter stopped when core is halted"]
pub type DBG_TIM16_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM16_STOP` writer - TIM16 counter stopped when core is halted"]
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17_STOP` reader - TIM17 counter stopped when core is halted"]
pub type DBG_TIM17_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM17_STOP` writer - TIM17 counter stopped when core is halted"]
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM12_STOP` reader - TIM12 counter stopped when core is halted"]
pub type DBG_TIM12_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM12_STOP` writer - TIM12 counter stopped when core is halted"]
pub type DBG_TIM12_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM13_STOP` reader - TIM13 counter stopped when core is halted"]
pub type DBG_TIM13_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM13_STOP` writer - TIM13 counter stopped when core is halted"]
pub type DBG_TIM13_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM14_STOP` reader - TIM14 counter stopped when core is halted"]
pub type DBG_TIM14_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM14_STOP` writer - TIM14 counter stopped when core is halted"]
pub type DBG_TIM14_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - DBG_IWDG_STOP"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DBG_WWDG_STOP"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBG_TIM1_STOP"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBG_TIM2_STOP"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DBG_TIM3_STOP"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DBG_TIM4_STOP"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - DBG_I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&self) -> DBG_I2C2_SMBUS_TIMEOUT_R {
        DBG_I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - DBG_TIM5_STOP"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DBG_TIM6_STOP"]
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DBG_TIM7_STOP"]
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - TIM15 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TIM16 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TIM17 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TIM12 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim12_stop(&self) -> DBG_TIM12_STOP_R {
        DBG_TIM12_STOP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TIM13 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim13_stop(&self) -> DBG_TIM13_STOP_R {
        DBG_TIM13_STOP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TIM14 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<CRrs> {
        DBG_SLEEP_W::new(self, 0)
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    #[must_use]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<CRrs> {
        TRACE_IOEN_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<CRrs> {
        TRACE_MODE_W::new(self, 6)
    }
    #[doc = "Bit 8 - DBG_IWDG_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<CRrs> {
        DBG_IWDG_STOP_W::new(self, 8)
    }
    #[doc = "Bit 9 - DBG_WWDG_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<CRrs> {
        DBG_WWDG_STOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - DBG_TIM1_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<CRrs> {
        DBG_TIM1_STOP_W::new(self, 10)
    }
    #[doc = "Bit 11 - DBG_TIM2_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<CRrs> {
        DBG_TIM2_STOP_W::new(self, 11)
    }
    #[doc = "Bit 12 - DBG_TIM3_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<CRrs> {
        DBG_TIM3_STOP_W::new(self, 12)
    }
    #[doc = "Bit 13 - DBG_TIM4_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<CRrs> {
        DBG_TIM4_STOP_W::new(self, 13)
    }
    #[doc = "Bit 15 - DBG_I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W<CRrs> {
        DBG_I2C1_SMBUS_TIMEOUT_W::new(self, 15)
    }
    #[doc = "Bit 16 - DBG_I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c2_smbus_timeout(&mut self) -> DBG_I2C2_SMBUS_TIMEOUT_W<CRrs> {
        DBG_I2C2_SMBUS_TIMEOUT_W::new(self, 16)
    }
    #[doc = "Bit 18 - DBG_TIM5_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<CRrs> {
        DBG_TIM5_STOP_W::new(self, 18)
    }
    #[doc = "Bit 19 - DBG_TIM6_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<CRrs> {
        DBG_TIM6_STOP_W::new(self, 19)
    }
    #[doc = "Bit 20 - DBG_TIM7_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<CRrs> {
        DBG_TIM7_STOP_W::new(self, 20)
    }
    #[doc = "Bit 22 - TIM15 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<CRrs> {
        DBG_TIM15_STOP_W::new(self, 22)
    }
    #[doc = "Bit 23 - TIM16 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<CRrs> {
        DBG_TIM16_STOP_W::new(self, 23)
    }
    #[doc = "Bit 24 - TIM17 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<CRrs> {
        DBG_TIM17_STOP_W::new(self, 24)
    }
    #[doc = "Bit 25 - TIM12 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim12_stop(&mut self) -> DBG_TIM12_STOP_W<CRrs> {
        DBG_TIM12_STOP_W::new(self, 25)
    }
    #[doc = "Bit 26 - TIM13 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim13_stop(&mut self) -> DBG_TIM13_STOP_W<CRrs> {
        DBG_TIM13_STOP_W::new(self, 26)
    }
    #[doc = "Bit 27 - TIM14 counter stopped when core is halted"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<CRrs> {
        DBG_TIM14_STOP_W::new(self, 27)
    }
}
#[doc = "DBGMCU_CR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
