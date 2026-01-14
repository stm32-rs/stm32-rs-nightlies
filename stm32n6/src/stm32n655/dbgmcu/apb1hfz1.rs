///Register `APB1HFZ1` reader
pub type R = crate::R<APB1HFZ1rs>;
///Register `APB1HFZ1` writer
pub type W = crate::W<APB1HFZ1rs>;
///Field `DBG_FDCAN_STOP` reader - FDCAN stop in debug
pub type DBG_FDCAN_STOP_R = crate::BitReader;
///Field `DBG_FDCAN_STOP` writer - FDCAN stop in debug
pub type DBG_FDCAN_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - FDCAN stop in debug
    #[inline(always)]
    pub fn dbg_fdcan_stop(&self) -> DBG_FDCAN_STOP_R {
        DBG_FDCAN_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HFZ1")
            .field("dbg_fdcan_stop", &self.dbg_fdcan_stop())
            .finish()
    }
}
impl W {
    ///Bit 8 - FDCAN stop in debug
    #[inline(always)]
    pub fn dbg_fdcan_stop(&mut self) -> DBG_FDCAN_STOP_W<'_, APB1HFZ1rs> {
        DBG_FDCAN_STOP_W::new(self, 8)
    }
}
/**DBGMCU APB1H peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb1hfz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hfz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DBGMCU:APB1HFZ1)*/
pub struct APB1HFZ1rs;
impl crate::RegisterSpec for APB1HFZ1rs {
    type Ux = u32;
}
///`read()` method returns [`apb1hfz1::R`](R) reader structure
impl crate::Readable for APB1HFZ1rs {}
///`write(|w| ..)` method takes [`apb1hfz1::W`](W) writer structure
impl crate::Writable for APB1HFZ1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HFZ1 to value 0
impl crate::Resettable for APB1HFZ1rs {}
