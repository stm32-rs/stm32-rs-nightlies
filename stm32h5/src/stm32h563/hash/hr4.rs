#[doc = "Register `HR4` reader"]
pub type R = crate::R<HR4rs>;
#[doc = "Field `Hx` reader - Hash data x Refer to introduction."]
pub type HX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to introduction."]
    #[inline(always)]
    pub fn hx(&self) -> HX_R {
        HX_R::new(self.bits)
    }
}
#[doc = "HASH digest register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR4rs;
impl crate::RegisterSpec for HR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr4::R`](R) reader structure"]
impl crate::Readable for HR4rs {}
#[doc = "`reset()` method sets HR4 to value 0"]
impl crate::Resettable for HR4rs {
    const RESET_VALUE: u32 = 0;
}
