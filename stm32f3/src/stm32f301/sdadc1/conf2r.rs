#[doc = "Reader of register CONF2R"]
pub type R = crate::R<u32, super::CONF2R>;
#[doc = "Writer for register CONF2R"]
pub type W = crate::W<u32, super::CONF2R>;
#[doc = "Register CONF2R `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMON2`"]
pub type COMMON2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMMON2`"]
pub struct COMMON2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMON2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `SE2`"]
pub type SE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SE2`"]
pub struct SE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `GAIN2`"]
pub type GAIN2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAIN2`"]
pub struct GAIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `OFFSET2`"]
pub type OFFSET2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OFFSET2`"]
pub struct OFFSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Common mode for configuration 2"]
    #[inline(always)]
    pub fn common2(&self) -> COMMON2_R {
        COMMON2_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 2"]
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 2"]
    #[inline(always)]
    pub fn gain2(&self) -> GAIN2_R {
        GAIN2_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 2"]
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 30:31 - Common mode for configuration 2"]
    #[inline(always)]
    pub fn common2(&mut self) -> COMMON2_W {
        COMMON2_W { w: self }
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 2"]
    #[inline(always)]
    pub fn se2(&mut self) -> SE2_W {
        SE2_W { w: self }
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 2"]
    #[inline(always)]
    pub fn gain2(&mut self) -> GAIN2_W {
        GAIN2_W { w: self }
    }
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 2"]
    #[inline(always)]
    pub fn offset2(&mut self) -> OFFSET2_W {
        OFFSET2_W { w: self }
    }
}
