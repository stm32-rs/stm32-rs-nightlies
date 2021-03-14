#[doc = "Reader of register MACRWUFFER"]
pub type R = crate::R<u32, super::MACRWUFFER>;
#[doc = "Writer for register MACRWUFFER"]
pub type W = crate::W<u32, super::MACRWUFFER>;
#[doc = "Register MACRWUFFER `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MACRWUFFER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
