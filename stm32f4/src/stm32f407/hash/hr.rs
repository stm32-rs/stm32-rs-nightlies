#[doc = "Register `HR%s` reader"]
pub type R = crate::R<HRrs>;
#[doc = "Field `H` reader - H0"]
pub type H_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H0"]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(self.bits)
    }
}
#[doc = "digest registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRrs;
impl crate::RegisterSpec for HRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr::R`](R) reader structure"]
impl crate::Readable for HRrs {}
#[doc = "`reset()` method sets HR%s to value 0"]
impl crate::Resettable for HRrs {
    const RESET_VALUE: u32 = 0;
}
