///Register `MACTxTSSSR` reader
pub type R = crate::R<MACTX_TSSSRrs>;
///Field `TXTSSHI` reader - TXTSSHI
pub type TXTSSHI_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - TXTSSHI
    #[inline(always)]
    pub fn txtsshi(&self) -> TXTSSHI_R {
        TXTSSHI_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTxTSSSR")
            .field("txtsshi", &self.txtsshi())
            .finish()
    }
}
/**The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted.

You can [`read`](crate::Reg::read) this register and get [`mactx_tsssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTxTSSSR)*/
pub struct MACTX_TSSSRrs;
impl crate::RegisterSpec for MACTX_TSSSRrs {
    type Ux = u32;
}
///`read()` method returns [`mactx_tsssr::R`](R) reader structure
impl crate::Readable for MACTX_TSSSRrs {}
///`reset()` method sets MACTxTSSSR to value 0
impl crate::Resettable for MACTX_TSSSRrs {}
