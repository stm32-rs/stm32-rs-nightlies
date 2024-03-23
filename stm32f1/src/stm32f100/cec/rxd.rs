#[doc = "Register `RXD` reader"]
pub type R = crate::R<RXDrs>;
#[doc = "Field `RXD` reader - Rx data"]
pub type RXD_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Rx data"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CEC Rx data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDrs;
impl crate::RegisterSpec for RXDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxd::R`](R) reader structure"]
impl crate::Readable for RXDrs {}
#[doc = "`reset()` method sets RXD to value 0"]
impl crate::Resettable for RXDrs {
    const RESET_VALUE: u32 = 0;
}
