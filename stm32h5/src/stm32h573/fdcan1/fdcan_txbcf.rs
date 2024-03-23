#[doc = "Register `FDCAN_TXBCF` reader"]
pub type R = crate::R<FDCAN_TXBCFrs>;
#[doc = "Field `CF` reader - Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
pub type CF_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Cancellation finished Each Tx buffer has its own CF bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a 1 to the corresponding bit of register TXBAR."]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 7) as u8)
    }
}
#[doc = "FDCAN Tx buffer cancellation finished register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbcf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBCFrs;
impl crate::RegisterSpec for FDCAN_TXBCFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbcf::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBCFrs {}
#[doc = "`reset()` method sets FDCAN_TXBCF to value 0"]
impl crate::Resettable for FDCAN_TXBCFrs {
    const RESET_VALUE: u32 = 0;
}
