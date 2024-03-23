#[doc = "Register `RDATA` reader"]
pub type R = crate::R<RDATArs>;
#[doc = "Field `RES` reader - RES"]
pub type RES_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RES"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits)
    }
}
#[doc = "CORDIC result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDATArs;
impl crate::RegisterSpec for RDATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdata::R`](R) reader structure"]
impl crate::Readable for RDATArs {}
#[doc = "`reset()` method sets RDATA to value 0"]
impl crate::Resettable for RDATArs {
    const RESET_VALUE: u32 = 0;
}
