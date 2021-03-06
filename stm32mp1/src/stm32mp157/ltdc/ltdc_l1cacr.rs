#[doc = "Reader of register LTDC_L1CACR"]
pub type R = crate::R<u32, super::LTDC_L1CACR>;
#[doc = "Writer for register LTDC_L1CACR"]
pub type W = crate::W<u32, super::LTDC_L1CACR>;
#[doc = "Register LTDC_L1CACR `reset()`'s with value 0xff"]
impl crate::ResetValue for super::LTDC_L1CACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `CONSTA`"]
pub type CONSTA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONSTA`"]
pub struct CONSTA_W<'a> {
    w: &'a mut W,
}
impl<'a> CONSTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CONSTA"]
    #[inline(always)]
    pub fn consta(&self) -> CONSTA_R {
        CONSTA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CONSTA"]
    #[inline(always)]
    pub fn consta(&mut self) -> CONSTA_W {
        CONSTA_W { w: self }
    }
}
