#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HBITCLKDIV` reader - HBITCLKDIV"]
pub struct HBITCLKDIV_R(crate::FieldReader<u8, u8>);
impl HBITCLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBITCLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBITCLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HBITCLKDIV` writer - HBITCLKDIV"]
pub struct HBITCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HBITCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `IFRGAP` reader - IFRGAP"]
pub struct IFRGAP_R(crate::FieldReader<u8, u8>);
impl IFRGAP_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFRGAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFRGAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFRGAP` writer - IFRGAP"]
pub struct IFRGAP_W<'a> {
    w: &'a mut W,
}
impl<'a> IFRGAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `TRANSWIN` reader - TRANSWIN"]
pub struct TRANSWIN_R(crate::FieldReader<u8, u8>);
impl TRANSWIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRANSWIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSWIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANSWIN` writer - TRANSWIN"]
pub struct TRANSWIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSWIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | ((value as u32 & 0x1f) << 11);
        self.w
    }
}
#[doc = "Field `PSC_USBPDCLK` reader - PSC_USBPDCLK"]
pub struct PSC_USBPDCLK_R(crate::FieldReader<u8, u8>);
impl PSC_USBPDCLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSC_USBPDCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSC_USBPDCLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSC_USBPDCLK` writer - PSC_USBPDCLK"]
pub struct PSC_USBPDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_USBPDCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `RXORDSETEN` reader - RXORDSETEN"]
pub struct RXORDSETEN_R(crate::FieldReader<u16, u16>);
impl RXORDSETEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXORDSETEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXORDSETEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXORDSETEN` writer - RXORDSETEN"]
pub struct RXORDSETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXORDSETEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 20)) | ((value as u32 & 0x01ff) << 20);
        self.w
    }
}
#[doc = "Field `TXDMAEN` reader - TXDMAEN"]
pub struct TXDMAEN_R(crate::FieldReader<bool, bool>);
impl TXDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMAEN` writer - TXDMAEN"]
pub struct TXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAEN_W<'a> {
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
#[doc = "Field `RXDMAEN` reader - RXDMAEN:"]
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
#[doc = "Field `RXDMAEN` writer - RXDMAEN:"]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `UCPDEN` reader - UCPDEN"]
pub struct UCPDEN_R(crate::FieldReader<bool, bool>);
impl UCPDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCPDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCPDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCPDEN` writer - UCPDEN"]
pub struct UCPDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPDEN_W<'a> {
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
    #[doc = "Bits 0:5 - HBITCLKDIV"]
    #[inline(always)]
    pub fn hbitclkdiv(&self) -> HBITCLKDIV_R {
        HBITCLKDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10 - IFRGAP"]
    #[inline(always)]
    pub fn ifrgap(&self) -> IFRGAP_R {
        IFRGAP_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - TRANSWIN"]
    #[inline(always)]
    pub fn transwin(&self) -> TRANSWIN_R {
        TRANSWIN_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19 - PSC_USBPDCLK"]
    #[inline(always)]
    pub fn psc_usbpdclk(&self) -> PSC_USBPDCLK_R {
        PSC_USBPDCLK_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:28 - RXORDSETEN"]
    #[inline(always)]
    pub fn rxordseten(&self) -> RXORDSETEN_R {
        RXORDSETEN_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - RXDMAEN:"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - UCPDEN"]
    #[inline(always)]
    pub fn ucpden(&self) -> UCPDEN_R {
        UCPDEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - HBITCLKDIV"]
    #[inline(always)]
    pub fn hbitclkdiv(&mut self) -> HBITCLKDIV_W {
        HBITCLKDIV_W { w: self }
    }
    #[doc = "Bits 6:10 - IFRGAP"]
    #[inline(always)]
    pub fn ifrgap(&mut self) -> IFRGAP_W {
        IFRGAP_W { w: self }
    }
    #[doc = "Bits 11:15 - TRANSWIN"]
    #[inline(always)]
    pub fn transwin(&mut self) -> TRANSWIN_W {
        TRANSWIN_W { w: self }
    }
    #[doc = "Bits 17:19 - PSC_USBPDCLK"]
    #[inline(always)]
    pub fn psc_usbpdclk(&mut self) -> PSC_USBPDCLK_W {
        PSC_USBPDCLK_W { w: self }
    }
    #[doc = "Bits 20:28 - RXORDSETEN"]
    #[inline(always)]
    pub fn rxordseten(&mut self) -> RXORDSETEN_W {
        RXORDSETEN_W { w: self }
    }
    #[doc = "Bit 29 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W {
        TXDMAEN_W { w: self }
    }
    #[doc = "Bit 30 - RXDMAEN:"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    #[doc = "Bit 31 - UCPDEN"]
    #[inline(always)]
    pub fn ucpden(&mut self) -> UCPDEN_W {
        UCPDEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UCPD configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
