///Register `MACTSECNR` reader
pub type R = crate::R<MACTSECNRrs>;
///Register `MACTSECNR` writer
pub type W = crate::W<MACTSECNRrs>;
///Field `TSEC` reader - TSEC
pub type TSEC_R = crate::FieldReader<u32>;
///Field `TSEC` writer - TSEC
pub type TSEC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - TSEC
    #[inline(always)]
    pub fn tsec(&self) -> TSEC_R {
        TSEC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTSECNR")
            .field("tsec", &self.tsec())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - TSEC
    #[inline(always)]
    pub fn tsec(&mut self) -> TSEC_W<MACTSECNRrs> {
        TSEC_W::new(self, 0)
    }
}
/**This register contains the correction value in nanoseconds to be used with the captured timestamp value in the egress path.

You can [`read`](crate::Reg::read) this register and get [`mactsecnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsecnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACTSECNR)*/
pub struct MACTSECNRrs;
impl crate::RegisterSpec for MACTSECNRrs {
    type Ux = u32;
}
///`read()` method returns [`mactsecnr::R`](R) reader structure
impl crate::Readable for MACTSECNRrs {}
///`write(|w| ..)` method takes [`mactsecnr::W`](W) writer structure
impl crate::Writable for MACTSECNRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACTSECNR to value 0
impl crate::Resettable for MACTSECNRrs {}
