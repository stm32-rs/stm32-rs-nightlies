#[doc = "Register `DDRPHYC_DTPR2` reader"]
pub struct R(crate::R<DDRPHYC_DTPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DTPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DTPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DTPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DTPR2` writer"]
pub struct W(crate::W<DDRPHYC_DTPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DTPR2_SPEC>;
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
impl From<crate::W<DDRPHYC_DTPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DTPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXS` reader - TXS"]
pub struct TXS_R(crate::FieldReader<u16, u16>);
impl TXS_R {
    pub(crate) fn new(bits: u16) -> Self {
        TXS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXS` writer - TXS"]
pub struct TXS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `TXP` reader - TXP"]
pub struct TXP_R(crate::FieldReader<u8, u8>);
impl TXP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXP` writer - TXP"]
pub struct TXP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `TCKE` reader - TCKE"]
pub struct TCKE_R(crate::FieldReader<u8, u8>);
impl TCKE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCKE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCKE` writer - TCKE"]
pub struct TCKE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCKE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 15)) | ((value as u32 & 0x0f) << 15);
        self.w
    }
}
#[doc = "Field `TDLLK` reader - TDLLK"]
pub struct TDLLK_R(crate::FieldReader<u16, u16>);
impl TDLLK_R {
    pub(crate) fn new(bits: u16) -> Self {
        TDLLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDLLK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDLLK` writer - TDLLK"]
pub struct TDLLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TDLLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | ((value as u32 & 0x03ff) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - TXS"]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:14 - TXP"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:18 - TCKE"]
    #[inline(always)]
    pub fn tcke(&self) -> TCKE_R {
        TCKE_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:28 - TDLLK"]
    #[inline(always)]
    pub fn tdllk(&self) -> TDLLK_R {
        TDLLK_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TXS"]
    #[inline(always)]
    pub fn txs(&mut self) -> TXS_W {
        TXS_W { w: self }
    }
    #[doc = "Bits 10:14 - TXP"]
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W {
        TXP_W { w: self }
    }
    #[doc = "Bits 15:18 - TCKE"]
    #[inline(always)]
    pub fn tcke(&mut self) -> TCKE_W {
        TCKE_W { w: self }
    }
    #[doc = "Bits 19:28 - TDLLK"]
    #[inline(always)]
    pub fn tdllk(&mut self) -> TDLLK_W {
        TDLLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DTP register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtpr2](index.html) module"]
pub struct DDRPHYC_DTPR2_SPEC;
impl crate::RegisterSpec for DDRPHYC_DTPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dtpr2::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtpr2::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DTPR2 to value 0x2004_0d84"]
impl crate::Resettable for DDRPHYC_DTPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2004_0d84
    }
}
