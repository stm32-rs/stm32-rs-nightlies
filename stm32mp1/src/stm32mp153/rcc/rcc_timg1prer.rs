#[doc = "Register `RCC_TIMG1PRER` reader"]
pub struct R(crate::R<RCC_TIMG1PRER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_TIMG1PRER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_TIMG1PRER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_TIMG1PRER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_TIMG1PRER` writer"]
pub struct W(crate::W<RCC_TIMG1PRER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_TIMG1PRER_SPEC>;
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
impl From<crate::W<RCC_TIMG1PRER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_TIMG1PRER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMG1PRE` reader - TIMG1PRE"]
pub struct TIMG1PRE_R(crate::FieldReader<bool, bool>);
impl TIMG1PRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMG1PRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMG1PRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMG1PRE` writer - TIMG1PRE"]
pub struct TIMG1PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG1PRE_W<'a> {
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
#[doc = "Field `TIMG1PRERDY` reader - TIMG1PRERDY"]
pub struct TIMG1PRERDY_R(crate::FieldReader<bool, bool>);
impl TIMG1PRERDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMG1PRERDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMG1PRERDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TIMG1PRE"]
    #[inline(always)]
    pub fn timg1pre(&self) -> TIMG1PRE_R {
        TIMG1PRE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 31 - TIMG1PRERDY"]
    #[inline(always)]
    pub fn timg1prerdy(&self) -> TIMG1PRERDY_R {
        TIMG1PRERDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMG1PRE"]
    #[inline(always)]
    pub fn timg1pre(&mut self) -> TIMG1PRE_W {
        TIMG1PRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_timg1prer](index.html) module"]
pub struct RCC_TIMG1PRER_SPEC;
impl crate::RegisterSpec for RCC_TIMG1PRER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_timg1prer::R](R) reader structure"]
impl crate::Readable for RCC_TIMG1PRER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_timg1prer::W](W) writer structure"]
impl crate::Writable for RCC_TIMG1PRER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_TIMG1PRER to value 0x8000_0000"]
impl crate::Resettable for RCC_TIMG1PRER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
