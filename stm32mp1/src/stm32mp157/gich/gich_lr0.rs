#[doc = "Reader of register GICH_LR0"]
pub type R = crate::R<u32, super::GICH_LR0>;
#[doc = "Writer for register GICH_LR0"]
pub type W = crate::W<u32, super::GICH_LR0>;
#[doc = "Register GICH_LR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICH_LR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VIRTUALID`"]
pub type VIRTUALID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VIRTUALID`"]
pub struct VIRTUALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VIRTUALID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `PHYSICALID`"]
pub type PHYSICALID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHYSICALID`"]
pub struct PHYSICALID_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYSICALID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `PRIORITY`"]
pub type PRIORITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIORITY`"]
pub struct PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 23)) | (((value as u32) & 0x1f) << 23);
        self.w
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `GRP1`"]
pub type GRP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GRP1`"]
pub struct GRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> GRP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `HW`"]
pub type HW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW`"]
pub struct HW_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_W<'a> {
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
impl R {
    #[doc = "Bits 0:9 - VIRTUALID"]
    #[inline(always)]
    pub fn virtualid(&self) -> VIRTUALID_R {
        VIRTUALID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - PHYSICALID"]
    #[inline(always)]
    pub fn physicalid(&self) -> PHYSICALID_R {
        PHYSICALID_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 23:27 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - STATE"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - GRP1"]
    #[inline(always)]
    pub fn grp1(&self) -> GRP1_R {
        GRP1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - HW"]
    #[inline(always)]
    pub fn hw(&self) -> HW_R {
        HW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - VIRTUALID"]
    #[inline(always)]
    pub fn virtualid(&mut self) -> VIRTUALID_W {
        VIRTUALID_W { w: self }
    }
    #[doc = "Bits 10:19 - PHYSICALID"]
    #[inline(always)]
    pub fn physicalid(&mut self) -> PHYSICALID_W {
        PHYSICALID_W { w: self }
    }
    #[doc = "Bits 23:27 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&mut self) -> PRIORITY_W {
        PRIORITY_W { w: self }
    }
    #[doc = "Bits 28:29 - STATE"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
    #[doc = "Bit 30 - GRP1"]
    #[inline(always)]
    pub fn grp1(&mut self) -> GRP1_W {
        GRP1_W { w: self }
    }
    #[doc = "Bit 31 - HW"]
    #[inline(always)]
    pub fn hw(&mut self) -> HW_W {
        HW_W { w: self }
    }
}
