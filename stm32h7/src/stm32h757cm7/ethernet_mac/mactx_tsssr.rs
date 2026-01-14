///Register `MACTxTSSSR` reader
pub type R = crate::R<MACTX_TSSSRrs>;
///Field `TXTSSHI` reader - Transmit Timestamp Status High
pub type TXTSSHI_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Transmit Timestamp Status High
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
/**Tx timestamp status seconds register

You can [`read`](crate::Reg::read) this register and get [`mactx_tsssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#Ethernet_MAC:MACTxTSSSR)*/
pub struct MACTX_TSSSRrs;
impl crate::RegisterSpec for MACTX_TSSSRrs {
    type Ux = u32;
}
///`read()` method returns [`mactx_tsssr::R`](R) reader structure
impl crate::Readable for MACTX_TSSSRrs {}
///`reset()` method sets MACTxTSSSR to value 0
impl crate::Resettable for MACTX_TSSSRrs {}
