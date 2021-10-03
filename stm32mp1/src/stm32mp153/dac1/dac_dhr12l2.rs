#[doc = "Register `DAC_DHR12L2` reader"]
pub struct R(crate::R<DAC_DHR12L2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DHR12L2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DHR12L2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DHR12L2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_DHR12L2` writer"]
pub struct W(crate::W<DAC_DHR12L2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_DHR12L2_SPEC>;
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
impl From<crate::W<DAC_DHR12L2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_DHR12L2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACC2DHR` reader - DACC2DHR"]
pub struct DACC2DHR_R(crate::FieldReader<u16, u16>);
impl DACC2DHR_R {
    pub(crate) fn new(bits: u16) -> Self {
        DACC2DHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACC2DHR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACC2DHR` writer - DACC2DHR"]
pub struct DACC2DHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DACC2DHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | ((value as u32 & 0x0fff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:15 - DACC2DHR"]
    #[inline(always)]
    pub fn dacc2dhr(&self) -> DACC2DHR_R {
        DACC2DHR_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DACC2DHR"]
    #[inline(always)]
    pub fn dacc2dhr(&mut self) -> DACC2DHR_W {
        DACC2DHR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is available only on dual-channel DACs. Refer to Section29.3: DAC implementation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr12l2](index.html) module"]
pub struct DAC_DHR12L2_SPEC;
impl crate::RegisterSpec for DAC_DHR12L2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_dhr12l2::R](R) reader structure"]
impl crate::Readable for DAC_DHR12L2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_dhr12l2::W](W) writer structure"]
impl crate::Writable for DAC_DHR12L2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_DHR12L2 to value 0"]
impl crate::Resettable for DAC_DHR12L2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}