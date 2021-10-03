#[doc = "Register `RCC_ASSCKSELR` reader"]
pub struct R(crate::R<RCC_ASSCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_ASSCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_ASSCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_ASSCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_ASSCKSELR` writer"]
pub struct W(crate::W<RCC_ASSCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_ASSCKSELR_SPEC>;
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
impl From<crate::W<RCC_ASSCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_ASSCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AXISSRC` reader - AXISSRC"]
pub struct AXISSRC_R(crate::FieldReader<u8, u8>);
impl AXISSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        AXISSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXISSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AXISSRC` writer - AXISSRC"]
pub struct AXISSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AXISSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `AXISSRCRDY` reader - AXISSRCRDY"]
pub struct AXISSRCRDY_R(crate::FieldReader<bool, bool>);
impl AXISSRCRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        AXISSRCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AXISSRCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - AXISSRC"]
    #[inline(always)]
    pub fn axissrc(&self) -> AXISSRC_R {
        AXISSRC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - AXISSRCRDY"]
    #[inline(always)]
    pub fn axissrcrdy(&self) -> AXISSRCRDY_R {
        AXISSRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXISSRC"]
    #[inline(always)]
    pub fn axissrc(&mut self) -> AXISSRC_W {
        AXISSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_assckselr](index.html) module"]
pub struct RCC_ASSCKSELR_SPEC;
impl crate::RegisterSpec for RCC_ASSCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_assckselr::R](R) reader structure"]
impl crate::Readable for RCC_ASSCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_assckselr::W](W) writer structure"]
impl crate::Writable for RCC_ASSCKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_ASSCKSELR to value 0x8000_0000"]
impl crate::Resettable for RCC_ASSCKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
