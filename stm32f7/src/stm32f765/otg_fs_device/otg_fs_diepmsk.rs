#[doc = "Register `OTG_FS_DIEPMSK` reader"]
pub struct R(crate::R<OTG_FS_DIEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_DIEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_DIEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_DIEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_DIEPMSK` writer"]
pub struct W(crate::W<OTG_FS_DIEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_DIEPMSK_SPEC>;
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
impl From<crate::W<OTG_FS_DIEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_DIEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub struct XFRCM_R(crate::FieldReader<bool, bool>);
impl XFRCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        XFRCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFRCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub struct EPDM_R(crate::FieldReader<bool, bool>);
impl EPDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub struct EPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TOM` reader - Timeout condition mask (Non-isochronous endpoints)"]
pub struct TOM_R(crate::FieldReader<bool, bool>);
impl TOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOM` writer - Timeout condition mask (Non-isochronous endpoints)"]
pub struct TOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TOM_W<'a> {
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
#[doc = "Field `ITTXFEMSK` reader - IN token received when TxFIFO empty mask"]
pub struct ITTXFEMSK_R(crate::FieldReader<bool, bool>);
impl ITTXFEMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITTXFEMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITTXFEMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITTXFEMSK` writer - IN token received when TxFIFO empty mask"]
pub struct ITTXFEMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ITTXFEMSK_W<'a> {
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
#[doc = "Field `INEPNMM` reader - IN token received with EP mismatch mask"]
pub struct INEPNMM_R(crate::FieldReader<bool, bool>);
impl INEPNMM_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEPNMM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPNMM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPNMM` writer - IN token received with EP mismatch mask"]
pub struct INEPNMM_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNMM_W<'a> {
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
#[doc = "Field `INEPNEM` reader - IN endpoint NAK effective mask"]
pub struct INEPNEM_R(crate::FieldReader<bool, bool>);
impl INEPNEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEPNEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPNEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPNEM` writer - IN endpoint NAK effective mask"]
pub struct INEPNEM_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNEM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timeout condition mask (Non-isochronous endpoints)"]
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&self) -> INEPNMM_R {
        INEPNMM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W {
        XFRCM_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W {
        EPDM_W { w: self }
    }
    #[doc = "Bit 3 - Timeout condition mask (Non-isochronous endpoints)"]
    #[inline(always)]
    pub fn tom(&mut self) -> TOM_W {
        TOM_W { w: self }
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W {
        ITTXFEMSK_W { w: self }
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&mut self) -> INEPNMM_W {
        INEPNMM_W { w: self }
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&mut self) -> INEPNEM_W {
        INEPNEM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_diepmsk](index.html) module"]
pub struct OTG_FS_DIEPMSK_SPEC;
impl crate::RegisterSpec for OTG_FS_DIEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_diepmsk::R](R) reader structure"]
impl crate::Readable for OTG_FS_DIEPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_diepmsk::W](W) writer structure"]
impl crate::Writable for OTG_FS_DIEPMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_DIEPMSK to value 0"]
impl crate::Resettable for OTG_FS_DIEPMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
