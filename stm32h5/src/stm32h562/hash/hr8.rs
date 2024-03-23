#[doc = "Register `HR8` reader"]
pub type R = crate::R<HR8rs>;
#[doc = "Field `Hx` reader - Hash data x Refer to introduction."]
pub type HX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to introduction."]
    #[inline(always)]
    pub fn hx(&self) -> HX_R {
        HX_R::new(self.bits)
    }
}
#[doc = "HASH digest register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR8rs;
impl crate::RegisterSpec for HR8rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr8::R`](R) reader structure"]
impl crate::Readable for HR8rs {}
#[doc = "`reset()` method sets HR8 to value 0"]
impl crate::Resettable for HR8rs {
    const RESET_VALUE: u32 = 0;
}
