#[doc = "Reader of register OTG_GUSBCFG"]
pub type R = crate::R<u32, super::OTG_GUSBCFG>;
#[doc = "Writer for register OTG_GUSBCFG"]
pub type W = crate::W<u32, super::OTG_GUSBCFG>;
#[doc = "Register OTG_GUSBCFG `reset()`'s with value 0x1400"]
impl crate::ResetValue for super::OTG_GUSBCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1400
    }
}
#[doc = "Reader of field `TOCAL`"]
pub type TOCAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOCAL`"]
pub struct TOCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `PHYSEL`"]
pub type PHYSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHYSEL`"]
pub struct PHYSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYSEL_W<'a> {
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
#[doc = "Reader of field `SRPCAP`"]
pub type SRPCAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRPCAP`"]
pub struct SRPCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPCAP_W<'a> {
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
#[doc = "Reader of field `HNPCAP`"]
pub type HNPCAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HNPCAP`"]
pub struct HNPCAP_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPCAP_W<'a> {
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
#[doc = "Reader of field `TRDT`"]
pub type TRDT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRDT`"]
pub struct TRDT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `PHYLPC`"]
pub type PHYLPC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHYLPC`"]
pub struct PHYLPC_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYLPC_W<'a> {
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
#[doc = "Reader of field `TSDPS`"]
pub type TSDPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSDPS`"]
pub struct TSDPS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSDPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `FHMOD`"]
pub type FHMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FHMOD`"]
pub struct FHMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> FHMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `FDMOD`"]
pub type FDMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FDMOD`"]
pub struct FDMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> FDMOD_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - TOCAL"]
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 6 - PHYSEL"]
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SRPCAP"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HNPCAP"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - TRDT"]
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - PHYLPC"]
    #[inline(always)]
    pub fn phylpc(&self) -> PHYLPC_R {
        PHYLPC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TSDPS"]
    #[inline(always)]
    pub fn tsdps(&self) -> TSDPS_R {
        TSDPS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 29 - FHMOD"]
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - FDMOD"]
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TOCAL"]
    #[inline(always)]
    pub fn tocal(&mut self) -> TOCAL_W {
        TOCAL_W { w: self }
    }
    #[doc = "Bit 6 - PHYSEL"]
    #[inline(always)]
    pub fn physel(&mut self) -> PHYSEL_W {
        PHYSEL_W { w: self }
    }
    #[doc = "Bit 8 - SRPCAP"]
    #[inline(always)]
    pub fn srpcap(&mut self) -> SRPCAP_W {
        SRPCAP_W { w: self }
    }
    #[doc = "Bit 9 - HNPCAP"]
    #[inline(always)]
    pub fn hnpcap(&mut self) -> HNPCAP_W {
        HNPCAP_W { w: self }
    }
    #[doc = "Bits 10:13 - TRDT"]
    #[inline(always)]
    pub fn trdt(&mut self) -> TRDT_W {
        TRDT_W { w: self }
    }
    #[doc = "Bit 15 - PHYLPC"]
    #[inline(always)]
    pub fn phylpc(&mut self) -> PHYLPC_W {
        PHYLPC_W { w: self }
    }
    #[doc = "Bit 22 - TSDPS"]
    #[inline(always)]
    pub fn tsdps(&mut self) -> TSDPS_W {
        TSDPS_W { w: self }
    }
    #[doc = "Bit 29 - FHMOD"]
    #[inline(always)]
    pub fn fhmod(&mut self) -> FHMOD_W {
        FHMOD_W { w: self }
    }
    #[doc = "Bit 30 - FDMOD"]
    #[inline(always)]
    pub fn fdmod(&mut self) -> FDMOD_W {
        FDMOD_W { w: self }
    }
}
