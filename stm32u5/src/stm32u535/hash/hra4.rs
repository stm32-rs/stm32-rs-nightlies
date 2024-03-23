#[doc = "Register `HRA4` reader"]
pub type R = crate::R<HRA4rs>;
#[doc = "Field `H4` reader - H4"]
pub type H4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H4"]
    #[inline(always)]
    pub fn h4(&self) -> H4_R {
        H4_R::new(self.bits)
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
