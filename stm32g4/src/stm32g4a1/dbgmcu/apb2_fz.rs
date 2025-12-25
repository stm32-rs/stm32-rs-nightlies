///Register `APB2_FZ` reader
pub type R = crate::R<APB2_FZrs>;
///Register `APB2_FZ` writer
pub type W = crate::W<APB2_FZrs>;
///Field `DBG_TIM1_STOP` reader - TIM1 counter stopped when core is halted
pub type DBG_TIM1_STOP_R = crate::BitReader;
///Field `DBG_TIM1_STOP` writer - TIM1 counter stopped when core is halted
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM8_STOP` reader - TIM8 counter stopped when core is halted
pub type DBG_TIM8_STOP_R = crate::BitReader;
///Field `DBG_TIM8_STOP` writer - TIM8 counter stopped when core is halted
pub type DBG_TIM8_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `DBG_TIM20_STOP` reader - TIM20counter stopped when core is halted
pub type DBG_TIM20_STOP_R = crate::BitReader;
///Field `DBG_TIM20_STOP` writer - TIM20counter stopped when core is halted
pub type DBG_TIM20_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HRTIM0_STOP` reader - DBG_HRTIM0_STOP
pub type DBG_HRTIM0_STOP_R = crate::BitReader;
///Field `DBG_HRTIM0_STOP` writer - DBG_HRTIM0_STOP
pub type DBG_HRTIM0_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HRTIM1_STOP` reader - DBG_HRTIM0_STOP
pub type DBG_HRTIM1_STOP_R = crate::BitReader;
///Field `DBG_HRTIM1_STOP` writer - DBG_HRTIM0_STOP
pub type DBG_HRTIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HRTIM2_STOP` reader - DBG_HRTIM0_STOP
pub type DBG_HRTIM2_STOP_R = crate::BitReader;
///Field `DBG_HRTIM2_STOP` writer - DBG_HRTIM0_STOP
pub type DBG_HRTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_HRTIM3_STOP` reader - DBG_HRTIM0_STOP
pub type DBG_HRTIM3_STOP_R = crate::BitReader;
///Field `DBG_HRTIM3_STOP` writer - DBG_HRTIM0_STOP
pub type DBG_HRTIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 11 - TIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - TIM8 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DBG_TIM8_STOP_R {
        DBG_TIM8_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - TIM15 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - TIM20counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim20_stop(&self) -> DBG_TIM20_STOP_R {
        DBG_TIM20_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 26 - DBG_HRTIM0_STOP
    #[inline(always)]
    pub fn dbg_hrtim0_stop(&self) -> DBG_HRTIM0_STOP_R {
        DBG_HRTIM0_STOP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DBG_HRTIM0_STOP
    #[inline(always)]
    pub fn dbg_hrtim1_stop(&self) -> DBG_HRTIM1_STOP_R {
        DBG_HRTIM1_STOP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - DBG_HRTIM0_STOP
    #[inline(always)]
    pub fn dbg_hrtim2_stop(&self) -> DBG_HRTIM2_STOP_R {
        DBG_HRTIM2_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DBG_HRTIM0_STOP
    #[inline(always)]
    pub fn dbg_hrtim3_stop(&self) -> DBG_HRTIM3_STOP_R {
        DBG_HRTIM3_STOP_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2_FZ")
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .field("dbg_tim8_stop", &self.dbg_tim8_stop())
            .field("dbg_tim15_stop", &self.dbg_tim15_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .field("dbg_tim17_stop", &self.dbg_tim17_stop())
            .field("dbg_tim20_stop", &self.dbg_tim20_stop())
            .field("dbg_hrtim0_stop", &self.dbg_hrtim0_stop())
            .field("dbg_hrtim1_stop", &self.dbg_hrtim1_stop())
            .field("dbg_hrtim2_stop", &self.dbg_hrtim2_stop())
            .field("dbg_hrtim3_stop", &self.dbg_hrtim3_stop())
            .finish()
    }
}
impl W {
    ///Bit 11 - TIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<'_, APB2_FZrs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    ///Bit 13 - TIM8 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim8_stop(&mut self) -> DBG_TIM8_STOP_W<'_, APB2_FZrs> {
        DBG_TIM8_STOP_W::new(self, 13)
    }
    ///Bit 16 - TIM15 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<'_, APB2_FZrs> {
        DBG_TIM15_STOP_W::new(self, 16)
    }
    ///Bit 17 - TIM16 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<'_, APB2_FZrs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    ///Bit 18 - TIM17 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<'_, APB2_FZrs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
    ///Bit 20 - TIM20counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim20_stop(&mut self) -> DBG_TIM20_STOP_W<'_, APB2_FZrs> {
        DBG_TIM20_STOP_W::new(self, 20)
    }
    ///Bit 26 - DBG_HRTIM0_STOP
    #[inline(always)]
    pub fn dbg_hrtim0_stop(&mut self) -> DBG_HRTIM0_STOP_W<'_, APB2_FZrs> {
        DBG_HRTIM0_STOP_W::new(self, 26)
    }
    ///Bit 27 - DBG_HRTIM0_STOP
    #[inline(always)]
    pub fn dbg_hrtim1_stop(&mut self) -> DBG_HRTIM1_STOP_W<'_, APB2_FZrs> {
        DBG_HRTIM1_STOP_W::new(self, 27)
    }
    ///Bit 28 - DBG_HRTIM0_STOP
    #[inline(always)]
    pub fn dbg_hrtim2_stop(&mut self) -> DBG_HRTIM2_STOP_W<'_, APB2_FZrs> {
        DBG_HRTIM2_STOP_W::new(self, 28)
    }
    ///Bit 29 - DBG_HRTIM0_STOP
    #[inline(always)]
    pub fn dbg_hrtim3_stop(&mut self) -> DBG_HRTIM3_STOP_W<'_, APB2_FZrs> {
        DBG_HRTIM3_STOP_W::new(self, 29)
    }
}
/**APB High Freeze Register

You can [`read`](crate::Reg::read) this register and get [`apb2_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G4A1.html#DBGMCU:APB2_FZ)*/
pub struct APB2_FZrs;
impl crate::RegisterSpec for APB2_FZrs {
    type Ux = u32;
}
///`read()` method returns [`apb2_fz::R`](R) reader structure
impl crate::Readable for APB2_FZrs {}
///`write(|w| ..)` method takes [`apb2_fz::W`](W) writer structure
impl crate::Writable for APB2_FZrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2_FZ to value 0
impl crate::Resettable for APB2_FZrs {}
