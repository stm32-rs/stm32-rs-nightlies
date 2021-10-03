#[doc = "Register `MDMA_C9MDR` reader"]
pub struct R(crate::R<MDMA_C9MDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C9MDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C9MDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C9MDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMA_C9MDR` writer"]
pub struct W(crate::W<MDMA_C9MDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C9MDR_SPEC>;
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
impl From<crate::W<MDMA_C9MDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C9MDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDR` reader - MDR"]
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
#[doc = "Field `MDR` writer - MDR"]
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
    #[doc = "Bits 0:31 - MDR"]
    #[inline(always)]
    pub fn mdr(&self) -> MDR_R {
        MDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MDR"]
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
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x24).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c9mdr](index.html) module"]
pub struct MDMA_C9MDR_SPEC;
impl crate::RegisterSpec for MDMA_C9MDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdma_c9mdr::R](R) reader structure"]
impl crate::Readable for MDMA_C9MDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdma_c9mdr::W](W) writer structure"]
impl crate::Writable for MDMA_C9MDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDMA_C9MDR to value 0"]
impl crate::Resettable for MDMA_C9MDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
