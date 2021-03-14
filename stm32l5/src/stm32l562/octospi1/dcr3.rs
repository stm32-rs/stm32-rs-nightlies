#[doc = "Reader of register DCR3"]
pub type R = crate::R<u32, super::DCR3>;
#[doc = "Writer for register DCR3"]
pub type W = crate::W<u32, super::DCR3>;
#[doc = "Register DCR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DCR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSBOUND`"]
pub type CSBOUND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSBOUND`"]
pub struct CSBOUND_W<'a> {
    w: &'a mut W,
}
impl<'a> CSBOUND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:20 - CS boundary"]
    #[inline(always)]
    pub fn csbound(&self) -> CSBOUND_R {
        CSBOUND_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - CS boundary"]
    #[inline(always)]
    pub fn csbound(&mut self) -> CSBOUND_W {
        CSBOUND_W { w: self }
    }
}
