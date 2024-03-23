#[doc = "Register `FDCAN_TXBTO` reader"]
pub type R = crate::R<FDCAN_TXBTOrs>;
#[doc = "Field `TO` reader - TO"]
pub type TO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TO"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(self.bits)
    }
}
#[doc = "FDCAN Tx buffer transmission occurred register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_txbto::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_TXBTOrs;
impl crate::RegisterSpec for FDCAN_TXBTOrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbto::R`](R) reader structure"]
impl crate::Readable for FDCAN_TXBTOrs {}
#[doc = "`reset()` method sets FDCAN_TXBTO to value 0"]
impl crate::Resettable for FDCAN_TXBTOrs {
    const RESET_VALUE: u32 = 0;
}
