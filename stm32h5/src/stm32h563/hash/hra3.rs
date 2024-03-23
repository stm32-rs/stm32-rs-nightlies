#[doc = "Register `HRA3` reader"]
pub type R = crate::R<HRA3rs>;
#[doc = "Field `Hx` reader - Hash data x Refer to introduction."]
pub type HX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to introduction."]
    #[inline(always)]
    pub fn hx(&self) -> HX_R {
        HX_R::new(self.bits)
    }
}
#[doc = "HASH aliased digest register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hra3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRA3rs;
impl crate::RegisterSpec for HRA3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hra3::R`](R) reader structure"]
impl crate::Readable for HRA3rs {}
#[doc = "`reset()` method sets HRA3 to value 0"]
impl crate::Resettable for HRA3rs {
    const RESET_VALUE: u32 = 0;
}
