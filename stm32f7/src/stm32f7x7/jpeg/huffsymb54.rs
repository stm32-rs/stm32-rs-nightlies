#[doc = "Register `HUFFSYMB54` reader"]
pub struct R(crate::R<HUFFSYMB54_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFSYMB54_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFSYMB54_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFSYMB54_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HUFFSYMB54` writer"]
pub struct W(crate::W<HUFFSYMB54_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HUFFSYMB54_SPEC>;
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
impl From<crate::W<HUFFSYMB54_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HUFFSYMB54_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HuffSymb_RAM` reader - DHTSymb RAM"]
pub struct HUFFSYMB_RAM_R(crate::FieldReader<u32, u32>);
impl HUFFSYMB_RAM_R {
    pub(crate) fn new(bits: u32) -> Self {
        HUFFSYMB_RAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HUFFSYMB_RAM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HuffSymb_RAM` writer - DHTSymb RAM"]
pub struct HUFFSYMB_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> HUFFSYMB_RAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DHTSymb RAM"]
    #[inline(always)]
    pub fn huff_symb_ram(&self) -> HUFFSYMB_RAM_R {
        HUFFSYMB_RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DHTSymb RAM"]
    #[inline(always)]
    pub fn huff_symb_ram(&mut self) -> HUFFSYMB_RAM_W {
        HUFFSYMB_RAM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb54](index.html) module"]
pub struct HUFFSYMB54_SPEC;
impl crate::RegisterSpec for HUFFSYMB54_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffsymb54::R](R) reader structure"]
impl crate::Readable for HUFFSYMB54_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [huffsymb54::W](W) writer structure"]
impl crate::Writable for HUFFSYMB54_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HUFFSYMB54 to value 0"]
impl crate::Resettable for HUFFSYMB54_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
