#[doc = "Reader of register MODER"]
pub type R = crate::R<u32, super::MODER>;
#[doc = "Writer for register MODER"]
pub type W = crate::W<u32, super::MODER>;
#[doc = "Register MODER `reset()`'s with value 0xc0"]
impl crate::ResetValue for super::MODER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc0
    }
}
#[doc = "Reader of field `MODER3`"]
pub type MODER3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODER3`"]
pub struct MODER3_W<'a> {
    w: &'a mut W,
}
impl<'a> MODER3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder3(&self) -> MODER3_R {
        MODER3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn moder3(&mut self) -> MODER3_W {
        MODER3_W { w: self }
    }
}
