#[doc = "Reader of register SMPSCR"]
pub type R = crate::R<u32, super::SMPSCR>;
#[doc = "Writer for register SMPSCR"]
pub type W = crate::W<u32, super::SMPSCR>;
#[doc = "Register SMPSCR `reset()`'s with value 0x0301"]
impl crate::ResetValue for super::SMPSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0301
    }
}
#[doc = "Reader of field `SMPSSWS`"]
pub type SMPSSWS_R = crate::R<u8, u8>;
#[doc = "Reader of field `SMPSDIV`"]
pub type SMPSDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMPSDIV`"]
pub struct SMPSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SMPSSEL`"]
pub type SMPSSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SMPSSEL`"]
pub struct SMPSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - Step Down converter clock switch status"]
    #[inline(always)]
    pub fn smpssws(&self) -> SMPSSWS_R {
        SMPSSWS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Step Down converter clock prescaler"]
    #[inline(always)]
    pub fn smpsdiv(&self) -> SMPSDIV_R {
        SMPSDIV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Step Down converter clock selection"]
    #[inline(always)]
    pub fn smpssel(&self) -> SMPSSEL_R {
        SMPSSEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Step Down converter clock prescaler"]
    #[inline(always)]
    pub fn smpsdiv(&mut self) -> SMPSDIV_W {
        SMPSDIV_W { w: self }
    }
    #[doc = "Bits 0:1 - Step Down converter clock selection"]
    #[inline(always)]
    pub fn smpssel(&mut self) -> SMPSSEL_W {
        SMPSSEL_W { w: self }
    }
}
