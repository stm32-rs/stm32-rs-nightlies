#[doc = "Register `CRCDATAR` reader"]
pub struct R(crate::R<CRCDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCDATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCDATAR` writer"]
pub struct W(crate::W<CRCDATAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCDATAR_SPEC>;
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
impl From<crate::W<CRCDATAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCDATAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC_DATA` reader - CRC result"]
pub struct CRC_DATA_R(crate::FieldReader<u32, u32>);
impl CRC_DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        CRC_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_DATA` writer - CRC result"]
pub struct CRC_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRC result"]
    #[inline(always)]
    pub fn crc_data(&self) -> CRC_DATA_R {
        CRC_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC result"]
    #[inline(always)]
    pub fn crc_data(&mut self) -> CRC_DATA_W {
        CRC_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH CRC data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcdatar](index.html) module"]
pub struct CRCDATAR_SPEC;
impl crate::RegisterSpec for CRCDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcdatar::R](R) reader structure"]
impl crate::Readable for CRCDATAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcdatar::W](W) writer structure"]
impl crate::Writable for CRCDATAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCDATAR to value 0"]
impl crate::Resettable for CRCDATAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
