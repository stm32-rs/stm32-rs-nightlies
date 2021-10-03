#[doc = "Register `UR1` reader"]
pub struct R(crate::R<UR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR1` writer"]
pub struct W(crate::W<UR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR1_SPEC>;
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
impl From<crate::W<UR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCM4` reader - Boot Cortex-M4"]
pub struct BCM4_R(crate::FieldReader<bool, bool>);
impl BCM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCM4` writer - Boot Cortex-M4"]
pub struct BCM4_W<'a> {
    w: &'a mut W,
}
impl<'a> BCM4_W<'a> {
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
#[doc = "Field `BCM7` reader - Boot Cortex-M7"]
pub struct BCM7_R(crate::FieldReader<bool, bool>);
impl BCM7_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCM7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCM7` writer - Boot Cortex-M7"]
pub struct BCM7_W<'a> {
    w: &'a mut W,
}
impl<'a> BCM7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Boot Cortex-M4"]
    #[inline(always)]
    pub fn bcm4(&self) -> BCM4_R {
        BCM4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Boot Cortex-M7"]
    #[inline(always)]
    pub fn bcm7(&self) -> BCM7_R {
        BCM7_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Cortex-M4"]
    #[inline(always)]
    pub fn bcm4(&mut self) -> BCM4_W {
        BCM4_W { w: self }
    }
    #[doc = "Bit 16 - Boot Cortex-M7"]
    #[inline(always)]
    pub fn bcm7(&mut self) -> BCM7_W {
        BCM7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG user register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur1](index.html) module"]
pub struct UR1_SPEC;
impl crate::RegisterSpec for UR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur1::R](R) reader structure"]
impl crate::Readable for UR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur1::W](W) writer structure"]
impl crate::Writable for UR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UR1 to value 0"]
impl crate::Resettable for UR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
