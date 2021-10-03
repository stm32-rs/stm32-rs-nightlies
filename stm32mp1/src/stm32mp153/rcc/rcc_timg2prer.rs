#[doc = "Register `RCC_TIMG2PRER` reader"]
pub struct R(crate::R<RCC_TIMG2PRER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_TIMG2PRER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_TIMG2PRER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_TIMG2PRER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_TIMG2PRER` writer"]
pub struct W(crate::W<RCC_TIMG2PRER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_TIMG2PRER_SPEC>;
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
impl From<crate::W<RCC_TIMG2PRER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_TIMG2PRER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMG2PRE` reader - TIMG2PRE"]
pub struct TIMG2PRE_R(crate::FieldReader<bool, bool>);
impl TIMG2PRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMG2PRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMG2PRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMG2PRE` writer - TIMG2PRE"]
pub struct TIMG2PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG2PRE_W<'a> {
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
#[doc = "Field `TIMG2PRERDY` reader - TIMG2PRERDY"]
pub struct TIMG2PRERDY_R(crate::FieldReader<bool, bool>);
impl TIMG2PRERDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMG2PRERDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMG2PRERDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - TIMG2PRE"]
    #[inline(always)]
    pub fn timg2pre(&self) -> TIMG2PRE_R {
        TIMG2PRE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 31 - TIMG2PRERDY"]
    #[inline(always)]
    pub fn timg2prerdy(&self) -> TIMG2PRERDY_R {
        TIMG2PRERDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMG2PRE"]
    #[inline(always)]
    pub fn timg2pre(&mut self) -> TIMG2PRE_W {
        TIMG2PRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_timg2prer](index.html) module"]
pub struct RCC_TIMG2PRER_SPEC;
impl crate::RegisterSpec for RCC_TIMG2PRER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_timg2prer::R](R) reader structure"]
impl crate::Readable for RCC_TIMG2PRER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_timg2prer::W](W) writer structure"]
impl crate::Writable for RCC_TIMG2PRER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_TIMG2PRER to value 0x8000_0000"]
impl crate::Resettable for RCC_TIMG2PRER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
