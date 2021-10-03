#[doc = "Register `DDRPHYC_DX0GCR` reader"]
pub struct R(crate::R<DDRPHYC_DX0GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DX0GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DX0GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DX0GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DX0GCR` writer"]
pub struct W(crate::W<DDRPHYC_DX0GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DX0GCR_SPEC>;
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
impl From<crate::W<DDRPHYC_DX0GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DX0GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DXEN` reader - DXEN"]
pub struct DXEN_R(crate::FieldReader<bool, bool>);
impl DXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DXEN` writer - DXEN"]
pub struct DXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DXEN_W<'a> {
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
#[doc = "Field `DQSODT` reader - DQSODT"]
pub struct DQSODT_R(crate::FieldReader<bool, bool>);
impl DQSODT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQSODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSODT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSODT` writer - DQSODT"]
pub struct DQSODT_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSODT_W<'a> {
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
#[doc = "Field `DQODT` reader - DQODT"]
pub struct DQODT_R(crate::FieldReader<bool, bool>);
impl DQODT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQODT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQODT` writer - DQODT"]
pub struct DQODT_W<'a> {
    w: &'a mut W,
}
impl<'a> DQODT_W<'a> {
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
#[doc = "Field `DXIOM` reader - DXIOM"]
pub struct DXIOM_R(crate::FieldReader<bool, bool>);
impl DXIOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DXIOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DXIOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DXIOM` writer - DXIOM"]
pub struct DXIOM_W<'a> {
    w: &'a mut W,
}
impl<'a> DXIOM_W<'a> {
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
#[doc = "Field `DXPDD` reader - DXPDD"]
pub struct DXPDD_R(crate::FieldReader<bool, bool>);
impl DXPDD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DXPDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DXPDD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DXPDD` writer - DXPDD"]
pub struct DXPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> DXPDD_W<'a> {
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
#[doc = "Field `DXPDR` reader - DXPDR"]
pub struct DXPDR_R(crate::FieldReader<bool, bool>);
impl DXPDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DXPDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DXPDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DXPDR` writer - DXPDR"]
pub struct DXPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DXPDR_W<'a> {
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
#[doc = "Field `DQSRPD` reader - DQSRPD"]
pub struct DQSRPD_R(crate::FieldReader<bool, bool>);
impl DQSRPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQSRPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSRPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSRPD` writer - DQSRPD"]
pub struct DQSRPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSRPD_W<'a> {
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
#[doc = "Field `DSEN` reader - DSEN"]
pub struct DSEN_R(crate::FieldReader<u8, u8>);
impl DSEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSEN` writer - DSEN"]
pub struct DSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `DQSRTT` reader - DQSRTT"]
pub struct DQSRTT_R(crate::FieldReader<bool, bool>);
impl DQSRTT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQSRTT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSRTT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSRTT` writer - DQSRTT"]
pub struct DQSRTT_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSRTT_W<'a> {
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
#[doc = "Field `DQRTT` reader - DQRTT"]
pub struct DQRTT_R(crate::FieldReader<bool, bool>);
impl DQRTT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQRTT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQRTT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQRTT` writer - DQRTT"]
pub struct DQRTT_W<'a> {
    w: &'a mut W,
}
impl<'a> DQRTT_W<'a> {
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
#[doc = "Field `RTTOH` reader - RTTOH"]
pub struct RTTOH_R(crate::FieldReader<u8, u8>);
impl RTTOH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTTOH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTTOH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTTOH` writer - RTTOH"]
pub struct RTTOH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTOH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `RTTOAL` reader - RTTOAL"]
pub struct RTTOAL_R(crate::FieldReader<bool, bool>);
impl RTTOAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTTOAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTTOAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTTOAL` writer - RTTOAL"]
pub struct RTTOAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTOAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `R0RVSL` reader - R0RVSL"]
pub struct R0RVSL_R(crate::FieldReader<u8, u8>);
impl R0RVSL_R {
    pub(crate) fn new(bits: u8) -> Self {
        R0RVSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R0RVSL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R0RVSL` writer - R0RVSL"]
pub struct R0RVSL_W<'a> {
    w: &'a mut W,
}
impl<'a> R0RVSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | ((value as u32 & 0x07) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DXEN"]
    #[inline(always)]
    pub fn dxen(&self) -> DXEN_R {
        DXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DQSODT"]
    #[inline(always)]
    pub fn dqsodt(&self) -> DQSODT_R {
        DQSODT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DQODT"]
    #[inline(always)]
    pub fn dqodt(&self) -> DQODT_R {
        DQODT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DXIOM"]
    #[inline(always)]
    pub fn dxiom(&self) -> DXIOM_R {
        DXIOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DXPDD"]
    #[inline(always)]
    pub fn dxpdd(&self) -> DXPDD_R {
        DXPDD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DXPDR"]
    #[inline(always)]
    pub fn dxpdr(&self) -> DXPDR_R {
        DXPDR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DQSRPD"]
    #[inline(always)]
    pub fn dqsrpd(&self) -> DQSRPD_R {
        DQSRPD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - DSEN"]
    #[inline(always)]
    pub fn dsen(&self) -> DSEN_R {
        DSEN_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - DQSRTT"]
    #[inline(always)]
    pub fn dqsrtt(&self) -> DQSRTT_R {
        DQSRTT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DQRTT"]
    #[inline(always)]
    pub fn dqrtt(&self) -> DQRTT_R {
        DQRTT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - RTTOH"]
    #[inline(always)]
    pub fn rttoh(&self) -> RTTOH_R {
        RTTOH_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - RTTOAL"]
    #[inline(always)]
    pub fn rttoal(&self) -> RTTOAL_R {
        RTTOAL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:16 - R0RVSL"]
    #[inline(always)]
    pub fn r0rvsl(&self) -> R0RVSL_R {
        R0RVSL_R::new(((self.bits >> 14) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DXEN"]
    #[inline(always)]
    pub fn dxen(&mut self) -> DXEN_W {
        DXEN_W { w: self }
    }
    #[doc = "Bit 1 - DQSODT"]
    #[inline(always)]
    pub fn dqsodt(&mut self) -> DQSODT_W {
        DQSODT_W { w: self }
    }
    #[doc = "Bit 2 - DQODT"]
    #[inline(always)]
    pub fn dqodt(&mut self) -> DQODT_W {
        DQODT_W { w: self }
    }
    #[doc = "Bit 3 - DXIOM"]
    #[inline(always)]
    pub fn dxiom(&mut self) -> DXIOM_W {
        DXIOM_W { w: self }
    }
    #[doc = "Bit 4 - DXPDD"]
    #[inline(always)]
    pub fn dxpdd(&mut self) -> DXPDD_W {
        DXPDD_W { w: self }
    }
    #[doc = "Bit 5 - DXPDR"]
    #[inline(always)]
    pub fn dxpdr(&mut self) -> DXPDR_W {
        DXPDR_W { w: self }
    }
    #[doc = "Bit 6 - DQSRPD"]
    #[inline(always)]
    pub fn dqsrpd(&mut self) -> DQSRPD_W {
        DQSRPD_W { w: self }
    }
    #[doc = "Bits 7:8 - DSEN"]
    #[inline(always)]
    pub fn dsen(&mut self) -> DSEN_W {
        DSEN_W { w: self }
    }
    #[doc = "Bit 9 - DQSRTT"]
    #[inline(always)]
    pub fn dqsrtt(&mut self) -> DQSRTT_W {
        DQSRTT_W { w: self }
    }
    #[doc = "Bit 10 - DQRTT"]
    #[inline(always)]
    pub fn dqrtt(&mut self) -> DQRTT_W {
        DQRTT_W { w: self }
    }
    #[doc = "Bits 11:12 - RTTOH"]
    #[inline(always)]
    pub fn rttoh(&mut self) -> RTTOH_W {
        RTTOH_W { w: self }
    }
    #[doc = "Bit 13 - RTTOAL"]
    #[inline(always)]
    pub fn rttoal(&mut self) -> RTTOAL_W {
        RTTOAL_W { w: self }
    }
    #[doc = "Bits 14:16 - R0RVSL"]
    #[inline(always)]
    pub fn r0rvsl(&mut self) -> R0RVSL_W {
        R0RVSL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC byte lane 0 GC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0gcr](index.html) module"]
pub struct DDRPHYC_DX0GCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DX0GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dx0gcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DX0GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx0gcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DX0GCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DX0GCR to value 0xee81"]
impl crate::Resettable for DDRPHYC_DX0GCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xee81
    }
}
