#[doc = "Reader of register MACIVIR"]
pub type R = crate::R<u32, super::MACIVIR>;
#[doc = "Writer for register MACIVIR"]
pub type W = crate::W<u32, super::MACIVIR>;
#[doc = "Register MACIVIR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACIVIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VLT`"]
pub type VLT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VLT`"]
pub struct VLT_W<'a> {
    w: &'a mut W,
}
impl<'a> VLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `VLC`"]
pub type VLC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VLC`"]
pub struct VLC_W<'a> {
    w: &'a mut W,
}
impl<'a> VLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `VLP`"]
pub type VLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLP`"]
pub struct VLP_W<'a> {
    w: &'a mut W,
}
impl<'a> VLP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CSVL`"]
pub type CSVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSVL`"]
pub struct CSVL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSVL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `VLTI`"]
pub type VLTI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLTI`"]
pub struct VLTI_W<'a> {
    w: &'a mut W,
}
impl<'a> VLTI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - VLT"]
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - VLC"]
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - VLP"]
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CSVL"]
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - VLTI"]
    #[inline(always)]
    pub fn vlti(&self) -> VLTI_R {
        VLTI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLT"]
    #[inline(always)]
    pub fn vlt(&mut self) -> VLT_W {
        VLT_W { w: self }
    }
    #[doc = "Bits 16:17 - VLC"]
    #[inline(always)]
    pub fn vlc(&mut self) -> VLC_W {
        VLC_W { w: self }
    }
    #[doc = "Bit 18 - VLP"]
    #[inline(always)]
    pub fn vlp(&mut self) -> VLP_W {
        VLP_W { w: self }
    }
    #[doc = "Bit 19 - CSVL"]
    #[inline(always)]
    pub fn csvl(&mut self) -> CSVL_W {
        CSVL_W { w: self }
    }
    #[doc = "Bit 20 - VLTI"]
    #[inline(always)]
    pub fn vlti(&mut self) -> VLTI_W {
        VLTI_W { w: self }
    }
}
