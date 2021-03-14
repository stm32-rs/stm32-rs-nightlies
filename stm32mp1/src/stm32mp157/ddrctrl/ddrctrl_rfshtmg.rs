#[doc = "Reader of register DDRCTRL_RFSHTMG"]
pub type R = crate::R<u32, super::DDRCTRL_RFSHTMG>;
#[doc = "Writer for register DDRCTRL_RFSHTMG"]
pub type W = crate::W<u32, super::DDRCTRL_RFSHTMG>;
#[doc = "Register DDRCTRL_RFSHTMG `reset()`'s with value 0x0062_008c"]
impl crate::ResetValue for super::DDRCTRL_RFSHTMG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0062_008c
    }
}
#[doc = "Reader of field `T_RFC_MIN`"]
pub type T_RFC_MIN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `T_RFC_MIN`"]
pub struct T_RFC_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> T_RFC_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `LPDDR3_TREFBW_EN`"]
pub type LPDDR3_TREFBW_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPDDR3_TREFBW_EN`"]
pub struct LPDDR3_TREFBW_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDDR3_TREFBW_EN_W<'a> {
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
#[doc = "Reader of field `T_RFC_NOM_X1_X32`"]
pub type T_RFC_NOM_X1_X32_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `T_RFC_NOM_X1_X32`"]
pub struct T_RFC_NOM_X1_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> T_RFC_NOM_X1_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `T_RFC_NOM_X1_SEL`"]
pub type T_RFC_NOM_X1_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T_RFC_NOM_X1_SEL`"]
pub struct T_RFC_NOM_X1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> T_RFC_NOM_X1_SEL_W<'a> {
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
    #[doc = "Bits 0:9 - T_RFC_MIN"]
    #[inline(always)]
    pub fn t_rfc_min(&self) -> T_RFC_MIN_R {
        T_RFC_MIN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - LPDDR3_TREFBW_EN"]
    #[inline(always)]
    pub fn lpddr3_trefbw_en(&self) -> LPDDR3_TREFBW_EN_R {
        LPDDR3_TREFBW_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - T_RFC_NOM_X1_X32"]
    #[inline(always)]
    pub fn t_rfc_nom_x1_x32(&self) -> T_RFC_NOM_X1_X32_R {
        T_RFC_NOM_X1_X32_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - T_RFC_NOM_X1_SEL"]
    #[inline(always)]
    pub fn t_rfc_nom_x1_sel(&self) -> T_RFC_NOM_X1_SEL_R {
        T_RFC_NOM_X1_SEL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - T_RFC_MIN"]
    #[inline(always)]
    pub fn t_rfc_min(&mut self) -> T_RFC_MIN_W {
        T_RFC_MIN_W { w: self }
    }
    #[doc = "Bit 15 - LPDDR3_TREFBW_EN"]
    #[inline(always)]
    pub fn lpddr3_trefbw_en(&mut self) -> LPDDR3_TREFBW_EN_W {
        LPDDR3_TREFBW_EN_W { w: self }
    }
    #[doc = "Bits 16:27 - T_RFC_NOM_X1_X32"]
    #[inline(always)]
    pub fn t_rfc_nom_x1_x32(&mut self) -> T_RFC_NOM_X1_X32_W {
        T_RFC_NOM_X1_X32_W { w: self }
    }
    #[doc = "Bit 31 - T_RFC_NOM_X1_SEL"]
    #[inline(always)]
    pub fn t_rfc_nom_x1_sel(&mut self) -> T_RFC_NOM_X1_SEL_W {
        T_RFC_NOM_X1_SEL_W { w: self }
    }
}
