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
#[doc = "Field `CFG1` reader - LUART hardware configuration 1"]
pub struct CFG1_R(crate::FieldReader<u8, u8>);
impl CFG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG1` writer - LUART hardware configuration 1"]
pub struct CFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CFG2` reader - LUART hardware configuration 2"]
pub struct CFG2_R(crate::FieldReader<u8, u8>);
impl CFG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG2` writer - LUART hardware configuration 2"]
pub struct CFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CFG3` reader - LUART hardware configuration 1"]
pub struct CFG3_R(crate::FieldReader<u8, u8>);
impl CFG3_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG3` writer - LUART hardware configuration 1"]
pub struct CFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CFG4` reader - LUART hardware configuration 2"]
pub struct CFG4_R(crate::FieldReader<u8, u8>);
impl CFG4_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG4` writer - LUART hardware configuration 2"]
pub struct CFG4_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `CFG5` reader - LUART hardware configuration 2"]
pub struct CFG5_R(crate::FieldReader<u8, u8>);
impl CFG5_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG5` writer - LUART hardware configuration 2"]
pub struct CFG5_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `CFG6` reader - LUART hardware configuration 2"]
pub struct CFG6_R(crate::FieldReader<u8, u8>);
impl CFG6_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG6` writer - LUART hardware configuration 2"]
pub struct CFG6_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `CFG7` reader - LUART hardware configuration 2"]
pub struct CFG7_R(crate::FieldReader<u8, u8>);
impl CFG7_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG7` writer - LUART hardware configuration 2"]
pub struct CFG7_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `CFG8` reader - LUART hardware configuration 2"]
pub struct CFG8_R(crate::FieldReader<u8, u8>);
impl CFG8_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG8` writer - LUART hardware configuration 2"]
pub struct CFG8_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - LUART hardware configuration 1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - LUART hardware configuration 1"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg8(&self) -> CFG8_R {
        CFG8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LUART hardware configuration 1"]
    #[inline(always)]
    pub fn cfg1(&mut self) -> CFG1_W {
        CFG1_W { w: self }
    }
    #[doc = "Bits 4:7 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg2(&mut self) -> CFG2_W {
        CFG2_W { w: self }
    }
    #[doc = "Bits 8:11 - LUART hardware configuration 1"]
    #[inline(always)]
    pub fn cfg3(&mut self) -> CFG3_W {
        CFG3_W { w: self }
    }
    #[doc = "Bits 12:15 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg4(&mut self) -> CFG4_W {
        CFG4_W { w: self }
    }
    #[doc = "Bits 16:19 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg5(&mut self) -> CFG5_W {
        CFG5_W { w: self }
    }
    #[doc = "Bits 20:23 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg6(&mut self) -> CFG6_W {
        CFG6_W { w: self }
    }
    #[doc = "Bits 24:27 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg7(&mut self) -> CFG7_W {
        CFG7_W { w: self }
    }
    #[doc = "Bits 28:31 - LUART hardware configuration 2"]
    #[inline(always)]
    pub fn cfg8(&mut self) -> CFG8_W {
        CFG8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Hardware Configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr1](index.html) module"]
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
#[doc = "`reset()` method sets HWCFGR1 to value 0x3110_0000"]
impl crate::Resettable for HWCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3110_0000
    }
}
