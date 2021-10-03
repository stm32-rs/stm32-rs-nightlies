#[doc = "Register `GICD_ICENABLER4` reader"]
pub struct R(crate::R<GICD_ICENABLER4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICENABLER4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICENABLER4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICENABLER4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICENABLER4` writer"]
pub struct W(crate::W<GICD_ICENABLER4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICENABLER4_SPEC>;
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
impl From<crate::W<GICD_ICENABLER4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICENABLER4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICENABLER4` reader - ICENABLER4"]
pub struct ICENABLER4_R(crate::FieldReader<u32, u32>);
impl ICENABLER4_R {
    pub(crate) fn new(bits: u32) -> Self {
        ICENABLER4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICENABLER4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICENABLER4` writer - ICENABLER4"]
pub struct ICENABLER4_W<'a> {
    w: &'a mut W,
}
impl<'a> ICENABLER4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICENABLER4"]
    #[inline(always)]
    pub fn icenabler4(&self) -> ICENABLER4_R {
        ICENABLER4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICENABLER4"]
    #[inline(always)]
    pub fn icenabler4(&mut self) -> ICENABLER4_W {
        ICENABLER4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For interrupts ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler4](index.html) module"]
pub struct GICD_ICENABLER4_SPEC;
impl crate::RegisterSpec for GICD_ICENABLER4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icenabler4::R](R) reader structure"]
impl crate::Readable for GICD_ICENABLER4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler4::W](W) writer structure"]
impl crate::Writable for GICD_ICENABLER4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICD_ICENABLER4 to value 0"]
impl crate::Resettable for GICD_ICENABLER4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
