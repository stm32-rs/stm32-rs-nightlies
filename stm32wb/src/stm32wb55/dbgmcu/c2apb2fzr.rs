///Register `C2APB2FZR` reader
pub type R = crate::R<C2APB2FZRrs>;
///Register `C2APB2FZR` writer
pub type W = crate::W<C2APB2FZRrs>;
///Field `DBG_TIM1_STOP` reader - TIM1 counter stopped when core is halted
pub type DBG_TIM1_STOP_R = crate::BitReader;
///Field `DBG_TIM1_STOP` writer - TIM1 counter stopped when core is halted
pub type DBG_TIM1_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM16_STOP` reader - TIM16 counter stopped when core is halted
pub type DBG_TIM16_STOP_R = crate::BitReader;
///Field `DBG_TIM16_STOP` writer - TIM16 counter stopped when core is halted
pub type DBG_TIM16_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM17_STOP` reader - TIM17 counter stopped when core is halted
pub type DBG_TIM17_STOP_R = crate::BitReader;
///Field `DBG_TIM17_STOP` writer - TIM17 counter stopped when core is halted
pub type DBG_TIM17_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 11 - TIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB2FZR")
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .field("dbg_tim16_stop", &self.dbg_tim16_stop())
            .field("dbg_tim17_stop", &self.dbg_tim17_stop())
            .finish()
    }
}
impl W {
    ///Bit 11 - TIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<'_, C2APB2FZRrs> {
        DBG_TIM1_STOP_W::new(self, 11)
    }
    ///Bit 17 - TIM16 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<'_, C2APB2FZRrs> {
        DBG_TIM16_STOP_W::new(self, 17)
    }
    ///Bit 18 - TIM17 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<'_, C2APB2FZRrs> {
        DBG_TIM17_STOP_W::new(self, 18)
    }
}
/**APB2 Freeze Register CPU2

You can [`read`](crate::Reg::read) this register and get [`c2apb2fzr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb2fzr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DBGMCU:C2APB2FZR)*/
pub struct C2APB2FZRrs;
impl crate::RegisterSpec for C2APB2FZRrs {
    type Ux = u32;
}
///`read()` method returns [`c2apb2fzr::R`](R) reader structure
impl crate::Readable for C2APB2FZRrs {}
///`write(|w| ..)` method takes [`c2apb2fzr::W`](W) writer structure
impl crate::Writable for C2APB2FZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2APB2FZR to value 0
impl crate::Resettable for C2APB2FZRrs {}
