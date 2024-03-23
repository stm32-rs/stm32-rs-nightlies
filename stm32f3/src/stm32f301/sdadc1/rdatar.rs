#[doc = "Register `RDATAR` reader"]
pub type R = crate::R<RDATARrs>;
#[doc = "Field `RDATA` reader - Regular channel conversion data"]
pub type RDATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Regular channel conversion data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "data register for the regular channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdatar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDATARrs;
impl crate::RegisterSpec for RDATARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdatar::R`](R) reader structure"]
impl crate::Readable for RDATARrs {}
#[doc = "`reset()` method sets RDATAR to value 0"]
impl crate::Resettable for RDATARrs {
    const RESET_VALUE: u32 = 0;
}
