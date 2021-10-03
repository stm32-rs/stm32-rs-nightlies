#[doc = "Register `CHPER` reader"]
pub struct R(crate::R<CHPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHPER` writer"]
pub struct W(crate::W<CHPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHPER_SPEC>;
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
impl From<crate::W<CHPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRTPW` reader - STRTPW"]
pub struct STRTPW_R(crate::FieldReader<u8, u8>);
impl STRTPW_R {
    pub(crate) fn new(bits: u8) -> Self {
        STRTPW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRTPW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRTPW` writer - STRTPW"]
pub struct STRTPW_W<'a> {
    w: &'a mut W,
}
impl<'a> STRTPW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `CARDTY` reader - Timerx chopper duty cycle value"]
pub struct CARDTY_R(crate::FieldReader<u8, u8>);
impl CARDTY_R {
    pub(crate) fn new(bits: u8) -> Self {
        CARDTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARDTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARDTY` writer - Timerx chopper duty cycle value"]
pub struct CARDTY_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `CARFRQ` reader - Timerx carrier frequency value"]
pub struct CARFRQ_R(crate::FieldReader<u8, u8>);
impl CARFRQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        CARFRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARFRQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARFRQ` writer - Timerx carrier frequency value"]
pub struct CARFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CARFRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    pub fn strtpw(&self) -> STRTPW_R {
        STRTPW_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    pub fn cardty(&self) -> CARDTY_R {
        CARDTY_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn carfrq(&self) -> CARFRQ_R {
        CARFRQ_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    pub fn strtpw(&mut self) -> STRTPW_W {
        STRTPW_W { w: self }
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    pub fn cardty(&mut self) -> CARDTY_W {
        CARDTY_W { w: self }
    }
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn carfrq(&mut self) -> CARFRQ_W {
        CARFRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Chopper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chper](index.html) module"]
pub struct CHPER_SPEC;
impl crate::RegisterSpec for CHPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chper::R](R) reader structure"]
impl crate::Readable for CHPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chper::W](W) writer structure"]
impl crate::Writable for CHPER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHPER to value 0"]
impl crate::Resettable for CHPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
