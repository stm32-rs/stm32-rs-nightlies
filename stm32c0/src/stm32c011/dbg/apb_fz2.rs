///Register `APB_FZ2` reader
pub type R = crate::R<APB_FZ2rs>;
///Register `APB_FZ2` writer
pub type W = crate::W<APB_FZ2rs>;
///Field `DBG_TIM1_STOP` reader - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
pub type DBG_TIM1_STOP_R = crate::BitReader;
///Field `DBG_TIM1_STOP` writer - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM14_STOP` reader - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
pub type DBG_TIM14_STOP_R = crate::BitReader;
///Field `DBG_TIM14_STOP` writer - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
pub type DBG_TIM14_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM16_STOP` reader - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
pub type DBG_TIM16_STOP_R = crate::BitReader;
///Field `DBG_TIM16_STOP` writer - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM17_STOP` reader - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
pub type DBG_TIM17_STOP_R = crate::BitReader;
///Field `DBG_TIM17_STOP` writer - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_FZ2")
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .field("dbg_tim14_stop", &self.dbg_tim14_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .field("dbg_tim17_stop", &self.dbg_tim17_stop())
            .finish()
    }
}
impl W {
    ///Bit 11 - Clocking of TIM1 counter when the core is halted This bit enables/disables the clock to the counter of TIM1 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<APB_FZ2rs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    ///Bit 15 - Clocking of TIM14 counter when the core is halted This bit enables/disables the clock to the counter of TIM14 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<APB_FZ2rs> {
        DBG_TIM14_STOP_W::new(self, 15)
    }
    ///Bit 17 - Clocking of TIM16 counter when the core is halted This bit enables/disables the clock to the counter of TIM16 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<APB_FZ2rs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    ///Bit 18 - Clocking of TIM17 counter when the core is halted This bit enables/disables the clock to the counter of TIM17 when the core is halted:
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<APB_FZ2rs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
}
/**DBG APB freeze register 2

You can [`read`](crate::Reg::read) this register and get [`apb_fz2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb_fz2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C011.html#DBG:APB_FZ2)*/
pub struct APB_FZ2rs;
impl crate::RegisterSpec for APB_FZ2rs {
    type Ux = u32;
}
///`read()` method returns [`apb_fz2::R`](R) reader structure
impl crate::Readable for APB_FZ2rs {}
///`write(|w| ..)` method takes [`apb_fz2::W`](W) writer structure
impl crate::Writable for APB_FZ2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APB_FZ2 to value 0
impl crate::Resettable for APB_FZ2rs {
    const RESET_VALUE: u32 = 0;
}
