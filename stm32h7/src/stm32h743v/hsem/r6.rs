#[doc = "Register `R6` reader"]
pub struct R(crate::R<R6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R6` writer"]
pub struct W(crate::W<R6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R6_SPEC>;
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
impl From<crate::W<R6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROCID` reader - Semaphore ProcessID"]
pub struct PROCID_R(crate::FieldReader<u8, u8>);
impl PROCID_R {
    pub(crate) fn new(bits: u8) -> Self {
        PROCID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROCID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROCID` writer - Semaphore ProcessID"]
pub struct PROCID_W<'a> {
    w: &'a mut W,
}
impl<'a> PROCID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `MASTERID` reader - Semaphore MasterID"]
pub struct MASTERID_R(crate::FieldReader<u8, u8>);
impl MASTERID_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASTERID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTERID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTERID` writer - Semaphore MasterID"]
pub struct MASTERID_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTERID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `LOCK` reader - Lock indication"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - Lock indication"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Semaphore MasterID"]
    #[inline(always)]
    pub fn masterid(&self) -> MASTERID_R {
        MASTERID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&mut self) -> PROCID_W {
        PROCID_W { w: self }
    }
    #[doc = "Bits 8:15 - Semaphore MasterID"]
    #[inline(always)]
    pub fn masterid(&mut self) -> MASTERID_W {
        MASTERID_W { w: self }
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM register HSEM_R0 HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r6](index.html) module"]
pub struct R6_SPEC;
impl crate::RegisterSpec for R6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r6::R](R) reader structure"]
impl crate::Readable for R6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r6::W](W) writer structure"]
impl crate::Writable for R6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R6 to value 0"]
impl crate::Resettable for R6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
