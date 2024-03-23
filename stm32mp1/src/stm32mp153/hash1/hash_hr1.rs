#[doc = "Register `HASH_HR1` reader"]
pub type R = crate::R<HASH_HR1rs>;
#[doc = "Field `H1` reader - H1"]
pub type H1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H1"]
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
#[doc = "HASH digest register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_hr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_HR1rs;
impl crate::RegisterSpec for HASH_HR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_hr1::R`](R) reader structure"]
impl crate::Readable for HASH_HR1rs {}
#[doc = "`reset()` method sets HASH_HR1 to value 0"]
impl crate::Resettable for HASH_HR1rs {
    const RESET_VALUE: u32 = 0;
}
