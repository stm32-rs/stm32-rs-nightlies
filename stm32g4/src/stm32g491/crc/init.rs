#[doc = "Register `INIT` reader"]
pub struct R(crate::R<INIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INIT` writer"]
pub struct W(crate::W<INIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INIT_SPEC>;
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
impl From<crate::W<INIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_INIT` reader - Programmable initial CRC value"]
pub struct CRC_INIT_R(crate::FieldReader<u32, u32>);
impl CRC_INIT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CRC_INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_INIT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_INIT` writer - Programmable initial CRC value"]
pub struct CRC_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Programmable initial CRC value"]
    #[inline(always)]
    pub fn crc_init(&self) -> CRC_INIT_R {
        CRC_INIT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable initial CRC value"]
    #[inline(always)]
    pub fn crc_init(&mut self) -> CRC_INIT_W {
        CRC_INIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initial CRC value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init](index.html) module"]
pub struct INIT_SPEC;
impl crate::RegisterSpec for INIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [init::R](R) reader structure"]
impl crate::Readable for INIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [init::W](W) writer structure"]
impl crate::Writable for INIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INIT to value 0xffff_ffff"]
impl crate::Resettable for INIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
