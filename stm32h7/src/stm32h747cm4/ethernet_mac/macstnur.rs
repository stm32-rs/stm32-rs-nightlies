#[doc = "Register `MACSTNUR` reader"]
pub struct R(crate::R<MACSTNUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSTNUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSTNUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSTNUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACSTNUR` writer"]
pub struct W(crate::W<MACSTNUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACSTNUR_SPEC>;
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
impl From<crate::W<MACSTNUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACSTNUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSSS` reader - Timestamp Sub-seconds"]
pub struct TSSS_R(crate::FieldReader<u32, u32>);
impl TSSS_R {
    pub(crate) fn new(bits: u32) -> Self {
        TSSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSSS` writer - Timestamp Sub-seconds"]
pub struct TSSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Field `ADDSUB` reader - Add or Subtract Time"]
pub struct ADDSUB_R(crate::FieldReader<bool, bool>);
impl ADDSUB_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDSUB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDSUB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDSUB` writer - Add or Subtract Time"]
pub struct ADDSUB_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDSUB_W<'a> {
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
    #[doc = "Bits 0:30 - Timestamp Sub-seconds"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Add or Subtract Time"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Timestamp Sub-seconds"]
    #[inline(always)]
    pub fn tsss(&mut self) -> TSSS_W {
        TSSS_W { w: self }
    }
    #[doc = "Bit 31 - Add or Subtract Time"]
    #[inline(always)]
    pub fn addsub(&mut self) -> ADDSUB_W {
        ADDSUB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System time nanoseconds update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macstnur](index.html) module"]
pub struct MACSTNUR_SPEC;
impl crate::RegisterSpec for MACSTNUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macstnur::R](R) reader structure"]
impl crate::Readable for MACSTNUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macstnur::W](W) writer structure"]
impl crate::Writable for MACSTNUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACSTNUR to value 0"]
impl crate::Resettable for MACSTNUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
