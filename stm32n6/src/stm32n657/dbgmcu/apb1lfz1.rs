///Register `APB1LFZ1` reader
pub type R = crate::R<APB1LFZ1rs>;
///Register `APB1LFZ1` writer
pub type W = crate::W<APB1LFZ1rs>;
///Field `DBG_TIM2_STOP` reader - TIM2 stop in debug
pub type DBG_TIM2_STOP_R = crate::BitReader;
///Field `DBG_TIM2_STOP` writer - TIM2 stop in debug
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM3_STOP` reader - TIM3 stop in debug
pub type DBG_TIM3_STOP_R = crate::BitReader;
///Field `DBG_TIM3_STOP` writer - TIM3 stop in debug
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM4_STOP` reader - TIM4 stop in debug
pub type DBG_TIM4_STOP_R = crate::BitReader;
///Field `DBG_TIM4_STOP` writer - TIM4 stop in debug
pub type DBG_TIM4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM5_STOP` reader - TIM5 stop in debug
pub type DBG_TIM5_STOP_R = crate::BitReader;
///Field `DBG_TIM5_STOP` writer - TIM5 stop in debug
pub type DBG_TIM5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM6_STOP` reader - TIM6 stop in debug
pub type DBG_TIM6_STOP_R = crate::BitReader;
///Field `DBG_TIM6_STOP` writer - TIM6 stop in debug
pub type DBG_TIM6_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM7_STOP` reader - TIM7 stop in debug
pub type DBG_TIM7_STOP_R = crate::BitReader;
///Field `DBG_TIM7_STOP` writer - TIM7 stop in debug
pub type DBG_TIM7_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM12_STOP` reader - TIM12 stop in debug
pub type DBG_TIM12_STOP_R = crate::BitReader;
///Field `DBG_TIM12_STOP` writer - TIM12 stop in debug
pub type DBG_TIM12_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM13_STOP` reader - TIM13 stop in debug
pub type DBG_TIM13_STOP_R = crate::BitReader;
///Field `DBG_TIM13_STOP` writer - TIM13 stop in debug
pub type DBG_TIM13_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM14_STOP` reader - TIM14 stop in debug
pub type DBG_TIM14_STOP_R = crate::BitReader;
///Field `DBG_TIM14_STOP` writer - TIM14 stop in debug
pub type DBG_TIM14_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM1_STOP` reader - LPTIM1 stop in debug
pub type DBG_LPTIM1_STOP_R = crate::BitReader;
///Field `DBG_LPTIM1_STOP` writer - LPTIM1 stop in debug
pub type DBG_LPTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_WWDG1_STOP` reader - WWDG1 stop in debug
pub type DBG_WWDG1_STOP_R = crate::BitReader;
///Field `DBG_WWDG1_STOP` writer - WWDG1 stop in debug
pub type DBG_WWDG1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM10_STOP` reader - TIM10 stop in debug
pub type DBG_TIM10_STOP_R = crate::BitReader;
///Field `DBG_TIM10_STOP` writer - TIM10 stop in debug
pub type DBG_TIM10_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM11_STOP` reader - TIM11 stop in debug
pub type DBG_TIM11_STOP_R = crate::BitReader;
///Field `DBG_TIM11_STOP` writer - TIM11 stop in debug
pub type DBG_TIM11_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout stop in debug
pub type DBG_I2C1_STOP_R = crate::BitReader;
///Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout stop in debug
pub type DBG_I2C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C2_STOP` reader - I2C2 SMBUS timeout stop in debug
pub type DBG_I2C2_STOP_R = crate::BitReader;
///Field `DBG_I2C2_STOP` writer - I2C2 SMBUS timeout stop in debug
pub type DBG_I2C2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout stop in debug
pub type DBG_I2C3_STOP_R = crate::BitReader;
///Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout stop in debug
pub type DBG_I2C3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I3C1_STOP` reader - I3C1 SMBUS timeout stop in debug
pub type DBG_I3C1_STOP_R = crate::BitReader;
///Field `DBG_I3C1_STOP` writer - I3C1 SMBUS timeout stop in debug
pub type DBG_I3C1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_I3C2_STOP` reader - I3C2 SMBUS timeout stop in debug
pub type DBG_I3C2_STOP_R = crate::BitReader;
///Field `DBG_I3C2_STOP` writer - I3C2 SMBUS timeout stop in debug
pub type DBG_I3C2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 stop in debug
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 stop in debug
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 stop in debug
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 stop in debug
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 stop in debug
    #[inline(always)]
    pub fn dbg_tim6_stop(&self) -> DBG_TIM6_STOP_R {
        DBG_TIM6_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 stop in debug
    #[inline(always)]
    pub fn dbg_tim7_stop(&self) -> DBG_TIM7_STOP_R {
        DBG_TIM7_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIM12 stop in debug
    #[inline(always)]
    pub fn dbg_tim12_stop(&self) -> DBG_TIM12_STOP_R {
        DBG_TIM12_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TIM13 stop in debug
    #[inline(always)]
    pub fn dbg_tim13_stop(&self) -> DBG_TIM13_STOP_R {
        DBG_TIM13_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIM14 stop in debug
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LPTIM1 stop in debug
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - WWDG1 stop in debug
    #[inline(always)]
    pub fn dbg_wwdg1_stop(&self) -> DBG_WWDG1_STOP_R {
        DBG_WWDG1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TIM10 stop in debug
    #[inline(always)]
    pub fn dbg_tim10_stop(&self) -> DBG_TIM10_STOP_R {
        DBG_TIM10_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TIM11 stop in debug
    #[inline(always)]
    pub fn dbg_tim11_stop(&self) -> DBG_TIM11_STOP_R {
        DBG_TIM11_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 21 - I2C1 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c2_stop(&self) -> DBG_I2C2_STOP_R {
        DBG_I2C2_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - I3C1 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i3c1_stop(&self) -> DBG_I3C1_STOP_R {
        DBG_I3C1_STOP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - I3C2 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i3c2_stop(&self) -> DBG_I3C2_STOP_R {
        DBG_I3C2_STOP_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LFZ1")
            .field("dbg_tim2_stop", &self.dbg_tim2_stop())
            .field("dbg_tim3_stop", &self.dbg_tim3_stop())
            .field("dbg_tim4_stop", &self.dbg_tim4_stop())
            .field("dbg_tim5_stop", &self.dbg_tim5_stop())
            .field("dbg_tim6_stop", &self.dbg_tim6_stop())
            .field("dbg_tim7_stop", &self.dbg_tim7_stop())
            .field("dbg_tim12_stop", &self.dbg_tim12_stop())
            .field("dbg_tim13_stop", &self.dbg_tim13_stop())
            .field("dbg_tim14_stop", &self.dbg_tim14_stop())
            .field("dbg_lptim1_stop", &self.dbg_lptim1_stop())
            .field("dbg_wwdg1_stop", &self.dbg_wwdg1_stop())
            .field("dbg_tim10_stop", &self.dbg_tim10_stop())
            .field("dbg_tim11_stop", &self.dbg_tim11_stop())
            .field("dbg_i2c1_stop", &self.dbg_i2c1_stop())
            .field("dbg_i2c2_stop", &self.dbg_i2c2_stop())
            .field("dbg_i2c3_stop", &self.dbg_i2c3_stop())
            .field("dbg_i3c1_stop", &self.dbg_i3c1_stop())
            .field("dbg_i3c2_stop", &self.dbg_i3c2_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 stop in debug
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<'_, APB1LFZ1rs> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    ///Bit 1 - TIM3 stop in debug
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<'_, APB1LFZ1rs> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    ///Bit 2 - TIM4 stop in debug
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<'_, APB1LFZ1rs> {
        DBG_TIM4_STOP_W::new(self, 2)
    }
    ///Bit 3 - TIM5 stop in debug
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<'_, APB1LFZ1rs> {
        DBG_TIM5_STOP_W::new(self, 3)
    }
    ///Bit 4 - TIM6 stop in debug
    #[inline(always)]
    pub fn dbg_tim6_stop(&mut self) -> DBG_TIM6_STOP_W<'_, APB1LFZ1rs> {
        DBG_TIM6_STOP_W::new(self, 4)
    }
    ///Bit 5 - TIM7 stop in debug
    #[inline(always)]
    pub fn dbg_tim7_stop(&mut self) -> DBG_TIM7_STOP_W<'_, APB1LFZ1rs> {
        DBG_TIM7_STOP_W::new(self, 5)
    }
    ///Bit 6 - TIM12 stop in debug
    #[inline(always)]
    pub fn dbg_tim12_stop(&mut self) -> DBG_TIM12_STOP_W<'_, APB1LFZ1rs> {
        DBG_TIM12_STOP_W::new(self, 6)
    }
    ///Bit 7 - TIM13 stop in debug
    #[inline(always)]
    pub fn dbg_tim13_stop(&mut self) -> DBG_TIM13_STOP_W<'_, APB1LFZ1rs> {
        DBG_TIM13_STOP_W::new(self, 7)
    }
    ///Bit 8 - TIM14 stop in debug
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<'_, APB1LFZ1rs> {
        DBG_TIM14_STOP_W::new(self, 8)
    }
    ///Bit 9 - LPTIM1 stop in debug
    #[inline(always)]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<'_, APB1LFZ1rs> {
        DBG_LPTIM1_STOP_W::new(self, 9)
    }
    ///Bit 11 - WWDG1 stop in debug
    #[inline(always)]
    pub fn dbg_wwdg1_stop(&mut self) -> DBG_WWDG1_STOP_W<'_, APB1LFZ1rs> {
        DBG_WWDG1_STOP_W::new(self, 11)
    }
    ///Bit 12 - TIM10 stop in debug
    #[inline(always)]
    pub fn dbg_tim10_stop(&mut self) -> DBG_TIM10_STOP_W<'_, APB1LFZ1rs> {
        DBG_TIM10_STOP_W::new(self, 12)
    }
    ///Bit 13 - TIM11 stop in debug
    #[inline(always)]
    pub fn dbg_tim11_stop(&mut self) -> DBG_TIM11_STOP_W<'_, APB1LFZ1rs> {
        DBG_TIM11_STOP_W::new(self, 13)
    }
    ///Bit 21 - I2C1 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<'_, APB1LFZ1rs> {
        DBG_I2C1_STOP_W::new(self, 21)
    }
    ///Bit 22 - I2C2 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c2_stop(&mut self) -> DBG_I2C2_STOP_W<'_, APB1LFZ1rs> {
        DBG_I2C2_STOP_W::new(self, 22)
    }
    ///Bit 23 - I2C3 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<'_, APB1LFZ1rs> {
        DBG_I2C3_STOP_W::new(self, 23)
    }
    ///Bit 24 - I3C1 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i3c1_stop(&mut self) -> DBG_I3C1_STOP_W<'_, APB1LFZ1rs> {
        DBG_I3C1_STOP_W::new(self, 24)
    }
    ///Bit 25 - I3C2 SMBUS timeout stop in debug
    #[inline(always)]
    pub fn dbg_i3c2_stop(&mut self) -> DBG_I3C2_STOP_W<'_, APB1LFZ1rs> {
        DBG_I3C2_STOP_W::new(self, 25)
    }
}
/**DBGMCU APB1L peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb1lfz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DBGMCU:APB1LFZ1)*/
pub struct APB1LFZ1rs;
impl crate::RegisterSpec for APB1LFZ1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1lfz1::R`](R) reader structure
impl crate::Readable for APB1LFZ1rs {}
///`write(|w| ..)` method takes [`apb1lfz1::W`](W) writer structure
impl crate::Writable for APB1LFZ1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LFZ1 to value 0
impl crate::Resettable for APB1LFZ1rs {}
