#[doc = "Reader of register WPCR2"]
pub type R = crate::R<u32, super::WPCR2>;
#[doc = "Writer for register WPCR2"]
pub type W = crate::W<u32, super::WPCR2>;
#[doc = "Register WPCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::WPCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCLKPOST`"]
pub type TCLKPOST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCLKPOST`"]
pub struct TCLKPOST_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKPOST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    pub fn tclkpost(&self) -> TCLKPOST_R {
        TCLKPOST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    pub fn tclkpost(&mut self) -> TCLKPOST_W {
        TCLKPOST_W { w: self }
    }
}
