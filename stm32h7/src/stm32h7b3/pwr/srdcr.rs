#[doc = "Register `SRDCR` reader"]
pub struct R(crate::R<SRDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRDCR` writer"]
pub struct W(crate::W<SRDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRDCR_SPEC>;
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
impl From<crate::W<SRDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOSRDY` reader - VOS Ready bit for VCORE voltage scaling output selection. This bit is set to 1 by hardware when Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
pub struct VOSRDY_R(crate::FieldReader<bool, bool>);
impl VOSRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        VOSRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOSRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOS` reader - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
pub struct VOS_R(crate::FieldReader<u8, u8>);
impl VOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        VOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOS` writer - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
pub struct VOS_W<'a> {
    w: &'a mut W,
}
impl<'a> VOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - VOS Ready bit for VCORE voltage scaling output selection. This bit is set to 1 by hardware when Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15 - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling."]
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W {
        VOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdcr](index.html) module"]
pub struct SRDCR_SPEC;
impl crate::RegisterSpec for SRDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srdcr::R](R) reader structure"]
impl crate::Readable for SRDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srdcr::W](W) writer structure"]
impl crate::Writable for SRDCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRDCR to value 0x4000"]
impl crate::Resettable for SRDCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
