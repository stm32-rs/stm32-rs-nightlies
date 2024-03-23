#[doc = "Register `TXBRP` reader"]
pub type R = crate::R<TXBRPrs>;
#[doc = "Field `TRP` reader - Transmission Request Pending"]
pub type TRP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission Request Pending"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(self.bits)
    }
}
#[doc = "FDCAN Tx Buffer Request Pending Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbrp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBRPrs;
impl crate::RegisterSpec for TXBRPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbrp::R`](R) reader structure"]
impl crate::Readable for TXBRPrs {}
#[doc = "`reset()` method sets TXBRP to value 0"]
impl crate::Resettable for TXBRPrs {
    const RESET_VALUE: u32 = 0;
}
