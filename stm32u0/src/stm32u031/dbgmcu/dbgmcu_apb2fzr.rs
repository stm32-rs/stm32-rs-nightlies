///Register `DBGMCU_APB2FZR` reader
pub type R = crate::R<DBGMCU_APB2FZRrs>;
///Register `DBGMCU_APB2FZR` writer
pub type W = crate::W<DBGMCU_APB2FZRrs>;
///Field `DBG_TIM1_STOP` reader - TIM1 stop in debug
pub type DBG_TIM1_STOP_R = crate::BitReader;
///Field `DBG_TIM1_STOP` writer - TIM1 stop in debug
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM14_STOP` reader - TIM14 stop in debug
pub type DBG_TIM14_STOP_R = crate::BitReader;
///Field `DBG_TIM14_STOP` writer - TIM14 stop in debug
pub type DBG_TIM14_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM15_STOP` reader - TIM15 stop in debug
pub type DBG_TIM15_STOP_R = crate::BitReader;
///Field `DBG_TIM15_STOP` writer - TIM15 stop in debug
pub type DBG_TIM15_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM16_STOP` reader - TIM16 stop in debug
pub type DBG_TIM16_STOP_R = crate::BitReader;
///Field `DBG_TIM16_STOP` writer - TIM16 stop in debug
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 11 - TIM1 stop in debug
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - TIM14 stop in debug
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DBG_TIM14_STOP_R {
        DBG_TIM14_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TIM15 stop in debug
    #[inline(always)]
    pub fn dbg_tim15_stop(&self) -> DBG_TIM15_STOP_R {
        DBG_TIM15_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 stop in debug
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_APB2FZR")
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .field("dbg_tim14_stop", &self.dbg_tim14_stop())
            .field("dbg_tim15_stop", &self.dbg_tim15_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .finish()
    }
}
impl W {
    ///Bit 11 - TIM1 stop in debug
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<DBGMCU_APB2FZRrs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    ///Bit 15 - TIM14 stop in debug
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DBG_TIM14_STOP_W<DBGMCU_APB2FZRrs> {
        DBG_TIM14_STOP_W::new(self, 15)
    }
    ///Bit 16 - TIM15 stop in debug
    #[inline(always)]
    pub fn dbg_tim15_stop(&mut self) -> DBG_TIM15_STOP_W<DBGMCU_APB2FZRrs> {
        DBG_TIM15_STOP_W::new(self, 16)
    }
    ///Bit 17 - TIM16 stop in debug
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<DBGMCU_APB2FZRrs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
}
/**DBG APB2 freeze register

You can [`read`](crate::Reg::read) this register and get [`dbgmcu_apb2fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_apb2fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#DBGMCU:DBGMCU_APB2FZR)*/
pub struct DBGMCU_APB2FZRrs;
impl crate::RegisterSpec for DBGMCU_APB2FZRrs {
    type Ux = u32;
}
///`read()` method returns [`dbgmcu_apb2fzr::R`](R) reader structure
impl crate::Readable for DBGMCU_APB2FZRrs {}
///`write(|w| ..)` method takes [`dbgmcu_apb2fzr::W`](W) writer structure
impl crate::Writable for DBGMCU_APB2FZRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBGMCU_APB2FZR to value 0
impl crate::Resettable for DBGMCU_APB2FZRrs {
    const RESET_VALUE: u32 = 0;
}