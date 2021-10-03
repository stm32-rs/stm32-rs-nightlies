#[doc = "Register `RCC_APB2DIVR` reader"]
pub struct R(crate::R<RCC_APB2DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB2DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB2DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB2DIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB2DIVR` writer"]
pub struct W(crate::W<RCC_APB2DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB2DIVR_SPEC>;
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
impl From<crate::W<RCC_APB2DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB2DIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB2DIV` reader - APB2DIV"]
pub struct APB2DIV_R(crate::FieldReader<u8, u8>);
impl APB2DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        APB2DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB2DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB2DIV` writer - APB2DIV"]
pub struct APB2DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB2DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `APB2DIVRDY` reader - APB2DIVRDY"]
pub struct APB2DIVRDY_R(crate::FieldReader<bool, bool>);
impl APB2DIVRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        APB2DIVRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB2DIVRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - APB2DIV"]
    #[inline(always)]
    pub fn apb2div(&self) -> APB2DIV_R {
        APB2DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - APB2DIVRDY"]
    #[inline(always)]
    pub fn apb2divrdy(&self) -> APB2DIVRDY_R {
        APB2DIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB2DIV"]
    #[inline(always)]
    pub fn apb2div(&mut self) -> APB2DIV_W {
        APB2DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb2divr](index.html) module"]
pub struct RCC_APB2DIVR_SPEC;
impl crate::RegisterSpec for RCC_APB2DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb2divr::R](R) reader structure"]
impl crate::Readable for RCC_APB2DIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb2divr::W](W) writer structure"]
impl crate::Writable for RCC_APB2DIVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_APB2DIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_APB2DIVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
