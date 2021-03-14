#[doc = "Reader of register OSPEEDR"]
pub type R = crate::R<u32, super::OSPEEDR>;
#[doc = "Writer for register OSPEEDR"]
pub type W = crate::W<u32, super::OSPEEDR>;
#[doc = "Register OSPEEDR `reset()`'s with value 0"]
impl crate::ResetValue for super::OSPEEDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSPEEDR3`"]
pub type OSPEEDR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OSPEEDR3`"]
pub struct OSPEEDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSPEEDR3_W<'a> {
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
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Port x configuration bits (y = 0..15)"]
    #[inline(always)]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W {
        OSPEEDR3_W { w: self }
    }
}
