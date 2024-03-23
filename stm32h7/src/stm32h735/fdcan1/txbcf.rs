#[doc = "Register `TXBCF` reader"]
pub type R = crate::R<TXBCFrs>;
#[doc = "Field `CF` reader - Cancellation Finished"]
pub type CF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(self.bits)
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Finished Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBCFrs;
impl crate::RegisterSpec for TXBCFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcf::R`](R) reader structure"]
impl crate::Readable for TXBCFrs {}
#[doc = "`reset()` method sets TXBCF to value 0"]
impl crate::Resettable for TXBCFrs {
    const RESET_VALUE: u32 = 0;
}
