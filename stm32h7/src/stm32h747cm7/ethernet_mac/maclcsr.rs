#[doc = "Reader of register MACLCSR"]
pub type R = crate::R<u32, super::MACLCSR>;
#[doc = "Writer for register MACLCSR"]
pub type W = crate::W<u32, super::MACLCSR>;
#[doc = "Register MACLCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACLCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TLPIEN`"]
pub type TLPIEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TLPIEX`"]
pub type TLPIEX_R = crate::R<bool, bool>;
#[doc = "Reader of field `RLPIEN`"]
pub type RLPIEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RLPIEX`"]
pub type RLPIEX_R = crate::R<bool, bool>;
#[doc = "Reader of field `TLPIST`"]
pub type TLPIST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RLPIST`"]
pub type RLPIST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPIEN`"]
pub type LPIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPIEN`"]
pub struct LPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIEN_W<'a> {
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
#[doc = "Reader of field `PLS`"]
pub type PLS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLS`"]
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
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
#[doc = "Reader of field `PLSEN`"]
pub type PLSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLSEN`"]
pub struct PLSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLSEN_W<'a> {
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
#[doc = "Reader of field `LPITXA`"]
pub type LPITXA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPITXA`"]
pub struct LPITXA_W<'a> {
    w: &'a mut W,
}
impl<'a> LPITXA_W<'a> {
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
#[doc = "Reader of field `LPITE`"]
pub type LPITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPITE`"]
pub struct LPITE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPITE_W<'a> {
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
#[doc = "Reader of field `LPITCSE`"]
pub type LPITCSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPITCSE`"]
pub struct LPITCSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPITCSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit LPI Entry"]
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit LPI Exit"]
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive LPI Entry"]
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive LPI Exit"]
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit LPI State"]
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive LPI State"]
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LPI Enable"]
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PHY Link Status"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PHY Link Status Enable"]
    #[inline(always)]
    pub fn plsen(&self) -> PLSEN_R {
        PLSEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LPI Tx Automate"]
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPI Timer Enable"]
    #[inline(always)]
    pub fn lpite(&self) -> LPITE_R {
        LPITE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPITCSE"]
    #[inline(always)]
    pub fn lpitcse(&self) -> LPITCSE_R {
        LPITCSE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - LPI Enable"]
    #[inline(always)]
    pub fn lpien(&mut self) -> LPIEN_W {
        LPIEN_W { w: self }
    }
    #[doc = "Bit 17 - PHY Link Status"]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    #[doc = "Bit 18 - PHY Link Status Enable"]
    #[inline(always)]
    pub fn plsen(&mut self) -> PLSEN_W {
        PLSEN_W { w: self }
    }
    #[doc = "Bit 19 - LPI Tx Automate"]
    #[inline(always)]
    pub fn lpitxa(&mut self) -> LPITXA_W {
        LPITXA_W { w: self }
    }
    #[doc = "Bit 20 - LPI Timer Enable"]
    #[inline(always)]
    pub fn lpite(&mut self) -> LPITE_W {
        LPITE_W { w: self }
    }
    #[doc = "Bit 21 - LPITCSE"]
    #[inline(always)]
    pub fn lpitcse(&mut self) -> LPITCSE_W {
        LPITCSE_W { w: self }
    }
}
