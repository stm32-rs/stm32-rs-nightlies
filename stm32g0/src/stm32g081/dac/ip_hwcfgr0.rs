#[doc = "Register `IP_HWCFGR0` reader"]
pub struct R(crate::R<IP_HWCFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IP_HWCFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IP_HWCFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IP_HWCFGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IP_HWCFGR0` writer"]
pub struct W(crate::W<IP_HWCFGR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IP_HWCFGR0_SPEC>;
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
impl From<crate::W<IP_HWCFGR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IP_HWCFGR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUAL` reader - Dual DAC capability"]
pub struct DUAL_R(crate::FieldReader<u8, u8>);
impl DUAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUAL` writer - Dual DAC capability"]
pub struct DUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `LFSR` reader - Pseudonoise wave generation capability"]
pub struct LFSR_R(crate::FieldReader<u8, u8>);
impl LFSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        LFSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFSR` writer - Pseudonoise wave generation capability"]
pub struct LFSR_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `TRIANGLE` reader - Triangle wave generation capability"]
pub struct TRIANGLE_R(crate::FieldReader<u8, u8>);
impl TRIANGLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIANGLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIANGLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIANGLE` writer - Triangle wave generation capability"]
pub struct TRIANGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIANGLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `SAMPLE` reader - Sample and hold mode capability"]
pub struct SAMPLE_R(crate::FieldReader<u8, u8>);
impl SAMPLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAMPLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMPLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPLE` writer - Sample and hold mode capability"]
pub struct SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `OR_CFG` reader - option register bit width"]
pub struct OR_CFG_R(crate::FieldReader<u8, u8>);
impl OR_CFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        OR_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OR_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OR_CFG` writer - option register bit width"]
pub struct OR_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> OR_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Dual DAC capability"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pseudonoise wave generation capability"]
    #[inline(always)]
    pub fn lfsr(&self) -> LFSR_R {
        LFSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Triangle wave generation capability"]
    #[inline(always)]
    pub fn triangle(&self) -> TRIANGLE_R {
        TRIANGLE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Sample and hold mode capability"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - option register bit width"]
    #[inline(always)]
    pub fn or_cfg(&self) -> OR_CFG_R {
        OR_CFG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dual DAC capability"]
    #[inline(always)]
    pub fn dual(&mut self) -> DUAL_W {
        DUAL_W { w: self }
    }
    #[doc = "Bits 4:7 - Pseudonoise wave generation capability"]
    #[inline(always)]
    pub fn lfsr(&mut self) -> LFSR_W {
        LFSR_W { w: self }
    }
    #[doc = "Bits 8:11 - Triangle wave generation capability"]
    #[inline(always)]
    pub fn triangle(&mut self) -> TRIANGLE_W {
        TRIANGLE_W { w: self }
    }
    #[doc = "Bits 12:15 - Sample and hold mode capability"]
    #[inline(always)]
    pub fn sample(&mut self) -> SAMPLE_W {
        SAMPLE_W { w: self }
    }
    #[doc = "Bits 16:23 - option register bit width"]
    #[inline(always)]
    pub fn or_cfg(&mut self) -> OR_CFG_W {
        OR_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC IP Hardware Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ip_hwcfgr0](index.html) module"]
pub struct IP_HWCFGR0_SPEC;
impl crate::RegisterSpec for IP_HWCFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ip_hwcfgr0::R](R) reader structure"]
impl crate::Readable for IP_HWCFGR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ip_hwcfgr0::W](W) writer structure"]
impl crate::Writable for IP_HWCFGR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IP_HWCFGR0 to value 0x1111"]
impl crate::Resettable for IP_HWCFGR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1111
    }
}
