#[doc = "Register `GICD_ISENABLER3` reader"]
pub struct R(crate::R<GICD_ISENABLER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ISENABLER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ISENABLER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ISENABLER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ISENABLER3` writer"]
pub struct W(crate::W<GICD_ISENABLER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ISENABLER3_SPEC>;
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
impl From<crate::W<GICD_ISENABLER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ISENABLER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISENABLER3` reader - ISENABLER3"]
pub struct ISENABLER3_R(crate::FieldReader<u32, u32>);
impl ISENABLER3_R {
    pub(crate) fn new(bits: u32) -> Self {
        ISENABLER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISENABLER3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISENABLER3` writer - ISENABLER3"]
pub struct ISENABLER3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENABLER3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISENABLER3"]
    #[inline(always)]
    pub fn isenabler3(&self) -> ISENABLER3_R {
        ISENABLER3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISENABLER3"]
    #[inline(always)]
    pub fn isenabler3(&mut self) -> ISENABLER3_W {
        ISENABLER3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isenabler3](index.html) module"]
pub struct GICD_ISENABLER3_SPEC;
impl crate::RegisterSpec for GICD_ISENABLER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_isenabler3::R](R) reader structure"]
impl crate::Readable for GICD_ISENABLER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_isenabler3::W](W) writer structure"]
impl crate::Writable for GICD_ISENABLER3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_ISENABLER3 to value 0"]
impl crate::Resettable for GICD_ISENABLER3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
