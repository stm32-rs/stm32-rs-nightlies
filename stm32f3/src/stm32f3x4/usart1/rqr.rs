#[doc = "Register `RQR` reader"]
pub struct R(crate::R<RQR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RQR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RQR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RQR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RQR` writer"]
pub struct W(crate::W<RQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RQR_SPEC>;
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
impl From<crate::W<RQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RQR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRQ_A {
    #[doc = "1: Set the TXE flags. This allows to discard the transmit data"]
    DISCARD = 1,
}
impl From<TXFRQ_A> for bool {
    #[inline(always)]
    fn from(variant: TXFRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRQ` reader - Transmit data flush request"]
pub struct TXFRQ_R(crate::FieldReader<bool, TXFRQ_A>);
impl TXFRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXFRQ_A> {
        match self.bits {
            true => Some(TXFRQ_A::DISCARD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISCARD`"]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        **self == TXFRQ_A::DISCARD
    }
}
impl core::ops::Deref for TXFRQ_R {
    type Target = crate::FieldReader<bool, TXFRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFRQ` writer - Transmit data flush request"]
pub struct TXFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set the TXE flags. This allows to discard the transmit data"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(TXFRQ_A::DISCARD)
    }
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
#[doc = "Receive data flush request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRQ_A {
    #[doc = "1: clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    DISCARD = 1,
}
impl From<RXFRQ_A> for bool {
    #[inline(always)]
    fn from(variant: RXFRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFRQ` reader - Receive data flush request"]
pub struct RXFRQ_R(crate::FieldReader<bool, RXFRQ_A>);
impl RXFRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXFRQ_A> {
        match self.bits {
            true => Some(RXFRQ_A::DISCARD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISCARD`"]
    #[inline(always)]
    pub fn is_discard(&self) -> bool {
        **self == RXFRQ_A::DISCARD
    }
}
impl core::ops::Deref for RXFRQ_R {
    type Target = crate::FieldReader<bool, RXFRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFRQ` writer - Receive data flush request"]
pub struct RXFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    #[inline(always)]
    pub fn discard(self) -> &'a mut W {
        self.variant(RXFRQ_A::DISCARD)
    }
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
#[doc = "Mute mode request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMRQ_A {
    #[doc = "1: Puts the USART in mute mode and sets the RWU flag"]
    MUTE = 1,
}
impl From<MMRQ_A> for bool {
    #[inline(always)]
    fn from(variant: MMRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMRQ` reader - Mute mode request"]
pub struct MMRQ_R(crate::FieldReader<bool, MMRQ_A>);
impl MMRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MMRQ_A> {
        match self.bits {
            true => Some(MMRQ_A::MUTE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MUTE`"]
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        **self == MMRQ_A::MUTE
    }
}
impl core::ops::Deref for MMRQ_R {
    type Target = crate::FieldReader<bool, MMRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMRQ` writer - Mute mode request"]
pub struct MMRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MMRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Puts the USART in mute mode and sets the RWU flag"]
    #[inline(always)]
    pub fn mute(self) -> &'a mut W {
        self.variant(MMRQ_A::MUTE)
    }
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
#[doc = "Send break request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBKRQ_A {
    #[doc = "1: sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    BREAK = 1,
}
impl From<SBKRQ_A> for bool {
    #[inline(always)]
    fn from(variant: SBKRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBKRQ` reader - Send break request"]
pub struct SBKRQ_R(crate::FieldReader<bool, SBKRQ_A>);
impl SBKRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBKRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SBKRQ_A> {
        match self.bits {
            true => Some(SBKRQ_A::BREAK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BREAK`"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        **self == SBKRQ_A::BREAK
    }
}
impl core::ops::Deref for SBKRQ_R {
    type Target = crate::FieldReader<bool, SBKRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBKRQ` writer - Send break request"]
pub struct SBKRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SBKRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBKRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut W {
        self.variant(SBKRQ_A::BREAK)
    }
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
#[doc = "Auto baud rate request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABRRQ_A {
    #[doc = "1: resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    REQUEST = 1,
}
impl From<ABRRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ABRRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABRRQ` reader - Auto baud rate request"]
pub struct ABRRQ_R(crate::FieldReader<bool, ABRRQ_A>);
impl ABRRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABRRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ABRRQ_A> {
        match self.bits {
            true => Some(ABRRQ_A::REQUEST),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REQUEST`"]
    #[inline(always)]
    pub fn is_request(&self) -> bool {
        **self == ABRRQ_A::REQUEST
    }
}
impl core::ops::Deref for ABRRQ_R {
    type Target = crate::FieldReader<bool, ABRRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABRRQ` writer - Auto baud rate request"]
pub struct ABRRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> ABRRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABRRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    #[inline(always)]
    pub fn request(self) -> &'a mut W {
        self.variant(ABRRQ_A::REQUEST)
    }
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
impl R {
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfrq(&self) -> TXFRQ_R {
        TXFRQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    pub fn rxfrq(&self) -> RXFRQ_R {
        RXFRQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    pub fn mmrq(&self) -> MMRQ_R {
        MMRQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    pub fn sbkrq(&self) -> SBKRQ_R {
        SBKRQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    pub fn abrrq(&self) -> ABRRQ_R {
        ABRRQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn txfrq(&mut self) -> TXFRQ_W {
        TXFRQ_W { w: self }
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    pub fn rxfrq(&mut self) -> RXFRQ_W {
        RXFRQ_W { w: self }
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    pub fn mmrq(&mut self) -> MMRQ_W {
        MMRQ_W { w: self }
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    pub fn sbkrq(&mut self) -> SBKRQ_W {
        SBKRQ_W { w: self }
    }
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    pub fn abrrq(&mut self) -> ABRRQ_W {
        ABRRQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Request register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqr](index.html) module"]
pub struct RQR_SPEC;
impl crate::RegisterSpec for RQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rqr::R](R) reader structure"]
impl crate::Readable for RQR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rqr::W](W) writer structure"]
impl crate::Writable for RQR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RQR to value 0"]
impl crate::Resettable for RQR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
