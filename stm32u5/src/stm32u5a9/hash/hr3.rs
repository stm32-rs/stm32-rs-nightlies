#[doc = "Register `HR3` reader"]
pub type R = crate::R<HR3rs>;
#[doc = "Field `H3` reader - H3"]
pub type H3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H3"]
    #[inline(always)]
    pub fn h3(&self) -> H3_R {
        H3_R::new(self.bits)
    }
}
#[doc = "digest register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR3rs;
impl crate::RegisterSpec for HR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr3::R`](R) reader structure"]
impl crate::Readable for HR3rs {}
#[doc = "`reset()` method sets HR3 to value 0"]
impl crate::Resettable for HR3rs {
    const RESET_VALUE: u32 = 0;
}
