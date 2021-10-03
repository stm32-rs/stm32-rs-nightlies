#[doc = "Register `RCC_MPCKSELR` reader"]
pub struct R(crate::R<RCC_MPCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MPCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MPCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MPCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MPCKSELR` writer"]
pub struct W(crate::W<RCC_MPCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MPCKSELR_SPEC>;
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
impl From<crate::W<RCC_MPCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MPCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPUSRC` reader - MPUSRC"]
pub struct MPUSRC_R(crate::FieldReader<u8, u8>);
impl MPUSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        MPUSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPUSRC` writer - MPUSRC"]
pub struct MPUSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `MPUSRCRDY` reader - MPUSRCRDY"]
pub struct MPUSRCRDY_R(crate::FieldReader<bool, bool>);
impl MPUSRCRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPUSRCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPUSRCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - MPUSRC"]
    #[inline(always)]
    pub fn mpusrc(&self) -> MPUSRC_R {
        MPUSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - MPUSRCRDY"]
    #[inline(always)]
    pub fn mpusrcrdy(&self) -> MPUSRCRDY_R {
        MPUSRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MPUSRC"]
    #[inline(always)]
    pub fn mpusrc(&mut self) -> MPUSRC_W {
        MPUSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mpckselr](index.html) module"]
pub struct RCC_MPCKSELR_SPEC;
impl crate::RegisterSpec for RCC_MPCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mpckselr::R](R) reader structure"]
impl crate::Readable for RCC_MPCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mpckselr::W](W) writer structure"]
impl crate::Writable for RCC_MPCKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MPCKSELR to value 0x8000_0000"]
impl crate::Resettable for RCC_MPCKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
