#[doc = "Register `RXDR` reader"]
pub type R = crate::R<RXDRrs>;
#[doc = "Field `RXDR` reader - CEC Rx Data Register"]
pub type RXDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - CEC Rx Data Register"]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Rx Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDRrs;
impl crate::RegisterSpec for RXDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdr::R`](R) reader structure"]
impl crate::Readable for RXDRrs {}
#[doc = "`reset()` method sets RXDR to value 0"]
impl crate::Resettable for RXDRrs {
    const RESET_VALUE: u32 = 0;
}
