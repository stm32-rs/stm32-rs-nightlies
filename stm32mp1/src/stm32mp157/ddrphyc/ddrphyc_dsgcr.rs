#[doc = "Reader of register DDRPHYC_DSGCR"]
pub type R = crate::R<u32, super::DDRPHYC_DSGCR>;
#[doc = "Writer for register DDRPHYC_DSGCR"]
pub type W = crate::W<u32, super::DDRPHYC_DSGCR>;
#[doc = "Register DDRPHYC_DSGCR `reset()`'s with value 0xfa00_001f"]
impl crate::ResetValue for super::DDRPHYC_DSGCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfa00_001f
    }
}
#[doc = "Reader of field `PUREN`"]
pub type PUREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUREN`"]
pub struct PUREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PUREN_W<'a> {
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
#[doc = "Reader of field `BDISEN`"]
pub type BDISEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BDISEN`"]
pub struct BDISEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BDISEN_W<'a> {
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
#[doc = "Reader of field `ZUEN`"]
pub type ZUEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ZUEN`"]
pub struct ZUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZUEN_W<'a> {
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
#[doc = "Reader of field `LPIOPD`"]
pub type LPIOPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPIOPD`"]
pub struct LPIOPD_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIOPD_W<'a> {
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
#[doc = "Reader of field `LPDLLPD`"]
pub type LPDLLPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPDLLPD`"]
pub struct LPDLLPD_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDLLPD_W<'a> {
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
#[doc = "Reader of field `DQSGX`"]
pub type DQSGX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQSGX`"]
pub struct DQSGX_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSGX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `DQSGE`"]
pub type DQSGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DQSGE`"]
pub struct DQSGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `NOBUB`"]
pub type NOBUB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOBUB`"]
pub struct NOBUB_W<'a> {
    w: &'a mut W,
}
impl<'a> NOBUB_W<'a> {
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
#[doc = "Reader of field `FXDLAT`"]
pub type FXDLAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FXDLAT`"]
pub struct FXDLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FXDLAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CKEPDD`"]
pub type CKEPDD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKEPDD`"]
pub struct CKEPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEPDD_W<'a> {
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
#[doc = "Reader of field `ODTPDD`"]
pub type ODTPDD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODTPDD`"]
pub struct ODTPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> ODTPDD_W<'a> {
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
#[doc = "Reader of field `NL2PD`"]
pub type NL2PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NL2PD`"]
pub struct NL2PD_W<'a> {
    w: &'a mut W,
}
impl<'a> NL2PD_W<'a> {
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
#[doc = "Reader of field `NL2OE`"]
pub type NL2OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NL2OE`"]
pub struct NL2OE_W<'a> {
    w: &'a mut W,
}
impl<'a> NL2OE_W<'a> {
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
#[doc = "Reader of field `TPDPD`"]
pub type TPDPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPDPD`"]
pub struct TPDPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TPDPD_W<'a> {
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
#[doc = "Reader of field `TPDOE`"]
pub type TPDOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPDOE`"]
pub struct TPDOE_W<'a> {
    w: &'a mut W,
}
impl<'a> TPDOE_W<'a> {
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
#[doc = "Reader of field `CKOE`"]
pub type CKOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKOE`"]
pub struct CKOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ODTOE`"]
pub type ODTOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODTOE`"]
pub struct ODTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> ODTOE_W<'a> {
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
#[doc = "Reader of field `RSTOE`"]
pub type RSTOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTOE`"]
pub struct RSTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTOE_W<'a> {
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
#[doc = "Reader of field `CKEOE`"]
pub type CKEOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKEOE`"]
pub struct CKEOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEOE_W<'a> {
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
    #[doc = "Bit 0 - PUREN"]
    #[inline(always)]
    pub fn puren(&self) -> PUREN_R {
        PUREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BDISEN"]
    #[inline(always)]
    pub fn bdisen(&self) -> BDISEN_R {
        BDISEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ZUEN"]
    #[inline(always)]
    pub fn zuen(&self) -> ZUEN_R {
        ZUEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPIOPD"]
    #[inline(always)]
    pub fn lpiopd(&self) -> LPIOPD_R {
        LPIOPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LPDLLPD"]
    #[inline(always)]
    pub fn lpdllpd(&self) -> LPDLLPD_R {
        LPDLLPD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - DQSGX"]
    #[inline(always)]
    pub fn dqsgx(&self) -> DQSGX_R {
        DQSGX_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - DQSGE"]
    #[inline(always)]
    pub fn dqsge(&self) -> DQSGE_R {
        DQSGE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - NOBUB"]
    #[inline(always)]
    pub fn nobub(&self) -> NOBUB_R {
        NOBUB_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FXDLAT"]
    #[inline(always)]
    pub fn fxdlat(&self) -> FXDLAT_R {
        FXDLAT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CKEPDD"]
    #[inline(always)]
    pub fn ckepdd(&self) -> CKEPDD_R {
        CKEPDD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ODTPDD"]
    #[inline(always)]
    pub fn odtpdd(&self) -> ODTPDD_R {
        ODTPDD_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - NL2PD"]
    #[inline(always)]
    pub fn nl2pd(&self) -> NL2PD_R {
        NL2PD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - NL2OE"]
    #[inline(always)]
    pub fn nl2oe(&self) -> NL2OE_R {
        NL2OE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TPDPD"]
    #[inline(always)]
    pub fn tpdpd(&self) -> TPDPD_R {
        TPDPD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TPDOE"]
    #[inline(always)]
    pub fn tpdoe(&self) -> TPDOE_R {
        TPDOE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CKOE"]
    #[inline(always)]
    pub fn ckoe(&self) -> CKOE_R {
        CKOE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ODTOE"]
    #[inline(always)]
    pub fn odtoe(&self) -> ODTOE_R {
        ODTOE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - RSTOE"]
    #[inline(always)]
    pub fn rstoe(&self) -> RSTOE_R {
        RSTOE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - CKEOE"]
    #[inline(always)]
    pub fn ckeoe(&self) -> CKEOE_R {
        CKEOE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PUREN"]
    #[inline(always)]
    pub fn puren(&mut self) -> PUREN_W {
        PUREN_W { w: self }
    }
    #[doc = "Bit 1 - BDISEN"]
    #[inline(always)]
    pub fn bdisen(&mut self) -> BDISEN_W {
        BDISEN_W { w: self }
    }
    #[doc = "Bit 2 - ZUEN"]
    #[inline(always)]
    pub fn zuen(&mut self) -> ZUEN_W {
        ZUEN_W { w: self }
    }
    #[doc = "Bit 3 - LPIOPD"]
    #[inline(always)]
    pub fn lpiopd(&mut self) -> LPIOPD_W {
        LPIOPD_W { w: self }
    }
    #[doc = "Bit 4 - LPDLLPD"]
    #[inline(always)]
    pub fn lpdllpd(&mut self) -> LPDLLPD_W {
        LPDLLPD_W { w: self }
    }
    #[doc = "Bits 5:7 - DQSGX"]
    #[inline(always)]
    pub fn dqsgx(&mut self) -> DQSGX_W {
        DQSGX_W { w: self }
    }
    #[doc = "Bits 8:10 - DQSGE"]
    #[inline(always)]
    pub fn dqsge(&mut self) -> DQSGE_W {
        DQSGE_W { w: self }
    }
    #[doc = "Bit 11 - NOBUB"]
    #[inline(always)]
    pub fn nobub(&mut self) -> NOBUB_W {
        NOBUB_W { w: self }
    }
    #[doc = "Bit 12 - FXDLAT"]
    #[inline(always)]
    pub fn fxdlat(&mut self) -> FXDLAT_W {
        FXDLAT_W { w: self }
    }
    #[doc = "Bit 16 - CKEPDD"]
    #[inline(always)]
    pub fn ckepdd(&mut self) -> CKEPDD_W {
        CKEPDD_W { w: self }
    }
    #[doc = "Bit 20 - ODTPDD"]
    #[inline(always)]
    pub fn odtpdd(&mut self) -> ODTPDD_W {
        ODTPDD_W { w: self }
    }
    #[doc = "Bit 24 - NL2PD"]
    #[inline(always)]
    pub fn nl2pd(&mut self) -> NL2PD_W {
        NL2PD_W { w: self }
    }
    #[doc = "Bit 25 - NL2OE"]
    #[inline(always)]
    pub fn nl2oe(&mut self) -> NL2OE_W {
        NL2OE_W { w: self }
    }
    #[doc = "Bit 26 - TPDPD"]
    #[inline(always)]
    pub fn tpdpd(&mut self) -> TPDPD_W {
        TPDPD_W { w: self }
    }
    #[doc = "Bit 27 - TPDOE"]
    #[inline(always)]
    pub fn tpdoe(&mut self) -> TPDOE_W {
        TPDOE_W { w: self }
    }
    #[doc = "Bit 28 - CKOE"]
    #[inline(always)]
    pub fn ckoe(&mut self) -> CKOE_W {
        CKOE_W { w: self }
    }
    #[doc = "Bit 29 - ODTOE"]
    #[inline(always)]
    pub fn odtoe(&mut self) -> ODTOE_W {
        ODTOE_W { w: self }
    }
    #[doc = "Bit 30 - RSTOE"]
    #[inline(always)]
    pub fn rstoe(&mut self) -> RSTOE_W {
        RSTOE_W { w: self }
    }
    #[doc = "Bit 31 - CKEOE"]
    #[inline(always)]
    pub fn ckeoe(&mut self) -> CKEOE_W {
        CKEOE_W { w: self }
    }
}
