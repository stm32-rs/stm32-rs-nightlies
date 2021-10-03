#[doc = "Register `DDRPHYC_ACIOCR` reader"]
pub struct R(crate::R<DDRPHYC_ACIOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ACIOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ACIOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ACIOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_ACIOCR` writer"]
pub struct W(crate::W<DDRPHYC_ACIOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_ACIOCR_SPEC>;
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
impl From<crate::W<DDRPHYC_ACIOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_ACIOCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACIOM` reader - ACIOM"]
pub struct ACIOM_R(crate::FieldReader<bool, bool>);
impl ACIOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACIOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACIOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACIOM` writer - ACIOM"]
pub struct ACIOM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACIOM_W<'a> {
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
#[doc = "Field `ACOE` reader - ACOE"]
pub struct ACOE_R(crate::FieldReader<bool, bool>);
impl ACOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACOE` writer - ACOE"]
pub struct ACOE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOE_W<'a> {
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
#[doc = "Field `ACODT` reader - ACODT"]
pub struct ACODT_R(crate::FieldReader<bool, bool>);
impl ACODT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACODT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACODT` writer - ACODT"]
pub struct ACODT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACODT_W<'a> {
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
#[doc = "Field `ACPDD` reader - ACPDD"]
pub struct ACPDD_R(crate::FieldReader<bool, bool>);
impl ACPDD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACPDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACPDD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACPDD` writer - ACPDD"]
pub struct ACPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> ACPDD_W<'a> {
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
#[doc = "Field `ACPDR` reader - ACPDR"]
pub struct ACPDR_R(crate::FieldReader<bool, bool>);
impl ACPDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACPDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACPDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACPDR` writer - ACPDR"]
pub struct ACPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACPDR_W<'a> {
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
#[doc = "Field `CKODT` reader - CKODT"]
pub struct CKODT_R(crate::FieldReader<u8, u8>);
impl CKODT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKODT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKODT` writer - CKODT"]
pub struct CKODT_W<'a> {
    w: &'a mut W,
}
impl<'a> CKODT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `CKPDD` reader - CKPDD"]
pub struct CKPDD_R(crate::FieldReader<u8, u8>);
impl CKPDD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKPDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKPDD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKPDD` writer - CKPDD"]
pub struct CKPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPDD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `CKPDR` reader - CKPDR"]
pub struct CKPDR_R(crate::FieldReader<u8, u8>);
impl CKPDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKPDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKPDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKPDR` writer - CKPDR"]
pub struct CKPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Field `RANKODT` reader - RANKODT"]
pub struct RANKODT_R(crate::FieldReader<bool, bool>);
impl RANKODT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RANKODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANKODT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANKODT` writer - RANKODT"]
pub struct RANKODT_W<'a> {
    w: &'a mut W,
}
impl<'a> RANKODT_W<'a> {
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
#[doc = "Field `CSPDD` reader - CSPDD"]
pub struct CSPDD_R(crate::FieldReader<bool, bool>);
impl CSPDD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSPDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSPDD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSPDD` writer - CSPDD"]
pub struct CSPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> CSPDD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `RANKPDR` reader - RANKPDR"]
pub struct RANKPDR_R(crate::FieldReader<bool, bool>);
impl RANKPDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RANKPDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANKPDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANKPDR` writer - RANKPDR"]
pub struct RANKPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RANKPDR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `RSTODT` reader - RSTODT"]
pub struct RSTODT_R(crate::FieldReader<bool, bool>);
impl RSTODT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTODT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTODT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTODT` writer - RSTODT"]
pub struct RSTODT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTODT_W<'a> {
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
#[doc = "Field `RSTPDD` reader - RSTPDD"]
pub struct RSTPDD_R(crate::FieldReader<bool, bool>);
impl RSTPDD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTPDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTPDD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTPDD` writer - RSTPDD"]
pub struct RSTPDD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTPDD_W<'a> {
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
#[doc = "Field `RSTPDR` reader - RSTPDR"]
pub struct RSTPDR_R(crate::FieldReader<bool, bool>);
impl RSTPDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTPDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTPDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTPDR` writer - RSTPDR"]
pub struct RSTPDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTPDR_W<'a> {
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
#[doc = "Field `RSTIOM` reader - RSTIOM"]
pub struct RSTIOM_R(crate::FieldReader<bool, bool>);
impl RSTIOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTIOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTIOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTIOM` writer - RSTIOM"]
pub struct RSTIOM_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIOM_W<'a> {
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
#[doc = "Field `ACSR` reader - ACSR"]
pub struct ACSR_R(crate::FieldReader<u8, u8>);
impl ACSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACSR` writer - ACSR"]
pub struct ACSR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ACIOM"]
    #[inline(always)]
    pub fn aciom(&self) -> ACIOM_R {
        ACIOM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ACOE"]
    #[inline(always)]
    pub fn acoe(&self) -> ACOE_R {
        ACOE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ACODT"]
    #[inline(always)]
    pub fn acodt(&self) -> ACODT_R {
        ACODT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ACPDD"]
    #[inline(always)]
    pub fn acpdd(&self) -> ACPDD_R {
        ACPDD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ACPDR"]
    #[inline(always)]
    pub fn acpdr(&self) -> ACPDR_R {
        ACPDR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - CKODT"]
    #[inline(always)]
    pub fn ckodt(&self) -> CKODT_R {
        CKODT_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - CKPDD"]
    #[inline(always)]
    pub fn ckpdd(&self) -> CKPDD_R {
        CKPDD_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - CKPDR"]
    #[inline(always)]
    pub fn ckpdr(&self) -> CKPDR_R {
        CKPDR_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 14 - RANKODT"]
    #[inline(always)]
    pub fn rankodt(&self) -> RANKODT_R {
        RANKODT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CSPDD"]
    #[inline(always)]
    pub fn cspdd(&self) -> CSPDD_R {
        CSPDD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 22 - RANKPDR"]
    #[inline(always)]
    pub fn rankpdr(&self) -> RANKPDR_R {
        RANKPDR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 26 - RSTODT"]
    #[inline(always)]
    pub fn rstodt(&self) -> RSTODT_R {
        RSTODT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - RSTPDD"]
    #[inline(always)]
    pub fn rstpdd(&self) -> RSTPDD_R {
        RSTPDD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - RSTPDR"]
    #[inline(always)]
    pub fn rstpdr(&self) -> RSTPDR_R {
        RSTPDR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RSTIOM"]
    #[inline(always)]
    pub fn rstiom(&self) -> RSTIOM_R {
        RSTIOM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - ACSR"]
    #[inline(always)]
    pub fn acsr(&self) -> ACSR_R {
        ACSR_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ACIOM"]
    #[inline(always)]
    pub fn aciom(&mut self) -> ACIOM_W {
        ACIOM_W { w: self }
    }
    #[doc = "Bit 1 - ACOE"]
    #[inline(always)]
    pub fn acoe(&mut self) -> ACOE_W {
        ACOE_W { w: self }
    }
    #[doc = "Bit 2 - ACODT"]
    #[inline(always)]
    pub fn acodt(&mut self) -> ACODT_W {
        ACODT_W { w: self }
    }
    #[doc = "Bit 3 - ACPDD"]
    #[inline(always)]
    pub fn acpdd(&mut self) -> ACPDD_W {
        ACPDD_W { w: self }
    }
    #[doc = "Bit 4 - ACPDR"]
    #[inline(always)]
    pub fn acpdr(&mut self) -> ACPDR_W {
        ACPDR_W { w: self }
    }
    #[doc = "Bits 5:7 - CKODT"]
    #[inline(always)]
    pub fn ckodt(&mut self) -> CKODT_W {
        CKODT_W { w: self }
    }
    #[doc = "Bits 8:10 - CKPDD"]
    #[inline(always)]
    pub fn ckpdd(&mut self) -> CKPDD_W {
        CKPDD_W { w: self }
    }
    #[doc = "Bits 11:13 - CKPDR"]
    #[inline(always)]
    pub fn ckpdr(&mut self) -> CKPDR_W {
        CKPDR_W { w: self }
    }
    #[doc = "Bit 14 - RANKODT"]
    #[inline(always)]
    pub fn rankodt(&mut self) -> RANKODT_W {
        RANKODT_W { w: self }
    }
    #[doc = "Bit 18 - CSPDD"]
    #[inline(always)]
    pub fn cspdd(&mut self) -> CSPDD_W {
        CSPDD_W { w: self }
    }
    #[doc = "Bit 22 - RANKPDR"]
    #[inline(always)]
    pub fn rankpdr(&mut self) -> RANKPDR_W {
        RANKPDR_W { w: self }
    }
    #[doc = "Bit 26 - RSTODT"]
    #[inline(always)]
    pub fn rstodt(&mut self) -> RSTODT_W {
        RSTODT_W { w: self }
    }
    #[doc = "Bit 27 - RSTPDD"]
    #[inline(always)]
    pub fn rstpdd(&mut self) -> RSTPDD_W {
        RSTPDD_W { w: self }
    }
    #[doc = "Bit 28 - RSTPDR"]
    #[inline(always)]
    pub fn rstpdr(&mut self) -> RSTPDR_W {
        RSTPDR_W { w: self }
    }
    #[doc = "Bit 29 - RSTIOM"]
    #[inline(always)]
    pub fn rstiom(&mut self) -> RSTIOM_W {
        RSTIOM_W { w: self }
    }
    #[doc = "Bits 30:31 - ACSR"]
    #[inline(always)]
    pub fn acsr(&mut self) -> ACSR_W {
        ACSR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC ACIOC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_aciocr](index.html) module"]
pub struct DDRPHYC_ACIOCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_ACIOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_aciocr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_ACIOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_aciocr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_ACIOCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_ACIOCR to value 0x33c0_3812"]
impl crate::Resettable for DDRPHYC_ACIOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x33c0_3812
    }
}
