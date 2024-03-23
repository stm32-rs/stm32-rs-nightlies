#[doc = "Register `CDR2` reader"]
pub type R = crate::R<CDR2rs>;
#[doc = "Field `RDATA_ALT` reader - RDATA_ALT"]
pub type RDATA_ALT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RDATA_ALT"]
    #[inline(always)]
    pub fn rdata_alt(&self) -> RDATA_ALT_R {
        RDATA_ALT_R::new(self.bits)
    }
}
#[doc = "Common regular data register for dual mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDR2rs;
impl crate::RegisterSpec for CDR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr2::R`](R) reader structure"]
impl crate::Readable for CDR2rs {}
#[doc = "`reset()` method sets CDR2 to value 0"]
impl crate::Resettable for CDR2rs {
    const RESET_VALUE: u32 = 0;
}
