#[doc = "Register `ISR1` reader"]
pub struct R(crate::R<ISR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR1` writer"]
pub struct W(crate::W<ISR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR1_SPEC>;
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
impl From<crate::W<ISR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPRXE` reader - Generic payload receive error"]
pub struct GPRXE_R(crate::FieldReader<bool, bool>);
impl GPRXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPRXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPRXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPRXE` writer - Generic payload receive error"]
pub struct GPRXE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPRXE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `GPRDE` reader - Generic payload read error"]
pub struct GPRDE_R(crate::FieldReader<bool, bool>);
impl GPRDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPRDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPRDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPRDE` writer - Generic payload read error"]
pub struct GPRDE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPRDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `GPTXE` reader - Generic payload transmit error"]
pub struct GPTXE_R(crate::FieldReader<bool, bool>);
impl GPTXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPTXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPTXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPTXE` writer - Generic payload transmit error"]
pub struct GPTXE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTXE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `GPWRE` reader - Generic payload write error"]
pub struct GPWRE_R(crate::FieldReader<bool, bool>);
impl GPWRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPWRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPWRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPWRE` writer - Generic payload write error"]
pub struct GPWRE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPWRE_W<'a> {
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
#[doc = "Field `GCWRE` reader - Generic command write error"]
pub struct GCWRE_R(crate::FieldReader<bool, bool>);
impl GCWRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GCWRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCWRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCWRE` writer - Generic command write error"]
pub struct GCWRE_W<'a> {
    w: &'a mut W,
}
impl<'a> GCWRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `LPWRE` reader - LTDC payload write error"]
pub struct LPWRE_R(crate::FieldReader<bool, bool>);
impl LPWRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPWRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPWRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPWRE` writer - LTDC payload write error"]
pub struct LPWRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPWRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `EOTPE` reader - EoTp error"]
pub struct EOTPE_R(crate::FieldReader<bool, bool>);
impl EOTPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOTPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOTPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOTPE` writer - EoTp error"]
pub struct EOTPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PSE` reader - Packet size error"]
pub struct PSE_R(crate::FieldReader<bool, bool>);
impl PSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSE` writer - Packet size error"]
pub struct PSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CRCE` reader - CRC error"]
pub struct CRCE_R(crate::FieldReader<bool, bool>);
impl CRCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCE` writer - CRC error"]
pub struct CRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ECCME` reader - ECC multi"]
pub struct ECCME_R(crate::FieldReader<bool, bool>);
impl ECCME_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCME` writer - ECC multi"]
pub struct ECCME_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ECCSE` reader - ECC single"]
pub struct ECCSE_R(crate::FieldReader<bool, bool>);
impl ECCSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECCSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCSE` writer - ECC single"]
pub struct ECCSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TOLPRX` reader - Timeout low"]
pub struct TOLPRX_R(crate::FieldReader<bool, bool>);
impl TOLPRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOLPRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOLPRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOLPRX` writer - Timeout low"]
pub struct TOLPRX_W<'a> {
    w: &'a mut W,
}
impl<'a> TOLPRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TOHSTX` reader - Timeout high"]
pub struct TOHSTX_R(crate::FieldReader<bool, bool>);
impl TOHSTX_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOHSTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOHSTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOHSTX` writer - Timeout high"]
pub struct TOHSTX_W<'a> {
    w: &'a mut W,
}
impl<'a> TOHSTX_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - Generic payload receive error"]
    #[inline(always)]
    pub fn gprxe(&self) -> GPRXE_R {
        GPRXE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Generic payload read error"]
    #[inline(always)]
    pub fn gprde(&self) -> GPRDE_R {
        GPRDE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Generic payload transmit error"]
    #[inline(always)]
    pub fn gptxe(&self) -> GPTXE_R {
        GPTXE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Generic payload write error"]
    #[inline(always)]
    pub fn gpwre(&self) -> GPWRE_R {
        GPWRE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generic command write error"]
    #[inline(always)]
    pub fn gcwre(&self) -> GCWRE_R {
        GCWRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LTDC payload write error"]
    #[inline(always)]
    pub fn lpwre(&self) -> LPWRE_R {
        LPWRE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EoTp error"]
    #[inline(always)]
    pub fn eotpe(&self) -> EOTPE_R {
        EOTPE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Packet size error"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRC error"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ECC multi"]
    #[inline(always)]
    pub fn eccme(&self) -> ECCME_R {
        ECCME_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ECC single"]
    #[inline(always)]
    pub fn eccse(&self) -> ECCSE_R {
        ECCSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timeout low"]
    #[inline(always)]
    pub fn tolprx(&self) -> TOLPRX_R {
        TOLPRX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Timeout high"]
    #[inline(always)]
    pub fn tohstx(&self) -> TOHSTX_R {
        TOHSTX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Generic payload receive error"]
    #[inline(always)]
    pub fn gprxe(&mut self) -> GPRXE_W {
        GPRXE_W { w: self }
    }
    #[doc = "Bit 11 - Generic payload read error"]
    #[inline(always)]
    pub fn gprde(&mut self) -> GPRDE_W {
        GPRDE_W { w: self }
    }
    #[doc = "Bit 10 - Generic payload transmit error"]
    #[inline(always)]
    pub fn gptxe(&mut self) -> GPTXE_W {
        GPTXE_W { w: self }
    }
    #[doc = "Bit 9 - Generic payload write error"]
    #[inline(always)]
    pub fn gpwre(&mut self) -> GPWRE_W {
        GPWRE_W { w: self }
    }
    #[doc = "Bit 8 - Generic command write error"]
    #[inline(always)]
    pub fn gcwre(&mut self) -> GCWRE_W {
        GCWRE_W { w: self }
    }
    #[doc = "Bit 7 - LTDC payload write error"]
    #[inline(always)]
    pub fn lpwre(&mut self) -> LPWRE_W {
        LPWRE_W { w: self }
    }
    #[doc = "Bit 6 - EoTp error"]
    #[inline(always)]
    pub fn eotpe(&mut self) -> EOTPE_W {
        EOTPE_W { w: self }
    }
    #[doc = "Bit 5 - Packet size error"]
    #[inline(always)]
    pub fn pse(&mut self) -> PSE_W {
        PSE_W { w: self }
    }
    #[doc = "Bit 4 - CRC error"]
    #[inline(always)]
    pub fn crce(&mut self) -> CRCE_W {
        CRCE_W { w: self }
    }
    #[doc = "Bit 3 - ECC multi"]
    #[inline(always)]
    pub fn eccme(&mut self) -> ECCME_W {
        ECCME_W { w: self }
    }
    #[doc = "Bit 2 - ECC single"]
    #[inline(always)]
    pub fn eccse(&mut self) -> ECCSE_W {
        ECCSE_W { w: self }
    }
    #[doc = "Bit 1 - Timeout low"]
    #[inline(always)]
    pub fn tolprx(&mut self) -> TOLPRX_W {
        TOLPRX_W { w: self }
    }
    #[doc = "Bit 0 - Timeout high"]
    #[inline(always)]
    pub fn tohstx(&mut self) -> TOHSTX_W {
        TOHSTX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host interrupt and status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr1](index.html) module"]
pub struct ISR1_SPEC;
impl crate::RegisterSpec for ISR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr1::R](R) reader structure"]
impl crate::Readable for ISR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr1::W](W) writer structure"]
impl crate::Writable for ISR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ISR1 to value 0"]
impl crate::Resettable for ISR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
