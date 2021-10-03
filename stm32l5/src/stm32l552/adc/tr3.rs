#[doc = "Register `TR3` reader"]
pub struct R(crate::R<TR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR3` writer"]
pub struct W(crate::W<TR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR3_SPEC>;
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
impl From<crate::W<TR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT3` reader - HT3"]
pub struct HT3_R(crate::FieldReader<u8, u8>);
impl HT3_R {
    pub(crate) fn new(bits: u8) -> Self {
        HT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HT3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HT3` writer - HT3"]
pub struct HT3_W<'a> {
    w: &'a mut W,
}
impl<'a> HT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `LT3` reader - LT3"]
pub struct LT3_R(crate::FieldReader<u8, u8>);
impl LT3_R {
    pub(crate) fn new(bits: u8) -> Self {
        LT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LT3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LT3` writer - LT3"]
pub struct LT3_W<'a> {
    w: &'a mut W,
}
impl<'a> LT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - HT3"]
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - LT3"]
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - HT3"]
    #[inline(always)]
    pub fn ht3(&mut self) -> HT3_W {
        HT3_W { w: self }
    }
    #[doc = "Bits 0:7 - LT3"]
    #[inline(always)]
    pub fn lt3(&mut self) -> LT3_W {
        LT3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "watchdog threshold register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr3](index.html) module"]
pub struct TR3_SPEC;
impl crate::RegisterSpec for TR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr3::R](R) reader structure"]
impl crate::Readable for TR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr3::W](W) writer structure"]
impl crate::Writable for TR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR3 to value 0x0fff_0000"]
impl crate::Resettable for TR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}
