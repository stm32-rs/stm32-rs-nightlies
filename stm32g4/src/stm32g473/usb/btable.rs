#[doc = "Register `BTABLE` reader"]
pub struct R(crate::R<BTABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTABLE` writer"]
pub struct W(crate::W<BTABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTABLE_SPEC>;
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
impl From<crate::W<BTABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BTABLE` reader - BTABLE"]
pub struct BTABLE_R(crate::FieldReader<u16, u16>);
impl BTABLE_R {
    pub(crate) fn new(bits: u16) -> Self {
        BTABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTABLE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTABLE` writer - BTABLE"]
pub struct BTABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 3)) | ((value as u32 & 0x1fff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:15 - BTABLE"]
    #[inline(always)]
    pub fn btable(&self) -> BTABLE_R {
        BTABLE_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - BTABLE"]
    #[inline(always)]
    pub fn btable(&mut self) -> BTABLE_W {
        BTABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer table address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btable](index.html) module"]
pub struct BTABLE_SPEC;
impl crate::RegisterSpec for BTABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btable::R](R) reader structure"]
impl crate::Readable for BTABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btable::W](W) writer structure"]
impl crate::Writable for BTABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BTABLE to value 0"]
impl crate::Resettable for BTABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
