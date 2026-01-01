///Register `APB1LFZ2` reader
pub type R = crate::R<APB1LFZ2rs>;
///Register `APB1LFZ2` writer
pub type W = crate::W<APB1LFZ2rs>;
///Field `DBG_TIM2` reader - TIM2 stop in debug
pub type DBG_TIM2_R = crate::BitReader;
///Field `DBG_TIM2` writer - TIM2 stop in debug
pub type DBG_TIM2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM3` reader - TIM3 stop in debug
pub type DBG_TIM3_R = crate::BitReader;
///Field `DBG_TIM3` writer - TIM3 stop in debug
pub type DBG_TIM3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM4` reader - TIM4 stop in debug
pub type DBG_TIM4_R = crate::BitReader;
///Field `DBG_TIM4` writer - TIM4 stop in debug
pub type DBG_TIM4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM5` reader - TIM5 stop in debug
pub type DBG_TIM5_R = crate::BitReader;
///Field `DBG_TIM5` writer - TIM5 stop in debug
pub type DBG_TIM5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM6` reader - TIM6 stop in debug
pub type DBG_TIM6_R = crate::BitReader;
///Field `DBG_TIM6` writer - TIM6 stop in debug
pub type DBG_TIM6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM7` reader - TIM4 stop in debug
pub type DBG_TIM7_R = crate::BitReader;
///Field `DBG_TIM7` writer - TIM4 stop in debug
pub type DBG_TIM7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM12` reader - TIM12 stop in debug
pub type DBG_TIM12_R = crate::BitReader;
///Field `DBG_TIM12` writer - TIM12 stop in debug
pub type DBG_TIM12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM13` reader - TIM13 stop in debug
pub type DBG_TIM13_R = crate::BitReader;
///Field `DBG_TIM13` writer - TIM13 stop in debug
pub type DBG_TIM13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM14` reader - TIM14 stop in debug
pub type DBG_TIM14_R = crate::BitReader;
///Field `DBG_TIM14` writer - TIM14 stop in debug
pub type DBG_TIM14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM1` reader - LPTIM1 stop in debug
pub type DBG_LPTIM1_R = crate::BitReader;
///Field `DBG_LPTIM1` writer - LPTIM1 stop in debug
pub type DBG_LPTIM1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_WWDG2` reader - WWDG2 stop in debug
pub type DBG_WWDG2_R = crate::BitReader;
///Field `DBG_WWDG2` writer - WWDG2 stop in debug
pub type DBG_WWDG2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C1` reader - I2C1 SMBUS timeout stop in debug
pub type DBG_I2C1_R = crate::BitReader;
///Field `DBG_I2C1` writer - I2C1 SMBUS timeout stop in debug
pub type DBG_I2C1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C2` reader - I2C2 SMBUS timeout stop in debug
pub type DBG_I2C2_R = crate::BitReader;
///Field `DBG_I2C2` writer - I2C2 SMBUS timeout stop in debug
pub type DBG_I2C2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C3` reader - I2C3 SMBUS timeout stop in debug
pub type DBG_I2C3_R = crate::BitReader;
///Field `DBG_I2C3` writer - I2C3 SMBUS timeout stop in debug
pub type DBG_I2C3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 stop in debug
    #[inline(always)]
    pub fn dbg_tim2(&self) -> DBG_TIM2_R {
        DBG_TIM2_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 stop in debug
    #[inline(always)]
    pub fn dbg_tim3(&self) -> DBG_TIM3_R {
        DBG_TIM3_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 stop in debug
    #[inline(always)]
    pub fn dbg_tim4(&self) -> DBG_TIM4_R {
        DBG_TIM4_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 stop in debug
    #[inline(always)]
    pub fn dbg_tim5(&self) -> DBG_TIM5_R {
        DBG_TIM5_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 stop in debug
    #[inline(always)]
    pub fn dbg_tim6(&self) -> DBG_TIM6_R {
        DBG_TIM6_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM4 stop in debug
    #[inline(always)]
    pub fn dbg_tim7(&self) -> DBG_TIM7_R {
        DBG_TIM7_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIM12 stop in debug
    #[inline(always)]
    pub fn dbg_tim12(&self) -> DBG_TIM12_R {
        DBG_TIM12_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TIM13 stop in debug
    #[inline(always)]
    pub fn dbg_tim13(&self) -> DBG_TIM13_R {
        DBG_TIM13_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIM14 stop in debug
    #[inline(always)]
    pub fn dbg_tim14(&self) -> DBG_TIM14_R {
        DBG_TIM14_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LPTIM1 stop in debug
    #[inline(always)]
    pub fn dbg_lptim1(&self) -> DBG_LPTIM1_R {
        DBG_LPTIM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - WWDG2 stop in debug
    #[inline(always)]
    pub fn dbg_wwdg2(&self) -> DBG_WWDG2_R {
        DBG_WWDG2_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 21 - I2C1 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c1(&self) -> DBG_I2C1_R {
        DBG_I2C1_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c2(&self) -> DBG_I2C2_R {
        DBG_I2C2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c3(&self) -> DBG_I2C3_R {
        DBG_I2C3_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LFZ2")
            .field("dbg_tim2", &self.dbg_tim2())
            .field("dbg_tim3", &self.dbg_tim3())
            .field("dbg_tim4", &self.dbg_tim4())
            .field("dbg_tim5", &self.dbg_tim5())
            .field("dbg_tim6", &self.dbg_tim6())
            .field("dbg_tim7", &self.dbg_tim7())
            .field("dbg_tim12", &self.dbg_tim12())
            .field("dbg_tim13", &self.dbg_tim13())
            .field("dbg_tim14", &self.dbg_tim14())
            .field("dbg_lptim1", &self.dbg_lptim1())
            .field("dbg_wwdg2", &self.dbg_wwdg2())
            .field("dbg_i2c1", &self.dbg_i2c1())
            .field("dbg_i2c2", &self.dbg_i2c2())
            .field("dbg_i2c3", &self.dbg_i2c3())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 stop in debug
    #[inline(always)]
    pub fn dbg_tim2(&mut self) -> DBG_TIM2_W<'_, APB1LFZ2rs> {
        DBG_TIM2_W::new(self, 0)
    }
    ///Bit 1 - TIM3 stop in debug
    #[inline(always)]
    pub fn dbg_tim3(&mut self) -> DBG_TIM3_W<'_, APB1LFZ2rs> {
        DBG_TIM3_W::new(self, 1)
    }
    ///Bit 2 - TIM4 stop in debug
    #[inline(always)]
    pub fn dbg_tim4(&mut self) -> DBG_TIM4_W<'_, APB1LFZ2rs> {
        DBG_TIM4_W::new(self, 2)
    }
    ///Bit 3 - TIM5 stop in debug
    #[inline(always)]
    pub fn dbg_tim5(&mut self) -> DBG_TIM5_W<'_, APB1LFZ2rs> {
        DBG_TIM5_W::new(self, 3)
    }
    ///Bit 4 - TIM6 stop in debug
    #[inline(always)]
    pub fn dbg_tim6(&mut self) -> DBG_TIM6_W<'_, APB1LFZ2rs> {
        DBG_TIM6_W::new(self, 4)
    }
    ///Bit 5 - TIM4 stop in debug
    #[inline(always)]
    pub fn dbg_tim7(&mut self) -> DBG_TIM7_W<'_, APB1LFZ2rs> {
        DBG_TIM7_W::new(self, 5)
    }
    ///Bit 6 - TIM12 stop in debug
    #[inline(always)]
    pub fn dbg_tim12(&mut self) -> DBG_TIM12_W<'_, APB1LFZ2rs> {
        DBG_TIM12_W::new(self, 6)
    }
    ///Bit 7 - TIM13 stop in debug
    #[inline(always)]
    pub fn dbg_tim13(&mut self) -> DBG_TIM13_W<'_, APB1LFZ2rs> {
        DBG_TIM13_W::new(self, 7)
    }
    ///Bit 8 - TIM14 stop in debug
    #[inline(always)]
    pub fn dbg_tim14(&mut self) -> DBG_TIM14_W<'_, APB1LFZ2rs> {
        DBG_TIM14_W::new(self, 8)
    }
    ///Bit 9 - LPTIM1 stop in debug
    #[inline(always)]
    pub fn dbg_lptim1(&mut self) -> DBG_LPTIM1_W<'_, APB1LFZ2rs> {
        DBG_LPTIM1_W::new(self, 9)
    }
    ///Bit 11 - WWDG2 stop in debug
    #[inline(always)]
    pub fn dbg_wwdg2(&mut self) -> DBG_WWDG2_W<'_, APB1LFZ2rs> {
        DBG_WWDG2_W::new(self, 11)
    }
    ///Bit 21 - I2C1 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c1(&mut self) -> DBG_I2C1_W<'_, APB1LFZ2rs> {
        DBG_I2C1_W::new(self, 21)
    }
    ///Bit 22 - I2C2 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c2(&mut self) -> DBG_I2C2_W<'_, APB1LFZ2rs> {
        DBG_I2C2_W::new(self, 22)
    }
    ///Bit 23 - I2C3 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c3(&mut self) -> DBG_I2C3_W<'_, APB1LFZ2rs> {
        DBG_I2C3_W::new(self, 23)
    }
}
/**DBGMCU APB1L peripheral freeze register CPU2

You can [`read`](crate::Reg::read) this register and get [`apb1lfz2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfz2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#DBGMCU:APB1LFZ2)*/
pub struct APB1LFZ2rs;
impl crate::RegisterSpec for APB1LFZ2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1lfz2::R`](R) reader structure
impl crate::Readable for APB1LFZ2rs {}
///`write(|w| ..)` method takes [`apb1lfz2::W`](W) writer structure
impl crate::Writable for APB1LFZ2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LFZ2 to value 0
impl crate::Resettable for APB1LFZ2rs {}
