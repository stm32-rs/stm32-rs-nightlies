#[doc = "Reader of register OTG_GINTSTS"]
pub type R = crate::R<u32, super::OTG_GINTSTS>;
#[doc = "Writer for register OTG_GINTSTS"]
pub type W = crate::W<u32, super::OTG_GINTSTS>;
#[doc = "Register OTG_GINTSTS `reset()`'s with value 0x1400_0020"]
impl crate::ResetValue for super::OTG_GINTSTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1400_0020
    }
}
#[doc = "Reader of field `CMOD`"]
pub type CMOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMIS`"]
pub type MMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MMIS`"]
pub struct MMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MMIS_W<'a> {
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
#[doc = "Reader of field `OTGINT`"]
pub type OTGINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF`"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
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
#[doc = "Reader of field `RXFLVL`"]
pub type RXFLVL_R = crate::R<bool, bool>;
#[doc = "Reader of field `NPTXFE`"]
pub type NPTXFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `GINAKEFF`"]
pub type GINAKEFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `GONAKEFF`"]
pub type GONAKEFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ESUSP`"]
pub type ESUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESUSP`"]
pub struct ESUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> ESUSP_W<'a> {
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
#[doc = "Reader of field `USBSUSP`"]
pub type USBSUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBSUSP`"]
pub struct USBSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSUSP_W<'a> {
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
#[doc = "Reader of field `USBRST`"]
pub type USBRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBRST`"]
pub struct USBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRST_W<'a> {
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
#[doc = "Reader of field `ENUMDNE`"]
pub type ENUMDNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENUMDNE`"]
pub struct ENUMDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUMDNE_W<'a> {
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
#[doc = "Reader of field `ISOODRP`"]
pub type ISOODRP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOODRP`"]
pub struct ISOODRP_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOODRP_W<'a> {
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
#[doc = "Reader of field `EOPF`"]
pub type EOPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOPF`"]
pub struct EOPF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPF_W<'a> {
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
#[doc = "Reader of field `IEPINT`"]
pub type IEPINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OEPINT`"]
pub type OEPINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `IISOIXFR`"]
pub type IISOIXFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IISOIXFR`"]
pub struct IISOIXFR_W<'a> {
    w: &'a mut W,
}
impl<'a> IISOIXFR_W<'a> {
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
#[doc = "Reader of field `IPXFR`"]
pub type IPXFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPXFR`"]
pub struct IPXFR_W<'a> {
    w: &'a mut W,
}
impl<'a> IPXFR_W<'a> {
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
#[doc = "Reader of field `DATAFSUSP`"]
pub type DATAFSUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAFSUSP`"]
pub struct DATAFSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAFSUSP_W<'a> {
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
#[doc = "Reader of field `HPRTINT`"]
pub type HPRTINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `HCINT`"]
pub type HCINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PTXFE`"]
pub type PTXFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CIDSCHG`"]
pub type CIDSCHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CIDSCHG`"]
pub struct CIDSCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> CIDSCHG_W<'a> {
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
#[doc = "Reader of field `DISCINT`"]
pub type DISCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCINT`"]
pub struct DISCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCINT_W<'a> {
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
#[doc = "Reader of field `SRQINT`"]
pub type SRQINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRQINT`"]
pub struct SRQINT_W<'a> {
    w: &'a mut W,
}
impl<'a> SRQINT_W<'a> {
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
#[doc = "Reader of field `WKUPINT`"]
pub type WKUPINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPINT`"]
pub struct WKUPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPINT_W<'a> {
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
    #[doc = "Bit 0 - CMOD"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MMIS"]
    #[inline(always)]
    pub fn mmis(&self) -> MMIS_R {
        MMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTGINT"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SOF"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RXFLVL"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - NPTXFE"]
    #[inline(always)]
    pub fn nptxfe(&self) -> NPTXFE_R {
        NPTXFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GINAKEFF"]
    #[inline(always)]
    pub fn ginakeff(&self) -> GINAKEFF_R {
        GINAKEFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GONAKEFF"]
    #[inline(always)]
    pub fn gonakeff(&self) -> GONAKEFF_R {
        GONAKEFF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ESUSP"]
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USBSUSP"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ENUMDNE"]
    #[inline(always)]
    pub fn enumdne(&self) -> ENUMDNE_R {
        ENUMDNE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ISOODRP"]
    #[inline(always)]
    pub fn isoodrp(&self) -> ISOODRP_R {
        ISOODRP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EOPF"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IEPINT"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OEPINT"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IISOIXFR"]
    #[inline(always)]
    pub fn iisoixfr(&self) -> IISOIXFR_R {
        IISOIXFR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - IPXFR"]
    #[inline(always)]
    pub fn ipxfr(&self) -> IPXFR_R {
        IPXFR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DATAFSUSP"]
    #[inline(always)]
    pub fn datafsusp(&self) -> DATAFSUSP_R {
        DATAFSUSP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - HPRTINT"]
    #[inline(always)]
    pub fn hprtint(&self) -> HPRTINT_R {
        HPRTINT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - HCINT"]
    #[inline(always)]
    pub fn hcint(&self) -> HCINT_R {
        HCINT_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PTXFE"]
    #[inline(always)]
    pub fn ptxfe(&self) -> PTXFE_R {
        PTXFE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CIDSCHG"]
    #[inline(always)]
    pub fn cidschg(&self) -> CIDSCHG_R {
        CIDSCHG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - SRQINT"]
    #[inline(always)]
    pub fn srqint(&self) -> SRQINT_R {
        SRQINT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - WKUPINT"]
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - MMIS"]
    #[inline(always)]
    pub fn mmis(&mut self) -> MMIS_W {
        MMIS_W { w: self }
    }
    #[doc = "Bit 3 - SOF"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 10 - ESUSP"]
    #[inline(always)]
    pub fn esusp(&mut self) -> ESUSP_W {
        ESUSP_W { w: self }
    }
    #[doc = "Bit 11 - USBSUSP"]
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W {
        USBSUSP_W { w: self }
    }
    #[doc = "Bit 12 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 13 - ENUMDNE"]
    #[inline(always)]
    pub fn enumdne(&mut self) -> ENUMDNE_W {
        ENUMDNE_W { w: self }
    }
    #[doc = "Bit 14 - ISOODRP"]
    #[inline(always)]
    pub fn isoodrp(&mut self) -> ISOODRP_W {
        ISOODRP_W { w: self }
    }
    #[doc = "Bit 15 - EOPF"]
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W {
        EOPF_W { w: self }
    }
    #[doc = "Bit 20 - IISOIXFR"]
    #[inline(always)]
    pub fn iisoixfr(&mut self) -> IISOIXFR_W {
        IISOIXFR_W { w: self }
    }
    #[doc = "Bit 21 - IPXFR"]
    #[inline(always)]
    pub fn ipxfr(&mut self) -> IPXFR_W {
        IPXFR_W { w: self }
    }
    #[doc = "Bit 22 - DATAFSUSP"]
    #[inline(always)]
    pub fn datafsusp(&mut self) -> DATAFSUSP_W {
        DATAFSUSP_W { w: self }
    }
    #[doc = "Bit 28 - CIDSCHG"]
    #[inline(always)]
    pub fn cidschg(&mut self) -> CIDSCHG_W {
        CIDSCHG_W { w: self }
    }
    #[doc = "Bit 29 - DISCINT"]
    #[inline(always)]
    pub fn discint(&mut self) -> DISCINT_W {
        DISCINT_W { w: self }
    }
    #[doc = "Bit 30 - SRQINT"]
    #[inline(always)]
    pub fn srqint(&mut self) -> SRQINT_W {
        SRQINT_W { w: self }
    }
    #[doc = "Bit 31 - WKUPINT"]
    #[inline(always)]
    pub fn wkupint(&mut self) -> WKUPINT_W {
        WKUPINT_W { w: self }
    }
}
