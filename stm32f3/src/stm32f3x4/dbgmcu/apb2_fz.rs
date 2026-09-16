///Register `APB2_FZ` reader
pub type R = crate::R<APB2_FZrs>;
///Register `APB2_FZ` writer
pub type W = crate::W<APB2_FZrs>;
///Field `DBG_TIM15_STOP` reader - Debug Timer 15 stopped when Core is halted
pub type DBG_TIM15_STOP_R = crate::BitReader;
///Field `DBG_TIM15_STOP` writer - Debug Timer 15 stopped when Core is halted
pub type DBG_TIM15_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM16_STOP` reader - Debug Timer 16 stopped when Core is halted
pub type DBG_TIM16_STOP_R = crate::BitReader;
///Field `DBG_TIM16_STOP` writer - Debug Timer 16 stopped when Core is halted
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM17_STO` reader - Debug Timer 17 stopped when Core is halted
pub type DBG_TIM17_STO_R = crate::BitReader;
///Field `DBG_TIM17_STO` writer - Debug Timer 17 stopped when Core is halted
pub type DBG_TIM17_STO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM19_STOP` reader - Debug Timer 19 stopped when Core is halted
pub type DBG_TIM19_STOP_R = crate::BitReader;
///Field `DBG_TIM19_STOP` writer - Debug Timer 19 stopped when Core is halted
pub type DBG_TIM19_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - Debug Timer 15 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Debug Timer 16 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Debug Timer 17 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_tim17_sto(&self) -> DBG_TIM17_STO_R {
        DBG_TIM17_STO_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Debug Timer 19 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_tim19_stop(&self) -> DBG_TIM19_STOP_R {
        DBG_TIM19_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2_FZ")
            .field("dbg_tim15_stop", &self.dbg_tim15_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .field("dbg_tim17_sto", &self.dbg_tim17_sto())
            .field("dbg_tim19_stop", &self.dbg_tim19_stop())
            .finish()
    }
}
impl W {
    ///Bit 2 - Debug Timer 15 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<'_, APB2_FZrs> {
        DBG_TIM15_STOP_W::new(self, 2)
    }
    ///Bit 3 - Debug Timer 16 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<'_, APB2_FZrs> {
        DBG_TIM16_STOP_W::new(self, 3)
    }
    ///Bit 4 - Debug Timer 17 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_tim17_sto(&mut self) -> DBG_TIM17_STO_W<'_, APB2_FZrs> {
        DBG_TIM17_STO_W::new(self, 4)
    }
    ///Bit 5 - Debug Timer 19 stopped when Core is halted
    #[inline(always)]
    pub fn dbg_tim19_stop(&mut self) -> DBG_TIM19_STOP_W<'_, APB2_FZrs> {
        DBG_TIM19_STOP_W::new(self, 5)
    }
}
/**APB High Freeze Register

You can [`read`](crate::Reg::read) this register and get [`apb2_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#DBGMCU:APB2_FZ)*/
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
