#[doc = "Register `ADC_CCR` reader"]
pub struct R(crate::R<ADC_CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CCR` writer"]
pub struct W(crate::W<ADC_CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CCR_SPEC>;
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
impl From<crate::W<ADC_CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC0` reader - PRESC0"]
pub struct PRESC0_R(crate::FieldReader<bool, bool>);
impl PRESC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRESC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESC0` writer - PRESC0"]
pub struct PRESC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC0_W<'a> {
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
#[doc = "Field `PRESC1` reader - PRESC1"]
pub struct PRESC1_R(crate::FieldReader<bool, bool>);
impl PRESC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRESC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESC1` writer - PRESC1"]
pub struct PRESC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC1_W<'a> {
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
#[doc = "Field `PRESC2` reader - PRESC2"]
pub struct PRESC2_R(crate::FieldReader<bool, bool>);
impl PRESC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRESC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESC2` writer - PRESC2"]
pub struct PRESC2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC2_W<'a> {
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
#[doc = "Field `PRESC3` reader - PRESC3"]
pub struct PRESC3_R(crate::FieldReader<bool, bool>);
impl PRESC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRESC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESC3` writer - PRESC3"]
pub struct PRESC3_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC3_W<'a> {
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
#[doc = "Field `VREFEN` reader - VREFEN"]
pub struct VREFEN_R(crate::FieldReader<bool, bool>);
impl VREFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREFEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREFEN` writer - VREFEN"]
pub struct VREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFEN_W<'a> {
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
#[doc = "Field `TSEN` reader - TSEN"]
pub struct TSEN_R(crate::FieldReader<bool, bool>);
impl TSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEN` writer - TSEN"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
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
#[doc = "Field `VBATEN` reader - VBATEN"]
pub struct VBATEN_R(crate::FieldReader<bool, bool>);
impl VBATEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBATEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBATEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBATEN` writer - VBATEN"]
pub struct VBATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATEN_W<'a> {
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
    #[doc = "Bit 18 - PRESC0"]
    #[inline(always)]
    pub fn presc0(&self) -> PRESC0_R {
        PRESC0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PRESC1"]
    #[inline(always)]
    pub fn presc1(&self) -> PRESC1_R {
        PRESC1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PRESC2"]
    #[inline(always)]
    pub fn presc2(&self) -> PRESC2_R {
        PRESC2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - PRESC3"]
    #[inline(always)]
    pub fn presc3(&self) -> PRESC3_R {
        PRESC3_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - TSEN"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - VBATEN"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - PRESC0"]
    #[inline(always)]
    pub fn presc0(&mut self) -> PRESC0_W {
        PRESC0_W { w: self }
    }
    #[doc = "Bit 19 - PRESC1"]
    #[inline(always)]
    pub fn presc1(&mut self) -> PRESC1_W {
        PRESC1_W { w: self }
    }
    #[doc = "Bit 20 - PRESC2"]
    #[inline(always)]
    pub fn presc2(&mut self) -> PRESC2_W {
        PRESC2_W { w: self }
    }
    #[doc = "Bit 21 - PRESC3"]
    #[inline(always)]
    pub fn presc3(&mut self) -> PRESC3_W {
        PRESC3_W { w: self }
    }
    #[doc = "Bit 22 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W {
        VREFEN_W { w: self }
    }
    #[doc = "Bit 23 - TSEN"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
    #[doc = "Bit 24 - VBATEN"]
    #[inline(always)]
    pub fn vbaten(&mut self) -> VBATEN_W {
        VBATEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC common configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ccr](index.html) module"]
pub struct ADC_CCR_SPEC;
impl crate::RegisterSpec for ADC_CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ccr::R](R) reader structure"]
impl crate::Readable for ADC_CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ccr::W](W) writer structure"]
impl crate::Writable for ADC_CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CCR to value 0"]
impl crate::Resettable for ADC_CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
