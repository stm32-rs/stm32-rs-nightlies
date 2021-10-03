#[doc = "Register `ETH_MACMDIOAR` reader"]
pub struct R(crate::R<ETH_MACMDIOAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACMDIOAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACMDIOAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACMDIOAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACMDIOAR` writer"]
pub struct W(crate::W<ETH_MACMDIOAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACMDIOAR_SPEC>;
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
impl From<crate::W<ETH_MACMDIOAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACMDIOAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GB` reader - GB"]
pub struct GB_R(crate::FieldReader<bool, bool>);
impl GB_R {
    pub(crate) fn new(bits: bool) -> Self {
        GB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GB` writer - GB"]
pub struct GB_W<'a> {
    w: &'a mut W,
}
impl<'a> GB_W<'a> {
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
#[doc = "Field `C45E` reader - C45E"]
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
#[doc = "Field `C45E` writer - C45E"]
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
#[doc = "Field `GOC` reader - GOC"]
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
#[doc = "Field `GOC` writer - GOC"]
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
#[doc = "Field `SKAP` reader - SKAP"]
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
#[doc = "Field `SKAP` writer - SKAP"]
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
#[doc = "Field `CR` reader - CR"]
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
#[doc = "Field `CR` writer - CR"]
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
#[doc = "Field `NTC` reader - NTC"]
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
#[doc = "Field `NTC` writer - NTC"]
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
#[doc = "Field `RDA` reader - RDA"]
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
#[doc = "Field `RDA` writer - RDA"]
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
#[doc = "Field `PA` reader - PA"]
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
#[doc = "Field `PA` writer - PA"]
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
#[doc = "Field `BTB` reader - BTB"]
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
#[doc = "Field `BTB` writer - BTB"]
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
#[doc = "Field `PSE` reader - PSE"]
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
#[doc = "Field `PSE` writer - PSE"]
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
    #[doc = "Bit 0 - GB"]
    #[inline(always)]
    pub fn gb(&self) -> GB_R {
        GB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - C45E"]
    #[inline(always)]
    pub fn c45e(&self) -> C45E_R {
        C45E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - GOC"]
    #[inline(always)]
    pub fn goc(&self) -> GOC_R {
        GOC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - SKAP"]
    #[inline(always)]
    pub fn skap(&self) -> SKAP_R {
        SKAP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - CR"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - NTC"]
    #[inline(always)]
    pub fn ntc(&self) -> NTC_R {
        NTC_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - RDA"]
    #[inline(always)]
    pub fn rda(&self) -> RDA_R {
        RDA_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - PA"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - BTB"]
    #[inline(always)]
    pub fn btb(&self) -> BTB_R {
        BTB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PSE"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GB"]
    #[inline(always)]
    pub fn gb(&mut self) -> GB_W {
        GB_W { w: self }
    }
    #[doc = "Bit 1 - C45E"]
    #[inline(always)]
    pub fn c45e(&mut self) -> C45E_W {
        C45E_W { w: self }
    }
    #[doc = "Bits 2:3 - GOC"]
    #[inline(always)]
    pub fn goc(&mut self) -> GOC_W {
        GOC_W { w: self }
    }
    #[doc = "Bit 4 - SKAP"]
    #[inline(always)]
    pub fn skap(&mut self) -> SKAP_W {
        SKAP_W { w: self }
    }
    #[doc = "Bits 8:11 - CR"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
    #[doc = "Bits 12:14 - NTC"]
    #[inline(always)]
    pub fn ntc(&mut self) -> NTC_W {
        NTC_W { w: self }
    }
    #[doc = "Bits 16:20 - RDA"]
    #[inline(always)]
    pub fn rda(&mut self) -> RDA_W {
        RDA_W { w: self }
    }
    #[doc = "Bits 21:25 - PA"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
    #[doc = "Bit 26 - BTB"]
    #[inline(always)]
    pub fn btb(&mut self) -> BTB_W {
        BTB_W { w: self }
    }
    #[doc = "Bit 27 - PSE"]
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
#[doc = "The MDIO Address register controls the management cycles to external PHY through a management interface.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macmdioar](index.html) module"]
pub struct ETH_MACMDIOAR_SPEC;
impl crate::RegisterSpec for ETH_MACMDIOAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macmdioar::R](R) reader structure"]
impl crate::Readable for ETH_MACMDIOAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macmdioar::W](W) writer structure"]
impl crate::Writable for ETH_MACMDIOAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_MACMDIOAR to value 0"]
impl crate::Resettable for ETH_MACMDIOAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
