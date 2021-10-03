#[doc = "Register `DLYR` reader"]
pub struct R(crate::R<DLYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLYR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLYR` writer"]
pub struct W(crate::W<DLYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLYR_SPEC>;
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
impl From<crate::W<DLYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLSSKP` reader - Pulses to skip for input data skipping function immediately after writing to this field. Reading of PLSSKP\\[5:0\\]
returns current value of pulses which will be skipped. If PLSSKP\\[5:0\\]=0 then all required data samples were already skipped. Note: User can update PLSSKP\\[5:0\\]
also when PLSSKP\\[5:0\\]
is not zero. 0-63: Defines the number of serial input samples that will be skipped. Skipping is applied"]
pub struct PLSSKP_R(crate::FieldReader<u8, u8>);
impl PLSSKP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLSSKP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLSSKP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLSSKP` writer - Pulses to skip for input data skipping function immediately after writing to this field. Reading of PLSSKP\\[5:0\\]
returns current value of pulses which will be skipped. If PLSSKP\\[5:0\\]=0 then all required data samples were already skipped. Note: User can update PLSSKP\\[5:0\\]
also when PLSSKP\\[5:0\\]
is not zero. 0-63: Defines the number of serial input samples that will be skipped. Skipping is applied"]
pub struct PLSSKP_W<'a> {
    w: &'a mut W,
}
impl<'a> PLSSKP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Pulses to skip for input data skipping function immediately after writing to this field. Reading of PLSSKP\\[5:0\\]
returns current value of pulses which will be skipped. If PLSSKP\\[5:0\\]=0 then all required data samples were already skipped. Note: User can update PLSSKP\\[5:0\\]
also when PLSSKP\\[5:0\\]
is not zero. 0-63: Defines the number of serial input samples that will be skipped. Skipping is applied"]
    #[inline(always)]
    pub fn plsskp(&self) -> PLSSKP_R {
        PLSSKP_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Pulses to skip for input data skipping function immediately after writing to this field. Reading of PLSSKP\\[5:0\\]
returns current value of pulses which will be skipped. If PLSSKP\\[5:0\\]=0 then all required data samples were already skipped. Note: User can update PLSSKP\\[5:0\\]
also when PLSSKP\\[5:0\\]
is not zero. 0-63: Defines the number of serial input samples that will be skipped. Skipping is applied"]
    #[inline(always)]
    pub fn plsskp(&mut self) -> PLSSKP_W {
        PLSSKP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyr](index.html) module"]
pub struct DLYR_SPEC;
impl crate::RegisterSpec for DLYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlyr::R](R) reader structure"]
impl crate::Readable for DLYR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlyr::W](W) writer structure"]
impl crate::Writable for DLYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLYR to value 0"]
impl crate::Resettable for DLYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
