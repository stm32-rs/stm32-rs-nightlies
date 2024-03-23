#[doc = "Register `ETH_MACTxTSSSR` reader"]
pub type R = crate::R<ETH_MACTX_TSSSRrs>;
#[doc = "Field `TXTSSHI` reader - TXTSSHI"]
pub type TXTSSHI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXTSSHI"]
    #[inline(always)]
    pub fn txtsshi(&self) -> TXTSSHI_R {
        TXTSSHI_R::new(self.bits)
    }
}
#[doc = "The register contains the higher 32 bits of the timestamp (in seconds) captured when a PTP packet is transmitted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactx_tsssr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACTX_TSSSRrs;
impl crate::RegisterSpec for ETH_MACTX_TSSSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mactx_tsssr::R`](R) reader structure"]
impl crate::Readable for ETH_MACTX_TSSSRrs {}
#[doc = "`reset()` method sets ETH_MACTxTSSSR to value 0"]
impl crate::Resettable for ETH_MACTX_TSSSRrs {
    const RESET_VALUE: u32 = 0;
}
