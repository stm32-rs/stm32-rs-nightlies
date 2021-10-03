#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BYPASS` reader - Power management unit bypass"]
pub struct BYPASS_R(crate::FieldReader<bool, bool>);
impl BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYPASS` writer - Power management unit bypass"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Field `LDOEN` reader - Low drop-out regulator enable"]
pub struct LDOEN_R(crate::FieldReader<bool, bool>);
impl LDOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDOEN` writer - Low drop-out regulator enable"]
pub struct LDOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOEN_W<'a> {
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
#[doc = "Field `SMPSEN` reader - SMPSEN"]
pub struct SMPSEN_R(crate::FieldReader<bool, bool>);
impl SMPSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSEN` writer - SMPSEN"]
pub struct SMPSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEN_W<'a> {
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
#[doc = "Field `SMPSEXTHP` reader - SMPSEXTHP"]
pub struct SMPSEXTHP_R(crate::FieldReader<bool, bool>);
impl SMPSEXTHP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPSEXTHP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPSEXTHP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSEXTHP` writer - SMPSEXTHP"]
pub struct SMPSEXTHP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEXTHP_W<'a> {
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
#[doc = "Field `SMPSLEVEL` reader - SMPSLEVEL"]
pub struct SMPSLEVEL_R(crate::FieldReader<u8, u8>);
impl SMPSLEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMPSLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPSLEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSLEVEL` writer - SMPSLEVEL"]
pub struct SMPSLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSLEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `VBE` reader - VBAT charging enable"]
pub struct VBE_R(crate::FieldReader<bool, bool>);
impl VBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBE` writer - VBAT charging enable"]
pub struct VBE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBE_W<'a> {
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
#[doc = "Field `VBRS` reader - VBAT charging resistor selection"]
pub struct VBRS_R(crate::FieldReader<bool, bool>);
impl VBRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBRS` writer - VBAT charging resistor selection"]
pub struct VBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBRS_W<'a> {
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
#[doc = "Field `SMPSEXTRDY` reader - SMPSEXTRDY"]
pub struct SMPSEXTRDY_R(crate::FieldReader<bool, bool>);
impl SMPSEXTRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMPSEXTRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPSEXTRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB33DEN` reader - VDD33USB voltage level detector enable."]
pub struct USB33DEN_R(crate::FieldReader<bool, bool>);
impl USB33DEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB33DEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB33DEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB33DEN` writer - VDD33USB voltage level detector enable."]
pub struct USB33DEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB33DEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `USBREGEN` reader - USB regulator enable."]
pub struct USBREGEN_R(crate::FieldReader<bool, bool>);
impl USBREGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBREGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBREGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBREGEN` writer - USB regulator enable."]
pub struct USBREGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBREGEN_W<'a> {
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
#[doc = "Field `USB33RDY` reader - USB supply ready."]
pub struct USB33RDY_R(crate::FieldReader<bool, bool>);
impl USB33RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB33RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB33RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Power management unit bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Low drop-out regulator enable"]
    #[inline(always)]
    pub fn ldoen(&self) -> LDOEN_R {
        LDOEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SMPSEN"]
    #[inline(always)]
    pub fn smpsen(&self) -> SMPSEN_R {
        SMPSEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMPSEXTHP"]
    #[inline(always)]
    pub fn smpsexthp(&self) -> SMPSEXTHP_R {
        SMPSEXTHP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - SMPSLEVEL"]
    #[inline(always)]
    pub fn smpslevel(&self) -> SMPSLEVEL_R {
        SMPSLEVEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - VBAT charging enable"]
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VBAT charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SMPSEXTRDY"]
    #[inline(always)]
    pub fn smpsextrdy(&self) -> SMPSEXTRDY_R {
        SMPSEXTRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - VDD33USB voltage level detector enable."]
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USB regulator enable."]
    #[inline(always)]
    pub fn usbregen(&self) -> USBREGEN_R {
        USBREGEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USB supply ready."]
    #[inline(always)]
    pub fn usb33rdy(&self) -> USB33RDY_R {
        USB33RDY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power management unit bypass"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 1 - Low drop-out regulator enable"]
    #[inline(always)]
    pub fn ldoen(&mut self) -> LDOEN_W {
        LDOEN_W { w: self }
    }
    #[doc = "Bit 2 - SMPSEN"]
    #[inline(always)]
    pub fn smpsen(&mut self) -> SMPSEN_W {
        SMPSEN_W { w: self }
    }
    #[doc = "Bit 3 - SMPSEXTHP"]
    #[inline(always)]
    pub fn smpsexthp(&mut self) -> SMPSEXTHP_W {
        SMPSEXTHP_W { w: self }
    }
    #[doc = "Bits 4:5 - SMPSLEVEL"]
    #[inline(always)]
    pub fn smpslevel(&mut self) -> SMPSLEVEL_W {
        SMPSLEVEL_W { w: self }
    }
    #[doc = "Bit 8 - VBAT charging enable"]
    #[inline(always)]
    pub fn vbe(&mut self) -> VBE_W {
        VBE_W { w: self }
    }
    #[doc = "Bit 9 - VBAT charging resistor selection"]
    #[inline(always)]
    pub fn vbrs(&mut self) -> VBRS_W {
        VBRS_W { w: self }
    }
    #[doc = "Bit 24 - VDD33USB voltage level detector enable."]
    #[inline(always)]
    pub fn usb33den(&mut self) -> USB33DEN_W {
        USB33DEN_W { w: self }
    }
    #[doc = "Bit 25 - USB regulator enable."]
    #[inline(always)]
    pub fn usbregen(&mut self) -> USBREGEN_W {
        USBREGEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR3 to value 0x06"]
impl crate::Resettable for CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
