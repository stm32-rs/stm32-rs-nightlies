///Register `APB5FZ1` reader
pub type R = crate::R<APB5FZ1rs>;
///Register `APB5FZ1` writer
pub type W = crate::W<APB5FZ1rs>;
///Field `DBG_GFXTIM_STOP` reader - GFXTIM stop in debug
pub type DBG_GFXTIM_STOP_R = crate::BitReader;
///Field `DBG_GFXTIM_STOP` writer - GFXTIM stop in debug
pub type DBG_GFXTIM_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - GFXTIM stop in debug
    #[inline(always)]
    pub fn dbg_gfxtim_stop(&self) -> DBG_GFXTIM_STOP_R {
        DBG_GFXTIM_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB5FZ1")
            .field("dbg_gfxtim_stop", &self.dbg_gfxtim_stop())
            .finish()
    }
}
impl W {
    ///Bit 4 - GFXTIM stop in debug
    #[inline(always)]
    pub fn dbg_gfxtim_stop(&mut self) -> DBG_GFXTIM_STOP_W<'_, APB5FZ1rs> {
        DBG_GFXTIM_STOP_W::new(self, 4)
    }
}
/**DBGMCU APB5 peripheral freeze register

You can [`read`](crate::Reg::read) this register and get [`apb5fz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5fz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DBGMCU:APB5FZ1)*/
pub struct APB5FZ1rs;
impl crate::RegisterSpec for APB5FZ1rs {
    type Ux = u32;
}
///`read()` method returns [`apb5fz1::R`](R) reader structure
impl crate::Readable for APB5FZ1rs {}
///`write(|w| ..)` method takes [`apb5fz1::W`](W) writer structure
impl crate::Writable for APB5FZ1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5FZ1 to value 0
impl crate::Resettable for APB5FZ1rs {}
