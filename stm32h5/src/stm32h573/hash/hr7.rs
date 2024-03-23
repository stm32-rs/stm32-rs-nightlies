#[doc = "Register `HR7` reader"]
pub type R = crate::R<HR7rs>;
#[doc = "Field `Hx` reader - Hash data x Refer to introduction."]
pub type HX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to introduction."]
    #[inline(always)]
    pub fn hx(&self) -> HX_R {
        HX_R::new(self.bits)
    }
}
#[doc = "HASH digest register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR7rs;
impl crate::RegisterSpec for HR7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr7::R`](R) reader structure"]
impl crate::Readable for HR7rs {}
#[doc = "`reset()` method sets HR7 to value 0"]
impl crate::Resettable for HR7rs {
    const RESET_VALUE: u32 = 0;
}
