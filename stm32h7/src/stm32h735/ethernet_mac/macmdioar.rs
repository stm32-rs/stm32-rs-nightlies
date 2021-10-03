#[doc = "Register `MACMDIOAR` reader"]
pub struct R(crate::R<MACMDIOAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACMDIOAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACMDIOAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACMDIOAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACMDIOAR` writer"]
pub struct W(crate::W<MACMDIOAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACMDIOAR_SPEC>;
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
impl From<crate::W<MACMDIOAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACMDIOAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MB` reader - MII Busy"]
pub struct MB_R(crate::FieldReader<bool, bool>);
impl MB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MB` writer - MII Busy"]
pub struct MB_W<'a> {
    w: &'a mut W,
}
impl<'a> MB_W<'a> {
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
#[doc = "Field `C45E` reader - Clause 45 PHY Enable"]
pub struct C45E_R(crate::FieldReader<bool, bool>);
impl C45E_R {
    pub(crate) fn new(bits: bool) -> Self {
        C45E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C45E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C45E` writer - Clause 45 PHY Enable"]
pub struct C45E_W<'a> {
    w: &'a mut W,
}
impl<'a> C45E_W<'a> {
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
#[doc = "Field `GOC` reader - MII Operation Command"]
pub struct GOC_R(crate::FieldReader<u8, u8>);
impl GOC_R {
    pub(crate) fn new(bits: u8) -> Self {
        GOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GOC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GOC` writer - MII Operation Command"]
pub struct GOC_W<'a> {
    w: &'a mut W,
}
impl<'a> GOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `SKAP` reader - Skip Address Packet"]
pub struct SKAP_R(crate::FieldReader<bool, bool>);
impl SKAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SKAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SKAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKAP` writer - Skip Address Packet"]
pub struct SKAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SKAP_W<'a> {
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
#[doc = "Field `CR` reader - CSR Clock Range"]
pub struct CR_R(crate::FieldReader<u8, u8>);
impl CR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CR` writer - CSR Clock Range"]
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `NTC` reader - Number of Training Clocks"]
pub struct NTC_R(crate::FieldReader<u8, u8>);
impl NTC_R {
    pub(crate) fn new(bits: u8) -> Self {
        NTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NTC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NTC` writer - Number of Training Clocks"]
pub struct NTC_W<'a> {
    w: &'a mut W,
}
impl<'a> NTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `RDA` reader - Register/Device Address"]
pub struct RDA_R(crate::FieldReader<u8, u8>);
impl RDA_R {
    pub(crate) fn new(bits: u8) -> Self {
        RDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDA` writer - Register/Device Address"]
pub struct RDA_W<'a> {
    w: &'a mut W,
}
impl<'a> RDA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `PA` reader - Physical Layer Address"]
pub struct PA_R(crate::FieldReader<u8, u8>);
impl PA_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA` writer - Physical Layer Address"]
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | ((value as u32 & 0x1f) << 21);
        self.w
    }
}
#[doc = "Field `BTB` reader - Back to Back transactions"]
pub struct BTB_R(crate::FieldReader<bool, bool>);
impl BTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        BTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BTB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BTB` writer - Back to Back transactions"]
pub struct BTB_W<'a> {
    w: &'a mut W,
}
impl<'a> BTB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PSE` reader - Preamble Suppression Enable"]
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
#[doc = "Field `PSE` writer - Preamble Suppression Enable"]
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clause 45 PHY Enable"]
    #[inline(always)]
    pub fn c45e(&self) -> C45E_R {
        C45E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - MII Operation Command"]
    #[inline(always)]
    pub fn goc(&self) -> GOC_R {
        GOC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Skip Address Packet"]
    #[inline(always)]
    pub fn skap(&self) -> SKAP_R {
        SKAP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - CSR Clock Range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Number of Training Clocks"]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - Register/Device Address"]
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Back to Back transactions"]
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Preamble Suppression Enable"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MII Busy"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W { w: self }
    }
    #[doc = "Bit 1 - Clause 45 PHY Enable"]
    #[inline(always)]
    pub fn c45e(&mut self) -> C45E_W {
        C45E_W { w: self }
    }
    #[doc = "Bits 2:3 - MII Operation Command"]
    #[inline(always)]
    pub fn goc(&mut self) -> GOC_W {
        GOC_W { w: self }
    }
    #[doc = "Bit 4 - Skip Address Packet"]
    #[inline(always)]
    pub fn skap(&mut self) -> SKAP_W {
        SKAP_W { w: self }
    }
    #[doc = "Bits 8:11 - CSR Clock Range"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    #[doc = "Bits 12:14 - Number of Training Clocks"]
    #[inline(always)]
    pub fn ntc(&mut self) -> NTC_W {
        NTC_W { w: self }
    }
    #[doc = "Bits 16:20 - Register/Device Address"]
    #[inline(always)]
    pub fn rda(&mut self) -> RDA_W {
        RDA_W { w: self }
    }
    #[doc = "Bits 21:25 - Physical Layer Address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
    #[doc = "Bit 26 - Back to Back transactions"]
    #[inline(always)]
    pub fn btb(&mut self) -> BTB_W {
        BTB_W { w: self }
    }
    #[doc = "Bit 27 - Preamble Suppression Enable"]
    #[inline(always)]
    pub fn pse(&mut self) -> PSE_W {
        PSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIO address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macmdioar](index.html) module"]
pub struct MACMDIOAR_SPEC;
impl crate::RegisterSpec for MACMDIOAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macmdioar::R](R) reader structure"]
impl crate::Readable for MACMDIOAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macmdioar::W](W) writer structure"]
impl crate::Writable for MACMDIOAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACMDIOAR to value 0"]
impl crate::Resettable for MACMDIOAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
