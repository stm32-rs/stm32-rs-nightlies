#[doc = "Reader of register JPEG_CONFR3"]
pub type R = crate::R<u32, super::JPEG_CONFR3>;
#[doc = "Writer for register JPEG_CONFR3"]
pub type W = crate::W<u32, super::JPEG_CONFR3>;
#[doc = "Register JPEG_CONFR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::JPEG_CONFR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XSIZE`"]
pub type XSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XSIZE`"]
pub struct XSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> XSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - X size"]
    #[inline(always)]
    pub fn xsize(&self) -> XSIZE_R {
        XSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - X size"]
    #[inline(always)]
    pub fn xsize(&mut self) -> XSIZE_W {
        XSIZE_W { w: self }
    }
}
