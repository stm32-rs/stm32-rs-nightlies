#[doc = "Register `SECCFGR` reader"]
pub struct R(crate::R<SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR` writer"]
pub struct W(crate::W<SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR_SPEC>;
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
impl From<crate::W<SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APCSEC` reader - APCSEC"]
pub struct APCSEC_R(crate::FieldReader<bool, bool>);
impl APCSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        APCSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APCSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APCSEC` writer - APCSEC"]
pub struct APCSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> APCSEC_W<'a> {
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
#[doc = "Field `VBSEC` reader - VBSEC"]
pub struct VBSEC_R(crate::FieldReader<bool, bool>);
impl VBSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBSEC` writer - VBSEC"]
pub struct VBSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> VBSEC_W<'a> {
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
#[doc = "Field `VDMSEC` reader - VDMSEC"]
pub struct VDMSEC_R(crate::FieldReader<bool, bool>);
impl VDMSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDMSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDMSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDMSEC` writer - VDMSEC"]
pub struct VDMSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> VDMSEC_W<'a> {
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
#[doc = "Field `LPMSEC` reader - LPMSEC"]
pub struct LPMSEC_R(crate::FieldReader<bool, bool>);
impl LPMSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPMSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMSEC` writer - LPMSEC"]
pub struct LPMSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMSEC_W<'a> {
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
#[doc = "Field `WUP5SEC` reader - WKUP5 pin security"]
pub struct WUP5SEC_R(crate::FieldReader<bool, bool>);
impl WUP5SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUP5SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUP5SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUP5SEC` writer - WKUP5 pin security"]
pub struct WUP5SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP5SEC_W<'a> {
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
#[doc = "Field `WUP4SEC` reader - WKUP4 pin security"]
pub struct WUP4SEC_R(crate::FieldReader<bool, bool>);
impl WUP4SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUP4SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUP4SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUP4SEC` writer - WKUP4 pin security"]
pub struct WUP4SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP4SEC_W<'a> {
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
#[doc = "Field `WUP3SEC` reader - WKUP3 pin security"]
pub struct WUP3SEC_R(crate::FieldReader<bool, bool>);
impl WUP3SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUP3SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUP3SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUP3SEC` writer - WKUP3 pin security"]
pub struct WUP3SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP3SEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `WUP2SEC` reader - WKUP2 pin security"]
pub struct WUP2SEC_R(crate::FieldReader<bool, bool>);
impl WUP2SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUP2SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUP2SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUP2SEC` writer - WKUP2 pin security"]
pub struct WUP2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP2SEC_W<'a> {
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
#[doc = "Field `WUP1SEC` reader - WKUP1 pin security"]
pub struct WUP1SEC_R(crate::FieldReader<bool, bool>);
impl WUP1SEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUP1SEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUP1SEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUP1SEC` writer - WKUP1 pin security"]
pub struct WUP1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP1SEC_W<'a> {
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
impl R {
    #[doc = "Bit 11 - APCSEC"]
    #[inline(always)]
    pub fn apcsec(&self) -> APCSEC_R {
        APCSEC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VBSEC"]
    #[inline(always)]
    pub fn vbsec(&self) -> VBSEC_R {
        VBSEC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VDMSEC"]
    #[inline(always)]
    pub fn vdmsec(&self) -> VDMSEC_R {
        VDMSEC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LPMSEC"]
    #[inline(always)]
    pub fn lpmsec(&self) -> LPMSEC_R {
        LPMSEC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WKUP5 pin security"]
    #[inline(always)]
    pub fn wup5sec(&self) -> WUP5SEC_R {
        WUP5SEC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WKUP4 pin security"]
    #[inline(always)]
    pub fn wup4sec(&self) -> WUP4SEC_R {
        WUP4SEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WKUP3 pin security"]
    #[inline(always)]
    pub fn wup3sec(&self) -> WUP3SEC_R {
        WUP3SEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - WKUP2 pin security"]
    #[inline(always)]
    pub fn wup2sec(&self) -> WUP2SEC_R {
        WUP2SEC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - WKUP1 pin security"]
    #[inline(always)]
    pub fn wup1sec(&self) -> WUP1SEC_R {
        WUP1SEC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - APCSEC"]
    #[inline(always)]
    pub fn apcsec(&mut self) -> APCSEC_W {
        APCSEC_W { w: self }
    }
    #[doc = "Bit 10 - VBSEC"]
    #[inline(always)]
    pub fn vbsec(&mut self) -> VBSEC_W {
        VBSEC_W { w: self }
    }
    #[doc = "Bit 9 - VDMSEC"]
    #[inline(always)]
    pub fn vdmsec(&mut self) -> VDMSEC_W {
        VDMSEC_W { w: self }
    }
    #[doc = "Bit 8 - LPMSEC"]
    #[inline(always)]
    pub fn lpmsec(&mut self) -> LPMSEC_W {
        LPMSEC_W { w: self }
    }
    #[doc = "Bit 4 - WKUP5 pin security"]
    #[inline(always)]
    pub fn wup5sec(&mut self) -> WUP5SEC_W {
        WUP5SEC_W { w: self }
    }
    #[doc = "Bit 3 - WKUP4 pin security"]
    #[inline(always)]
    pub fn wup4sec(&mut self) -> WUP4SEC_W {
        WUP4SEC_W { w: self }
    }
    #[doc = "Bit 2 - WKUP3 pin security"]
    #[inline(always)]
    pub fn wup3sec(&mut self) -> WUP3SEC_W {
        WUP3SEC_W { w: self }
    }
    #[doc = "Bit 1 - WKUP2 pin security"]
    #[inline(always)]
    pub fn wup2sec(&mut self) -> WUP2SEC_W {
        WUP2SEC_W { w: self }
    }
    #[doc = "Bit 0 - WKUP1 pin security"]
    #[inline(always)]
    pub fn wup1sec(&mut self) -> WUP1SEC_W {
        WUP1SEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power secure configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr](index.html) module"]
pub struct SECCFGR_SPEC;
impl crate::RegisterSpec for SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr::R](R) reader structure"]
impl crate::Readable for SECCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr::W](W) writer structure"]
impl crate::Writable for SECCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
