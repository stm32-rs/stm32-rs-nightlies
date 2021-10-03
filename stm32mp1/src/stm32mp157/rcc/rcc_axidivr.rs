#[doc = "Register `RCC_AXIDIVR` reader"]
pub struct R(crate::R<RCC_AXIDIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AXIDIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AXIDIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AXIDIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AXIDIVR` writer"]
pub struct W(crate::W<RCC_AXIDIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AXIDIVR_SPEC>;
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
impl From<crate::W<RCC_AXIDIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AXIDIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AXIDIV` reader - AXIDIV"]
pub struct AXIDIV_R(crate::FieldReader<u8, u8>);
impl AXIDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        AXIDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXIDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXIDIV` writer - AXIDIV"]
pub struct AXIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `AXIDIVRDY` reader - AXIDIVRDY"]
pub struct AXIDIVRDY_R(crate::FieldReader<bool, bool>);
impl AXIDIVRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXIDIVRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXIDIVRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - AXIDIV"]
    #[inline(always)]
    pub fn axidiv(&self) -> AXIDIV_R {
        AXIDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - AXIDIVRDY"]
    #[inline(always)]
    pub fn axidivrdy(&self) -> AXIDIVRDY_R {
        AXIDIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXIDIV"]
    #[inline(always)]
    pub fn axidiv(&mut self) -> AXIDIV_W {
        AXIDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_axidivr](index.html) module"]
pub struct RCC_AXIDIVR_SPEC;
impl crate::RegisterSpec for RCC_AXIDIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_axidivr::R](R) reader structure"]
impl crate::Readable for RCC_AXIDIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_axidivr::W](W) writer structure"]
impl crate::Writable for RCC_AXIDIVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_AXIDIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_AXIDIVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
