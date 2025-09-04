///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `DBG_SLEEP` reader - DBG_SLEEP
pub type DBG_SLEEP_R = crate::BitReader;
///Field `DBG_SLEEP` writer - DBG_SLEEP
pub type DBG_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_STOP` reader - DBG_STOP
pub type DBG_STOP_R = crate::BitReader;
///Field `DBG_STOP` writer - DBG_STOP
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_STANDBY` reader - DBG_STANDBY
pub type DBG_STANDBY_R = crate::BitReader;
///Field `DBG_STANDBY` writer - DBG_STANDBY
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRACE_IOEN` reader - TRACE_IOEN
pub type TRACE_IOEN_R = crate::BitReader;
///Field `TRACE_IOEN` writer - TRACE_IOEN
pub type TRACE_IOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRACE_MODE` reader - TRACE_MODE
pub type TRACE_MODE_R = crate::FieldReader;
///Field `TRACE_MODE` writer - TRACE_MODE
pub type TRACE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DBG_IWDG_STOP` reader - DBG_IWDG_STOP
pub type DBG_IWDG_STOP_R = crate::BitReader;
///Field `DBG_IWDG_STOP` writer - DBG_IWDG_STOP
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_WWDG_STOP` reader - DBG_WWDG_STOP
pub type DBG_WWDG_STOP_R = crate::BitReader;
///Field `DBG_WWDG_STOP` writer - DBG_WWDG_STOP
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM1_STOP` reader - DBG_TIM1_STOP
pub type DBG_TIM1_STOP_R = crate::BitReader;
///Field `DBG_TIM1_STOP` writer - DBG_TIM1_STOP
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM2_STOP` reader - DBG_TIM2_STOP
pub type DBG_TIM2_STOP_R = crate::BitReader;
///Field `DBG_TIM2_STOP` writer - DBG_TIM2_STOP
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM3_STOP` reader - DBG_TIM3_STOP
pub type DBG_TIM3_STOP_R = crate::BitReader;
///Field `DBG_TIM3_STOP` writer - DBG_TIM3_STOP
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM4_STOP` reader - DBG_TIM4_STOP
pub type DBG_TIM4_STOP_R = crate::BitReader;
///Field `DBG_TIM4_STOP` writer - DBG_TIM4_STOP
pub type DBG_TIM4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C1_SMBUS_TIMEOUT` reader - DBG_I2C1_SMBUS_TIMEOUT
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::BitReader;
///Field `DBG_I2C1_SMBUS_TIMEOUT` writer - DBG_I2C1_SMBUS_TIMEOUT
pub type DBG_I2C1_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C2_SMBUS_TIMEOUT` reader - DBG_I2C2_SMBUS_TIMEOUT
pub type DBG_I2C2_SMBUS_TIMEOUT_R = crate::BitReader;
///Field `DBG_I2C2_SMBUS_TIMEOUT` writer - DBG_I2C2_SMBUS_TIMEOUT
pub type DBG_I2C2_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM5_STOP` reader - DBG_TIM5_STOP
pub type DBG_TIM5_STOP_R = crate::BitReader;
///Field `DBG_TIM5_STOP` writer - DBG_TIM5_STOP
pub type DBG_TIM5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM6_STOP` reader - DBG_TIM6_STOP
pub type DBG_TIM6_STOP_R = crate::BitReader;
///Field `DBG_TIM6_STOP` writer - DBG_TIM6_STOP
pub type DBG_TIM6_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM7_STOP` reader - DBG_TIM7_STOP
pub type DBG_TIM7_STOP_R = crate::BitReader;
///Field `DBG_TIM7_STOP` writer - DBG_TIM7_STOP
pub type DBG_TIM7_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM15_STOP` reader - TIM15 counter stopped when core is halted
pub type DBG_TIM15_STOP_R = crate::BitReader;
///Field `DBG_TIM15_STOP` writer - TIM15 counter stopped when core is halted
pub type DBG_TIM15_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM16_STOP` reader - TIM16 counter stopped when core is halted
pub type DBG_TIM16_STOP_R = crate::BitReader;
///Field `DBG_TIM16_STOP` writer - TIM16 counter stopped when core is halted
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM17_STOP` reader - TIM17 counter stopped when core is halted
pub type DBG_TIM17_STOP_R = crate::BitReader;
///Field `DBG_TIM17_STOP` writer - TIM17 counter stopped when core is halted
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM12_STOP` reader - TIM12 counter stopped when core is halted
pub type DBG_TIM12_STOP_R = crate::BitReader;
///Field `DBG_TIM12_STOP` writer - TIM12 counter stopped when core is halted
pub type DBG_TIM12_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM13_STOP` reader - TIM13 counter stopped when core is halted
pub type DBG_TIM13_STOP_R = crate::BitReader;
///Field `DBG_TIM13_STOP` writer - TIM13 counter stopped when core is halted
pub type DBG_TIM13_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM14_STOP` reader - TIM14 counter stopped when core is halted
pub type DBG_TIM14_STOP_R = crate::BitReader;
///Field `DBG_TIM14_STOP` writer - TIM14 counter stopped when core is halted
pub type DBG_TIM14_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DBG_SLEEP
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DBG_STOP
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DBG_STANDBY
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - TRACE_IOEN
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - TRACE_MODE
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - DBG_IWDG_STOP
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DBG_WWDG_STOP
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DBG_TIM1_STOP
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DBG_TIM2_STOP
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DBG_TIM3_STOP
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DBG_TIM4_STOP
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - DBG_I2C1_SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - DBG_I2C2_SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&self) -> DBG_I2C2_SMBUS_TIMEOUT_R {
        DBG_I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - DBG_TIM5_STOP
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DBG_TIM6_STOP
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - DBG_TIM7_STOP
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - TIM15 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TIM16 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - TIM17 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TIM12 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim12_stop(&self) -> DBG_TIM12_STOP_R {
        DBG_TIM12_STOP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - TIM13 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim13_stop(&self) -> DBG_TIM13_STOP_R {
        DBG_TIM13_STOP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TIM14 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("dbg_sleep", &self.dbg_sleep())
            .field("dbg_stop", &self.dbg_stop())
            .field("dbg_standby", &self.dbg_standby())
            .field("trace_ioen", &self.trace_ioen())
            .field("trace_mode", &self.trace_mode())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .field("dbg_tim2_stop", &self.dbg_tim2_stop())
            .field("dbg_tim3_stop", &self.dbg_tim3_stop())
            .field("dbg_tim4_stop", &self.dbg_tim4_stop())
            .field("dbg_i2c1_smbus_timeout", &self.dbg_i2c1_smbus_timeout())
            .field("dbg_i2c2_smbus_timeout", &self.dbg_i2c2_smbus_timeout())
            .field("dbg_tim5_stop", &self.dbg_tim5_stop())
            .field("dbg_tim6_stop", &self.dbg_tim6_stop())
            .field("dbg_tim7_stop", &self.dbg_tim7_stop())
            .field("dbg_tim15_stop", &self.dbg_tim15_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .field("dbg_tim17_stop", &self.dbg_tim17_stop())
            .field("dbg_tim12_stop", &self.dbg_tim12_stop())
            .field("dbg_tim13_stop", &self.dbg_tim13_stop())
            .field("dbg_tim14_stop", &self.dbg_tim14_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - DBG_SLEEP
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<CRrs> {
        DBG_SLEEP_W::new(self, 0)
    }
    ///Bit 1 - DBG_STOP
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    ///Bit 2 - DBG_STANDBY
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
    ///Bit 5 - TRACE_IOEN
    #[inline(always)]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<CRrs> {
        TRACE_IOEN_W::new(self, 5)
    }
    ///Bits 6:7 - TRACE_MODE
    #[inline(always)]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<CRrs> {
        TRACE_MODE_W::new(self, 6)
    }
    ///Bit 8 - DBG_IWDG_STOP
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<CRrs> {
        DBG_IWDG_STOP_W::new(self, 8)
    }
    ///Bit 9 - DBG_WWDG_STOP
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<CRrs> {
        DBG_WWDG_STOP_W::new(self, 9)
    }
    ///Bit 10 - DBG_TIM1_STOP
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<CRrs> {
        DBG_TIM1_STOP_W::new(self, 10)
    }
    ///Bit 11 - DBG_TIM2_STOP
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<CRrs> {
        DBG_TIM2_STOP_W::new(self, 11)
    }
    ///Bit 12 - DBG_TIM3_STOP
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<CRrs> {
        DBG_TIM3_STOP_W::new(self, 12)
    }
    ///Bit 13 - DBG_TIM4_STOP
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<CRrs> {
        DBG_TIM4_STOP_W::new(self, 13)
    }
    ///Bit 15 - DBG_I2C1_SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W<CRrs> {
        DBG_I2C1_SMBUS_TIMEOUT_W::new(self, 15)
    }
    ///Bit 16 - DBG_I2C2_SMBUS_TIMEOUT
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&mut self) -> DBG_I2C2_SMBUS_TIMEOUT_W<CRrs> {
        DBG_I2C2_SMBUS_TIMEOUT_W::new(self, 16)
    }
    ///Bit 18 - DBG_TIM5_STOP
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<CRrs> {
        DBG_TIM5_STOP_W::new(self, 18)
    }
    ///Bit 19 - DBG_TIM6_STOP
    #[inline(always)]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<CRrs> {
        DBG_TIM6_STOP_W::new(self, 19)
    }
    ///Bit 20 - DBG_TIM7_STOP
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<CRrs> {
        DBG_TIM7_STOP_W::new(self, 20)
    }
    ///Bit 22 - TIM15 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<CRrs> {
        DBG_TIM15_STOP_W::new(self, 22)
    }
    ///Bit 23 - TIM16 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<CRrs> {
        DBG_TIM16_STOP_W::new(self, 23)
    }
    ///Bit 24 - TIM17 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<CRrs> {
        DBG_TIM17_STOP_W::new(self, 24)
    }
    ///Bit 25 - TIM12 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim12_stop(&mut self) -> DBG_TIM12_STOP_W<CRrs> {
        DBG_TIM12_STOP_W::new(self, 25)
    }
    ///Bit 26 - TIM13 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim13_stop(&mut self) -> DBG_TIM13_STOP_W<CRrs> {
        DBG_TIM13_STOP_W::new(self, 26)
    }
    ///Bit 27 - TIM14 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<CRrs> {
        DBG_TIM14_STOP_W::new(self, 27)
    }
}
/**DBGMCU_CR

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#DBGMCU:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
