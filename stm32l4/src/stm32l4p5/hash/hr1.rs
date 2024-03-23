#[doc = "Register `HR1` reader"]
pub type R = crate::R<HR1rs>;
#[doc = "Field `H1` reader - H1"]
pub type H1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H1"]
    #[inline(always)]
    pub fn h1(&self) -> H1_R {
        H1_R::new(self.bits)
    }
}
#[doc = "read-only\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HR1rs;
impl crate::RegisterSpec for HR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr1::R`](R) reader structure"]
impl crate::Readable for HR1rs {}
#[doc = "`reset()` method sets HR1 to value 0"]
impl crate::Resettable for HR1rs {
    const RESET_VALUE: u32 = 0;
}
