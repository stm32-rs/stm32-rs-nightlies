///Register `APB1FZR2` reader
pub type R = crate::R<APB1FZR2rs>;
///Register `APB1FZR2` writer
pub type W = crate::W<APB1FZR2rs>;
///Field `DBG_LPTIM2_STOP` reader - LPTIM2 counter stopped when core is halted
pub type DBG_LPTIM2_STOP_R = crate::BitReader;
///Field `DBG_LPTIM2_STOP` writer - LPTIM2 counter stopped when core is halted
pub type DBG_LPTIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - LPTIM2 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1FZR2")
            .field("dbg_lptim2_stop", &self.dbg_lptim2_stop())
            .finish()
    }
}
impl W {
    ///Bit 5 - LPTIM2 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<'_, APB1FZR2rs> {
        DBG_LPTIM2_STOP_W::new(self, 5)
    }
}
/**APB1 High Freeze Register CPU1

You can [`read`](crate::Reg::read) this register and get [`apb1fzr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1fzr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DBGMCU:APB1FZR2)*/
pub struct APB1FZR2rs;
impl crate::RegisterSpec for APB1FZR2rs {
    type Ux = u32;
}
///`read()` method returns [`apb1fzr2::R`](R) reader structure
impl crate::Readable for APB1FZR2rs {}
///`write(|w| ..)` method takes [`apb1fzr2::W`](W) writer structure
impl crate::Writable for APB1FZR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1FZR2 to value 0
impl crate::Resettable for APB1FZR2rs {}
