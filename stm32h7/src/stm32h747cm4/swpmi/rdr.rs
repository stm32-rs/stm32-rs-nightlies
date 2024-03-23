#[doc = "Register `RDR` reader"]
pub type R = crate::R<RDRrs>;
#[doc = "Field `RD` reader - received data"]
pub type RD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - received data"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(self.bits)
    }
}
#[doc = "SWPMI Receive data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDRrs;
impl crate::RegisterSpec for RDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RDRrs {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RDRrs {
    const RESET_VALUE: u32 = 0;
}
