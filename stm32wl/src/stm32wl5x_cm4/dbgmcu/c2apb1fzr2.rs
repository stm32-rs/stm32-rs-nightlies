///Register `C2APB1FZR2` reader
pub type R = crate::R<C2APB1FZR2rs>;
///Register `C2APB1FZR2` writer
pub type W = crate::W<C2APB1FZR2rs>;
///Field `DBG_LPTIM2_STOP` reader - DBG_LPTIM2_STOP
pub type DBG_LPTIM2_STOP_R = crate::BitReader;
///Field `DBG_LPTIM2_STOP` writer - DBG_LPTIM2_STOP
pub type DBG_LPTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_LPTIM3_STOP` reader - DBG_LPTIM3_STOP
pub type DBG_LPTIM3_STOP_R = crate::BitReader;
///Field `DBG_LPTIM3_STOP` writer - DBG_LPTIM3_STOP
pub type DBG_LPTIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - DBG_LPTIM2_STOP
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DBG_LPTIM3_STOP
    #[inline(always)]
    pub fn dbg_lptim3_stop(&self) -> DBG_LPTIM3_STOP_R {
        DBG_LPTIM3_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2APB1FZR2")
            .field("dbg_lptim2_stop", &self.dbg_lptim2_stop())
            .field("dbg_lptim3_stop", &self.dbg_lptim3_stop())
            .finish()
    }
}
impl W {
    ///Bit 5 - DBG_LPTIM2_STOP
    #[inline(always)]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<'_, C2APB1FZR2rs> {
        DBG_LPTIM2_STOP_W::new(self, 5)
    }
    ///Bit 6 - DBG_LPTIM3_STOP
    #[inline(always)]
    pub fn dbg_lptim3_stop(&mut self) -> DBG_LPTIM3_STOP_W<'_, C2APB1FZR2rs> {
        DBG_LPTIM3_STOP_W::new(self, 6)
    }
}
/**DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \[dual core device

You can [`read`](crate::Reg::read) this register and get [`c2apb1fzr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2apb1fzr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#DBGMCU:C2APB1FZR2)*/
pub struct C2APB1FZR2rs;
impl crate::RegisterSpec for C2APB1FZR2rs {
    type Ux = u32;
}
///`read()` method returns [`c2apb1fzr2::R`](R) reader structure
impl crate::Readable for C2APB1FZR2rs {}
///`write(|w| ..)` method takes [`c2apb1fzr2::W`](W) writer structure
impl crate::Writable for C2APB1FZR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2APB1FZR2 to value 0
impl crate::Resettable for C2APB1FZR2rs {}
