#[doc = "Reader of register OTR"]
pub type R = crate::R<u32, super::OTR>;
#[doc = "Writer for register OTR"]
pub type W = crate::W<u32, super::OTR>;
#[doc = "Register OTR `reset()`'s with value 0"]
impl crate::ResetValue for super::OTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OT_USER`"]
pub type OT_USER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OT_USER`"]
pub struct OT_USER_W<'a> {
    w: &'a mut W,
}
impl<'a> OT_USER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `AO3_OPT_OFFSET_TRIM`"]
pub type AO3_OPT_OFFSET_TRIM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AO3_OPT_OFFSET_TRIM`"]
pub struct AO3_OPT_OFFSET_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> AO3_OPT_OFFSET_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Reader of field `AO2_OPT_OFFSET_TRIM`"]
pub type AO2_OPT_OFFSET_TRIM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AO2_OPT_OFFSET_TRIM`"]
pub struct AO2_OPT_OFFSET_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> AO2_OPT_OFFSET_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `AO1_OPT_OFFSET_TRIM`"]
pub type AO1_OPT_OFFSET_TRIM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AO1_OPT_OFFSET_TRIM`"]
pub struct AO1_OPT_OFFSET_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> AO1_OPT_OFFSET_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Select user or factory trimming value"]
    #[inline(always)]
    pub fn ot_user(&self) -> OT_USER_R {
        OT_USER_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 20:29 - OPAMP3, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao3_opt_offset_trim(&self) -> AO3_OPT_OFFSET_TRIM_R {
        AO3_OPT_OFFSET_TRIM_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - OPAMP2, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao2_opt_offset_trim(&self) -> AO2_OPT_OFFSET_TRIM_R {
        AO2_OPT_OFFSET_TRIM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - OPAMP1, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao1_opt_offset_trim(&self) -> AO1_OPT_OFFSET_TRIM_R {
        AO1_OPT_OFFSET_TRIM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Select user or factory trimming value"]
    #[inline(always)]
    pub fn ot_user(&mut self) -> OT_USER_W {
        OT_USER_W { w: self }
    }
    #[doc = "Bits 20:29 - OPAMP3, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao3_opt_offset_trim(&mut self) -> AO3_OPT_OFFSET_TRIM_W {
        AO3_OPT_OFFSET_TRIM_W { w: self }
    }
    #[doc = "Bits 10:19 - OPAMP2, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao2_opt_offset_trim(&mut self) -> AO2_OPT_OFFSET_TRIM_W {
        AO2_OPT_OFFSET_TRIM_W { w: self }
    }
    #[doc = "Bits 0:9 - OPAMP1, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao1_opt_offset_trim(&mut self) -> AO1_OPT_OFFSET_TRIM_W {
        AO1_OPT_OFFSET_TRIM_W { w: self }
    }
}
