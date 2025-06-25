///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `SEC3` writer - I/O pin of port H secure bit enable 3 This bit is written by software to enabled the security I/O port pin.
pub type SEC3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<SECCFGRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - I/O pin of port H secure bit enable 3 This bit is written by software to enabled the security I/O port pin.
    #[inline(always)]
    pub fn sec3(&mut self) -> SEC3_W<SECCFGRrs> {
        SEC3_W::new(self, 3)
    }
}
/**GPIO port H secure configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#GPIOH:SECCFGR)*/
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
