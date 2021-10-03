#[doc = "Register `GICD_IPRIORITYR43` reader"]
pub struct R(crate::R<GICD_IPRIORITYR43_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR43_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR43_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR43_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR43` writer"]
pub struct W(crate::W<GICD_IPRIORITYR43_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR43_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR43_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR43_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIORITY0` reader - PRIORITY0"]
pub struct PRIORITY0_R(crate::FieldReader<u8, u8>);
impl PRIORITY0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIORITY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIORITY0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIORITY0` writer - PRIORITY0"]
pub struct PRIORITY0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `PRIORITY1` reader - PRIORITY1"]
pub struct PRIORITY1_R(crate::FieldReader<u8, u8>);
impl PRIORITY1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIORITY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIORITY1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIORITY1` writer - PRIORITY1"]
pub struct PRIORITY1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `PRIORITY2` reader - PRIORITY2"]
pub struct PRIORITY2_R(crate::FieldReader<u8, u8>);
impl PRIORITY2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIORITY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIORITY2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIORITY2` writer - PRIORITY2"]
pub struct PRIORITY2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Field `PRIORITY3` reader - PRIORITY3"]
pub struct PRIORITY3_R(crate::FieldReader<u8, u8>);
impl PRIORITY3_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIORITY3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIORITY3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIORITY3` writer - PRIORITY3"]
pub struct PRIORITY3_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - PRIORITY0"]
    #[inline(always)]
    pub fn priority0(&self) -> PRIORITY0_R {
        PRIORITY0_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PRIORITY1"]
    #[inline(always)]
    pub fn priority1(&self) -> PRIORITY1_R {
        PRIORITY1_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - PRIORITY2"]
    #[inline(always)]
    pub fn priority2(&self) -> PRIORITY2_R {
        PRIORITY2_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - PRIORITY3"]
    #[inline(always)]
    pub fn priority3(&self) -> PRIORITY3_R {
        PRIORITY3_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - PRIORITY0"]
    #[inline(always)]
    pub fn priority0(&mut self) -> PRIORITY0_W {
        PRIORITY0_W { w: self }
    }
    #[doc = "Bits 11:15 - PRIORITY1"]
    #[inline(always)]
    pub fn priority1(&mut self) -> PRIORITY1_W {
        PRIORITY1_W { w: self }
    }
    #[doc = "Bits 19:23 - PRIORITY2"]
    #[inline(always)]
    pub fn priority2(&mut self) -> PRIORITY2_W {
        PRIORITY2_W { w: self }
    }
    #[doc = "Bits 27:31 - PRIORITY3"]
    #[inline(always)]
    pub fn priority3(&mut self) -> PRIORITY3_W {
        PRIORITY3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICD interrupt priority register 43\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr43](index.html) module"]
pub struct GICD_IPRIORITYR43_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR43_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr43::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR43_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr43::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR43_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR43 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR43_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
