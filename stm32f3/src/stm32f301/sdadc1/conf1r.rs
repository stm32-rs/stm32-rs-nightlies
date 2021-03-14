#[doc = "Reader of register CONF1R"]
pub type R = crate::R<u32, super::CONF1R>;
#[doc = "Writer for register CONF1R"]
pub type W = crate::W<u32, super::CONF1R>;
#[doc = "Register CONF1R `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMON1`"]
pub type COMMON1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMMON1`"]
pub struct COMMON1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMON1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `SE1`"]
pub type SE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SE1`"]
pub struct SE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `GAIN1`"]
pub type GAIN1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAIN1`"]
pub struct GAIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `OFFSET1`"]
pub type OFFSET1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OFFSET1`"]
pub struct OFFSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Common mode for configuration 1"]
    #[inline(always)]
    pub fn common1(&self) -> COMMON1_R {
        COMMON1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 1"]
    #[inline(always)]
    pub fn se1(&self) -> SE1_R {
        SE1_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 1"]
    #[inline(always)]
    pub fn gain1(&self) -> GAIN1_R {
        GAIN1_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 1"]
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 30:31 - Common mode for configuration 1"]
    #[inline(always)]
    pub fn common1(&mut self) -> COMMON1_W {
        COMMON1_W { w: self }
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 1"]
    #[inline(always)]
    pub fn se1(&mut self) -> SE1_W {
        SE1_W { w: self }
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 1"]
    #[inline(always)]
    pub fn gain1(&mut self) -> GAIN1_W {
        GAIN1_W { w: self }
    }
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 1"]
    #[inline(always)]
    pub fn offset1(&mut self) -> OFFSET1_W {
        OFFSET1_W { w: self }
    }
}
