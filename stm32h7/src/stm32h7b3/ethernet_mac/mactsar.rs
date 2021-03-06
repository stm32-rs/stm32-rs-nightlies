#[doc = "Reader of register MACTSAR"]
pub type R = crate::R<u32, super::MACTSAR>;
#[doc = "Writer for register MACTSAR"]
pub type W = crate::W<u32, super::MACTSAR>;
#[doc = "Register MACTSAR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACTSAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSAR`"]
pub type TSAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TSAR`"]
pub struct TSAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TSAR"]
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSAR"]
    #[inline(always)]
    pub fn tsar(&mut self) -> TSAR_W {
        TSAR_W { w: self }
    }
}
