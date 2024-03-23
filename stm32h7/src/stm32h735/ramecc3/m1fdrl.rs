#[doc = "Register `M1FDRL` reader"]
pub type R = crate::R<M1FDRLrs>;
#[doc = "Field `FDATAL` reader - Failing data low"]
pub type FDATAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data low"]
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1fdrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1FDRLrs;
impl crate::RegisterSpec for M1FDRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1fdrl::R`](R) reader structure"]
impl crate::Readable for M1FDRLrs {}
#[doc = "`reset()` method sets M1FDRL to value 0"]
impl crate::Resettable for M1FDRLrs {
    const RESET_VALUE: u32 = 0;
}
