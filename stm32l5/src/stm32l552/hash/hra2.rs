#[doc = "Register `HRA2` reader"]
pub type R = crate::R<HRA2rs>;
#[doc = "Field `H2` reader - H2"]
pub type H2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H2"]
    #[inline(always)]
    pub fn h2(&self) -> H2_R {
        H2_R::new(self.bits)
    }
}
#[doc = "HASH aliased digest register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hra2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRA2rs;
impl crate::RegisterSpec for HRA2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hra2::R`](R) reader structure"]
impl crate::Readable for HRA2rs {}
#[doc = "`reset()` method sets HRA2 to value 0"]
impl crate::Resettable for HRA2rs {
    const RESET_VALUE: u32 = 0;
}
