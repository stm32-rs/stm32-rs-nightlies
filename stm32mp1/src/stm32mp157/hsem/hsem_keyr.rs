#[doc = "Register `HSEM_KEYR` reader"]
pub struct R(crate::R<HSEM_KEYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_KEYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_KEYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_KEYR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSEM_KEYR` writer"]
pub struct W(crate::W<HSEM_KEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSEM_KEYR_SPEC>;
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
impl From<crate::W<HSEM_KEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSEM_KEYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - KEY"]
pub struct KEY_R(crate::FieldReader<u16, u16>);
impl KEY_R {
    pub(crate) fn new(bits: u16) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - KEY"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - KEY"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - KEY"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_keyr](index.html) module"]
pub struct HSEM_KEYR_SPEC;
impl crate::RegisterSpec for HSEM_KEYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsem_keyr::R](R) reader structure"]
impl crate::Readable for HSEM_KEYR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsem_keyr::W](W) writer structure"]
impl crate::Writable for HSEM_KEYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSEM_KEYR to value 0"]
impl crate::Resettable for HSEM_KEYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
