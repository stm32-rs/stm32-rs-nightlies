#[doc = "Register `HR6` reader"]
pub type R = crate::R<HR6rs>;
#[doc = "Field `H6` reader - H6"]
pub type H6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H6"]
    #[inline(always)]
    pub fn h6(&self) -> H6_R {
        H6_R::new(self.bits)
    }
}
#[doc = "read-only\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR6rs;
impl crate::RegisterSpec for HR6rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr6::R`](R) reader structure"]
impl crate::Readable for HR6rs {}
#[doc = "`reset()` method sets HR6 to value 0"]
impl crate::Resettable for HR6rs {
    const RESET_VALUE: u32 = 0;
}
