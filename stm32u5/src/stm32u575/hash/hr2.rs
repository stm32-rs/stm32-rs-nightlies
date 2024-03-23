#[doc = "Register `HR2` reader"]
pub type R = crate::R<HR2rs>;
#[doc = "Field `H2` reader - H2"]
pub type H2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H2"]
    #[inline(always)]
    pub fn h2(&self) -> H2_R {
        H2_R::new(self.bits)
    }
}
#[doc = "digest register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR2rs;
impl crate::RegisterSpec for HR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr2::R`](R) reader structure"]
impl crate::Readable for HR2rs {}
#[doc = "`reset()` method sets HR2 to value 0"]
impl crate::Resettable for HR2rs {
    const RESET_VALUE: u32 = 0;
}
