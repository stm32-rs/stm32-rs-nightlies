#[doc = "Register `HASH_HR2` reader"]
pub type R = crate::R<HASH_HR2rs>;
#[doc = "Field `H2` reader - H2"]
pub type H2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H2"]
    #[inline(always)]
    pub fn h2(&self) -> H2_R {
        H2_R::new(self.bits)
    }
}
#[doc = "HASH digest register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_HR2rs;
impl crate::RegisterSpec for HASH_HR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_hr2::R`](R) reader structure"]
impl crate::Readable for HASH_HR2rs {}
#[doc = "`reset()` method sets HASH_HR2 to value 0"]
impl crate::Resettable for HASH_HR2rs {
    const RESET_VALUE: u32 = 0;
}
