///Register `APB2FZR` reader
pub type R = crate::R<APB2FZRrs>;
///Register `APB2FZR` writer
pub type W = crate::W<APB2FZRrs>;
///Field `DBG_TIM1_STOP` reader - TIM1 stop in debug
pub type DBG_TIM1_STOP_R = crate::BitReader;
///Field `DBG_TIM1_STOP` writer - TIM1 stop in debug
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 11 - TIM1 stop in debug
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2FZR")
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .finish()
    }
}
impl W {
    ///Bit 11 - TIM1 stop in debug
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<'_, APB2FZRrs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
}
/**DBGMCU APB2 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb2fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#DBGMCU:APB2FZR)*/
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
