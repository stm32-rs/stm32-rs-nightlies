#[doc = "Register `HR5` reader"]
pub type R = crate::R<HR5rs>;
#[doc = "Field `Hx` reader - Hash data x Refer to introduction."]
pub type HX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to introduction."]
    #[inline(always)]
    pub fn hx(&self) -> HX_R {
        HX_R::new(self.bits)
    }
}
#[doc = "HASH digest register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
