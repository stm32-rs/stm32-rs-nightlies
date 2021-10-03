#[doc = "Register `ILS` reader"]
pub struct R(crate::R<ILS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ILS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ILS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ILS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ILS` writer"]
pub struct W(crate::W<ILS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ILS_SPEC>;
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
impl From<crate::W<ILS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ILS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO0` reader - RX FIFO bit grouping the following interruption"]
pub struct RXFIFO0_R(crate::FieldReader<bool, bool>);
impl RXFIFO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO0` writer - RX FIFO bit grouping the following interruption"]
pub struct RXFIFO0_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO0_W<'a> {
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
#[doc = "Field `RXFIFO1` reader - RX FIFO bit grouping the following interruption"]
pub struct RXFIFO1_R(crate::FieldReader<bool, bool>);
impl RXFIFO1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFO1` writer - RX FIFO bit grouping the following interruption"]
pub struct RXFIFO1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO1_W<'a> {
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
#[doc = "Field `SMSG` reader - Status message bit grouping the following interruption"]
pub struct SMSG_R(crate::FieldReader<bool, bool>);
impl SMSG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMSG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMSG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMSG` writer - Status message bit grouping the following interruption"]
pub struct SMSG_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSG_W<'a> {
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
#[doc = "Field `TFERR` reader - TX FIFO error grouping the following interruption"]
pub struct TFERR_R(crate::FieldReader<bool, bool>);
impl TFERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFERR` writer - TX FIFO error grouping the following interruption"]
pub struct TFERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TFERR_W<'a> {
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
#[doc = "Field `MISC` reader - Interrupt regrouping the following interruption"]
pub struct MISC_R(crate::FieldReader<bool, bool>);
impl MISC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MISC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISC` writer - Interrupt regrouping the following interruption"]
pub struct MISC_W<'a> {
    w: &'a mut W,
}
impl<'a> MISC_W<'a> {
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
#[doc = "Field `BERR` reader - Bit and line error grouping the following interruption"]
pub struct BERR_R(crate::FieldReader<bool, bool>);
impl BERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BERR` writer - Bit and line error grouping the following interruption"]
pub struct BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BERR_W<'a> {
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
#[doc = "Field `PERR` reader - Protocol error grouping the following interruption"]
pub struct PERR_R(crate::FieldReader<bool, bool>);
impl PERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERR` writer - Protocol error grouping the following interruption"]
pub struct PERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PERR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RX FIFO bit grouping the following interruption"]
    #[inline(always)]
    pub fn rxfifo0(&self) -> RXFIFO0_R {
        RXFIFO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX FIFO bit grouping the following interruption"]
    #[inline(always)]
    pub fn rxfifo1(&self) -> RXFIFO1_R {
        RXFIFO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status message bit grouping the following interruption"]
    #[inline(always)]
    pub fn smsg(&self) -> SMSG_R {
        SMSG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX FIFO error grouping the following interruption"]
    #[inline(always)]
    pub fn tferr(&self) -> TFERR_R {
        TFERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt regrouping the following interruption"]
    #[inline(always)]
    pub fn misc(&self) -> MISC_R {
        MISC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bit and line error grouping the following interruption"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Protocol error grouping the following interruption"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX FIFO bit grouping the following interruption"]
    #[inline(always)]
    pub fn rxfifo0(&mut self) -> RXFIFO0_W {
        RXFIFO0_W { w: self }
    }
    #[doc = "Bit 1 - RX FIFO bit grouping the following interruption"]
    #[inline(always)]
    pub fn rxfifo1(&mut self) -> RXFIFO1_W {
        RXFIFO1_W { w: self }
    }
    #[doc = "Bit 2 - Status message bit grouping the following interruption"]
    #[inline(always)]
    pub fn smsg(&mut self) -> SMSG_W {
        SMSG_W { w: self }
    }
    #[doc = "Bit 3 - TX FIFO error grouping the following interruption"]
    #[inline(always)]
    pub fn tferr(&mut self) -> TFERR_W {
        TFERR_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt regrouping the following interruption"]
    #[inline(always)]
    pub fn misc(&mut self) -> MISC_W {
        MISC_W { w: self }
    }
    #[doc = "Bit 5 - Bit and line error grouping the following interruption"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W { w: self }
    }
    #[doc = "Bit 6 - Protocol error grouping the following interruption"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Interrupt Line Select register assigns an interrupt generated by a specific interrupt flag from the Interrupt Register to one of the two module interrupt lines. For interrupt generation the respective interrupt line has to be enabled via ILE\\[EINT0\\]
and ILE\\[EINT1\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ils](index.html) module"]
pub struct ILS_SPEC;
impl crate::RegisterSpec for ILS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ils::R](R) reader structure"]
impl crate::Readable for ILS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ils::W](W) writer structure"]
impl crate::Writable for ILS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ILS to value 0"]
impl crate::Resettable for ILS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
