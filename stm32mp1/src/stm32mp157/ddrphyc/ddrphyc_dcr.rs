#[doc = "Register `DDRPHYC_DCR` reader"]
pub struct R(crate::R<DDRPHYC_DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DCR` writer"]
pub struct W(crate::W<DDRPHYC_DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DCR_SPEC>;
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
impl From<crate::W<DDRPHYC_DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDRMD` reader - DDRMD"]
pub struct DDRMD_R(crate::FieldReader<u8, u8>);
impl DDRMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        DDRMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRMD` writer - DDRMD"]
pub struct DDRMD_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `DDR8BNK` reader - DDR8BNK"]
pub struct DDR8BNK_R(crate::FieldReader<bool, bool>);
impl DDR8BNK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDR8BNK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDR8BNK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDR8BNK` writer - DDR8BNK"]
pub struct DDR8BNK_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR8BNK_W<'a> {
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
#[doc = "Field `PDQ` reader - PDQ"]
pub struct PDQ_R(crate::FieldReader<u8, u8>);
impl PDQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDQ` writer - PDQ"]
pub struct PDQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PDQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `MPRDQ` reader - MPRDQ"]
pub struct MPRDQ_R(crate::FieldReader<bool, bool>);
impl MPRDQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPRDQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPRDQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPRDQ` writer - MPRDQ"]
pub struct MPRDQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MPRDQ_W<'a> {
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
#[doc = "Field `DDRTYPE` reader - DDRTYPE"]
pub struct DDRTYPE_R(crate::FieldReader<u8, u8>);
impl DDRTYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DDRTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRTYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDRTYPE` writer - DDRTYPE"]
pub struct DDRTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `NOSRA` reader - NOSRA"]
pub struct NOSRA_R(crate::FieldReader<bool, bool>);
impl NOSRA_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOSRA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOSRA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOSRA` writer - NOSRA"]
pub struct NOSRA_W<'a> {
    w: &'a mut W,
}
impl<'a> NOSRA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `DDR2T` reader - DDR2T"]
pub struct DDR2T_R(crate::FieldReader<bool, bool>);
impl DDR2T_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDR2T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDR2T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDR2T` writer - DDR2T"]
pub struct DDR2T_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR2T_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `UDIMM` reader - UDIMM"]
pub struct UDIMM_R(crate::FieldReader<bool, bool>);
impl UDIMM_R {
    pub(crate) fn new(bits: bool) -> Self {
        UDIMM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDIMM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDIMM` writer - UDIMM"]
pub struct UDIMM_W<'a> {
    w: &'a mut W,
}
impl<'a> UDIMM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `RDIMM` reader - RDIMM"]
pub struct RDIMM_R(crate::FieldReader<bool, bool>);
impl RDIMM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDIMM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDIMM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDIMM` writer - RDIMM"]
pub struct RDIMM_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIMM_W<'a> {
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
#[doc = "Field `TPD` reader - TPD"]
pub struct TPD_R(crate::FieldReader<bool, bool>);
impl TPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        TPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TPD` writer - TPD"]
pub struct TPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TPD_W<'a> {
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
    #[doc = "Bits 0:2 - DDRMD"]
    #[inline(always)]
    pub fn ddrmd(&self) -> DDRMD_R {
        DDRMD_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - DDR8BNK"]
    #[inline(always)]
    pub fn ddr8bnk(&self) -> DDR8BNK_R {
        DDR8BNK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - PDQ"]
    #[inline(always)]
    pub fn pdq(&self) -> PDQ_R {
        PDQ_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - MPRDQ"]
    #[inline(always)]
    pub fn mprdq(&self) -> MPRDQ_R {
        MPRDQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - DDRTYPE"]
    #[inline(always)]
    pub fn ddrtype(&self) -> DDRTYPE_R {
        DDRTYPE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 27 - NOSRA"]
    #[inline(always)]
    pub fn nosra(&self) -> NOSRA_R {
        NOSRA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DDR2T"]
    #[inline(always)]
    pub fn ddr2t(&self) -> DDR2T_R {
        DDR2T_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - UDIMM"]
    #[inline(always)]
    pub fn udimm(&self) -> UDIMM_R {
        UDIMM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - RDIMM"]
    #[inline(always)]
    pub fn rdimm(&self) -> RDIMM_R {
        RDIMM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TPD"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - DDRMD"]
    #[inline(always)]
    pub fn ddrmd(&mut self) -> DDRMD_W {
        DDRMD_W { w: self }
    }
    #[doc = "Bit 3 - DDR8BNK"]
    #[inline(always)]
    pub fn ddr8bnk(&mut self) -> DDR8BNK_W {
        DDR8BNK_W { w: self }
    }
    #[doc = "Bits 4:6 - PDQ"]
    #[inline(always)]
    pub fn pdq(&mut self) -> PDQ_W {
        PDQ_W { w: self }
    }
    #[doc = "Bit 7 - MPRDQ"]
    #[inline(always)]
    pub fn mprdq(&mut self) -> MPRDQ_W {
        MPRDQ_W { w: self }
    }
    #[doc = "Bits 8:9 - DDRTYPE"]
    #[inline(always)]
    pub fn ddrtype(&mut self) -> DDRTYPE_W {
        DDRTYPE_W { w: self }
    }
    #[doc = "Bit 27 - NOSRA"]
    #[inline(always)]
    pub fn nosra(&mut self) -> NOSRA_W {
        NOSRA_W { w: self }
    }
    #[doc = "Bit 28 - DDR2T"]
    #[inline(always)]
    pub fn ddr2t(&mut self) -> DDR2T_W {
        DDR2T_W { w: self }
    }
    #[doc = "Bit 29 - UDIMM"]
    #[inline(always)]
    pub fn udimm(&mut self) -> UDIMM_W {
        UDIMM_W { w: self }
    }
    #[doc = "Bit 30 - RDIMM"]
    #[inline(always)]
    pub fn rdimm(&mut self) -> RDIMM_W {
        RDIMM_W { w: self }
    }
    #[doc = "Bit 31 - TPD"]
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W {
        TPD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dcr](index.html) module"]
pub struct DDRPHYC_DCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dcr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dcr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DCR to value 0x0b"]
impl crate::Resettable for DDRPHYC_DCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
