#[doc = "Register `RCC_ETHCKSELR` reader"]
pub struct R(crate::R<RCC_ETHCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_ETHCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_ETHCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_ETHCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_ETHCKSELR` writer"]
pub struct W(crate::W<RCC_ETHCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_ETHCKSELR_SPEC>;
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
impl From<crate::W<RCC_ETHCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_ETHCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETHSRC` reader - ETHSRC"]
pub struct ETHSRC_R(crate::FieldReader<u8, u8>);
impl ETHSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETHSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHSRC` writer - ETHSRC"]
pub struct ETHSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `ETHPTPDIV` reader - ETHPTPDIV"]
pub struct ETHPTPDIV_R(crate::FieldReader<u8, u8>);
impl ETHPTPDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETHPTPDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHPTPDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHPTPDIV` writer - ETHPTPDIV"]
pub struct ETHPTPDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHPTPDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ETHSRC"]
    #[inline(always)]
    pub fn ethsrc(&self) -> ETHSRC_R {
        ETHSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - ETHPTPDIV"]
    #[inline(always)]
    pub fn ethptpdiv(&self) -> ETHPTPDIV_R {
        ETHPTPDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ETHSRC"]
    #[inline(always)]
    pub fn ethsrc(&mut self) -> ETHSRC_W {
        ETHSRC_W { w: self }
    }
    #[doc = "Bits 4:7 - ETHPTPDIV"]
    #[inline(always)]
    pub fn ethptpdiv(&mut self) -> ETHPTPDIV_W {
        ETHPTPDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ethckselr](index.html) module"]
pub struct RCC_ETHCKSELR_SPEC;
impl crate::RegisterSpec for RCC_ETHCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ethckselr::R](R) reader structure"]
impl crate::Readable for RCC_ETHCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ethckselr::W](W) writer structure"]
impl crate::Writable for RCC_ETHCKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_ETHCKSELR to value 0"]
impl crate::Resettable for RCC_ETHCKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
