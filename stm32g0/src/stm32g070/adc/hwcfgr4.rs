#[doc = "Register `HWCFGR4` reader"]
pub struct R(crate::R<HWCFGR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWCFGR4` writer"]
pub struct W(crate::W<HWCFGR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR4_SPEC>;
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
impl From<crate::W<HWCFGR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHMAP15` reader - Input channel mapping"]
pub struct CHMAP15_R(crate::FieldReader<u8, u8>);
impl CHMAP15_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMAP15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMAP15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMAP15` writer - Input channel mapping"]
pub struct CHMAP15_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `CHMAP14` reader - Input channel mapping"]
pub struct CHMAP14_R(crate::FieldReader<u8, u8>);
impl CHMAP14_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMAP14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMAP14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMAP14` writer - Input channel mapping"]
pub struct CHMAP14_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `CHMAP13` reader - Input channel mapping"]
pub struct CHMAP13_R(crate::FieldReader<u8, u8>);
impl CHMAP13_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMAP13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMAP13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMAP13` writer - Input channel mapping"]
pub struct CHMAP13_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `CHMAP12` reader - Input channel mapping"]
pub struct CHMAP12_R(crate::FieldReader<u8, u8>);
impl CHMAP12_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHMAP12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHMAP12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMAP12` writer - Input channel mapping"]
pub struct CHMAP12_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMAP12_W<'a> {
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
    pub fn chmap15(&self) -> CHMAP15_R {
        CHMAP15_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap14(&self) -> CHMAP14_R {
        CHMAP14_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap13(&self) -> CHMAP13_R {
        CHMAP13_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap12(&self) -> CHMAP12_R {
        CHMAP12_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap15(&mut self) -> CHMAP15_W {
        CHMAP15_W { w: self }
    }
    #[doc = "Bits 8:12 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap14(&mut self) -> CHMAP14_W {
        CHMAP14_W { w: self }
    }
    #[doc = "Bits 16:20 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap13(&mut self) -> CHMAP13_W {
        CHMAP13_W { w: self }
    }
    #[doc = "Bits 24:28 - Input channel mapping"]
    #[inline(always)]
    pub fn chmap12(&mut self) -> CHMAP12_W {
        CHMAP12_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr4](index.html) module"]
pub struct HWCFGR4_SPEC;
impl crate::RegisterSpec for HWCFGR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwcfgr4::R](R) reader structure"]
impl crate::Readable for HWCFGR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwcfgr4::W](W) writer structure"]
impl crate::Writable for HWCFGR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWCFGR4 to value 0x070b_0a09"]
impl crate::Resettable for HWCFGR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x070b_0a09
    }
}
