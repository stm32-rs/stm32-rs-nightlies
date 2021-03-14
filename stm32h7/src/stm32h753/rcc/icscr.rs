#[doc = "Reader of register ICSCR"]
pub type R = crate::R<u32, super::ICSCR>;
#[doc = "Writer for register ICSCR"]
pub type W = crate::W<u32, super::ICSCR>;
#[doc = "Register ICSCR `reset()`'s with value 0x4000_0000"]
impl crate::ResetValue for super::ICSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000_0000
    }
}
#[doc = "Reader of field `HSICAL`"]
pub type HSICAL_R = crate::R<u16, u16>;
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
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CSICAL`"]
pub type CSICAL_R = crate::R<u8, u8>;
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
        self.w.bits = (self.w.bits & !(0x1f << 26)) | (((value as u32) & 0x1f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - HSI clock calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:17 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:25 - CSI clock calibration"]
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 26:30 - CSI clock trimming"]
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:17 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W {
        HSITRIM_W { w: self }
    }
    #[doc = "Bits 26:30 - CSI clock trimming"]
    #[inline(always)]
    pub fn csitrim(&mut self) -> CSITRIM_W {
        CSITRIM_W { w: self }
    }
}
