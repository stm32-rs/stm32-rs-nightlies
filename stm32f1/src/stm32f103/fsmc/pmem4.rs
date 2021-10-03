#[doc = "Register `PMEM4` reader"]
pub struct R(crate::R<PMEM4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMEM4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMEM4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMEM4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMEM4` writer"]
pub struct W(crate::W<PMEM4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMEM4_SPEC>;
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
impl From<crate::W<PMEM4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMEM4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMHIZx` reader - MEMHIZx"]
pub struct MEMHIZX_R(crate::FieldReader<u8, u8>);
impl MEMHIZX_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEMHIZX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMHIZX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMHIZx` writer - MEMHIZx"]
pub struct MEMHIZX_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMHIZX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `MEMHOLDx` reader - MEMHOLDx"]
pub struct MEMHOLDX_R(crate::FieldReader<u8, u8>);
impl MEMHOLDX_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEMHOLDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMHOLDX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMHOLDx` writer - MEMHOLDx"]
pub struct MEMHOLDX_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMHOLDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `MEMWAITx` reader - MEMWAITx"]
pub struct MEMWAITX_R(crate::FieldReader<u8, u8>);
impl MEMWAITX_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEMWAITX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMWAITX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMWAITx` writer - MEMWAITx"]
pub struct MEMWAITX_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMWAITX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `MEMSETx` reader - MEMSETx"]
pub struct MEMSETX_R(crate::FieldReader<u8, u8>);
impl MEMSETX_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEMSETX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEMSETX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEMSETx` writer - MEMSETx"]
pub struct MEMSETX_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMSETX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhizx(&self) -> MEMHIZX_R {
        MEMHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memholdx(&self) -> MEMHOLDX_R {
        MEMHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwaitx(&self) -> MEMWAITX_R {
        MEMWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memsetx(&self) -> MEMSETX_R {
        MEMSETX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhizx(&mut self) -> MEMHIZX_W {
        MEMHIZX_W { w: self }
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memholdx(&mut self) -> MEMHOLDX_W {
        MEMHOLDX_W { w: self }
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwaitx(&mut self) -> MEMWAITX_W {
        MEMWAITX_W { w: self }
    }
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memsetx(&mut self) -> MEMSETX_W {
        MEMSETX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common memory space timing register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem4](index.html) module"]
pub struct PMEM4_SPEC;
impl crate::RegisterSpec for PMEM4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmem4::R](R) reader structure"]
impl crate::Readable for PMEM4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmem4::W](W) writer structure"]
impl crate::Writable for PMEM4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMEM4 to value 0xfcfc_fcfc"]
impl crate::Resettable for PMEM4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfcfc_fcfc
    }
}
