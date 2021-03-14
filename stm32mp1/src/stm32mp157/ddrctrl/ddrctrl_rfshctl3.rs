#[doc = "Reader of register DDRCTRL_RFSHCTL3"]
pub type R = crate::R<u32, super::DDRCTRL_RFSHCTL3>;
#[doc = "Writer for register DDRCTRL_RFSHCTL3"]
pub type W = crate::W<u32, super::DDRCTRL_RFSHCTL3>;
#[doc = "Register DDRCTRL_RFSHCTL3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_RFSHCTL3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIS_AUTO_REFRESH`"]
pub type DIS_AUTO_REFRESH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_AUTO_REFRESH`"]
pub struct DIS_AUTO_REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_AUTO_REFRESH_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `REFRESH_UPDATE_LEVEL`"]
pub type REFRESH_UPDATE_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFRESH_UPDATE_LEVEL`"]
pub struct REFRESH_UPDATE_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_UPDATE_LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DIS_AUTO_REFRESH"]
    #[inline(always)]
    pub fn dis_auto_refresh(&self) -> DIS_AUTO_REFRESH_R {
        DIS_AUTO_REFRESH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - REFRESH_UPDATE_LEVEL"]
    #[inline(always)]
    pub fn refresh_update_level(&self) -> REFRESH_UPDATE_LEVEL_R {
        REFRESH_UPDATE_LEVEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIS_AUTO_REFRESH"]
    #[inline(always)]
    pub fn dis_auto_refresh(&mut self) -> DIS_AUTO_REFRESH_W {
        DIS_AUTO_REFRESH_W { w: self }
    }
    #[doc = "Bit 1 - REFRESH_UPDATE_LEVEL"]
    #[inline(always)]
    pub fn refresh_update_level(&mut self) -> REFRESH_UPDATE_LEVEL_W {
        REFRESH_UPDATE_LEVEL_W { w: self }
    }
}
