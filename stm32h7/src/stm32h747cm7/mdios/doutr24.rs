#[doc = "Reader of register DOUTR24"]
pub type R = crate::R<u32, super::DOUTR24>;
#[doc = "Writer for register DOUTR24"]
pub type W = crate::W<u32, super::DOUTR24>;
#[doc = "Register DOUTR24 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUTR24 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOUT24`"]
pub type DOUT24_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DOUT24`"]
pub struct DOUT24_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout24(&self) -> DOUT24_R {
        DOUT24_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout24(&mut self) -> DOUT24_W {
        DOUT24_W { w: self }
    }
}
