#[doc = "Register `HR5` reader"]
pub type R = crate::R<HR5rs>;
#[doc = "Field `H5` reader - H5"]
pub type H5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H5"]
    #[inline(always)]
    pub fn h5(&self) -> H5_R {
        H5_R::new(self.bits)
    }
}
#[doc = "read-only\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR5rs;
impl crate::RegisterSpec for HR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr5::R`](R) reader structure"]
impl crate::Readable for HR5rs {}
#[doc = "`reset()` method sets HR5 to value 0"]
impl crate::Resettable for HR5rs {
    const RESET_VALUE: u32 = 0;
}
