#[doc = "Register `RCC_UART1CKSELR` reader"]
pub struct R(crate::R<RCC_UART1CKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_UART1CKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_UART1CKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_UART1CKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_UART1CKSELR` writer"]
pub struct W(crate::W<RCC_UART1CKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_UART1CKSELR_SPEC>;
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
impl From<crate::W<RCC_UART1CKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_UART1CKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART1SRC` reader - UART1SRC"]
pub struct UART1SRC_R(crate::FieldReader<u8, u8>);
impl UART1SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART1SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1SRC` writer - UART1SRC"]
pub struct UART1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - UART1SRC"]
    #[inline(always)]
    pub fn uart1src(&self) -> UART1SRC_R {
        UART1SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART1SRC"]
    #[inline(always)]
    pub fn uart1src(&mut self) -> UART1SRC_W {
        UART1SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_uart1ckselr](index.html) module"]
pub struct RCC_UART1CKSELR_SPEC;
impl crate::RegisterSpec for RCC_UART1CKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_uart1ckselr::R](R) reader structure"]
impl crate::Readable for RCC_UART1CKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_uart1ckselr::W](W) writer structure"]
impl crate::Writable for RCC_UART1CKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_UART1CKSELR to value 0"]
impl crate::Resettable for RCC_UART1CKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
