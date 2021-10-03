#[doc = "Register `HUFFMIN_8` reader"]
pub struct R(crate::R<HUFFMIN_8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFMIN_8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFMIN_8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFMIN_8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HUFFMIN_8` writer"]
pub struct W(crate::W<HUFFMIN_8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HUFFMIN_8_SPEC>;
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
impl From<crate::W<HUFFMIN_8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HUFFMIN_8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HuffMin_RAM` reader - HuffMin RAM"]
pub struct HUFFMIN_RAM_R(crate::FieldReader<u32, u32>);
impl HUFFMIN_RAM_R {
    pub(crate) fn new(bits: u32) -> Self {
        HUFFMIN_RAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HUFFMIN_RAM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HuffMin_RAM` writer - HuffMin RAM"]
pub struct HUFFMIN_RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> HUFFMIN_RAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HuffMin RAM"]
    #[inline(always)]
    pub fn huff_min_ram(&self) -> HUFFMIN_RAM_R {
        HUFFMIN_RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HuffMin RAM"]
    #[inline(always)]
    pub fn huff_min_ram(&mut self) -> HUFFMIN_RAM_W {
        HUFFMIN_RAM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin_8](index.html) module"]
pub struct HUFFMIN_8_SPEC;
impl crate::RegisterSpec for HUFFMIN_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffmin_8::R](R) reader structure"]
impl crate::Readable for HUFFMIN_8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [huffmin_8::W](W) writer structure"]
impl crate::Writable for HUFFMIN_8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HUFFMIN_8 to value 0"]
impl crate::Resettable for HUFFMIN_8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
