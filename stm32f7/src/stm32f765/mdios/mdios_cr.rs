#[doc = "Register `MDIOS_CR` reader"]
pub struct R(crate::R<MDIOS_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDIOS_CR` writer"]
pub struct W(crate::W<MDIOS_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIOS_CR_SPEC>;
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
impl From<crate::W<MDIOS_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIOS_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Peripheral enable"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Peripheral enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `WRIE` reader - Register write interrupt enable"]
pub struct WRIE_R(crate::FieldReader<bool, bool>);
impl WRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRIE` writer - Register write interrupt enable"]
pub struct WRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRIE_W<'a> {
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
#[doc = "Field `RDIE` reader - Register Read Interrupt Enable"]
pub struct RDIE_R(crate::FieldReader<bool, bool>);
impl RDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDIE` writer - Register Read Interrupt Enable"]
pub struct RDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIE_W<'a> {
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
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub struct EIE_R(crate::FieldReader<bool, bool>);
impl EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub struct EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIE_W<'a> {
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
#[doc = "Field `DPC` reader - Disable Preamble Check"]
pub struct DPC_R(crate::FieldReader<bool, bool>);
impl DPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPC` writer - Disable Preamble Check"]
pub struct DPC_W<'a> {
    w: &'a mut W,
}
impl<'a> DPC_W<'a> {
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
#[doc = "Field `PORT_ADDRESS` reader - Slaves's address"]
pub struct PORT_ADDRESS_R(crate::FieldReader<u8, u8>);
impl PORT_ADDRESS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PORT_ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_ADDRESS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_ADDRESS` writer - Slaves's address"]
pub struct PORT_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Register write interrupt enable"]
    #[inline(always)]
    pub fn wrie(&self) -> WRIE_R {
        WRIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Register Read Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Disable Preamble Check"]
    #[inline(always)]
    pub fn dpc(&self) -> DPC_R {
        DPC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Slaves's address"]
    #[inline(always)]
    pub fn port_address(&self) -> PORT_ADDRESS_R {
        PORT_ADDRESS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Register write interrupt enable"]
    #[inline(always)]
    pub fn wrie(&mut self) -> WRIE_W {
        WRIE_W { w: self }
    }
    #[doc = "Bit 2 - Register Read Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&mut self) -> RDIE_W {
        RDIE_W { w: self }
    }
    #[doc = "Bit 3 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W { w: self }
    }
    #[doc = "Bit 7 - Disable Preamble Check"]
    #[inline(always)]
    pub fn dpc(&mut self) -> DPC_W {
        DPC_W { w: self }
    }
    #[doc = "Bits 8:12 - Slaves's address"]
    #[inline(always)]
    pub fn port_address(&mut self) -> PORT_ADDRESS_W {
        PORT_ADDRESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_cr](index.html) module"]
pub struct MDIOS_CR_SPEC;
impl crate::RegisterSpec for MDIOS_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_cr::R](R) reader structure"]
impl crate::Readable for MDIOS_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdios_cr::W](W) writer structure"]
impl crate::Writable for MDIOS_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDIOS_CR to value 0"]
impl crate::Resettable for MDIOS_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
