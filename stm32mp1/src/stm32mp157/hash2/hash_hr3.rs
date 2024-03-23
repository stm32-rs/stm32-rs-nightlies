#[doc = "Register `HASH_HR3` reader"]
pub type R = crate::R<HASH_HR3rs>;
#[doc = "Field `H3` reader - H3"]
pub type H3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H3"]
    #[inline(always)]
    pub fn h3(&self) -> H3_R {
        H3_R::new(self.bits)
    }
}
#[doc = "HASH digest register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_HR3rs;
impl crate::RegisterSpec for HASH_HR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_hr3::R`](R) reader structure"]
impl crate::Readable for HASH_HR3rs {}
#[doc = "`reset()` method sets HASH_HR3 to value 0"]
impl crate::Resettable for HASH_HR3rs {
    const RESET_VALUE: u32 = 0;
}
