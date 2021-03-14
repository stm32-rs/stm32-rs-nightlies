#[doc = "Reader of register M5FAR"]
pub type R = crate::R<u32, super::M5FAR>;
#[doc = "Writer for register M5FAR"]
pub type W = crate::W<u32, super::M5FAR>;
#[doc = "Register M5FAR `reset()`'s with value 0"]
impl crate::ResetValue for super::M5FAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FADD`"]
pub type FADD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {}
