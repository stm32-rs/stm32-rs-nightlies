#[doc = "Reader of register CMP3ER"]
pub type R = crate::R<u32, super::CMP3ER>;
#[doc = "Writer for register CMP3ER"]
pub type W = crate::W<u32, super::CMP3ER>;
#[doc = "Register CMP3ER `reset()`'s with value 0"]
impl crate::ResetValue for super::CMP3ER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP3x`"]
pub type CMP3X_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMP3x`"]
pub struct CMP3X_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    pub fn cmp3x(&self) -> CMP3X_R {
        CMP3X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 3 value"]
    #[inline(always)]
    pub fn cmp3x(&mut self) -> CMP3X_W {
        CMP3X_W { w: self }
    }
}
