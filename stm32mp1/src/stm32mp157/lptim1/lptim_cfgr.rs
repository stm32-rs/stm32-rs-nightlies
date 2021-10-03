#[doc = "Register `LPTIM_CFGR` reader"]
pub struct R(crate::R<LPTIM_CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTIM_CFGR` writer"]
pub struct W(crate::W<LPTIM_CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_CFGR_SPEC>;
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
impl From<crate::W<LPTIM_CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKSEL` reader - CKSEL"]
pub struct CKSEL_R(crate::FieldReader<bool, bool>);
impl CKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKSEL` writer - CKSEL"]
pub struct CKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSEL_W<'a> {
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
#[doc = "Field `CKPOL` reader - CKPOL"]
pub struct CKPOL_R(crate::FieldReader<u8, u8>);
impl CKPOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKPOL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKPOL` writer - CKPOL"]
pub struct CKPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `CKFLT` reader - CKFLT"]
pub struct CKFLT_R(crate::FieldReader<u8, u8>);
impl CKFLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKFLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKFLT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKFLT` writer - CKFLT"]
pub struct CKFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> CKFLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `TRGFLT` reader - TRGFLT"]
pub struct TRGFLT_R(crate::FieldReader<u8, u8>);
impl TRGFLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGFLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGFLT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGFLT` writer - TRGFLT"]
pub struct TRGFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGFLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `PRESC` reader - PRESC"]
pub struct PRESC_R(crate::FieldReader<u8, u8>);
impl PRESC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESC` writer - PRESC"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `TRIGSEL` reader - TRIGSEL"]
pub struct TRIGSEL_R(crate::FieldReader<u8, u8>);
impl TRIGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIGSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGSEL` writer - TRIGSEL"]
pub struct TRIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `TRIGEN` reader - TRIGEN"]
pub struct TRIGEN_R(crate::FieldReader<u8, u8>);
impl TRIGEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGEN` writer - TRIGEN"]
pub struct TRIGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `TIMOUT` reader - TIMOUT"]
pub struct TIMOUT_R(crate::FieldReader<bool, bool>);
impl TIMOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMOUT` writer - TIMOUT"]
pub struct TIMOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `WAVE` reader - WAVE"]
pub struct WAVE_R(crate::FieldReader<bool, bool>);
impl WAVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVE` writer - WAVE"]
pub struct WAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `WAVPOL` reader - WAVPOL"]
pub struct WAVPOL_R(crate::FieldReader<bool, bool>);
impl WAVPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAVPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAVPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVPOL` writer - WAVPOL"]
pub struct WAVPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `PRELOAD` reader - PRELOAD"]
pub struct PRELOAD_R(crate::FieldReader<bool, bool>);
impl PRELOAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRELOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRELOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRELOAD` writer - PRELOAD"]
pub struct PRELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRELOAD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `COUNTMODE` reader - COUNTMODE"]
pub struct COUNTMODE_R(crate::FieldReader<bool, bool>);
impl COUNTMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COUNTMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTMODE` writer - COUNTMODE"]
pub struct COUNTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ENC` reader - ENC"]
pub struct ENC_R(crate::FieldReader<bool, bool>);
impl ENC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENC` writer - ENC"]
pub struct ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CKSEL"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - CKPOL"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - CKFLT"]
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - TRGFLT"]
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 9:11 - PRESC"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 13:15 - TRIGSEL"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 17:18 - TRIGEN"]
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - TIMOUT"]
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - WAVE"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - WAVPOL"]
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - PRELOAD"]
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - COUNTMODE"]
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ENC"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CKSEL"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W {
        CKSEL_W { w: self }
    }
    #[doc = "Bits 1:2 - CKPOL"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W {
        CKPOL_W { w: self }
    }
    #[doc = "Bits 3:4 - CKFLT"]
    #[inline(always)]
    pub fn ckflt(&mut self) -> CKFLT_W {
        CKFLT_W { w: self }
    }
    #[doc = "Bits 6:7 - TRGFLT"]
    #[inline(always)]
    pub fn trgflt(&mut self) -> TRGFLT_W {
        TRGFLT_W { w: self }
    }
    #[doc = "Bits 9:11 - PRESC"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bits 13:15 - TRIGSEL"]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W {
        TRIGSEL_W { w: self }
    }
    #[doc = "Bits 17:18 - TRIGEN"]
    #[inline(always)]
    pub fn trigen(&mut self) -> TRIGEN_W {
        TRIGEN_W { w: self }
    }
    #[doc = "Bit 19 - TIMOUT"]
    #[inline(always)]
    pub fn timout(&mut self) -> TIMOUT_W {
        TIMOUT_W { w: self }
    }
    #[doc = "Bit 20 - WAVE"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W {
        WAVE_W { w: self }
    }
    #[doc = "Bit 21 - WAVPOL"]
    #[inline(always)]
    pub fn wavpol(&mut self) -> WAVPOL_W {
        WAVPOL_W { w: self }
    }
    #[doc = "Bit 22 - PRELOAD"]
    #[inline(always)]
    pub fn preload(&mut self) -> PRELOAD_W {
        PRELOAD_W { w: self }
    }
    #[doc = "Bit 23 - COUNTMODE"]
    #[inline(always)]
    pub fn countmode(&mut self) -> COUNTMODE_W {
        COUNTMODE_W { w: self }
    }
    #[doc = "Bit 24 - ENC"]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPTIM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cfgr](index.html) module"]
pub struct LPTIM_CFGR_SPEC;
impl crate::RegisterSpec for LPTIM_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_cfgr::R](R) reader structure"]
impl crate::Readable for LPTIM_CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptim_cfgr::W](W) writer structure"]
impl crate::Writable for LPTIM_CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPTIM_CFGR to value 0"]
impl crate::Resettable for LPTIM_CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
