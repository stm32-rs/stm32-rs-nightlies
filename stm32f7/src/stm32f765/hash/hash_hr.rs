#[doc = "Register `HASH_HR%s` reader"]
pub type R = crate::R<HASH_HRrs>;
#[doc = "Field `H` reader - H0"]
pub type H_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H0"]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(self.bits)
    }
}
#[doc = "HASH digest register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_HRrs;
impl crate::RegisterSpec for HASH_HRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_hr::R`](R) reader structure"]
impl crate::Readable for HASH_HRrs {}
#[doc = "`reset()` method sets HASH_HR%s to value 0"]
impl crate::Resettable for HASH_HRrs {
    const RESET_VALUE: u32 = 0;
}
