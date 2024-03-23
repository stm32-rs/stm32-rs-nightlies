#[doc = "Register `ETH_MACTxTSSNR` reader"]
pub type R = crate::R<ETH_MACTX_TSSNRrs>;
#[doc = "Field `TXTSSLO` reader - TXTSSLO"]
pub type TXTSSLO_R = crate::FieldReader<u32>;
#[doc = "Field `TXTSSMIS` reader - TXTSSMIS"]
pub type TXTSSMIS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - TXTSSLO"]
    #[inline(always)]
    pub fn txtsslo(&self) -> TXTSSLO_R {
        TXTSSLO_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - TXTSSMIS"]
    #[inline(always)]
    pub fn txtssmis(&self) -> TXTSSMIS_R {
        TXTSSMIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "This register contains the nanosecond part of timestamp captured for Transmit packets when Tx status is disabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mactx_tssnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACTX_TSSNRrs;
impl crate::RegisterSpec for ETH_MACTX_TSSNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mactx_tssnr::R`](R) reader structure"]
impl crate::Readable for ETH_MACTX_TSSNRrs {}
#[doc = "`reset()` method sets ETH_MACTxTSSNR to value 0"]
impl crate::Resettable for ETH_MACTX_TSSNRrs {
    const RESET_VALUE: u32 = 0;
}
