#[doc = "Register `MACTxTSSNR` reader"]
pub type R = crate::R<MACTX_TSSNRrs>;
#[doc = "Field `TXTSSLO` reader - Transmit Timestamp Status Low"]
pub type TXTSSLO_R = crate::FieldReader<u32>;
#[doc = "Field `TXTSSMIS` reader - Transmit Timestamp Status Missed"]
pub type TXTSSMIS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - Transmit Timestamp Status Low"]
    #[inline(always)]
    pub fn txtsslo(&self) -> TXTSSLO_R {
        TXTSSLO_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Transmit Timestamp Status Missed"]
    #[inline(always)]
    pub fn txtssmis(&self) -> TXTSSMIS_R {
        TXTSSMIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Tx timestamp status nanoseconds register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactx_tssnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTX_TSSNRrs;
impl crate::RegisterSpec for MACTX_TSSNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactx_tssnr::R`](R) reader structure"]
impl crate::Readable for MACTX_TSSNRrs {}
#[doc = "`reset()` method sets MACTxTSSNR to value 0"]
impl crate::Resettable for MACTX_TSSNRrs {
    const RESET_VALUE: u32 = 0;
}
