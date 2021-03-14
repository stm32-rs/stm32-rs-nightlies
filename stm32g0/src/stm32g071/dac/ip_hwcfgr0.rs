#[doc = "Reader of register IP_HWCFGR0"]
pub type R = crate::R<u32, super::IP_HWCFGR0>;
#[doc = "Writer for register IP_HWCFGR0"]
pub type W = crate::W<u32, super::IP_HWCFGR0>;
#[doc = "Register IP_HWCFGR0 `reset()`'s with value 0x1111"]
impl crate::ResetValue for super::IP_HWCFGR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1111
    }
}
#[doc = "Reader of field `DUAL`"]
pub type DUAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DUAL`"]
pub struct DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `LFSR`"]
pub type LFSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LFSR`"]
pub struct LFSR_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `TRIANGLE`"]
pub type TRIANGLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIANGLE`"]
pub struct TRIANGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIANGLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SAMPLE`"]
pub type SAMPLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAMPLE`"]
pub struct SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `OR_CFG`"]
pub type OR_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OR_CFG`"]
pub struct OR_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> OR_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Dual DAC capability"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pseudonoise wave generation capability"]
    #[inline(always)]
    pub fn lfsr(&self) -> LFSR_R {
        LFSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Triangle wave generation capability"]
    #[inline(always)]
    pub fn triangle(&self) -> TRIANGLE_R {
        TRIANGLE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Sample and hold mode capability"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - option register bit width"]
    #[inline(always)]
    pub fn or_cfg(&self) -> OR_CFG_R {
        OR_CFG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dual DAC capability"]
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W {
        DUAL_W { w: self }
    }
    #[doc = "Bits 4:7 - Pseudonoise wave generation capability"]
    #[inline(always)]
    pub fn lfsr(&mut self) -> LFSR_W {
        LFSR_W { w: self }
    }
    #[doc = "Bits 8:11 - Triangle wave generation capability"]
    #[inline(always)]
    pub fn triangle(&mut self) -> TRIANGLE_W {
        TRIANGLE_W { w: self }
    }
    #[doc = "Bits 12:15 - Sample and hold mode capability"]
    #[inline(always)]
    pub fn sample(&mut self) -> SAMPLE_W {
        SAMPLE_W { w: self }
    }
    #[doc = "Bits 16:23 - option register bit width"]
    #[inline(always)]
    pub fn or_cfg(&mut self) -> OR_CFG_W {
        OR_CFG_W { w: self }
    }
}
