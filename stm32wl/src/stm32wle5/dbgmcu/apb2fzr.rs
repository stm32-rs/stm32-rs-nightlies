///Register `APB2FZR` reader
pub type R = crate::R<APB2FZRrs>;
///Register `APB2FZR` writer
pub type W = crate::W<APB2FZRrs>;
/**DBG_TIM1_STOP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_TIM1_STOP {
    ///0: The counter clock of TIMx is fed even if the core is halted
    Continue = 0,
    ///1: The counter clock of TIMx is stopped when the core is halted
    Stop = 1,
}
impl From<DBG_TIM1_STOP> for bool {
    #[inline(always)]
    fn from(variant: DBG_TIM1_STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG_TIM1_STOP` reader - DBG_TIM1_STOP
pub type DBG_TIM1_STOP_R = crate::BitReader<DBG_TIM1_STOP>;
impl DBG_TIM1_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG_TIM1_STOP {
        match self.bits {
            false => DBG_TIM1_STOP::Continue,
            true => DBG_TIM1_STOP::Stop,
        }
    }
    ///The counter clock of TIMx is fed even if the core is halted
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        *self == DBG_TIM1_STOP::Continue
    }
    ///The counter clock of TIMx is stopped when the core is halted
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == DBG_TIM1_STOP::Stop
    }
}
///Field `DBG_TIM1_STOP` writer - DBG_TIM1_STOP
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG, DBG_TIM1_STOP>;
impl<'a, REG> DBG_TIM1_STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The counter clock of TIMx is fed even if the core is halted
    #[inline(always)]
    pub fn continue_(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM1_STOP::Continue)
    }
    ///The counter clock of TIMx is stopped when the core is halted
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_TIM1_STOP::Stop)
    }
}
///Field `DBG_TIM16_STOP` reader - DBG_TIM16_STOP
pub use DBG_TIM1_STOP_R as DBG_TIM16_STOP_R;
///Field `DBG_TIM17_STOP` reader - DBG_TIM17_STOP
pub use DBG_TIM1_STOP_R as DBG_TIM17_STOP_R;
///Field `DBG_TIM16_STOP` writer - DBG_TIM16_STOP
pub use DBG_TIM1_STOP_W as DBG_TIM16_STOP_W;
///Field `DBG_TIM17_STOP` writer - DBG_TIM17_STOP
pub use DBG_TIM1_STOP_W as DBG_TIM17_STOP_W;
impl R {
    ///Bit 11 - DBG_TIM1_STOP
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 17 - DBG_TIM16_STOP
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DBG_TIM17_STOP
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2FZR")
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .field("dbg_tim17_stop", &self.dbg_tim17_stop())
            .finish()
    }
}
impl W {
    ///Bit 11 - DBG_TIM1_STOP
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<APB2FZRrs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    ///Bit 17 - DBG_TIM16_STOP
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<APB2FZRrs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    ///Bit 18 - DBG_TIM17_STOP
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<APB2FZRrs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
}
/**DBGMCU CPU1 APB2 Peripheral Freeze Register

You can [`read`](crate::Reg::read) this register and get [`apb2fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#DBGMCU:APB2FZR)*/
pub struct APB2FZRrs;
impl crate::RegisterSpec for APB2FZRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2fzr::R`](R) reader structure
impl crate::Readable for APB2FZRrs {}
///`write(|w| ..)` method takes [`apb2fzr::W`](W) writer structure
impl crate::Writable for APB2FZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2FZR to value 0
impl crate::Resettable for APB2FZRrs {}
