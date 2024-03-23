#[doc = "Register `CEC_RXDR` reader"]
pub type R = crate::R<CEC_RXDRrs>;
#[doc = "Field `RXD` reader - RXD"]
pub type RXD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RXD"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CEC Rx data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cec_rxdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CEC_RXDRrs;
impl crate::RegisterSpec for CEC_RXDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cec_rxdr::R`](R) reader structure"]
impl crate::Readable for CEC_RXDRrs {}
#[doc = "`reset()` method sets CEC_RXDR to value 0"]
impl crate::Resettable for CEC_RXDRrs {
    const RESET_VALUE: u32 = 0;
}
