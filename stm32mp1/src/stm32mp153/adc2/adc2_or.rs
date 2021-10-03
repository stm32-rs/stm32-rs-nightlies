#[doc = "Register `ADC2_OR` reader"]
pub struct R(crate::R<ADC2_OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC2_OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC2_OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC2_OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC2_OR` writer"]
pub struct W(crate::W<ADC2_OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC2_OR_SPEC>;
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
impl From<crate::W<ADC2_OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC2_OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDDCOREEN` reader - VDDCOREEN"]
pub struct VDDCOREEN_R(crate::FieldReader<bool, bool>);
impl VDDCOREEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDDCOREEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDCOREEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDCOREEN` writer - VDDCOREEN"]
pub struct VDDCOREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDCOREEN_W<'a> {
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
    #[doc = "Bit 0 - VDDCOREEN"]
    #[inline(always)]
    pub fn vddcoreen(&self) -> VDDCOREEN_R {
        VDDCOREEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VDDCOREEN"]
    #[inline(always)]
    pub fn vddcoreen(&mut self) -> VDDCOREEN_W {
        VDDCOREEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC2 option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc2_or](index.html) module"]
pub struct ADC2_OR_SPEC;
impl crate::RegisterSpec for ADC2_OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc2_or::R](R) reader structure"]
impl crate::Readable for ADC2_OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc2_or::W](W) writer structure"]
impl crate::Writable for ADC2_OR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC2_OR to value 0"]
impl crate::Resettable for ADC2_OR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
