#[doc = "Register `SPDIFRX_CR` reader"]
pub struct R(crate::R<SPDIFRX_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDIFRX_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDIFRX_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDIFRX_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPDIFRX_CR` writer"]
pub struct W(crate::W<SPDIFRX_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPDIFRX_CR_SPEC>;
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
impl From<crate::W<SPDIFRX_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPDIFRX_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPDIFRXEN` reader - SPDIFRXEN"]
pub struct SPDIFRXEN_R(crate::FieldReader<u8, u8>);
impl SPDIFRXEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPDIFRXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPDIFRXEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDIFRXEN` writer - SPDIFRXEN"]
pub struct SPDIFRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFRXEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `RXDMAEN` reader - RXDMAEN"]
pub struct RXDMAEN_R(crate::FieldReader<bool, bool>);
impl RXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMAEN` writer - RXDMAEN"]
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
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
#[doc = "Field `RXSTEO` reader - RXSTEO"]
pub struct RXSTEO_R(crate::FieldReader<bool, bool>);
impl RXSTEO_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTEO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTEO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTEO` writer - RXSTEO"]
pub struct RXSTEO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTEO_W<'a> {
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
#[doc = "Field `DRFMT` reader - DRFMT"]
pub struct DRFMT_R(crate::FieldReader<u8, u8>);
impl DRFMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRFMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRFMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRFMT` writer - DRFMT"]
pub struct DRFMT_W<'a> {
    w: &'a mut W,
}
impl<'a> DRFMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `PMSK` reader - PMSK"]
pub struct PMSK_R(crate::FieldReader<bool, bool>);
impl PMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMSK` writer - PMSK"]
pub struct PMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PMSK_W<'a> {
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
#[doc = "Field `VMSK` reader - VMSK"]
pub struct VMSK_R(crate::FieldReader<bool, bool>);
impl VMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        VMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VMSK` writer - VMSK"]
pub struct VMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> VMSK_W<'a> {
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
#[doc = "Field `CUMSK` reader - CUMSK"]
pub struct CUMSK_R(crate::FieldReader<bool, bool>);
impl CUMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CUMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CUMSK` writer - CUMSK"]
pub struct CUMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CUMSK_W<'a> {
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
#[doc = "Field `PTMSK` reader - PTMSK"]
pub struct PTMSK_R(crate::FieldReader<bool, bool>);
impl PTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTMSK` writer - PTMSK"]
pub struct PTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PTMSK_W<'a> {
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
#[doc = "Field `CBDMAEN` reader - CBDMAEN"]
pub struct CBDMAEN_R(crate::FieldReader<bool, bool>);
impl CBDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBDMAEN` writer - CBDMAEN"]
pub struct CBDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBDMAEN_W<'a> {
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
#[doc = "Field `CHSEL` reader - CHSEL"]
pub struct CHSEL_R(crate::FieldReader<bool, bool>);
impl CHSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL` writer - CHSEL"]
pub struct CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL_W<'a> {
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
#[doc = "Field `NBTR` reader - NBTR"]
pub struct NBTR_R(crate::FieldReader<u8, u8>);
impl NBTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBTR` writer - NBTR"]
pub struct NBTR_W<'a> {
    w: &'a mut W,
}
impl<'a> NBTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `WFA` reader - WFA"]
pub struct WFA_R(crate::FieldReader<bool, bool>);
impl WFA_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFA` writer - WFA"]
pub struct WFA_W<'a> {
    w: &'a mut W,
}
impl<'a> WFA_W<'a> {
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
#[doc = "Field `INSEL` reader - INSEL"]
pub struct INSEL_R(crate::FieldReader<u8, u8>);
impl INSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        INSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSEL` writer - INSEL"]
pub struct INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `CKSEN` reader - CKSEN"]
pub struct CKSEN_R(crate::FieldReader<bool, bool>);
impl CKSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKSEN` writer - CKSEN"]
pub struct CKSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CKSBKPEN` reader - CKSBKPEN"]
pub struct CKSBKPEN_R(crate::FieldReader<bool, bool>);
impl CKSBKPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKSBKPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKSBKPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKSBKPEN` writer - CKSBKPEN"]
pub struct CKSBKPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSBKPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SPDIFRXEN"]
    #[inline(always)]
    pub fn spdifrxen(&self) -> SPDIFRXEN_R {
        SPDIFRXEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RXSTEO"]
    #[inline(always)]
    pub fn rxsteo(&self) -> RXSTEO_R {
        RXSTEO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - DRFMT"]
    #[inline(always)]
    pub fn drfmt(&self) -> DRFMT_R {
        DRFMT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - PMSK"]
    #[inline(always)]
    pub fn pmsk(&self) -> PMSK_R {
        PMSK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VMSK"]
    #[inline(always)]
    pub fn vmsk(&self) -> VMSK_R {
        VMSK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CUMSK"]
    #[inline(always)]
    pub fn cumsk(&self) -> CUMSK_R {
        CUMSK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PTMSK"]
    #[inline(always)]
    pub fn ptmsk(&self) -> PTMSK_R {
        PTMSK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CBDMAEN"]
    #[inline(always)]
    pub fn cbdmaen(&self) -> CBDMAEN_R {
        CBDMAEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CHSEL"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - NBTR"]
    #[inline(always)]
    pub fn nbtr(&self) -> NBTR_R {
        NBTR_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - WFA"]
    #[inline(always)]
    pub fn wfa(&self) -> WFA_R {
        WFA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - INSEL"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - CKSEN"]
    #[inline(always)]
    pub fn cksen(&self) -> CKSEN_R {
        CKSEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CKSBKPEN"]
    #[inline(always)]
    pub fn cksbkpen(&self) -> CKSBKPEN_R {
        CKSBKPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPDIFRXEN"]
    #[inline(always)]
    pub fn spdifrxen(&mut self) -> SPDIFRXEN_W {
        SPDIFRXEN_W { w: self }
    }
    #[doc = "Bit 2 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    #[doc = "Bit 3 - RXSTEO"]
    #[inline(always)]
    pub fn rxsteo(&mut self) -> RXSTEO_W {
        RXSTEO_W { w: self }
    }
    #[doc = "Bits 4:5 - DRFMT"]
    #[inline(always)]
    pub fn drfmt(&mut self) -> DRFMT_W {
        DRFMT_W { w: self }
    }
    #[doc = "Bit 6 - PMSK"]
    #[inline(always)]
    pub fn pmsk(&mut self) -> PMSK_W {
        PMSK_W { w: self }
    }
    #[doc = "Bit 7 - VMSK"]
    #[inline(always)]
    pub fn vmsk(&mut self) -> VMSK_W {
        VMSK_W { w: self }
    }
    #[doc = "Bit 8 - CUMSK"]
    #[inline(always)]
    pub fn cumsk(&mut self) -> CUMSK_W {
        CUMSK_W { w: self }
    }
    #[doc = "Bit 9 - PTMSK"]
    #[inline(always)]
    pub fn ptmsk(&mut self) -> PTMSK_W {
        PTMSK_W { w: self }
    }
    #[doc = "Bit 10 - CBDMAEN"]
    #[inline(always)]
    pub fn cbdmaen(&mut self) -> CBDMAEN_W {
        CBDMAEN_W { w: self }
    }
    #[doc = "Bit 11 - CHSEL"]
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W {
        CHSEL_W { w: self }
    }
    #[doc = "Bits 12:13 - NBTR"]
    #[inline(always)]
    pub fn nbtr(&mut self) -> NBTR_W {
        NBTR_W { w: self }
    }
    #[doc = "Bit 14 - WFA"]
    #[inline(always)]
    pub fn wfa(&mut self) -> WFA_W {
        WFA_W { w: self }
    }
    #[doc = "Bits 16:18 - INSEL"]
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W {
        INSEL_W { w: self }
    }
    #[doc = "Bit 20 - CKSEN"]
    #[inline(always)]
    pub fn cksen(&mut self) -> CKSEN_W {
        CKSEN_W { w: self }
    }
    #[doc = "Bit 21 - CKSBKPEN"]
    #[inline(always)]
    pub fn cksbkpen(&mut self) -> CKSBKPEN_W {
        CKSBKPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_cr](index.html) module"]
pub struct SPDIFRX_CR_SPEC;
impl crate::RegisterSpec for SPDIFRX_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdifrx_cr::R](R) reader structure"]
impl crate::Readable for SPDIFRX_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spdifrx_cr::W](W) writer structure"]
impl crate::Writable for SPDIFRX_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPDIFRX_CR to value 0"]
impl crate::Resettable for SPDIFRX_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
