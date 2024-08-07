///Register `ETH_MACTSICNR` reader
pub type R = crate::R<ETH_MACTSICNRrs>;
///Register `ETH_MACTSICNR` writer
pub type W = crate::W<ETH_MACTSICNRrs>;
///Field `TSIC` reader - TSIC
pub type TSIC_R = crate::FieldReader<u32>;
///Field `TSIC` writer - TSIC
pub type TSIC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - TSIC
    #[inline(always)]
    pub fn tsic(&self) -> TSIC_R {
        TSIC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACTSICNR")
            .field("tsic", &self.tsic())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - TSIC
    #[inline(always)]
    #[must_use]
    pub fn tsic(&mut self) -> TSIC_W<ETH_MACTSICNRrs> {
        TSIC_W::new(self, 0)
    }
}
/**This register contains the correction value in nanoseconds to be used with the captured timestamp value in the ingress path.

You can [`read`](crate::Reg::read) this register and get [`eth_mactsicnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_mactsicnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:ETH_MACTSICNR)*/
pub struct ETH_MACTSICNRrs;
impl crate::RegisterSpec for ETH_MACTSICNRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_mactsicnr::R`](R) reader structure
impl crate::Readable for ETH_MACTSICNRrs {}
///`write(|w| ..)` method takes [`eth_mactsicnr::W`](W) writer structure
impl crate::Writable for ETH_MACTSICNRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACTSICNR to value 0
impl crate::Resettable for ETH_MACTSICNRrs {
    const RESET_VALUE: u32 = 0;
}
