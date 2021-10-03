#[doc = "Register `DTAR` reader"]
pub struct R(crate::R<DTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTAR` writer"]
pub struct W(crate::W<DTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTAR_SPEC>;
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
impl From<crate::W<DTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTFLKx` reader - Deadtime Falling Lock"]
pub struct DTFLKX_R(crate::FieldReader<bool, bool>);
impl DTFLKX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTFLKX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTFLKX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTFLKx` writer - Deadtime Falling Lock"]
pub struct DTFLKX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFLKX_W<'a> {
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
#[doc = "Field `DTFSLKx` reader - Deadtime Falling Sign Lock"]
pub struct DTFSLKX_R(crate::FieldReader<bool, bool>);
impl DTFSLKX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTFSLKX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTFSLKX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTFSLKx` writer - Deadtime Falling Sign Lock"]
pub struct DTFSLKX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFSLKX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `SDTFx` reader - Sign Deadtime Falling value"]
pub struct SDTFX_R(crate::FieldReader<bool, bool>);
impl SDTFX_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDTFX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDTFX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDTFx` writer - Sign Deadtime Falling value"]
pub struct SDTFX_W<'a> {
    w: &'a mut W,
}
impl<'a> SDTFX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `DTFx` reader - Deadtime Falling value"]
pub struct DTFX_R(crate::FieldReader<u16, u16>);
impl DTFX_R {
    pub(crate) fn new(bits: u16) -> Self {
        DTFX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTFX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTFx` writer - Deadtime Falling value"]
pub struct DTFX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTFX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
#[doc = "Field `DTRLKx` reader - Deadtime Rising Lock"]
pub struct DTRLKX_R(crate::FieldReader<bool, bool>);
impl DTRLKX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTRLKX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTRLKX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTRLKx` writer - Deadtime Rising Lock"]
pub struct DTRLKX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRLKX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `DTRSLKx` reader - Deadtime Rising Sign Lock"]
pub struct DTRSLKX_R(crate::FieldReader<bool, bool>);
impl DTRSLKX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTRSLKX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTRSLKX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTRSLKx` writer - Deadtime Rising Sign Lock"]
pub struct DTRSLKX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRSLKX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `DTPRSC` reader - Deadtime Prescaler"]
pub struct DTPRSC_R(crate::FieldReader<u8, u8>);
impl DTPRSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTPRSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTPRSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTPRSC` writer - Deadtime Prescaler"]
pub struct DTPRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | ((value as u32 & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `SDTRx` reader - Sign Deadtime Rising value"]
pub struct SDTRX_R(crate::FieldReader<bool, bool>);
impl SDTRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDTRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDTRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDTRx` writer - Sign Deadtime Rising value"]
pub struct SDTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> SDTRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `DTRx` reader - Deadtime Rising value"]
pub struct DTRX_R(crate::FieldReader<u16, u16>);
impl DTRX_R {
    pub(crate) fn new(bits: u16) -> Self {
        DTRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTRX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTRx` writer - Deadtime Rising value"]
pub struct DTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    pub fn dtflkx(&self) -> DTFLKX_R {
        DTFLKX_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    pub fn dtfslkx(&self) -> DTFSLKX_R {
        DTFSLKX_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    pub fn sdtfx(&self) -> SDTFX_R {
        SDTFX_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    pub fn dtfx(&self) -> DTFX_R {
        DTFX_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    pub fn dtrlkx(&self) -> DTRLKX_R {
        DTRLKX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    pub fn dtrslkx(&self) -> DTRSLKX_R {
        DTRSLKX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    pub fn dtprsc(&self) -> DTPRSC_R {
        DTPRSC_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    pub fn sdtrx(&self) -> SDTRX_R {
        SDTRX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    pub fn dtrx(&self) -> DTRX_R {
        DTRX_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    pub fn dtflkx(&mut self) -> DTFLKX_W {
        DTFLKX_W { w: self }
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    pub fn dtfslkx(&mut self) -> DTFSLKX_W {
        DTFSLKX_W { w: self }
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    pub fn sdtfx(&mut self) -> SDTFX_W {
        SDTFX_W { w: self }
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    pub fn dtfx(&mut self) -> DTFX_W {
        DTFX_W { w: self }
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    pub fn dtrlkx(&mut self) -> DTRLKX_W {
        DTRLKX_W { w: self }
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    pub fn dtrslkx(&mut self) -> DTRSLKX_W {
        DTRSLKX_W { w: self }
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    pub fn dtprsc(&mut self) -> DTPRSC_W {
        DTPRSC_W { w: self }
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    pub fn sdtrx(&mut self) -> SDTRX_W {
        SDTRX_W { w: self }
    }
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    pub fn dtrx(&mut self) -> DTRX_W {
        DTRX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Deadtime Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtar](index.html) module"]
pub struct DTAR_SPEC;
impl crate::RegisterSpec for DTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtar::R](R) reader structure"]
impl crate::Readable for DTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtar::W](W) writer structure"]
impl crate::Writable for DTAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTAR to value 0"]
impl crate::Resettable for DTAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
