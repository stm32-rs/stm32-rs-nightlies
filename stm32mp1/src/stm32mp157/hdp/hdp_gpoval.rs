#[doc = "Reader of register HDP_GPOVAL"]
pub type R = crate::R<u32, super::HDP_GPOVAL>;
#[doc = "Writer for register HDP_GPOVAL"]
pub type W = crate::W<u32, super::HDP_GPOVAL>;
#[doc = "Register HDP_GPOVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::HDP_GPOVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HDPGPOVAL`"]
pub type HDPGPOVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HDPGPOVAL`"]
pub struct HDPGPOVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPGPOVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - HDPGPOVAL"]
    #[inline(always)]
    pub fn hdpgpoval(&self) -> HDPGPOVAL_R {
        HDPGPOVAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HDPGPOVAL"]
    #[inline(always)]
    pub fn hdpgpoval(&mut self) -> HDPGPOVAL_W {
        HDPGPOVAL_W { w: self }
    }
}
