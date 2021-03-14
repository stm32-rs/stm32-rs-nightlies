#[doc = "Reader of register AF"]
pub type R = crate::R<u32, super::AF>;
#[doc = "Writer for register AF"]
pub type W = crate::W<u32, super::AF>;
#[doc = "Register AF `reset()`'s with value 0"]
impl crate::ResetValue for super::AF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ETRSEL`"]
pub type ETRSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETRSEL`"]
pub struct ETRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:16 - External trigger source selection"]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 14:16 - External trigger source selection"]
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W {
        ETRSEL_W { w: self }
    }
}
