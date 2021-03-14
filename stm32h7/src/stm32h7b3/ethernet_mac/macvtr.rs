#[doc = "Reader of register MACVTR"]
pub type R = crate::R<u32, super::MACVTR>;
#[doc = "Writer for register MACVTR"]
pub type W = crate::W<u32, super::MACVTR>;
#[doc = "Register MACVTR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACVTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VL`"]
pub type VL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VL`"]
pub struct VL_W<'a> {
    w: &'a mut W,
}
impl<'a> VL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ETV`"]
pub type ETV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETV`"]
pub struct ETV_W<'a> {
    w: &'a mut W,
}
impl<'a> ETV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `VTIM`"]
pub type VTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTIM`"]
pub struct VTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ESVL`"]
pub type ESVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESVL`"]
pub struct ESVL_W<'a> {
    w: &'a mut W,
}
impl<'a> ESVL_W<'a> {
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
#[doc = "Reader of field `ERSVLM`"]
pub type ERSVLM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERSVLM`"]
pub struct ERSVLM_W<'a> {
    w: &'a mut W,
}
impl<'a> ERSVLM_W<'a> {
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
#[doc = "Reader of field `DOVLTC`"]
pub type DOVLTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOVLTC`"]
pub struct DOVLTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DOVLTC_W<'a> {
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
#[doc = "Reader of field `EVLS`"]
pub type EVLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EVLS`"]
pub struct EVLS_W<'a> {
    w: &'a mut W,
}
impl<'a> EVLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `EVLRXS`"]
pub type EVLRXS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVLRXS`"]
pub struct EVLRXS_W<'a> {
    w: &'a mut W,
}
impl<'a> EVLRXS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `VTHM`"]
pub type VTHM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VTHM`"]
pub struct VTHM_W<'a> {
    w: &'a mut W,
}
impl<'a> VTHM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `EDVLP`"]
pub type EDVLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDVLP`"]
pub struct EDVLP_W<'a> {
    w: &'a mut W,
}
impl<'a> EDVLP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `ERIVLT`"]
pub type ERIVLT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERIVLT`"]
pub struct ERIVLT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERIVLT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `EIVLS`"]
pub type EIVLS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EIVLS`"]
pub struct EIVLS_W<'a> {
    w: &'a mut W,
}
impl<'a> EIVLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `EIVLRXS`"]
pub type EIVLRXS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EIVLRXS`"]
pub struct EIVLRXS_W<'a> {
    w: &'a mut W,
}
impl<'a> EIVLRXS_W<'a> {
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
    #[doc = "Bits 0:15 - VL"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - ETV"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - VTIM"]
    #[inline(always)]
    pub fn vtim(&self) -> VTIM_R {
        VTIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ESVL"]
    #[inline(always)]
    pub fn esvl(&self) -> ESVL_R {
        ESVL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ERSVLM"]
    #[inline(always)]
    pub fn ersvlm(&self) -> ERSVLM_R {
        ERSVLM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DOVLTC"]
    #[inline(always)]
    pub fn dovltc(&self) -> DOVLTC_R {
        DOVLTC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - EVLS"]
    #[inline(always)]
    pub fn evls(&self) -> EVLS_R {
        EVLS_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 24 - EVLRXS"]
    #[inline(always)]
    pub fn evlrxs(&self) -> EVLRXS_R {
        EVLRXS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - VTHM"]
    #[inline(always)]
    pub fn vthm(&self) -> VTHM_R {
        VTHM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - EDVLP"]
    #[inline(always)]
    pub fn edvlp(&self) -> EDVLP_R {
        EDVLP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ERIVLT"]
    #[inline(always)]
    pub fn erivlt(&self) -> ERIVLT_R {
        ERIVLT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - EIVLS"]
    #[inline(always)]
    pub fn eivls(&self) -> EIVLS_R {
        EIVLS_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 31 - EIVLRXS"]
    #[inline(always)]
    pub fn eivlrxs(&self) -> EIVLRXS_R {
        EIVLRXS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VL"]
    #[inline(always)]
    pub fn vl(&mut self) -> VL_W {
        VL_W { w: self }
    }
    #[doc = "Bit 16 - ETV"]
    #[inline(always)]
    pub fn etv(&mut self) -> ETV_W {
        ETV_W { w: self }
    }
    #[doc = "Bit 17 - VTIM"]
    #[inline(always)]
    pub fn vtim(&mut self) -> VTIM_W {
        VTIM_W { w: self }
    }
    #[doc = "Bit 18 - ESVL"]
    #[inline(always)]
    pub fn esvl(&mut self) -> ESVL_W {
        ESVL_W { w: self }
    }
    #[doc = "Bit 19 - ERSVLM"]
    #[inline(always)]
    pub fn ersvlm(&mut self) -> ERSVLM_W {
        ERSVLM_W { w: self }
    }
    #[doc = "Bit 20 - DOVLTC"]
    #[inline(always)]
    pub fn dovltc(&mut self) -> DOVLTC_W {
        DOVLTC_W { w: self }
    }
    #[doc = "Bits 21:22 - EVLS"]
    #[inline(always)]
    pub fn evls(&mut self) -> EVLS_W {
        EVLS_W { w: self }
    }
    #[doc = "Bit 24 - EVLRXS"]
    #[inline(always)]
    pub fn evlrxs(&mut self) -> EVLRXS_W {
        EVLRXS_W { w: self }
    }
    #[doc = "Bit 25 - VTHM"]
    #[inline(always)]
    pub fn vthm(&mut self) -> VTHM_W {
        VTHM_W { w: self }
    }
    #[doc = "Bit 26 - EDVLP"]
    #[inline(always)]
    pub fn edvlp(&mut self) -> EDVLP_W {
        EDVLP_W { w: self }
    }
    #[doc = "Bit 27 - ERIVLT"]
    #[inline(always)]
    pub fn erivlt(&mut self) -> ERIVLT_W {
        ERIVLT_W { w: self }
    }
    #[doc = "Bits 28:29 - EIVLS"]
    #[inline(always)]
    pub fn eivls(&mut self) -> EIVLS_W {
        EIVLS_W { w: self }
    }
    #[doc = "Bit 31 - EIVLRXS"]
    #[inline(always)]
    pub fn eivlrxs(&mut self) -> EIVLRXS_W {
        EIVLRXS_W { w: self }
    }
}
