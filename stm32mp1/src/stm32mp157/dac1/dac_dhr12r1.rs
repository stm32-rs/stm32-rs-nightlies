#[doc = "Register `DAC_DHR12R1` reader"]
pub struct R(crate::R<DAC_DHR12R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DHR12R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DHR12R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DHR12R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_DHR12R1` writer"]
pub struct W(crate::W<DAC_DHR12R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_DHR12R1_SPEC>;
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
impl From<crate::W<DAC_DHR12R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_DHR12R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACC1DHR` reader - DACC1DHR"]
pub struct DACC1DHR_R(crate::FieldReader<u16, u16>);
impl DACC1DHR_R {
    pub(crate) fn new(bits: u16) -> Self {
        DACC1DHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACC1DHR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACC1DHR` writer - DACC1DHR"]
pub struct DACC1DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC1DHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DACC1DHR"]
    #[inline(always)]
    pub fn dacc1dhr(&self) -> DACC1DHR_R {
        DACC1DHR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DACC1DHR"]
    #[inline(always)]
    pub fn dacc1dhr(&mut self) -> DACC1DHR_W {
        DACC1DHR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC channel1 12-bit right-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr12r1](index.html) module"]
pub struct DAC_DHR12R1_SPEC;
impl crate::RegisterSpec for DAC_DHR12R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_dhr12r1::R](R) reader structure"]
impl crate::Readable for DAC_DHR12R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_dhr12r1::W](W) writer structure"]
impl crate::Writable for DAC_DHR12R1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_DHR12R1 to value 0"]
impl crate::Resettable for DAC_DHR12R1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
