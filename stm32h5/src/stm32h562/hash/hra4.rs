#[doc = "Register `HRA4` reader"]
pub type R = crate::R<HRA4rs>;
#[doc = "Field `Hx` reader - Hash data x Refer to introduction."]
pub type HX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hash data x Refer to introduction."]
    #[inline(always)]
    pub fn hx(&self) -> HX_R {
        HX_R::new(self.bits)
    }
}
#[doc = "HASH aliased digest register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hra4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRA4rs;
impl crate::RegisterSpec for HRA4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hra4::R`](R) reader structure"]
impl crate::Readable for HRA4rs {}
#[doc = "`reset()` method sets HRA4 to value 0"]
impl crate::Resettable for HRA4rs {
    const RESET_VALUE: u32 = 0;
}
