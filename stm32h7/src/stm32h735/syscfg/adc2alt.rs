#[doc = "Register `ADC2ALT` reader"]
pub struct R(crate::R<ADC2ALT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC2ALT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC2ALT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC2ALT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC2ALT` writer"]
pub struct W(crate::W<ADC2ALT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC2ALT_SPEC>;
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
impl From<crate::W<ADC2ALT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC2ALT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC2_ROUT1` reader - ADC2 V_INP17 alternate connection"]
pub struct ADC2_ROUT1_R(crate::FieldReader<bool, bool>);
impl ADC2_ROUT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_ROUT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_ROUT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_ROUT1` writer - ADC2 V_INP17 alternate connection"]
pub struct ADC2_ROUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_ROUT1_W<'a> {
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
#[doc = "Field `ADC2_ROUT0` reader - ADC2 V_INP16 alternate connection"]
pub struct ADC2_ROUT0_R(crate::FieldReader<bool, bool>);
impl ADC2_ROUT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_ROUT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_ROUT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_ROUT0` writer - ADC2 V_INP16 alternate connection"]
pub struct ADC2_ROUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_ROUT0_W<'a> {
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
    #[doc = "Bit 1 - ADC2 V_INP17 alternate connection"]
    #[inline(always)]
    pub fn adc2_rout1(&self) -> ADC2_ROUT1_R {
        ADC2_ROUT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ADC2 V_INP16 alternate connection"]
    #[inline(always)]
    pub fn adc2_rout0(&self) -> ADC2_ROUT0_R {
        ADC2_ROUT0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ADC2 V_INP17 alternate connection"]
    #[inline(always)]
    pub fn adc2_rout1(&mut self) -> ADC2_ROUT1_W {
        ADC2_ROUT1_W { w: self }
    }
    #[doc = "Bit 0 - ADC2 V_INP16 alternate connection"]
    #[inline(always)]
    pub fn adc2_rout0(&mut self) -> ADC2_ROUT0_W {
        ADC2_ROUT0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC2 internal input alternate connection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc2alt](index.html) module"]
pub struct ADC2ALT_SPEC;
impl crate::RegisterSpec for ADC2ALT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc2alt::R](R) reader structure"]
impl crate::Readable for ADC2ALT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc2alt::W](W) writer structure"]
impl crate::Writable for ADC2ALT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC2ALT to value 0"]
impl crate::Resettable for ADC2ALT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
