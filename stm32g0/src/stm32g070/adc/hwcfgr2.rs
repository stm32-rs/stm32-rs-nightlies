#[doc = "Register `HWCFGR2` reader"]
pub struct R(crate::R<HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR2` writer"]
pub struct W(crate::W<HWCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR2_SPEC>;
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
impl From<crate::W<HWCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHMAP7` reader - Input channel mapping"]
pub struct CHMAP7_R(crate::FieldReader<u8, u8>);
impl CHMAP7_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMAP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMAP7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMAP7` writer - Input channel mapping"]
pub struct CHMAP7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `CHMAP6` reader - Input channel mapping"]
pub struct CHMAP6_R(crate::FieldReader<u8, u8>);
impl CHMAP6_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMAP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMAP6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMAP6` writer - Input channel mapping"]
pub struct CHMAP6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `CHMAP5` reader - Input channel mapping"]
pub struct CHMAP5_R(crate::FieldReader<u8, u8>);
impl CHMAP5_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMAP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMAP5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMAP5` writer - Input channel mapping"]
pub struct CHMAP5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `CHMAP4` reader - Input channel mapping"]
pub struct CHMAP4_R(crate::FieldReader<u8, u8>);
impl CHMAP4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMAP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMAP4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMAP4` writer - Input channel mapping"]
pub struct CHMAP4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP4_W<'a> {
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
    pub fn chmap7(&self) -> CHMAP7_R {
        CHMAP7_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap6(&self) -> CHMAP6_R {
        CHMAP6_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap5(&self) -> CHMAP5_R {
        CHMAP5_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap4(&self) -> CHMAP4_R {
        CHMAP4_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap7(&mut self) -> CHMAP7_W {
        CHMAP7_W { w: self }
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap6(&mut self) -> CHMAP6_W {
        CHMAP6_W { w: self }
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap5(&mut self) -> CHMAP5_W {
        CHMAP5_W { w: self }
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap4(&mut self) -> CHMAP4_W {
        CHMAP4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr2](index.html) module"]
pub struct HWCFGR2_SPEC;
impl crate::RegisterSpec for HWCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr2::R](R) reader structure"]
impl crate::Readable for HWCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr2::W](W) writer structure"]
impl crate::Writable for HWCFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWCFGR2 to value 0x0505_0404"]
impl crate::Resettable for HWCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0505_0404
    }
}
