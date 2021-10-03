#[doc = "Register `RCC_TZAHB6RSTSETR` reader"]
pub struct R(crate::R<RCC_TZAHB6RSTSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_TZAHB6RSTSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_TZAHB6RSTSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_TZAHB6RSTSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_TZAHB6RSTSETR` writer"]
pub struct W(crate::W<RCC_TZAHB6RSTSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_TZAHB6RSTSETR_SPEC>;
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
impl From<crate::W<RCC_TZAHB6RSTSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_TZAHB6RSTSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDMARST` reader - MDMARST"]
pub struct MDMARST_R(crate::FieldReader<bool, bool>);
impl MDMARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMARST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDMARST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMARST` writer - MDMARST"]
pub struct MDMARST_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMARST_W<'a> {
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
    #[doc = "Bit 0 - MDMARST"]
    #[inline(always)]
    pub fn mdmarst(&self) -> MDMARST_R {
        MDMARST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMARST"]
    #[inline(always)]
    pub fn mdmarst(&mut self) -> MDMARST_W {
        MDMARST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_tzahb6rstsetr](index.html) module"]
pub struct RCC_TZAHB6RSTSETR_SPEC;
impl crate::RegisterSpec for RCC_TZAHB6RSTSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_tzahb6rstsetr::R](R) reader structure"]
impl crate::Readable for RCC_TZAHB6RSTSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_tzahb6rstsetr::W](W) writer structure"]
impl crate::Writable for RCC_TZAHB6RSTSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_TZAHB6RSTSETR to value 0"]
impl crate::Resettable for RCC_TZAHB6RSTSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
