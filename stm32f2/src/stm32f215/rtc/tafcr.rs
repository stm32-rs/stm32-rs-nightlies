#[doc = "Register `TAFCR` reader"]
pub struct R(crate::R<TAFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAFCR` writer"]
pub struct W(crate::W<TAFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAFCR_SPEC>;
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
impl From<crate::W<TAFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARMOUTTYPE` reader - AFO_ALARM output type"]
pub struct ALARMOUTTYPE_R(crate::FieldReader<bool, bool>);
impl ALARMOUTTYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALARMOUTTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARMOUTTYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARMOUTTYPE` writer - AFO_ALARM output type"]
pub struct ALARMOUTTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARMOUTTYPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `TSINSEL` reader - TIMESTAMP mapping"]
pub struct TSINSEL_R(crate::FieldReader<bool, bool>);
impl TSINSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSINSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSINSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSINSEL` writer - TIMESTAMP mapping"]
pub struct TSINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSINSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `TAMP1INSEL` reader - TAMPER1 mapping"]
pub struct TAMP1INSEL_R(crate::FieldReader<bool, bool>);
impl TAMP1INSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1INSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1INSEL` writer - TAMPER1 mapping"]
pub struct TAMP1INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1INSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub struct TAMPIE_R(crate::FieldReader<bool, bool>);
impl TAMPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable"]
pub struct TAMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMPIE_W<'a> {
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
#[doc = "Field `TAMP1TRG` reader - Active level for tamper 1"]
pub struct TAMP1TRG_R(crate::FieldReader<bool, bool>);
impl TAMP1TRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1TRG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1TRG` writer - Active level for tamper 1"]
pub struct TAMP1TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1TRG_W<'a> {
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
#[doc = "Field `TAMP1E` reader - Tamper 1 detection enable"]
pub struct TAMP1E_R(crate::FieldReader<bool, bool>);
impl TAMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1E` writer - Tamper 1 detection enable"]
pub struct TAMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1E_W<'a> {
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
    #[doc = "Bit 18 - AFO_ALARM output type"]
    #[inline(always)]
    pub fn alarmouttype(&self) -> ALARMOUTTYPE_R {
        ALARMOUTTYPE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIMESTAMP mapping"]
    #[inline(always)]
    pub fn tsinsel(&self) -> TSINSEL_R {
        TSINSEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TAMPER1 mapping"]
    #[inline(always)]
    pub fn tamp1insel(&self) -> TAMP1INSEL_R {
        TAMP1INSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - AFO_ALARM output type"]
    #[inline(always)]
    pub fn alarmouttype(&mut self) -> ALARMOUTTYPE_W {
        ALARMOUTTYPE_W { w: self }
    }
    #[doc = "Bit 17 - TIMESTAMP mapping"]
    #[inline(always)]
    pub fn tsinsel(&mut self) -> TSINSEL_W {
        TSINSEL_W { w: self }
    }
    #[doc = "Bit 16 - TAMPER1 mapping"]
    #[inline(always)]
    pub fn tamp1insel(&mut self) -> TAMP1INSEL_W {
        TAMP1INSEL_W { w: self }
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W {
        TAMPIE_W { w: self }
    }
    #[doc = "Bit 1 - Active level for tamper 1"]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W {
        TAMP1TRG_W { w: self }
    }
    #[doc = "Bit 0 - Tamper 1 detection enable"]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W {
        TAMP1E_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tamper and alternate function configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tafcr](index.html) module"]
pub struct TAFCR_SPEC;
impl crate::RegisterSpec for TAFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tafcr::R](R) reader structure"]
impl crate::Readable for TAFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tafcr::W](W) writer structure"]
impl crate::Writable for TAFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAFCR to value 0"]
impl crate::Resettable for TAFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
