///Register `APB1LFZR` reader
pub type R = crate::R<APB1LFZRrs>;
///Register `APB1LFZR` writer
pub type W = crate::W<APB1LFZRrs>;
///Field `DBG_TIM2_STOP` reader - TIM2 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM2SEC.
pub type DBG_TIM2_STOP_R = crate::BitReader;
///Field `DBG_TIM2_STOP` writer - TIM2 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM2SEC.
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM3_STOP` reader - TIM3 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM3SEC.
pub type DBG_TIM3_STOP_R = crate::BitReader;
///Field `DBG_TIM3_STOP` writer - TIM3 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM3SEC.
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_WWDG_STOP` reader - WWDG stop in CPU debug Write access can be protected by GTZC_TZSC.WWDGSEC
pub type DBG_WWDG_STOP_R = crate::BitReader;
///Field `DBG_WWDG_STOP` writer - WWDG stop in CPU debug Write access can be protected by GTZC_TZSC.WWDGSEC
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_IWDG_STOP` reader - IWDG stop in CPU debug Write access can be protected by GTZC_TZSC.IWDGSEC.
pub type DBG_IWDG_STOP_R = crate::BitReader;
///Field `DBG_IWDG_STOP` writer - IWDG stop in CPU debug Write access can be protected by GTZC_TZSC.IWDGSEC.
pub type DBG_IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout stop in CPU debug Write access can be protected by GTZC_TZSC.I2C1SEC.
pub type DBG_I2C1_STOP_R = crate::BitReader;
///Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout stop in CPU debug Write access can be protected by GTZC_TZSC.I2C1SEC.
pub type DBG_I2C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM2SEC.
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM3SEC.
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 11 - WWDG stop in CPU debug Write access can be protected by GTZC_TZSC.WWDGSEC
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - IWDG stop in CPU debug Write access can be protected by GTZC_TZSC.IWDGSEC.
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - I2C1 SMBUS timeout stop in CPU debug Write access can be protected by GTZC_TZSC.I2C1SEC.
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LFZR")
            .field("dbg_tim2_stop", &self.dbg_tim2_stop())
            .field("dbg_tim3_stop", &self.dbg_tim3_stop())
            .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
            .field("dbg_iwdg_stop", &self.dbg_iwdg_stop())
            .field("dbg_i2c1_stop", &self.dbg_i2c1_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM2SEC.
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<'_, APB1LFZRrs> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    ///Bit 1 - TIM3 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM3SEC.
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<'_, APB1LFZRrs> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    ///Bit 11 - WWDG stop in CPU debug Write access can be protected by GTZC_TZSC.WWDGSEC
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<'_, APB1LFZRrs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    ///Bit 12 - IWDG stop in CPU debug Write access can be protected by GTZC_TZSC.IWDGSEC.
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<'_, APB1LFZRrs> {
        DBG_IWDG_STOP_W::new(self, 12)
    }
    ///Bit 21 - I2C1 SMBUS timeout stop in CPU debug Write access can be protected by GTZC_TZSC.I2C1SEC.
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<'_, APB1LFZRrs> {
        DBG_I2C1_STOP_W::new(self, 21)
    }
}
/**DBGMCU APB1L peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb1lfzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#DBGMCU:APB1LFZR)*/
pub struct APB1LFZRrs;
impl crate::RegisterSpec for APB1LFZRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1lfzr::R`](R) reader structure
impl crate::Readable for APB1LFZRrs {}
///`write(|w| ..)` method takes [`apb1lfzr::W`](W) writer structure
impl crate::Writable for APB1LFZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LFZR to value 0
impl crate::Resettable for APB1LFZRrs {}
