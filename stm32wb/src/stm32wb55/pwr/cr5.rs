#[doc = "Reader of register CR5"]
pub type R = crate::R<u32, super::CR5>;
#[doc = "Writer for register CR5"]
pub type W = crate::W<u32, super::CR5>;
#[doc = "Register CR5 `reset()`'s with value 0x4270"]
impl crate::ResetValue for super::CR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4270
    }
}
#[doc = "Reader of field `SDEB`"]
pub type SDEB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDEB`"]
pub struct SDEB_W<'a> {
    w: &'a mut W,
}
impl<'a> SDEB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SDBEN`"]
pub type SDBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDBEN`"]
pub struct SDBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDBEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SMPSCFG`"]
pub type SMPSCFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMPSCFG`"]
pub struct SMPSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSCFG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `BORHC`"]
pub type BORHC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BORHC`"]
pub struct BORHC_W<'a> {
    w: &'a mut W,
}
impl<'a> BORHC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SDSC`"]
pub type SDSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDSC`"]
pub struct SDSC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `SDVOS`"]
pub type SDVOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDVOS`"]
pub struct SDVOS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDVOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Enable Step Down converter SMPS mode enabled"]
    #[inline(always)]
    pub fn sdeb(&self) -> SDEB_R {
        SDEB_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Step Down converter Bypass mode enabled"]
    #[inline(always)]
    pub fn sdben(&self) -> SDBEN_R {
        SDBEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VOS configuration selection (non user)"]
    #[inline(always)]
    pub fn smpscfg(&self) -> SMPSCFG_R {
        SMPSCFG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BORH configuration selection"]
    #[inline(always)]
    pub fn borhc(&self) -> BORHC_R {
        BORHC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Step Down converter supplt startup current selection"]
    #[inline(always)]
    pub fn sdsc(&self) -> SDSC_R {
        SDSC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Step Down converter voltage output scaling"]
    #[inline(always)]
    pub fn sdvos(&self) -> SDVOS_R {
        SDVOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Enable Step Down converter SMPS mode enabled"]
    #[inline(always)]
    pub fn sdeb(&mut self) -> SDEB_W {
        SDEB_W { w: self }
    }
    #[doc = "Bit 14 - Enable Step Down converter Bypass mode enabled"]
    #[inline(always)]
    pub fn sdben(&mut self) -> SDBEN_W {
        SDBEN_W { w: self }
    }
    #[doc = "Bit 9 - VOS configuration selection (non user)"]
    #[inline(always)]
    pub fn smpscfg(&mut self) -> SMPSCFG_W {
        SMPSCFG_W { w: self }
    }
    #[doc = "Bit 8 - BORH configuration selection"]
    #[inline(always)]
    pub fn borhc(&mut self) -> BORHC_W {
        BORHC_W { w: self }
    }
    #[doc = "Bits 4:6 - Step Down converter supplt startup current selection"]
    #[inline(always)]
    pub fn sdsc(&mut self) -> SDSC_W {
        SDSC_W { w: self }
    }
    #[doc = "Bits 0:3 - Step Down converter voltage output scaling"]
    #[inline(always)]
    pub fn sdvos(&mut self) -> SDVOS_W {
        SDVOS_W { w: self }
    }
}
