#[doc = "Reader of register SECSR"]
pub type R = crate::R<u32, super::SECSR>;
#[doc = "Writer for register SECSR"]
pub type W = crate::W<u32, super::SECSR>;
#[doc = "Register SECSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SECSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SECEOP`"]
pub type SECEOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECEOP`"]
pub struct SECEOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SECEOP_W<'a> {
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
#[doc = "Reader of field `SECOPERR`"]
pub type SECOPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECOPERR`"]
pub struct SECOPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECOPERR_W<'a> {
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
#[doc = "Reader of field `SECPROGERR`"]
pub type SECPROGERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECPROGERR`"]
pub struct SECPROGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPROGERR_W<'a> {
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
#[doc = "Reader of field `SECWRPERR`"]
pub type SECWRPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECWRPERR`"]
pub struct SECWRPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECWRPERR_W<'a> {
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
#[doc = "Reader of field `SECPGAERR`"]
pub type SECPGAERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECPGAERR`"]
pub struct SECPGAERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPGAERR_W<'a> {
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
#[doc = "Reader of field `SECSIZERR`"]
pub type SECSIZERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECSIZERR`"]
pub struct SECSIZERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECSIZERR_W<'a> {
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
#[doc = "Reader of field `SECPGSERR`"]
pub type SECPGSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECPGSERR`"]
pub struct SECPGSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECPGSERR_W<'a> {
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
#[doc = "Reader of field `SECRDERR`"]
pub type SECRDERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SECRDERR`"]
pub struct SECRDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECRDERR_W<'a> {
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
#[doc = "Reader of field `SECBSY`"]
pub type SECBSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SECEOP"]
    #[inline(always)]
    pub fn seceop(&self) -> SECEOP_R {
        SECEOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SECOPERR"]
    #[inline(always)]
    pub fn secoperr(&self) -> SECOPERR_R {
        SECOPERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SECPROGERR"]
    #[inline(always)]
    pub fn secprogerr(&self) -> SECPROGERR_R {
        SECPROGERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SECWRPERR"]
    #[inline(always)]
    pub fn secwrperr(&self) -> SECWRPERR_R {
        SECWRPERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SECPGAERR"]
    #[inline(always)]
    pub fn secpgaerr(&self) -> SECPGAERR_R {
        SECPGAERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SECSIZERR"]
    #[inline(always)]
    pub fn secsizerr(&self) -> SECSIZERR_R {
        SECSIZERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SECPGSERR"]
    #[inline(always)]
    pub fn secpgserr(&self) -> SECPGSERR_R {
        SECPGSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Secure read protection error"]
    #[inline(always)]
    pub fn secrderr(&self) -> SECRDERR_R {
        SECRDERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SECBusy"]
    #[inline(always)]
    pub fn secbsy(&self) -> SECBSY_R {
        SECBSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SECEOP"]
    #[inline(always)]
    pub fn seceop(&mut self) -> SECEOP_W {
        SECEOP_W { w: self }
    }
    #[doc = "Bit 1 - SECOPERR"]
    #[inline(always)]
    pub fn secoperr(&mut self) -> SECOPERR_W {
        SECOPERR_W { w: self }
    }
    #[doc = "Bit 3 - SECPROGERR"]
    #[inline(always)]
    pub fn secprogerr(&mut self) -> SECPROGERR_W {
        SECPROGERR_W { w: self }
    }
    #[doc = "Bit 4 - SECWRPERR"]
    #[inline(always)]
    pub fn secwrperr(&mut self) -> SECWRPERR_W {
        SECWRPERR_W { w: self }
    }
    #[doc = "Bit 5 - SECPGAERR"]
    #[inline(always)]
    pub fn secpgaerr(&mut self) -> SECPGAERR_W {
        SECPGAERR_W { w: self }
    }
    #[doc = "Bit 6 - SECSIZERR"]
    #[inline(always)]
    pub fn secsizerr(&mut self) -> SECSIZERR_W {
        SECSIZERR_W { w: self }
    }
    #[doc = "Bit 7 - SECPGSERR"]
    #[inline(always)]
    pub fn secpgserr(&mut self) -> SECPGSERR_W {
        SECPGSERR_W { w: self }
    }
    #[doc = "Bit 14 - Secure read protection error"]
    #[inline(always)]
    pub fn secrderr(&mut self) -> SECRDERR_W {
        SECRDERR_W { w: self }
    }
}
