#[doc = "Register `PCROP1ASR` reader"]
pub struct R(crate::R<PCROP1ASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1ASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1ASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1ASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCROP1ASR` writer"]
pub struct W(crate::W<PCROP1ASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP1ASR_SPEC>;
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
impl From<crate::W<PCROP1ASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP1ASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCROP1A_STRT` reader - Bank 1 PCROPQ area start offset"]
pub struct PCROP1A_STRT_R(crate::FieldReader<u16, u16>);
impl PCROP1A_STRT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PCROP1A_STRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCROP1A_STRT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCROP1A_STRT` writer - Bank 1 PCROPQ area start offset"]
pub struct PCROP1A_STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCROP1A_STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Bank 1 PCROPQ area start offset"]
    #[inline(always)]
    pub fn pcrop1a_strt(&self) -> PCROP1A_STRT_R {
        PCROP1A_STRT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Bank 1 PCROPQ area start offset"]
    #[inline(always)]
    pub fn pcrop1a_strt(&mut self) -> PCROP1A_STRT_W {
        PCROP1A_STRT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Bank 1 PCROP Start address zone A register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrop1asr](index.html) module"]
pub struct PCROP1ASR_SPEC;
impl crate::RegisterSpec for PCROP1ASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrop1asr::R](R) reader structure"]
impl crate::Readable for PCROP1ASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrop1asr::W](W) writer structure"]
impl crate::Writable for PCROP1ASR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCROP1ASR to value 0xffff_fe00"]
impl crate::Resettable for PCROP1ASR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_fe00
    }
}
