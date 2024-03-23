#[doc = "Register `RXCRCR` reader"]
pub type R = crate::R<RXCRCRrs>;
#[doc = "Field `RxCRC` reader - Rx CRC register"]
pub type RX_CRC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Rx CRC register"]
    #[inline(always)]
    pub fn rx_crc(&self) -> RX_CRC_R {
        RX_CRC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RX CRC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCRCRrs;
impl crate::RegisterSpec for RXCRCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcrcr::R`](R) reader structure"]
impl crate::Readable for RXCRCRrs {}
#[doc = "`reset()` method sets RXCRCR to value 0"]
impl crate::Resettable for RXCRCRrs {
    const RESET_VALUE: u32 = 0;
}
