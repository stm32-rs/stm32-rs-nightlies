#[doc = "Register `HSEM_CR` writer"]
pub struct W(crate::W<HSEM_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSEM_CR_SPEC>;
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
impl From<crate::W<HSEM_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSEM_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COREID` writer - COREID"]
pub struct COREID_W<'a> {
    w: &'a mut W,
}
impl<'a> COREID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
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
impl W {
    #[doc = "Bits 8:11 - COREID"]
    #[inline(always)]
    pub fn coreid(&mut self) -> COREID_W {
        COREID_W { w: self }
    }
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
#[doc = "Only Write accesses with authorized AHB bus master IDs are granted. Write accesses with unauthorized AHB bus master IDs are discarded.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_cr](index.html) module"]
pub struct HSEM_CR_SPEC;
impl crate::RegisterSpec for HSEM_CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hsem_cr::W](W) writer structure"]
impl crate::Writable for HSEM_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSEM_CR to value 0"]
impl crate::Resettable for HSEM_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
