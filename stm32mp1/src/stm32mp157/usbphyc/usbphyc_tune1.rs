#[doc = "Reader of register USBPHYC_TUNE1"]
pub type R = crate::R<u32, super::USBPHYC_TUNE1>;
#[doc = "Writer for register USBPHYC_TUNE1"]
pub type W = crate::W<u32, super::USBPHYC_TUNE1>;
#[doc = "Register USBPHYC_TUNE1 `reset()`'s with value 0x0407_0004"]
impl crate::ResetValue for super::USBPHYC_TUNE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0407_0004
    }
}
#[doc = "Reader of field `INCURREN`"]
pub type INCURREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCURREN`"]
pub struct INCURREN_W<'a> {
    w: &'a mut W,
}
impl<'a> INCURREN_W<'a> {
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
#[doc = "Reader of field `INCURRINT`"]
pub type INCURRINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCURRINT`"]
pub struct INCURRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> INCURRINT_W<'a> {
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
#[doc = "Reader of field `LFSCAPEN`"]
pub type LFSCAPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFSCAPEN`"]
pub struct LFSCAPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSCAPEN_W<'a> {
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
#[doc = "Reader of field `HSDRVSLEW`"]
pub type HSDRVSLEW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSDRVSLEW`"]
pub struct HSDRVSLEW_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVSLEW_W<'a> {
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
#[doc = "Reader of field `HSDRVDCCUR`"]
pub type HSDRVDCCUR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSDRVDCCUR`"]
pub struct HSDRVDCCUR_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVDCCUR_W<'a> {
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
#[doc = "Reader of field `HSDRVDCLEV`"]
pub type HSDRVDCLEV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSDRVDCLEV`"]
pub struct HSDRVDCLEV_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVDCLEV_W<'a> {
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
#[doc = "Reader of field `HSDRVCURINCR`"]
pub type HSDRVCURINCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSDRVCURINCR`"]
pub struct HSDRVCURINCR_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVCURINCR_W<'a> {
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
#[doc = "Reader of field `FSDRVRFADJ`"]
pub type FSDRVRFADJ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSDRVRFADJ`"]
pub struct FSDRVRFADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> FSDRVRFADJ_W<'a> {
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
#[doc = "Reader of field `HSDRVRFRED`"]
pub type HSDRVRFRED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSDRVRFRED`"]
pub struct HSDRVRFRED_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVRFRED_W<'a> {
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
#[doc = "Reader of field `HSDRVCHKITRM`"]
pub type HSDRVCHKITRM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSDRVCHKITRM`"]
pub struct HSDRVCHKITRM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVCHKITRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u32) & 0x0f) << 9);
        self.w
    }
}
#[doc = "Reader of field `HSDRVCHKZTRM`"]
pub type HSDRVCHKZTRM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSDRVCHKZTRM`"]
pub struct HSDRVCHKZTRM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDRVCHKZTRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `OTPCOMP`"]
pub type OTPCOMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OTPCOMP`"]
pub struct OTPCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPCOMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `SQLCHCTL`"]
pub type SQLCHCTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQLCHCTL`"]
pub struct SQLCHCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> SQLCHCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `HDRXGNEQEN`"]
pub type HDRXGNEQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDRXGNEQEN`"]
pub struct HDRXGNEQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRXGNEQEN_W<'a> {
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
#[doc = "Reader of field `HSRXOFF`"]
pub type HSRXOFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSRXOFF`"]
pub struct HSRXOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSRXOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
#[doc = "Reader of field `HSFALLPREEM`"]
pub type HSFALLPREEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSFALLPREEM`"]
pub struct HSFALLPREEM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSFALLPREEM_W<'a> {
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
#[doc = "Reader of field `SHTCCTCTLPROT`"]
pub type SHTCCTCTLPROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHTCCTCTLPROT`"]
pub struct SHTCCTCTLPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHTCCTCTLPROT_W<'a> {
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
#[doc = "Reader of field `STAGSEL`"]
pub type STAGSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STAGSEL`"]
pub struct STAGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STAGSEL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - INCURREN"]
    #[inline(always)]
    pub fn incurren(&self) -> INCURREN_R {
        INCURREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - INCURRINT"]
    #[inline(always)]
    pub fn incurrint(&self) -> INCURRINT_R {
        INCURRINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LFSCAPEN"]
    #[inline(always)]
    pub fn lfscapen(&self) -> LFSCAPEN_R {
        LFSCAPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSDRVSLEW"]
    #[inline(always)]
    pub fn hsdrvslew(&self) -> HSDRVSLEW_R {
        HSDRVSLEW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HSDRVDCCUR"]
    #[inline(always)]
    pub fn hsdrvdccur(&self) -> HSDRVDCCUR_R {
        HSDRVDCCUR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HSDRVDCLEV"]
    #[inline(always)]
    pub fn hsdrvdclev(&self) -> HSDRVDCLEV_R {
        HSDRVDCLEV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HSDRVCURINCR"]
    #[inline(always)]
    pub fn hsdrvcurincr(&self) -> HSDRVCURINCR_R {
        HSDRVCURINCR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FSDRVRFADJ"]
    #[inline(always)]
    pub fn fsdrvrfadj(&self) -> FSDRVRFADJ_R {
        FSDRVRFADJ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HSDRVRFRED"]
    #[inline(always)]
    pub fn hsdrvrfred(&self) -> HSDRVRFRED_R {
        HSDRVRFRED_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:12 - HSDRVCHKITRM"]
    #[inline(always)]
    pub fn hsdrvchkitrm(&self) -> HSDRVCHKITRM_R {
        HSDRVCHKITRM_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:14 - HSDRVCHKZTRM"]
    #[inline(always)]
    pub fn hsdrvchkztrm(&self) -> HSDRVCHKZTRM_R {
        HSDRVCHKZTRM_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 15:19 - OTPCOMP"]
    #[inline(always)]
    pub fn otpcomp(&self) -> OTPCOMP_R {
        OTPCOMP_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - SQLCHCTL"]
    #[inline(always)]
    pub fn sqlchctl(&self) -> SQLCHCTL_R {
        SQLCHCTL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - HDRXGNEQEN"]
    #[inline(always)]
    pub fn hdrxgneqen(&self) -> HDRXGNEQEN_R {
        HDRXGNEQEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 23:24 - HSRXOFF"]
    #[inline(always)]
    pub fn hsrxoff(&self) -> HSRXOFF_R {
        HSRXOFF_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bit 25 - HSFALLPREEM"]
    #[inline(always)]
    pub fn hsfallpreem(&self) -> HSFALLPREEM_R {
        HSFALLPREEM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - SHTCCTCTLPROT"]
    #[inline(always)]
    pub fn shtcctctlprot(&self) -> SHTCCTCTLPROT_R {
        SHTCCTCTLPROT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - STAGSEL"]
    #[inline(always)]
    pub fn stagsel(&self) -> STAGSEL_R {
        STAGSEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INCURREN"]
    #[inline(always)]
    pub fn incurren(&mut self) -> INCURREN_W {
        INCURREN_W { w: self }
    }
    #[doc = "Bit 1 - INCURRINT"]
    #[inline(always)]
    pub fn incurrint(&mut self) -> INCURRINT_W {
        INCURRINT_W { w: self }
    }
    #[doc = "Bit 2 - LFSCAPEN"]
    #[inline(always)]
    pub fn lfscapen(&mut self) -> LFSCAPEN_W {
        LFSCAPEN_W { w: self }
    }
    #[doc = "Bit 3 - HSDRVSLEW"]
    #[inline(always)]
    pub fn hsdrvslew(&mut self) -> HSDRVSLEW_W {
        HSDRVSLEW_W { w: self }
    }
    #[doc = "Bit 4 - HSDRVDCCUR"]
    #[inline(always)]
    pub fn hsdrvdccur(&mut self) -> HSDRVDCCUR_W {
        HSDRVDCCUR_W { w: self }
    }
    #[doc = "Bit 5 - HSDRVDCLEV"]
    #[inline(always)]
    pub fn hsdrvdclev(&mut self) -> HSDRVDCLEV_W {
        HSDRVDCLEV_W { w: self }
    }
    #[doc = "Bit 6 - HSDRVCURINCR"]
    #[inline(always)]
    pub fn hsdrvcurincr(&mut self) -> HSDRVCURINCR_W {
        HSDRVCURINCR_W { w: self }
    }
    #[doc = "Bit 7 - FSDRVRFADJ"]
    #[inline(always)]
    pub fn fsdrvrfadj(&mut self) -> FSDRVRFADJ_W {
        FSDRVRFADJ_W { w: self }
    }
    #[doc = "Bit 8 - HSDRVRFRED"]
    #[inline(always)]
    pub fn hsdrvrfred(&mut self) -> HSDRVRFRED_W {
        HSDRVRFRED_W { w: self }
    }
    #[doc = "Bits 9:12 - HSDRVCHKITRM"]
    #[inline(always)]
    pub fn hsdrvchkitrm(&mut self) -> HSDRVCHKITRM_W {
        HSDRVCHKITRM_W { w: self }
    }
    #[doc = "Bits 13:14 - HSDRVCHKZTRM"]
    #[inline(always)]
    pub fn hsdrvchkztrm(&mut self) -> HSDRVCHKZTRM_W {
        HSDRVCHKZTRM_W { w: self }
    }
    #[doc = "Bits 15:19 - OTPCOMP"]
    #[inline(always)]
    pub fn otpcomp(&mut self) -> OTPCOMP_W {
        OTPCOMP_W { w: self }
    }
    #[doc = "Bits 20:21 - SQLCHCTL"]
    #[inline(always)]
    pub fn sqlchctl(&mut self) -> SQLCHCTL_W {
        SQLCHCTL_W { w: self }
    }
    #[doc = "Bit 22 - HDRXGNEQEN"]
    #[inline(always)]
    pub fn hdrxgneqen(&mut self) -> HDRXGNEQEN_W {
        HDRXGNEQEN_W { w: self }
    }
    #[doc = "Bits 23:24 - HSRXOFF"]
    #[inline(always)]
    pub fn hsrxoff(&mut self) -> HSRXOFF_W {
        HSRXOFF_W { w: self }
    }
    #[doc = "Bit 25 - HSFALLPREEM"]
    #[inline(always)]
    pub fn hsfallpreem(&mut self) -> HSFALLPREEM_W {
        HSFALLPREEM_W { w: self }
    }
    #[doc = "Bit 26 - SHTCCTCTLPROT"]
    #[inline(always)]
    pub fn shtcctctlprot(&mut self) -> SHTCCTCTLPROT_W {
        SHTCCTCTLPROT_W { w: self }
    }
    #[doc = "Bit 27 - STAGSEL"]
    #[inline(always)]
    pub fn stagsel(&mut self) -> STAGSEL_W {
        STAGSEL_W { w: self }
    }
}
