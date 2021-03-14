#[doc = "Reader of register HSICFGR"]
pub type R = crate::R<u32, super::HSICFGR>;
#[doc = "Writer for register HSICFGR"]
pub type W = crate::W<u32, super::HSICFGR>;
#[doc = "Register HSICFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::HSICFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSITRIM`"]
pub type HSITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSITRIM`"]
pub struct HSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "Reader of field `HSICAL`"]
pub type HSICAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HSICAL`"]
pub struct HSICAL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSICAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:30 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bits 0:11 - HSI clock calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:30 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W {
        HSITRIM_W { w: self }
    }
    #[doc = "Bits 0:11 - HSI clock calibration"]
    #[inline(always)]
    pub fn hsical(&mut self) -> HSICAL_W {
        HSICAL_W { w: self }
    }
}
