#[doc = "Reader of register DFSDM_FLT1EXMIN"]
pub type R = crate::R<u32, super::DFSDM_FLT1EXMIN>;
#[doc = "Writer for register DFSDM_FLT1EXMIN"]
pub type W = crate::W<u32, super::DFSDM_FLT1EXMIN>;
#[doc = "Register DFSDM_FLT1EXMIN `reset()`'s with value 0x7fff_ff00"]
impl crate::ResetValue for super::DFSDM_FLT1EXMIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fff_ff00
    }
}
#[doc = "Reader of field `EXMINCH`"]
pub type EXMINCH_R = crate::R<u8, u8>;
#[doc = "Reader of field `EXMIN`"]
pub type EXMIN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EXMIN`"]
pub struct EXMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - EXMINCH"]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&mut self) -> EXMIN_W {
        EXMIN_W { w: self }
    }
}
