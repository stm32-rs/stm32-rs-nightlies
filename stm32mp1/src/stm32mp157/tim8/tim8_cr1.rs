#[doc = "Register `TIM8_CR1` reader"]
pub struct R(crate::R<TIM8_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM8_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM8_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM8_CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM8_CR1` writer"]
pub struct W(crate::W<TIM8_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM8_CR1_SPEC>;
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
impl From<crate::W<TIM8_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM8_CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEN` reader - CEN"]
pub struct CEN_R(crate::FieldReader<bool, bool>);
impl CEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEN` writer - CEN"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `UDIS` reader - UDIS"]
pub struct UDIS_R(crate::FieldReader<bool, bool>);
impl UDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        UDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDIS` writer - UDIS"]
pub struct UDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `URS` reader - URS"]
pub struct URS_R(crate::FieldReader<bool, bool>);
impl URS_R {
    pub(crate) fn new(bits: bool) -> Self {
        URS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URS` writer - URS"]
pub struct URS_W<'a> {
    w: &'a mut W,
}
impl<'a> URS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `OPM` reader - OPM"]
pub struct OPM_R(crate::FieldReader<bool, bool>);
impl OPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPM` writer - OPM"]
pub struct OPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DIR` reader - DIR"]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - DIR"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CMS` reader - CMS"]
pub struct CMS_R(crate::FieldReader<u8, u8>);
impl CMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMS` writer - CMS"]
pub struct CMS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u16 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `ARPE` reader - ARPE"]
pub struct ARPE_R(crate::FieldReader<bool, bool>);
impl ARPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARPE` writer - ARPE"]
pub struct ARPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CKD` reader - CKD"]
pub struct CKD_R(crate::FieldReader<u8, u8>);
impl CKD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKD` writer - CKD"]
pub struct CKD_W<'a> {
    w: &'a mut W,
}
impl<'a> CKD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u16 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `UIFREMAP` reader - UIFREMAP"]
pub struct UIFREMAP_R(crate::FieldReader<bool, bool>);
impl UIFREMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        UIFREMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UIFREMAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UIFREMAP` writer - UIFREMAP"]
pub struct UIFREMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> UIFREMAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CEN"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UDIS"]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - URS"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OPM"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - CMS"]
    #[inline(always)]
    pub fn cms(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - ARPE"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - CKD"]
    #[inline(always)]
    pub fn ckd(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - UIFREMAP"]
    #[inline(always)]
    pub fn uifremap(&self) -> UIFREMAP_R {
        UIFREMAP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CEN"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    #[doc = "Bit 1 - UDIS"]
    #[inline(always)]
    pub fn udis(&mut self) -> UDIS_W {
        UDIS_W { w: self }
    }
    #[doc = "Bit 2 - URS"]
    #[inline(always)]
    pub fn urs(&mut self) -> URS_W {
        URS_W { w: self }
    }
    #[doc = "Bit 3 - OPM"]
    #[inline(always)]
    pub fn opm(&mut self) -> OPM_W {
        OPM_W { w: self }
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bits 5:6 - CMS"]
    #[inline(always)]
    pub fn cms(&mut self) -> CMS_W {
        CMS_W { w: self }
    }
    #[doc = "Bit 7 - ARPE"]
    #[inline(always)]
    pub fn arpe(&mut self) -> ARPE_W {
        ARPE_W { w: self }
    }
    #[doc = "Bits 8:9 - CKD"]
    #[inline(always)]
    pub fn ckd(&mut self) -> CKD_W {
        CKD_W { w: self }
    }
    #[doc = "Bit 11 - UIFREMAP"]
    #[inline(always)]
    pub fn uifremap(&mut self) -> UIFREMAP_W {
        UIFREMAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM8 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim8_cr1](index.html) module"]
pub struct TIM8_CR1_SPEC;
impl crate::RegisterSpec for TIM8_CR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim8_cr1::R](R) reader structure"]
impl crate::Readable for TIM8_CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim8_cr1::W](W) writer structure"]
impl crate::Writable for TIM8_CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM8_CR1 to value 0"]
impl crate::Resettable for TIM8_CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
