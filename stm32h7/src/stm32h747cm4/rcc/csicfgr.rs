#[doc = "Reader of register CSICFGR"]
pub type R = crate::R<u32, super::CSICFGR>;
#[doc = "Writer for register CSICFGR"]
pub type W = crate::W<u32, super::CSICFGR>;
#[doc = "Register CSICFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSICFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSITRIM`"]
pub type CSITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSITRIM`"]
pub struct CSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CSITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CSICAL`"]
pub type CSICAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CSICAL`"]
pub struct CSICAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSICAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29 - CSI clock trimming"]
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 0:9 - CSI clock calibration"]
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:29 - CSI clock trimming"]
    #[inline(always)]
    pub fn csitrim(&mut self) -> CSITRIM_W {
        CSITRIM_W { w: self }
    }
    #[doc = "Bits 0:9 - CSI clock calibration"]
    #[inline(always)]
    pub fn csical(&mut self) -> CSICAL_W {
        CSICAL_W { w: self }
    }
}
