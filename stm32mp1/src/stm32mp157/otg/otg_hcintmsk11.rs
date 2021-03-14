#[doc = "Reader of register OTG_HCINTMSK11"]
pub type R = crate::R<u32, super::OTG_HCINTMSK11>;
#[doc = "Writer for register OTG_HCINTMSK11"]
pub type W = crate::W<u32, super::OTG_HCINTMSK11>;
#[doc = "Register OTG_HCINTMSK11 `reset()`'s with value 0"]
impl crate::ResetValue for super::OTG_HCINTMSK11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XFRCM`"]
pub type XFRCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XFRCM`"]
pub struct XFRCM_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRCM_W<'a> {
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
#[doc = "Reader of field `CHHM`"]
pub type CHHM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHHM`"]
pub struct CHHM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHHM_W<'a> {
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
#[doc = "Reader of field `AHBERRM`"]
pub type AHBERRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBERRM`"]
pub struct AHBERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBERRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `STALLM`"]
pub type STALLM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STALLM`"]
pub struct STALLM_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `NAKM`"]
pub type NAKM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NAKM`"]
pub struct NAKM_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ACKM`"]
pub type ACKM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACKM`"]
pub struct ACKM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `NYET`"]
pub type NYET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NYET`"]
pub struct NYET_W<'a> {
    w: &'a mut W,
}
impl<'a> NYET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `TXERRM`"]
pub type TXERRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXERRM`"]
pub struct TXERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `BBERRM`"]
pub type BBERRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BBERRM`"]
pub struct BBERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> BBERRM_W<'a> {
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
#[doc = "Reader of field `FRMORM`"]
pub type FRMORM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMORM`"]
pub struct FRMORM_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMORM_W<'a> {
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
#[doc = "Reader of field `DTERRM`"]
pub type DTERRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTERRM`"]
pub struct DTERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> DTERRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `BNAMSK`"]
pub type BNAMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BNAMSK`"]
pub struct BNAMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> BNAMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DESCLSTROLLMSK`"]
pub type DESCLSTROLLMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DESCLSTROLLMSK`"]
pub struct DESCLSTROLLMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCLSTROLLMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CHHM"]
    #[inline(always)]
    pub fn chhm(&self) -> CHHM_R {
        CHHM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STALLM"]
    #[inline(always)]
    pub fn stallm(&self) -> STALLM_R {
        STALLM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKM"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ACKM"]
    #[inline(always)]
    pub fn ackm(&self) -> ACKM_R {
        ACKM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NYET"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TXERRM"]
    #[inline(always)]
    pub fn txerrm(&self) -> TXERRM_R {
        TXERRM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BBERRM"]
    #[inline(always)]
    pub fn bberrm(&self) -> BBERRM_R {
        BBERRM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FRMORM"]
    #[inline(always)]
    pub fn frmorm(&self) -> FRMORM_R {
        FRMORM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DTERRM"]
    #[inline(always)]
    pub fn dterrm(&self) -> DTERRM_R {
        DTERRM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BNAMSK"]
    #[inline(always)]
    pub fn bnamsk(&self) -> BNAMSK_R {
        BNAMSK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DESCLSTROLLMSK"]
    #[inline(always)]
    pub fn desclstrollmsk(&self) -> DESCLSTROLLMSK_R {
        DESCLSTROLLMSK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W {
        XFRCM_W { w: self }
    }
    #[doc = "Bit 1 - CHHM"]
    #[inline(always)]
    pub fn chhm(&mut self) -> CHHM_W {
        CHHM_W { w: self }
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    pub fn ahberrm(&mut self) -> AHBERRM_W {
        AHBERRM_W { w: self }
    }
    #[doc = "Bit 3 - STALLM"]
    #[inline(always)]
    pub fn stallm(&mut self) -> STALLM_W {
        STALLM_W { w: self }
    }
    #[doc = "Bit 4 - NAKM"]
    #[inline(always)]
    pub fn nakm(&mut self) -> NAKM_W {
        NAKM_W { w: self }
    }
    #[doc = "Bit 5 - ACKM"]
    #[inline(always)]
    pub fn ackm(&mut self) -> ACKM_W {
        ACKM_W { w: self }
    }
    #[doc = "Bit 6 - NYET"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W {
        NYET_W { w: self }
    }
    #[doc = "Bit 7 - TXERRM"]
    #[inline(always)]
    pub fn txerrm(&mut self) -> TXERRM_W {
        TXERRM_W { w: self }
    }
    #[doc = "Bit 8 - BBERRM"]
    #[inline(always)]
    pub fn bberrm(&mut self) -> BBERRM_W {
        BBERRM_W { w: self }
    }
    #[doc = "Bit 9 - FRMORM"]
    #[inline(always)]
    pub fn frmorm(&mut self) -> FRMORM_W {
        FRMORM_W { w: self }
    }
    #[doc = "Bit 10 - DTERRM"]
    #[inline(always)]
    pub fn dterrm(&mut self) -> DTERRM_W {
        DTERRM_W { w: self }
    }
    #[doc = "Bit 11 - BNAMSK"]
    #[inline(always)]
    pub fn bnamsk(&mut self) -> BNAMSK_W {
        BNAMSK_W { w: self }
    }
    #[doc = "Bit 13 - DESCLSTROLLMSK"]
    #[inline(always)]
    pub fn desclstrollmsk(&mut self) -> DESCLSTROLLMSK_W {
        DESCLSTROLLMSK_W { w: self }
    }
}
