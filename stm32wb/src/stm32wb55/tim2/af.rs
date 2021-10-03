#[doc = "Register `AF` reader"]
pub struct R(crate::R<AF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AF` writer"]
pub struct W(crate::W<AF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF_SPEC>;
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
impl From<crate::W<AF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETRSEL` reader - External trigger source selection"]
pub struct ETRSEL_R(crate::FieldReader<u8, u8>);
impl ETRSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETRSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETRSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETRSEL` writer - External trigger source selection"]
pub struct ETRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | ((value as u32 & 0x07) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:16 - External trigger source selection"]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 14:16 - External trigger source selection"]
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W {
        ETRSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM2 alternate function option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [af](index.html) module"]
pub struct AF_SPEC;
impl crate::RegisterSpec for AF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [af::R](R) reader structure"]
impl crate::Readable for AF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [af::W](W) writer structure"]
impl crate::Writable for AF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AF to value 0"]
impl crate::Resettable for AF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
