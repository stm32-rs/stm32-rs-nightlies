///Register `PRFCR` writer
pub type W = crate::W<PRFCRrs>;
///Field `CERRF` writer - Synchronization error interrupt status clear
pub type CERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PRFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 6 - Synchronization error interrupt status clear
    #[inline(always)]
    pub fn cerrf(&mut self) -> CERRF_W<'_, PRFCRrs> {
        CERRF_W::new(self, 6)
    }
}
/**DCMIPP parallel interface interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prfcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:PRFCR)*/
pub struct PRFCRrs;
impl crate::RegisterSpec for PRFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`prfcr::W`](W) writer structure
impl crate::Writable for PRFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRFCR to value 0
impl crate::Resettable for PRFCRrs {}
