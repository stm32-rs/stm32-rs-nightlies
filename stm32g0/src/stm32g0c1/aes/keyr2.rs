#[doc = "Register `KEYR2` reader"]
pub struct R(crate::R<KEYR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYR2` writer"]
pub struct W(crate::W<KEYR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR2_SPEC>;
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
impl From<crate::W<KEYR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - Cryptographic key, bits \\[95:64\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
pub struct KEY_R(crate::FieldReader<u32, u32>);
impl KEY_R {
    pub(crate) fn new(bits: u32) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - Cryptographic key, bits \\[95:64\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[95:64\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cryptographic key, bits \\[95:64\\]
Refer to the AES_KEYR0 register for description of the KEY\\[255:0\\]
bitfield."]
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
#[doc = "AES key register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyr2](index.html) module"]
pub struct KEYR2_SPEC;
impl crate::RegisterSpec for KEYR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyr2::R](R) reader structure"]
impl crate::Readable for KEYR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyr2::W](W) writer structure"]
impl crate::Writable for KEYR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYR2 to value 0"]
impl crate::Resettable for KEYR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
