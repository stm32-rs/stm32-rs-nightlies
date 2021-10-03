#[doc = "Register `RCC_CECCKSELR` reader"]
pub struct R(crate::R<RCC_CECCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CECCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CECCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CECCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CECCKSELR` writer"]
pub struct W(crate::W<RCC_CECCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CECCKSELR_SPEC>;
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
impl From<crate::W<RCC_CECCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CECCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CECSRC` reader - CECSRC"]
pub struct CECSRC_R(crate::FieldReader<u8, u8>);
impl CECSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CECSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CECSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CECSRC` writer - CECSRC"]
pub struct CECSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CECSRC"]
    #[inline(always)]
    pub fn cecsrc(&self) -> CECSRC_R {
        CECSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CECSRC"]
    #[inline(always)]
    pub fn cecsrc(&mut self) -> CECSRC_W {
        CECSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the CEC-HDMI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cecckselr](index.html) module"]
pub struct RCC_CECCKSELR_SPEC;
impl crate::RegisterSpec for RCC_CECCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_cecckselr::R](R) reader structure"]
impl crate::Readable for RCC_CECCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_cecckselr::W](W) writer structure"]
impl crate::Writable for RCC_CECCKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_CECCKSELR to value 0"]
impl crate::Resettable for RCC_CECCKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
