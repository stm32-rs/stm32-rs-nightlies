#[doc = "Register `HR0` reader"]
pub type R = crate::R<HR0rs>;
#[doc = "Field `Hx` reader - Hash data x Refer to introduction."]
pub type HX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to introduction."]
    #[inline(always)]
    pub fn hx(&self) -> HX_R {
        HX_R::new(self.bits)
    }
}
#[doc = "HASH digest register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR0rs;
impl crate::RegisterSpec for HR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr0::R`](R) reader structure"]
impl crate::Readable for HR0rs {}
#[doc = "`reset()` method sets HR0 to value 0"]
impl crate::Resettable for HR0rs {
    const RESET_VALUE: u32 = 0;
}
