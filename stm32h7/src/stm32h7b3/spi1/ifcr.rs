#[doc = "Register `IFCR` writer"]
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SUSPend flag clear"]
pub type SUSPC_AW = EOTC_AW;
#[doc = "Field `SUSPC` writer - SUSPend flag clear"]
pub struct SUSPC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSPC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SUSPC_AW::CLEAR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "TSERFC flag clear"]
pub type TSERFC_AW = EOTC_AW;
#[doc = "Field `TSERFC` writer - TSERFC flag clear"]
pub struct TSERFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSERFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSERFC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TSERFC_AW::CLEAR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Mode Fault flag clear"]
pub type MODFC_AW = EOTC_AW;
#[doc = "Field `MODFC` writer - Mode Fault flag clear"]
pub struct MODFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MODFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODFC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MODFC_AW::CLEAR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "TI frame format error flag clear"]
pub type TIFREC_AW = EOTC_AW;
#[doc = "Field `TIFREC` writer - TI frame format error flag clear"]
pub struct TIFREC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIFREC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIFREC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TIFREC_AW::CLEAR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "CRC Error flag clear"]
pub type CRCEC_AW = EOTC_AW;
#[doc = "Field `CRCEC` writer - CRC Error flag clear"]
pub struct CRCEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCEC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CRCEC_AW::CLEAR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Overrun flag clear"]
pub type OVRC_AW = EOTC_AW;
#[doc = "Field `OVRC` writer - Overrun flag clear"]
pub struct OVRC_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVRC_AW::CLEAR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Underrun flag clear"]
pub type UDRC_AW = EOTC_AW;
#[doc = "Field `UDRC` writer - Underrun flag clear"]
pub struct UDRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UDRC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UDRC_AW::CLEAR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Transmission Transfer Filled flag clear"]
pub type TXTFC_AW = EOTC_AW;
#[doc = "Field `TXTFC` writer - Transmission Transfer Filled flag clear"]
pub struct TXTFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXTFC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXTFC_AW::CLEAR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "End Of Transfer flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOTC_AW {
    #[doc = "1: Clear interrupt flag"]
    CLEAR = 1,
}
impl From<EOTC_AW> for bool {
    #[inline(always)]
    fn from(variant: EOTC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOTC` writer - End Of Transfer flag clear"]
pub struct EOTC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOTC_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOTC_AW::CLEAR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl W {
    #[doc = "Bit 11 - SUSPend flag clear"]
    #[inline(always)]
    pub fn suspc(&mut self) -> SUSPC_W {
        SUSPC_W { w: self }
    }
    #[doc = "Bit 10 - TSERFC flag clear"]
    #[inline(always)]
    pub fn tserfc(&mut self) -> TSERFC_W {
        TSERFC_W { w: self }
    }
    #[doc = "Bit 9 - Mode Fault flag clear"]
    #[inline(always)]
    pub fn modfc(&mut self) -> MODFC_W {
        MODFC_W { w: self }
    }
    #[doc = "Bit 8 - TI frame format error flag clear"]
    #[inline(always)]
    pub fn tifrec(&mut self) -> TIFREC_W {
        TIFREC_W { w: self }
    }
    #[doc = "Bit 7 - CRC Error flag clear"]
    #[inline(always)]
    pub fn crcec(&mut self) -> CRCEC_W {
        CRCEC_W { w: self }
    }
    #[doc = "Bit 6 - Overrun flag clear"]
    #[inline(always)]
    pub fn ovrc(&mut self) -> OVRC_W {
        OVRC_W { w: self }
    }
    #[doc = "Bit 5 - Underrun flag clear"]
    #[inline(always)]
    pub fn udrc(&mut self) -> UDRC_W {
        UDRC_W { w: self }
    }
    #[doc = "Bit 4 - Transmission Transfer Filled flag clear"]
    #[inline(always)]
    pub fn txtfc(&mut self) -> TXTFC_W {
        TXTFC_W { w: self }
    }
    #[doc = "Bit 3 - End Of Transfer flag clear"]
    #[inline(always)]
    pub fn eotc(&mut self) -> EOTC_W {
        EOTC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt/Status Flags Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifcr::W](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
