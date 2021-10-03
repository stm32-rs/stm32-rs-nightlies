#[doc = "Register `GICD_ITARGETSR48` reader"]
pub struct R(crate::R<GICD_ITARGETSR48_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR48_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR48_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR48_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR48` writer"]
pub struct W(crate::W<GICD_ITARGETSR48_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR48_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR48_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR48_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_TARGETS0` reader - CPU_TARGETS0"]
pub struct CPU_TARGETS0_R(crate::FieldReader<u8, u8>);
impl CPU_TARGETS0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_TARGETS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TARGETS0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TARGETS0` writer - CPU_TARGETS0"]
pub struct CPU_TARGETS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TARGETS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CPU_TARGETS1` reader - CPU_TARGETS1"]
pub struct CPU_TARGETS1_R(crate::FieldReader<u8, u8>);
impl CPU_TARGETS1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_TARGETS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TARGETS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TARGETS1` writer - CPU_TARGETS1"]
pub struct CPU_TARGETS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TARGETS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CPU_TARGETS2` reader - CPU_TARGETS2"]
pub struct CPU_TARGETS2_R(crate::FieldReader<u8, u8>);
impl CPU_TARGETS2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_TARGETS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TARGETS2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TARGETS2` writer - CPU_TARGETS2"]
pub struct CPU_TARGETS2_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TARGETS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CPU_TARGETS3` reader - CPU_TARGETS3"]
pub struct CPU_TARGETS3_R(crate::FieldReader<u8, u8>);
impl CPU_TARGETS3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_TARGETS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_TARGETS3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_TARGETS3` writer - CPU_TARGETS3"]
pub struct CPU_TARGETS3_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_TARGETS3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CPU_TARGETS0"]
    #[inline(always)]
    pub fn cpu_targets0(&self) -> CPU_TARGETS0_R {
        CPU_TARGETS0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - CPU_TARGETS1"]
    #[inline(always)]
    pub fn cpu_targets1(&self) -> CPU_TARGETS1_R {
        CPU_TARGETS1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - CPU_TARGETS2"]
    #[inline(always)]
    pub fn cpu_targets2(&self) -> CPU_TARGETS2_R {
        CPU_TARGETS2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - CPU_TARGETS3"]
    #[inline(always)]
    pub fn cpu_targets3(&self) -> CPU_TARGETS3_R {
        CPU_TARGETS3_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CPU_TARGETS0"]
    #[inline(always)]
    pub fn cpu_targets0(&mut self) -> CPU_TARGETS0_W {
        CPU_TARGETS0_W { w: self }
    }
    #[doc = "Bits 8:9 - CPU_TARGETS1"]
    #[inline(always)]
    pub fn cpu_targets1(&mut self) -> CPU_TARGETS1_W {
        CPU_TARGETS1_W { w: self }
    }
    #[doc = "Bits 16:17 - CPU_TARGETS2"]
    #[inline(always)]
    pub fn cpu_targets2(&mut self) -> CPU_TARGETS2_W {
        CPU_TARGETS2_W { w: self }
    }
    #[doc = "Bits 24:25 - CPU_TARGETS3"]
    #[inline(always)]
    pub fn cpu_targets3(&mut self) -> CPU_TARGETS3_W {
        CPU_TARGETS3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICD interrupt processor target register 48\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr48](index.html) module"]
pub struct GICD_ITARGETSR48_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr48::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR48_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr48::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR48_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_ITARGETSR48 to value 0"]
impl crate::Resettable for GICD_ITARGETSR48_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
