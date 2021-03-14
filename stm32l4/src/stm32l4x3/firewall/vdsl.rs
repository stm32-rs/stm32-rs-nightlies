#[doc = "Reader of register VDSL"]
pub type R = crate::R<u32, super::VDSL>;
#[doc = "Writer for register VDSL"]
pub type W = crate::W<u32, super::VDSL>;
#[doc = "Register VDSL `reset()`'s with value 0"]
impl crate::ResetValue for super::VDSL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LENG`"]
pub type LENG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LENG`"]
pub struct LENG_W<'a> {
    w: &'a mut W,
}
impl<'a> LENG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 6)) | (((value as u32) & 0x03ff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:15 - Non-volatile data segment length"]
    #[inline(always)]
    pub fn leng(&self) -> LENG_R {
        LENG_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:15 - Non-volatile data segment length"]
    #[inline(always)]
    pub fn leng(&mut self) -> LENG_W {
        LENG_W { w: self }
    }
}
