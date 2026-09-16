///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `C2EWILA` reader - wakeup on CPU2 illegal access interrupt enable
pub type C2EWILA_R = crate::BitReader;
///Field `C2EWILA` writer - wakeup on CPU2 illegal access interrupt enable
pub type C2EWILA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 15 - wakeup on CPU2 illegal access interrupt enable
    #[inline(always)]
    pub fn c2ewila(&self) -> C2EWILA_R {
        C2EWILA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("c2ewila", &self.c2ewila())
            .finish()
    }
}
impl W {
    ///Bit 15 - wakeup on CPU2 illegal access interrupt enable
    #[inline(always)]
    pub fn c2ewila(&mut self) -> C2EWILA_W<'_, SECCFGRrs> {
        C2EWILA_W::new(self, 15)
    }
}
/**Power security configuration register \[dual core device only\]

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#PWR:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0x8000
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0x8000;
}
