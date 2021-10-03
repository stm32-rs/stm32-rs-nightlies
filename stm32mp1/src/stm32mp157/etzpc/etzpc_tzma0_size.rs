#[doc = "Register `ETZPC_TZMA0_SIZE` reader"]
pub struct R(crate::R<ETZPC_TZMA0_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETZPC_TZMA0_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETZPC_TZMA0_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETZPC_TZMA0_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETZPC_TZMA0_SIZE` writer"]
pub struct W(crate::W<ETZPC_TZMA0_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETZPC_TZMA0_SIZE_SPEC>;
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
impl From<crate::W<ETZPC_TZMA0_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETZPC_TZMA0_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R0SIZE` reader - R0SIZE"]
pub struct R0SIZE_R(crate::FieldReader<u16, u16>);
impl R0SIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        R0SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R0SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R0SIZE` writer - R0SIZE"]
pub struct R0SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> R0SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `LOCK` reader - LOCK"]
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
#[doc = "Field `LOCK` writer - LOCK"]
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
    #[doc = "Bits 0:9 - R0SIZE"]
    #[inline(always)]
    pub fn r0size(&self) -> R0SIZE_R {
        R0SIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - R0SIZE"]
    #[inline(always)]
    pub fn r0size(&mut self) -> R0SIZE_W {
        R0SIZE_W { w: self }
    }
    #[doc = "Bit 31 - LOCK"]
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
#[doc = "ETZPC ROM secure size definition\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_tzma0_size](index.html) module"]
pub struct ETZPC_TZMA0_SIZE_SPEC;
impl crate::RegisterSpec for ETZPC_TZMA0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etzpc_tzma0_size::R](R) reader structure"]
impl crate::Readable for ETZPC_TZMA0_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etzpc_tzma0_size::W](W) writer structure"]
impl crate::Writable for ETZPC_TZMA0_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETZPC_TZMA0_SIZE to value 0x03ff"]
impl crate::Resettable for ETZPC_TZMA0_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
