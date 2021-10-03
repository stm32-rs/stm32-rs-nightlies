#[doc = "Register `HWCFGR1` reader"]
pub struct R(crate::R<HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR1` writer"]
pub struct W(crate::W<HWCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR1_SPEC>;
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
impl From<crate::W<HWCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHMAP3` reader - Input channel mapping"]
pub struct CHMAP3_R(crate::FieldReader<u8, u8>);
impl CHMAP3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMAP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMAP3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMAP3` writer - Input channel mapping"]
pub struct CHMAP3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `CHMAP2` reader - Input channel mapping"]
pub struct CHMAP2_R(crate::FieldReader<u8, u8>);
impl CHMAP2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMAP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMAP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMAP2` writer - Input channel mapping"]
pub struct CHMAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `CHMAP1` reader - Input channel mapping"]
pub struct CHMAP1_R(crate::FieldReader<u8, u8>);
impl CHMAP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMAP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMAP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMAP1` writer - Input channel mapping"]
pub struct CHMAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `CHMAP0` reader - Input channel mapping"]
pub struct CHMAP0_R(crate::FieldReader<u8, u8>);
impl CHMAP0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMAP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMAP0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMAP0` writer - Input channel mapping"]
pub struct CHMAP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap3(&self) -> CHMAP3_R {
        CHMAP3_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap2(&self) -> CHMAP2_R {
        CHMAP2_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap1(&self) -> CHMAP1_R {
        CHMAP1_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap0(&self) -> CHMAP0_R {
        CHMAP0_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap3(&mut self) -> CHMAP3_W {
        CHMAP3_W { w: self }
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap2(&mut self) -> CHMAP2_W {
        CHMAP2_W { w: self }
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap1(&mut self) -> CHMAP1_W {
        CHMAP1_W { w: self }
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap0(&mut self) -> CHMAP0_W {
        CHMAP0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr1](index.html) module"]
pub struct HWCFGR1_SPEC;
impl crate::RegisterSpec for HWCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr1::R](R) reader structure"]
impl crate::Readable for HWCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr1::W](W) writer structure"]
impl crate::Writable for HWCFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWCFGR1 to value 0x0302_0100"]
impl crate::Resettable for HWCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0302_0100
    }
}
