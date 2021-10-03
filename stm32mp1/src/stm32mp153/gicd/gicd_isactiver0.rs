#[doc = "Register `GICD_ISACTIVER0` reader"]
pub struct R(crate::R<GICD_ISACTIVER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ISACTIVER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ISACTIVER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ISACTIVER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ISACTIVER0` writer"]
pub struct W(crate::W<GICD_ISACTIVER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ISACTIVER0_SPEC>;
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
impl From<crate::W<GICD_ISACTIVER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ISACTIVER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISACTIVER0` reader - ISACTIVER0"]
pub struct ISACTIVER0_R(crate::FieldReader<u32, u32>);
impl ISACTIVER0_R {
    pub(crate) fn new(bits: u32) -> Self {
        ISACTIVER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISACTIVER0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISACTIVER0` writer - ISACTIVER0"]
pub struct ISACTIVER0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISACTIVER0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISACTIVER0"]
    #[inline(always)]
    pub fn isactiver0(&self) -> ISACTIVER0_R {
        ISACTIVER0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISACTIVER0"]
    #[inline(always)]
    pub fn isactiver0(&mut self) -> ISACTIVER0_W {
        ISACTIVER0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isactiver0](index.html) module"]
pub struct GICD_ISACTIVER0_SPEC;
impl crate::RegisterSpec for GICD_ISACTIVER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_isactiver0::R](R) reader structure"]
impl crate::Readable for GICD_ISACTIVER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_isactiver0::W](W) writer structure"]
impl crate::Writable for GICD_ISACTIVER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_ISACTIVER0 to value 0"]
impl crate::Resettable for GICD_ISACTIVER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}