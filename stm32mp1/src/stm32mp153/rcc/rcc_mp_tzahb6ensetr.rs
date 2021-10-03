#[doc = "Register `RCC_MP_TZAHB6ENSETR` reader"]
pub struct R(crate::R<RCC_MP_TZAHB6ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_TZAHB6ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_TZAHB6ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_TZAHB6ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_TZAHB6ENSETR` writer"]
pub struct W(crate::W<RCC_MP_TZAHB6ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_TZAHB6ENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_TZAHB6ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_TZAHB6ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDMAEN` reader - MDMAEN"]
pub struct MDMAEN_R(crate::FieldReader<bool, bool>);
impl MDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMAEN` writer - MDMAEN"]
pub struct MDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMAEN_W<'a> {
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
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&mut self) -> MDMAEN_W {
        MDMAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_tzahb6ensetr](index.html) module"]
pub struct RCC_MP_TZAHB6ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_TZAHB6ENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_tzahb6ensetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_TZAHB6ENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_tzahb6ensetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_TZAHB6ENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_TZAHB6ENSETR to value 0"]
impl crate::Resettable for RCC_MP_TZAHB6ENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
