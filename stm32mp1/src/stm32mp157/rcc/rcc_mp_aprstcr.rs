#[doc = "Register `RCC_MP_APRSTCR` reader"]
pub struct R(crate::R<RCC_MP_APRSTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APRSTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APRSTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APRSTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_APRSTCR` writer"]
pub struct W(crate::W<RCC_MP_APRSTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APRSTCR_SPEC>;
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
impl From<crate::W<RCC_MP_APRSTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APRSTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDCTLEN` reader - RDCTLEN"]
pub struct RDCTLEN_R(crate::FieldReader<bool, bool>);
impl RDCTLEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDCTLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDCTLEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDCTLEN` writer - RDCTLEN"]
pub struct RDCTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDCTLEN_W<'a> {
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
#[doc = "Field `RSTTO` reader - RSTTO"]
pub struct RSTTO_R(crate::FieldReader<u8, u8>);
impl RSTTO_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSTTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTTO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTTO` writer - RSTTO"]
pub struct RSTTO_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RDCTLEN"]
    #[inline(always)]
    pub fn rdctlen(&self) -> RDCTLEN_R {
        RDCTLEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - RSTTO"]
    #[inline(always)]
    pub fn rstto(&self) -> RSTTO_R {
        RSTTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RDCTLEN"]
    #[inline(always)]
    pub fn rdctlen(&mut self) -> RDCTLEN_W {
        RDCTLEN_W { w: self }
    }
    #[doc = "Bits 8:14 - RSTTO"]
    #[inline(always)]
    pub fn rstto(&mut self) -> RSTTO_W {
        RSTTO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_aprstcr](index.html) module"]
pub struct RCC_MP_APRSTCR_SPEC;
impl crate::RegisterSpec for RCC_MP_APRSTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_aprstcr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APRSTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_aprstcr::W](W) writer structure"]
impl crate::Writable for RCC_MP_APRSTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_APRSTCR to value 0x7f00"]
impl crate::Resettable for RCC_MP_APRSTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f00
    }
}
