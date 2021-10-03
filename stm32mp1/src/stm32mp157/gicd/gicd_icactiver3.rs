#[doc = "Register `GICD_ICACTIVER3` reader"]
pub struct R(crate::R<GICD_ICACTIVER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICACTIVER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICACTIVER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICACTIVER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICACTIVER3` writer"]
pub struct W(crate::W<GICD_ICACTIVER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICACTIVER3_SPEC>;
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
impl From<crate::W<GICD_ICACTIVER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICACTIVER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACTIVER3` reader - ICACTIVER3"]
pub struct ICACTIVER3_R(crate::FieldReader<u32, u32>);
impl ICACTIVER3_R {
    pub(crate) fn new(bits: u32) -> Self {
        ICACTIVER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICACTIVER3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICACTIVER3` writer - ICACTIVER3"]
pub struct ICACTIVER3_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACTIVER3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICACTIVER3"]
    #[inline(always)]
    pub fn icactiver3(&self) -> ICACTIVER3_R {
        ICACTIVER3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICACTIVER3"]
    #[inline(always)]
    pub fn icactiver3(&mut self) -> ICACTIVER3_W {
        ICACTIVER3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icactiver3](index.html) module"]
pub struct GICD_ICACTIVER3_SPEC;
impl crate::RegisterSpec for GICD_ICACTIVER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icactiver3::R](R) reader structure"]
impl crate::Readable for GICD_ICACTIVER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icactiver3::W](W) writer structure"]
impl crate::Writable for GICD_ICACTIVER3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_ICACTIVER3 to value 0"]
impl crate::Resettable for GICD_ICACTIVER3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
