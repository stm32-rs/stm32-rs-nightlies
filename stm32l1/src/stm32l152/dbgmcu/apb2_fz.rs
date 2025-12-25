///Register `APB2_FZ` reader
pub type R = crate::R<APB2_FZrs>;
///Register `APB2_FZ` writer
pub type W = crate::W<APB2_FZrs>;
///Field `DBG_TIM9_STOP` reader - TIM counter stopped when core is halted
pub type DBG_TIM9_STOP_R = crate::BitReader;
///Field `DBG_TIM9_STOP` writer - TIM counter stopped when core is halted
pub type DBG_TIM9_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM10_STOP` reader - TIM counter stopped when core is halted
pub type DBG_TIM10_STOP_R = crate::BitReader;
///Field `DBG_TIM10_STOP` writer - TIM counter stopped when core is halted
pub type DBG_TIM10_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM11_STOP` reader - TIM counter stopped when core is halted
pub type DBG_TIM11_STOP_R = crate::BitReader;
///Field `DBG_TIM11_STOP` writer - TIM counter stopped when core is halted
pub type DBG_TIM11_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - TIM counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim9_stop(&self) -> DBG_TIM9_STOP_R {
        DBG_TIM9_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim10_stop(&self) -> DBG_TIM10_STOP_R {
        DBG_TIM10_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim11_stop(&self) -> DBG_TIM11_STOP_R {
        DBG_TIM11_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2_FZ")
            .field("dbg_tim9_stop", &self.dbg_tim9_stop())
            .field("dbg_tim10_stop", &self.dbg_tim10_stop())
            .field("dbg_tim11_stop", &self.dbg_tim11_stop())
            .finish()
    }
}
impl W {
    ///Bit 2 - TIM counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim9_stop(&mut self) -> DBG_TIM9_STOP_W<'_, APB2_FZrs> {
        DBG_TIM9_STOP_W::new(self, 2)
    }
    ///Bit 3 - TIM counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim10_stop(&mut self) -> DBG_TIM10_STOP_W<'_, APB2_FZrs> {
        DBG_TIM10_STOP_W::new(self, 3)
    }
    ///Bit 4 - TIM counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim11_stop(&mut self) -> DBG_TIM11_STOP_W<'_, APB2_FZrs> {
        DBG_TIM11_STOP_W::new(self, 4)
    }
}
/**Debug MCU APB1 freeze register 2

You can [`read`](crate::Reg::read) this register and get [`apb2_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L152.html#DBGMCU:APB2_FZ)*/
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
