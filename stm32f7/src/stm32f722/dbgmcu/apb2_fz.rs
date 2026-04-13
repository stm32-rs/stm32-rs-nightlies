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
///Field `DBG_TIM9_STOP` reader - TIM9 counter stopped when core is halted
pub type DBG_TIM9_STOP_R = crate::BitReader;
///Field `DBG_TIM9_STOP` writer - TIM9 counter stopped when core is halted
pub type DBG_TIM9_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM10_STOP` reader - TIM10 counter stopped when core is halted
pub type DBG_TIM10_STOP_R = crate::BitReader;
///Field `DBG_TIM10_STOP` writer - TIM10 counter stopped when core is halted
pub type DBG_TIM10_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_TIM11_STOP` reader - TIM11 counter stopped when core is halted
pub type DBG_TIM11_STOP_R = crate::BitReader;
///Field `DBG_TIM11_STOP` writer - TIM11 counter stopped when core is halted
pub type DBG_TIM11_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim8_stop(&self) -> DBG_TIM8_STOP_R {
        DBG_TIM8_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - TIM9 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim9_stop(&self) -> DBG_TIM9_STOP_R {
        DBG_TIM9_STOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM10 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim10_stop(&self) -> DBG_TIM10_STOP_R {
        DBG_TIM10_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM11 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim11_stop(&self) -> DBG_TIM11_STOP_R {
        DBG_TIM11_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2_FZ")
            .field("dbg_tim1_stop", &self.dbg_tim1_stop())
            .field("dbg_tim8_stop", &self.dbg_tim8_stop())
            .field("dbg_tim9_stop", &self.dbg_tim9_stop())
            .field("dbg_tim10_stop", &self.dbg_tim10_stop())
            .field("dbg_tim11_stop", &self.dbg_tim11_stop())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<'_, APB2_FZrs> {
        DBG_TIM1_STOP_W::new(self, 0)
    }
    ///Bit 1 - TIM8 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim8_stop(&mut self) -> DBG_TIM8_STOP_W<'_, APB2_FZrs> {
        DBG_TIM8_STOP_W::new(self, 1)
    }
    ///Bit 16 - TIM9 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim9_stop(&mut self) -> DBG_TIM9_STOP_W<'_, APB2_FZrs> {
        DBG_TIM9_STOP_W::new(self, 16)
    }
    ///Bit 17 - TIM10 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim10_stop(&mut self) -> DBG_TIM10_STOP_W<'_, APB2_FZrs> {
        DBG_TIM10_STOP_W::new(self, 17)
    }
    ///Bit 18 - TIM11 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_tim11_stop(&mut self) -> DBG_TIM11_STOP_W<'_, APB2_FZrs> {
        DBG_TIM11_STOP_W::new(self, 18)
    }
}
/**Debug MCU APB2 Freeze registe

You can [`read`](crate::Reg::read) this register and get [`apb2_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F722.html#DBGMCU:APB2_FZ)*/
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
