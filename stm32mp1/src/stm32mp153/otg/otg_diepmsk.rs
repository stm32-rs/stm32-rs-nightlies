#[doc = "Register `OTG_DIEPMSK` reader"]
pub struct R(crate::R<OTG_DIEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DIEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DIEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DIEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DIEPMSK` writer"]
pub struct W(crate::W<OTG_DIEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DIEPMSK_SPEC>;
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
impl From<crate::W<OTG_DIEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DIEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - XFRCM"]
pub struct XFRCM_R(crate::FieldReader<bool, bool>);
impl XFRCM_R {
    pub(crate) fn new(bits: bool) -> Self {
        XFRCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFRCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFRCM` writer - XFRCM"]
pub struct XFRCM_W<'a> {
    w: &'a mut W,
}
impl<'a> XFRCM_W<'a> {
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
#[doc = "Field `EPDM` reader - EPDM"]
pub struct EPDM_R(crate::FieldReader<bool, bool>);
impl EPDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDM` writer - EPDM"]
pub struct EPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDM_W<'a> {
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
#[doc = "Field `AHBERRM` reader - AHBERRM"]
pub struct AHBERRM_R(crate::FieldReader<bool, bool>);
impl AHBERRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBERRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBERRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBERRM` writer - AHBERRM"]
pub struct AHBERRM_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBERRM_W<'a> {
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
#[doc = "Field `TOM` reader - TOM"]
pub struct TOM_R(crate::FieldReader<bool, bool>);
impl TOM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOM` writer - TOM"]
pub struct TOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TOM_W<'a> {
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
#[doc = "Field `ITTXFEMSK` reader - ITTXFEMSK"]
pub struct ITTXFEMSK_R(crate::FieldReader<bool, bool>);
impl ITTXFEMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITTXFEMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITTXFEMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITTXFEMSK` writer - ITTXFEMSK"]
pub struct ITTXFEMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ITTXFEMSK_W<'a> {
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
#[doc = "Field `INEPNMM` reader - INEPNMM"]
pub struct INEPNMM_R(crate::FieldReader<bool, bool>);
impl INEPNMM_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEPNMM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPNMM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPNMM` writer - INEPNMM"]
pub struct INEPNMM_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNMM_W<'a> {
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
#[doc = "Field `INEPNEM` reader - INEPNEM"]
pub struct INEPNEM_R(crate::FieldReader<bool, bool>);
impl INEPNEM_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEPNEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPNEM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPNEM` writer - INEPNEM"]
pub struct INEPNEM_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNEM_W<'a> {
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
#[doc = "Field `TXFURM` reader - TXFURM"]
pub struct TXFURM_R(crate::FieldReader<bool, bool>);
impl TXFURM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFURM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFURM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFURM` writer - TXFURM"]
pub struct TXFURM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFURM_W<'a> {
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
#[doc = "Field `BNAM` reader - BNAM"]
pub struct BNAM_R(crate::FieldReader<bool, bool>);
impl BNAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BNAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNAM` writer - BNAM"]
pub struct BNAM_W<'a> {
    w: &'a mut W,
}
impl<'a> BNAM_W<'a> {
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
#[doc = "Field `NAKM` reader - NAKM"]
pub struct NAKM_R(crate::FieldReader<bool, bool>);
impl NAKM_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKM` writer - NAKM"]
pub struct NAKM_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EPDM"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TOM"]
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ITTXFEMSK"]
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - INEPNMM"]
    #[inline(always)]
    pub fn inepnmm(&self) -> INEPNMM_R {
        INEPNMM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - INEPNEM"]
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TXFURM"]
    #[inline(always)]
    pub fn txfurm(&self) -> TXFURM_R {
        TXFURM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BNAM"]
    #[inline(always)]
    pub fn bnam(&self) -> BNAM_R {
        BNAM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NAKM"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W {
        XFRCM_W { w: self }
    }
    #[doc = "Bit 1 - EPDM"]
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W {
        EPDM_W { w: self }
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    pub fn ahberrm(&mut self) -> AHBERRM_W {
        AHBERRM_W { w: self }
    }
    #[doc = "Bit 3 - TOM"]
    #[inline(always)]
    pub fn tom(&mut self) -> TOM_W {
        TOM_W { w: self }
    }
    #[doc = "Bit 4 - ITTXFEMSK"]
    #[inline(always)]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W {
        ITTXFEMSK_W { w: self }
    }
    #[doc = "Bit 5 - INEPNMM"]
    #[inline(always)]
    pub fn inepnmm(&mut self) -> INEPNMM_W {
        INEPNMM_W { w: self }
    }
    #[doc = "Bit 6 - INEPNEM"]
    #[inline(always)]
    pub fn inepnem(&mut self) -> INEPNEM_W {
        INEPNEM_W { w: self }
    }
    #[doc = "Bit 8 - TXFURM"]
    #[inline(always)]
    pub fn txfurm(&mut self) -> TXFURM_W {
        TXFURM_W { w: self }
    }
    #[doc = "Bit 9 - BNAM"]
    #[inline(always)]
    pub fn bnam(&mut self) -> BNAM_W {
        BNAM_W { w: self }
    }
    #[doc = "Bit 13 - NAKM"]
    #[inline(always)]
    pub fn nakm(&mut self) -> NAKM_W {
        NAKM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register works with each of the OTG_DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_diepmsk](index.html) module"]
pub struct OTG_DIEPMSK_SPEC;
impl crate::RegisterSpec for OTG_DIEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_diepmsk::R](R) reader structure"]
impl crate::Readable for OTG_DIEPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_diepmsk::W](W) writer structure"]
impl crate::Writable for OTG_DIEPMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_DIEPMSK to value 0"]
impl crate::Resettable for OTG_DIEPMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
