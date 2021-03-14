#[doc = "Reader of register DLYB_CFGR"]
pub type R = crate::R<u32, super::DLYB_CFGR>;
#[doc = "Writer for register DLYB_CFGR"]
pub type W = crate::W<u32, super::DLYB_CFGR>;
#[doc = "Register DLYB_CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::DLYB_CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `UNIT`"]
pub type UNIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UNIT`"]
pub struct UNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> UNIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `LNG`"]
pub type LNG_R = crate::R<u16, u16>;
#[doc = "Reader of field `LNGF`"]
pub type LNGF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - SEL"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - UNIT"]
    #[inline(always)]
    pub fn unit(&self) -> UNIT_R {
        UNIT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - LNG"]
    #[inline(always)]
    pub fn lng(&self) -> LNG_R {
        LNG_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - LNGF"]
    #[inline(always)]
    pub fn lngf(&self) -> LNGF_R {
        LNGF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SEL"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bits 8:14 - UNIT"]
    #[inline(always)]
    pub fn unit(&mut self) -> UNIT_W {
        UNIT_W { w: self }
    }
}
