#[doc = "Register `RCC_APB3DIVR` reader"]
pub struct R(crate::R<RCC_APB3DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB3DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB3DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB3DIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB3DIVR` writer"]
pub struct W(crate::W<RCC_APB3DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB3DIVR_SPEC>;
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
impl From<crate::W<RCC_APB3DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB3DIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB3DIV` reader - APB3DIV"]
pub struct APB3DIV_R(crate::FieldReader<u8, u8>);
impl APB3DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        APB3DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB3DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APB3DIV` writer - APB3DIV"]
pub struct APB3DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB3DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `APB3DIVRDY` reader - APB3DIVRDY"]
pub struct APB3DIVRDY_R(crate::FieldReader<bool, bool>);
impl APB3DIVRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        APB3DIVRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB3DIVRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - APB3DIV"]
    #[inline(always)]
    pub fn apb3div(&self) -> APB3DIV_R {
        APB3DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - APB3DIVRDY"]
    #[inline(always)]
    pub fn apb3divrdy(&self) -> APB3DIVRDY_R {
        APB3DIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB3DIV"]
    #[inline(always)]
    pub fn apb3div(&mut self) -> APB3DIV_W {
        APB3DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb3divr](index.html) module"]
pub struct RCC_APB3DIVR_SPEC;
impl crate::RegisterSpec for RCC_APB3DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb3divr::R](R) reader structure"]
impl crate::Readable for RCC_APB3DIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb3divr::W](W) writer structure"]
impl crate::Writable for RCC_APB3DIVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_APB3DIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_APB3DIVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
