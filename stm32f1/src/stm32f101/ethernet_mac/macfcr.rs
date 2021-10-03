#[doc = "Register `MACFCR` reader"]
pub struct R(crate::R<MACFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACFCR` writer"]
pub struct W(crate::W<MACFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACFCR_SPEC>;
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
impl From<crate::W<MACFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCB_BPA` reader - Flow control busy/back pressure activate"]
pub struct FCB_BPA_R(crate::FieldReader<bool, bool>);
impl FCB_BPA_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCB_BPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCB_BPA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCB_BPA` writer - Flow control busy/back pressure activate"]
pub struct FCB_BPA_W<'a> {
    w: &'a mut W,
}
impl<'a> FCB_BPA_W<'a> {
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
#[doc = "Field `TFCE` reader - Transmit flow control enable"]
pub struct TFCE_R(crate::FieldReader<bool, bool>);
impl TFCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFCE` writer - Transmit flow control enable"]
pub struct TFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFCE_W<'a> {
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
#[doc = "Field `RFCE` reader - Receive flow control enable"]
pub struct RFCE_R(crate::FieldReader<bool, bool>);
impl RFCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFCE` writer - Receive flow control enable"]
pub struct RFCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCE_W<'a> {
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
#[doc = "Field `UPFD` reader - Unicast pause frame detect"]
pub struct UPFD_R(crate::FieldReader<bool, bool>);
impl UPFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPFD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPFD` writer - Unicast pause frame detect"]
pub struct UPFD_W<'a> {
    w: &'a mut W,
}
impl<'a> UPFD_W<'a> {
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
#[doc = "Field `PLT` reader - Pause low threshold"]
pub struct PLT_R(crate::FieldReader<u8, u8>);
impl PLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLT` writer - Pause low threshold"]
pub struct PLT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `ZQPD` reader - Zero-quanta pause disable"]
pub struct ZQPD_R(crate::FieldReader<bool, bool>);
impl ZQPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ZQPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZQPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZQPD` writer - Zero-quanta pause disable"]
pub struct ZQPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ZQPD_W<'a> {
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
#[doc = "Field `PT` reader - Pass control frames"]
pub struct PT_R(crate::FieldReader<u16, u16>);
impl PT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PT` writer - Pass control frames"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    pub fn tfce(&self) -> TFCE_R {
        TFCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    pub fn upfd(&self) -> UPFD_R {
        UPFD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Zero-quanta pause disable"]
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Pass control frames"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W {
        FCB_BPA_W { w: self }
    }
    #[doc = "Bit 1 - Transmit flow control enable"]
    #[inline(always)]
    pub fn tfce(&mut self) -> TFCE_W {
        TFCE_W { w: self }
    }
    #[doc = "Bit 2 - Receive flow control enable"]
    #[inline(always)]
    pub fn rfce(&mut self) -> RFCE_W {
        RFCE_W { w: self }
    }
    #[doc = "Bit 3 - Unicast pause frame detect"]
    #[inline(always)]
    pub fn upfd(&mut self) -> UPFD_W {
        UPFD_W { w: self }
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W {
        PLT_W { w: self }
    }
    #[doc = "Bit 7 - Zero-quanta pause disable"]
    #[inline(always)]
    pub fn zqpd(&mut self) -> ZQPD_W {
        ZQPD_W { w: self }
    }
    #[doc = "Bits 16:31 - Pass control frames"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC flow control register (ETH_MACFCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macfcr](index.html) module"]
pub struct MACFCR_SPEC;
impl crate::RegisterSpec for MACFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macfcr::R](R) reader structure"]
impl crate::Readable for MACFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macfcr::W](W) writer structure"]
impl crate::Writable for MACFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACFCR to value 0"]
impl crate::Resettable for MACFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
