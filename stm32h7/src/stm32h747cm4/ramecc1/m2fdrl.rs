#[doc = "Reader of register M2FDRL"]
pub type R = crate::R<u32, super::M2FDRL>;
#[doc = "Writer for register M2FDRL"]
pub type W = crate::W<u32, super::M2FDRL>;
#[doc = "Register M2FDRL `reset()`'s with value 0"]
impl crate::ResetValue for super::M2FDRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FDATAL`"]
pub type FDATAL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC failing data low"]
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {}
