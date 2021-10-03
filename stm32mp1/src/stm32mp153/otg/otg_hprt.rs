#[doc = "Register `OTG_HPRT` reader"]
pub struct R(crate::R<OTG_HPRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HPRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HPRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HPRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HPRT` writer"]
pub struct W(crate::W<OTG_HPRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HPRT_SPEC>;
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
impl From<crate::W<OTG_HPRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HPRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCSTS` reader - PCSTS"]
pub struct PCSTS_R(crate::FieldReader<bool, bool>);
impl PCSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCDET` reader - PCDET"]
pub struct PCDET_R(crate::FieldReader<bool, bool>);
impl PCDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCDET` writer - PCDET"]
pub struct PCDET_W<'a> {
    w: &'a mut W,
}
impl<'a> PCDET_W<'a> {
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
#[doc = "Field `PENA` reader - PENA"]
pub struct PENA_R(crate::FieldReader<bool, bool>);
impl PENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENA` writer - PENA"]
pub struct PENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PENA_W<'a> {
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
#[doc = "Field `PENCHNG` reader - PENCHNG"]
pub struct PENCHNG_R(crate::FieldReader<bool, bool>);
impl PENCHNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PENCHNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENCHNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PENCHNG` writer - PENCHNG"]
pub struct PENCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PENCHNG_W<'a> {
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
#[doc = "Field `POCA` reader - POCA"]
pub struct POCA_R(crate::FieldReader<bool, bool>);
impl POCA_R {
    pub(crate) fn new(bits: bool) -> Self {
        POCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POCA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POCCHNG` reader - POCCHNG"]
pub struct POCCHNG_R(crate::FieldReader<bool, bool>);
impl POCCHNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        POCCHNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POCCHNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POCCHNG` writer - POCCHNG"]
pub struct POCCHNG_W<'a> {
    w: &'a mut W,
}
impl<'a> POCCHNG_W<'a> {
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
#[doc = "Field `PRES` reader - PRES"]
pub struct PRES_R(crate::FieldReader<bool, bool>);
impl PRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRES` writer - PRES"]
pub struct PRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRES_W<'a> {
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
#[doc = "Field `PSUSP` reader - PSUSP"]
pub struct PSUSP_R(crate::FieldReader<bool, bool>);
impl PSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSUSP` writer - PSUSP"]
pub struct PSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> PSUSP_W<'a> {
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
#[doc = "Field `PRST` reader - PRST"]
pub struct PRST_R(crate::FieldReader<bool, bool>);
impl PRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRST` writer - PRST"]
pub struct PRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST_W<'a> {
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
#[doc = "Field `PLSTS` reader - PLSTS"]
pub struct PLSTS_R(crate::FieldReader<u8, u8>);
impl PLSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPWR` reader - PPWR"]
pub struct PPWR_R(crate::FieldReader<bool, bool>);
impl PPWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPWR` writer - PPWR"]
pub struct PPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PPWR_W<'a> {
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
#[doc = "Field `PTCTL` reader - PTCTL"]
pub struct PTCTL_R(crate::FieldReader<u8, u8>);
impl PTCTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PTCTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTCTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTCTL` writer - PTCTL"]
pub struct PTCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | ((value as u32 & 0x0f) << 13);
        self.w
    }
}
#[doc = "Field `PSPD` reader - PSPD"]
pub struct PSPD_R(crate::FieldReader<u8, u8>);
impl PSPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - PCSTS"]
    #[inline(always)]
    pub fn pcsts(&self) -> PCSTS_R {
        PCSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PCDET"]
    #[inline(always)]
    pub fn pcdet(&self) -> PCDET_R {
        PCDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PENA"]
    #[inline(always)]
    pub fn pena(&self) -> PENA_R {
        PENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PENCHNG"]
    #[inline(always)]
    pub fn penchng(&self) -> PENCHNG_R {
        PENCHNG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - POCA"]
    #[inline(always)]
    pub fn poca(&self) -> POCA_R {
        POCA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - POCCHNG"]
    #[inline(always)]
    pub fn pocchng(&self) -> POCCHNG_R {
        POCCHNG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PRES"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PSUSP"]
    #[inline(always)]
    pub fn psusp(&self) -> PSUSP_R {
        PSUSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PRST"]
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - PLSTS"]
    #[inline(always)]
    pub fn plsts(&self) -> PLSTS_R {
        PLSTS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - PPWR"]
    #[inline(always)]
    pub fn ppwr(&self) -> PPWR_R {
        PPWR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:16 - PTCTL"]
    #[inline(always)]
    pub fn ptctl(&self) -> PTCTL_R {
        PTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - PSPD"]
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - PCDET"]
    #[inline(always)]
    pub fn pcdet(&mut self) -> PCDET_W {
        PCDET_W { w: self }
    }
    #[doc = "Bit 2 - PENA"]
    #[inline(always)]
    pub fn pena(&mut self) -> PENA_W {
        PENA_W { w: self }
    }
    #[doc = "Bit 3 - PENCHNG"]
    #[inline(always)]
    pub fn penchng(&mut self) -> PENCHNG_W {
        PENCHNG_W { w: self }
    }
    #[doc = "Bit 5 - POCCHNG"]
    #[inline(always)]
    pub fn pocchng(&mut self) -> POCCHNG_W {
        POCCHNG_W { w: self }
    }
    #[doc = "Bit 6 - PRES"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W {
        PRES_W { w: self }
    }
    #[doc = "Bit 7 - PSUSP"]
    #[inline(always)]
    pub fn psusp(&mut self) -> PSUSP_W {
        PSUSP_W { w: self }
    }
    #[doc = "Bit 8 - PRST"]
    #[inline(always)]
    pub fn prst(&mut self) -> PRST_W {
        PRST_W { w: self }
    }
    #[doc = "Bit 12 - PPWR"]
    #[inline(always)]
    pub fn ppwr(&mut self) -> PPWR_W {
        PPWR_W { w: self }
    }
    #[doc = "Bits 13:16 - PTCTL"]
    #[inline(always)]
    pub fn ptctl(&mut self) -> PTCTL_W {
        PTCTL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hprt](index.html) module"]
pub struct OTG_HPRT_SPEC;
impl crate::RegisterSpec for OTG_HPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hprt::R](R) reader structure"]
impl crate::Readable for OTG_HPRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hprt::W](W) writer structure"]
impl crate::Writable for OTG_HPRT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HPRT to value 0"]
impl crate::Resettable for OTG_HPRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
