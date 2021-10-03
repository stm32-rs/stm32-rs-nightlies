#[doc = "Register `TZC_REGION_TOP_LOW1` reader"]
pub struct R(crate::R<TZC_REGION_TOP_LOW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_REGION_TOP_LOW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_REGION_TOP_LOW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_REGION_TOP_LOW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZC_REGION_TOP_LOW1` writer"]
pub struct W(crate::W<TZC_REGION_TOP_LOW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_REGION_TOP_LOW1_SPEC>;
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
impl From<crate::W<TZC_REGION_TOP_LOW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_REGION_TOP_LOW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOP_ADDRESS_LOW` reader - TOP_ADDRESS_LOW"]
pub struct TOP_ADDRESS_LOW_R(crate::FieldReader<u32, u32>);
impl TOP_ADDRESS_LOW_R {
    pub(crate) fn new(bits: u32) -> Self {
        TOP_ADDRESS_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOP_ADDRESS_LOW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOP_ADDRESS_LOW` writer - TOP_ADDRESS_LOW"]
pub struct TOP_ADDRESS_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_ADDRESS_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | ((value as u32 & 0x000f_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - TOP_ADDRESS_LOW"]
    #[inline(always)]
    pub fn top_address_low(&self) -> TOP_ADDRESS_LOW_R {
        TOP_ADDRESS_LOW_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 12:31 - TOP_ADDRESS_LOW"]
    #[inline(always)]
    pub fn top_address_low(&mut self) -> TOP_ADDRESS_LOW_W {
        TOP_ADDRESS_LOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Top address bits \\[31:12\\]
for region x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_region_top_low1](index.html) module"]
pub struct TZC_REGION_TOP_LOW1_SPEC;
impl crate::RegisterSpec for TZC_REGION_TOP_LOW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_region_top_low1::R](R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_region_top_low1::W](W) writer structure"]
impl crate::Writable for TZC_REGION_TOP_LOW1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZC_REGION_TOP_LOW1 to value 0x0fff"]
impl crate::Resettable for TZC_REGION_TOP_LOW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
