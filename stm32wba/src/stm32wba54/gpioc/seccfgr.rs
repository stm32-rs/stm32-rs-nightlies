///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `SEC13` writer - I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.
pub type SEC13_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC14` writer - I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.
pub type SEC14_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC15` writer - I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.
pub type SEC15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SECCFGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 13 - I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.
    #[inline(always)]
    pub fn sec13(&mut self) -> SEC13_W<SECCFGRrs> {
        SEC13_W::new(self, 13)
    }
    ///Bit 14 - I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.
    #[inline(always)]
    pub fn sec14(&mut self) -> SEC14_W<SECCFGRrs> {
        SEC14_W::new(self, 14)
    }
    ///Bit 15 - I/O pin of port C secure bit enable y These bits are written by software to enabled the security I/O port pin.
    #[inline(always)]
    pub fn sec15(&mut self) -> SEC15_W<SECCFGRrs> {
        SEC15_W::new(self, 15)
    }
}
/**GPIO port C secure configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GPIOC:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0xffff
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0xffff;
}
