#[doc = "Register `FDCAN_TXBRP` reader"]
pub type R = crate::R<FDCAN_TXBRPrs>;
#[doc = "Field `TRP` reader - Transmission Request Pending"]
pub type TRP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Transmission Request Pending"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 7) as u8)
    }
}
#[doc = "FDCAN Tx Buffer Request Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbrp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBRPrs;
impl crate::RegisterSpec for FDCAN_TXBRPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbrp::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBRPrs {}
#[doc = "`reset()` method sets FDCAN_TXBRP to value 0"]
impl crate::Resettable for FDCAN_TXBRPrs {
    const RESET_VALUE: u32 = 0;
}
