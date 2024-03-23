#[doc = "Register `M4CR` reader"]
pub type R = crate::R<M4CRrs>;
#[doc = "Field `FDATAL` reader - Failing data low"]
pub type FDATAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data low"]
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4cr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4CRrs;
impl crate::RegisterSpec for M4CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m4cr::R`](R) reader structure"]
impl crate::Readable for M4CRrs {}
#[doc = "`reset()` method sets M4CR to value 0"]
impl crate::Resettable for M4CRrs {
    const RESET_VALUE: u32 = 0;
}
