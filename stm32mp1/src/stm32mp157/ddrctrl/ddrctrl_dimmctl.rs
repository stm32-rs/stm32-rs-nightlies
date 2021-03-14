#[doc = "Reader of register DDRCTRL_DIMMCTL"]
pub type R = crate::R<u32, super::DDRCTRL_DIMMCTL>;
#[doc = "Writer for register DDRCTRL_DIMMCTL"]
pub type W = crate::W<u32, super::DDRCTRL_DIMMCTL>;
#[doc = "Register DDRCTRL_DIMMCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_DIMMCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIMM_STAGGER_CS_EN`"]
pub type DIMM_STAGGER_CS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIMM_STAGGER_CS_EN`"]
pub struct DIMM_STAGGER_CS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIMM_STAGGER_CS_EN_W<'a> {
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
#[doc = "Reader of field `DIMM_ADDR_MIRR_EN`"]
pub type DIMM_ADDR_MIRR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIMM_ADDR_MIRR_EN`"]
pub struct DIMM_ADDR_MIRR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIMM_ADDR_MIRR_EN_W<'a> {
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
    #[doc = "Bit 0 - DIMM_STAGGER_CS_EN"]
    #[inline(always)]
    pub fn dimm_stagger_cs_en(&self) -> DIMM_STAGGER_CS_EN_R {
        DIMM_STAGGER_CS_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIMM_ADDR_MIRR_EN"]
    #[inline(always)]
    pub fn dimm_addr_mirr_en(&self) -> DIMM_ADDR_MIRR_EN_R {
        DIMM_ADDR_MIRR_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIMM_STAGGER_CS_EN"]
    #[inline(always)]
    pub fn dimm_stagger_cs_en(&mut self) -> DIMM_STAGGER_CS_EN_W {
        DIMM_STAGGER_CS_EN_W { w: self }
    }
    #[doc = "Bit 1 - DIMM_ADDR_MIRR_EN"]
    #[inline(always)]
    pub fn dimm_addr_mirr_en(&mut self) -> DIMM_ADDR_MIRR_EN_W {
        DIMM_ADDR_MIRR_EN_W { w: self }
    }
}
