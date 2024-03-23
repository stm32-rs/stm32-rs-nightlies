#[doc = "Register `UCPD_RXDR` reader"]
pub type R = crate::R<UCPD_RXDRrs>;
#[doc = "Field `RXDATA` reader - Data byte received"]
pub type RXDATA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Data byte received"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucpd_rxdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UCPD_RXDRrs;
impl crate::RegisterSpec for UCPD_RXDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ucpd_rxdr::R`](R) reader structure"]
impl crate::Readable for UCPD_RXDRrs {}
#[doc = "`reset()` method sets UCPD_RXDR to value 0"]
impl crate::Resettable for UCPD_RXDRrs {
    const RESET_VALUE: u32 = 0;
}
