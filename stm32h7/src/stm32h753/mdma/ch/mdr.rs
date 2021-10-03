#[doc = "Register `MDR` reader"]
pub struct R(crate::R<MDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDR` writer"]
pub struct W(crate::W<MDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDR_SPEC>;
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
impl From<crate::W<MDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDR` reader - Mask data"]
pub struct MDR_R(crate::FieldReader<u32, u32>);
impl MDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        MDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDR` writer - Mask data"]
pub struct MDR_W<'a> {
    w: &'a mut W,
}
impl<'a> MDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask data"]
    #[inline(always)]
    pub fn mdr(&self) -> MDR_R {
        MDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask data"]
    #[inline(always)]
    pub fn mdr(&mut self) -> MDR_W {
        MDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDMA channel x Mask Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdr](index.html) module"]
pub struct MDR_SPEC;
impl crate::RegisterSpec for MDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdr::R](R) reader structure"]
impl crate::Readable for MDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdr::W](W) writer structure"]
impl crate::Writable for MDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
