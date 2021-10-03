#[doc = "Register `DDRPHYC_DTAR` reader"]
pub struct R(crate::R<DDRPHYC_DTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DTAR` writer"]
pub struct W(crate::W<DDRPHYC_DTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DTAR_SPEC>;
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
impl From<crate::W<DDRPHYC_DTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTCOL` reader - DTCOL"]
pub struct DTCOL_R(crate::FieldReader<u16, u16>);
impl DTCOL_R {
    pub(crate) fn new(bits: u16) -> Self {
        DTCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTCOL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTCOL` writer - DTCOL"]
pub struct DTCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `DTROW` reader - DTROW"]
pub struct DTROW_R(crate::FieldReader<u16, u16>);
impl DTROW_R {
    pub(crate) fn new(bits: u16) -> Self {
        DTROW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTROW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTROW` writer - DTROW"]
pub struct DTROW_W<'a> {
    w: &'a mut W,
}
impl<'a> DTROW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 12)) | ((value as u32 & 0xffff) << 12);
        self.w
    }
}
#[doc = "Field `DTBANK` reader - DTBANK"]
pub struct DTBANK_R(crate::FieldReader<u8, u8>);
impl DTBANK_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTBANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTBANK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTBANK` writer - DTBANK"]
pub struct DTBANK_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `DTMPR` reader - DTMPR"]
pub struct DTMPR_R(crate::FieldReader<bool, bool>);
impl DTMPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTMPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTMPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTMPR` writer - DTMPR"]
pub struct DTMPR_W<'a> {
    w: &'a mut W,
}
impl<'a> DTMPR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DTCOL"]
    #[inline(always)]
    pub fn dtcol(&self) -> DTCOL_R {
        DTCOL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:27 - DTROW"]
    #[inline(always)]
    pub fn dtrow(&self) -> DTROW_R {
        DTROW_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:30 - DTBANK"]
    #[inline(always)]
    pub fn dtbank(&self) -> DTBANK_R {
        DTBANK_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - DTMPR"]
    #[inline(always)]
    pub fn dtmpr(&self) -> DTMPR_R {
        DTMPR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - DTCOL"]
    #[inline(always)]
    pub fn dtcol(&mut self) -> DTCOL_W {
        DTCOL_W { w: self }
    }
    #[doc = "Bits 12:27 - DTROW"]
    #[inline(always)]
    pub fn dtrow(&mut self) -> DTROW_W {
        DTROW_W { w: self }
    }
    #[doc = "Bits 28:30 - DTBANK"]
    #[inline(always)]
    pub fn dtbank(&mut self) -> DTBANK_W {
        DTBANK_W { w: self }
    }
    #[doc = "Bit 31 - DTMPR"]
    #[inline(always)]
    pub fn dtmpr(&mut self) -> DTMPR_W {
        DTMPR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DTA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtar](index.html) module"]
pub struct DDRPHYC_DTAR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dtar::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtar::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DTAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DTAR to value 0"]
impl crate::Resettable for DDRPHYC_DTAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
