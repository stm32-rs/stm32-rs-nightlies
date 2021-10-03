#[doc = "Register `UR2` reader"]
pub struct R(crate::R<UR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR2` writer"]
pub struct W(crate::W<UR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR2_SPEC>;
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
impl From<crate::W<UR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BORH` reader - BOR_LVL Brownout Reset Threshold Level"]
pub struct BORH_R(crate::FieldReader<u8, u8>);
impl BORH_R {
    pub(crate) fn new(bits: u8) -> Self {
        BORH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BORH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCM7_ADD0` reader - Cortex-M7 Boot Address 0"]
pub struct BCM7_ADD0_R(crate::FieldReader<u16, u16>);
impl BCM7_ADD0_R {
    pub(crate) fn new(bits: u16) -> Self {
        BCM7_ADD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCM7_ADD0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCM7_ADD0` writer - Cortex-M7 Boot Address 0"]
pub struct BCM7_ADD0_W<'a> {
    w: &'a mut W,
}
impl<'a> BCM7_ADD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - BOR_LVL Brownout Reset Threshold Level"]
    #[inline(always)]
    pub fn borh(&self) -> BORH_R {
        BORH_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:31 - Cortex-M7 Boot Address 0"]
    #[inline(always)]
    pub fn bcm7_add0(&self) -> BCM7_ADD0_R {
        BCM7_ADD0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Cortex-M7 Boot Address 0"]
    #[inline(always)]
    pub fn bcm7_add0(&mut self) -> BCM7_ADD0_W {
        BCM7_ADD0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG user register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur2](index.html) module"]
pub struct UR2_SPEC;
impl crate::RegisterSpec for UR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur2::R](R) reader structure"]
impl crate::Readable for UR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur2::W](W) writer structure"]
impl crate::Writable for UR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UR2 to value 0"]
impl crate::Resettable for UR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
