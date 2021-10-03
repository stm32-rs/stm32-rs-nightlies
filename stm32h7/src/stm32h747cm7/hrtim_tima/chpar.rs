#[doc = "Register `CHPAR` reader"]
pub struct R(crate::R<CHPAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHPAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHPAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHPAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHPAR` writer"]
pub struct W(crate::W<CHPAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHPAR_SPEC>;
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
impl From<crate::W<CHPAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHPAR_SPEC>) -> Self {
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
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `CHPDTY` reader - Timerx chopper duty cycle value"]
pub struct CHPDTY_R(crate::FieldReader<u8, u8>);
impl CHPDTY_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHPDTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHPDTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHPDTY` writer - Timerx chopper duty cycle value"]
pub struct CHPDTY_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPDTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `CHPFRQ` reader - Timerx carrier frequency value"]
pub struct CHPFRQ_R(crate::FieldReader<u8, u8>);
impl CHPFRQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHPFRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHPFRQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHPFRQ` writer - Timerx carrier frequency value"]
pub struct CHPFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPFRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    pub fn chpdty(&self) -> CHPDTY_R {
        CHPDTY_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn chpfrq(&self) -> CHPFRQ_R {
        CHPFRQ_R::new((self.bits & 0x0f) as u8)
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
    pub fn chpdty(&mut self) -> CHPDTY_W {
        CHPDTY_W { w: self }
    }
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn chpfrq(&mut self) -> CHPFRQ_W {
        CHPFRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Chopper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpar](index.html) module"]
pub struct CHPAR_SPEC;
impl crate::RegisterSpec for CHPAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chpar::R](R) reader structure"]
impl crate::Readable for CHPAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chpar::W](W) writer structure"]
impl crate::Writable for CHPAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHPAR to value 0"]
impl crate::Resettable for CHPAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
