#[doc = "Register `KEYR7` reader"]
pub struct R(crate::R<KEYR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYR7` writer"]
pub struct W(crate::W<KEYR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR7_SPEC>;
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
impl From<crate::W<KEYR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_KEYR7` reader - AES key register (MSB key \\[255:224\\])"]
pub struct AES_KEYR7_R(crate::FieldReader<u32, u32>);
impl AES_KEYR7_R {
    pub(crate) fn new(bits: u32) -> Self {
        AES_KEYR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_KEYR7_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES_KEYR7` writer - AES key register (MSB key \\[255:224\\])"]
pub struct AES_KEYR7_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_KEYR7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[255:224\\])"]
    #[inline(always)]
    pub fn aes_keyr7(&self) -> AES_KEYR7_R {
        AES_KEYR7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[255:224\\])"]
    #[inline(always)]
    pub fn aes_keyr7(&mut self) -> AES_KEYR7_W {
        AES_KEYR7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr7](index.html) module"]
pub struct KEYR7_SPEC;
impl crate::RegisterSpec for KEYR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyr7::R](R) reader structure"]
impl crate::Readable for KEYR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyr7::W](W) writer structure"]
impl crate::Writable for KEYR7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYR7 to value 0"]
impl crate::Resettable for KEYR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
