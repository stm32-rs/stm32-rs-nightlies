#[doc = "Register `UR3` reader"]
pub struct R(crate::R<UR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR3` writer"]
pub struct W(crate::W<UR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR3_SPEC>;
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
impl From<crate::W<UR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCM4_ADD1` reader - Cortex-M4 Boot Address 0"]
pub struct BCM4_ADD1_R(crate::FieldReader<u16, u16>);
impl BCM4_ADD1_R {
    pub(crate) fn new(bits: u16) -> Self {
        BCM4_ADD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCM4_ADD1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCM4_ADD1` writer - Cortex-M4 Boot Address 0"]
pub struct BCM4_ADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> BCM4_ADD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `BCM7_ADD1` reader - Cortex-M7 Boot Address 1"]
pub struct BCM7_ADD1_R(crate::FieldReader<u16, u16>);
impl BCM7_ADD1_R {
    pub(crate) fn new(bits: u16) -> Self {
        BCM7_ADD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCM7_ADD1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCM7_ADD1` writer - Cortex-M7 Boot Address 1"]
pub struct BCM7_ADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> BCM7_ADD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Cortex-M4 Boot Address 0"]
    #[inline(always)]
    pub fn bcm4_add1(&self) -> BCM4_ADD1_R {
        BCM4_ADD1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Cortex-M7 Boot Address 1"]
    #[inline(always)]
    pub fn bcm7_add1(&self) -> BCM7_ADD1_R {
        BCM7_ADD1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Cortex-M4 Boot Address 0"]
    #[inline(always)]
    pub fn bcm4_add1(&mut self) -> BCM4_ADD1_W {
        BCM4_ADD1_W { w: self }
    }
    #[doc = "Bits 16:31 - Cortex-M7 Boot Address 1"]
    #[inline(always)]
    pub fn bcm7_add1(&mut self) -> BCM7_ADD1_W {
        BCM7_ADD1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG user register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur3](index.html) module"]
pub struct UR3_SPEC;
impl crate::RegisterSpec for UR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur3::R](R) reader structure"]
impl crate::Readable for UR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur3::W](W) writer structure"]
impl crate::Writable for UR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UR3 to value 0"]
impl crate::Resettable for UR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
