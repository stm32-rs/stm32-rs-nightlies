#[doc = "Register `C2MR` reader"]
pub struct R(crate::R<C2MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2MR` writer"]
pub struct W(crate::W<C2MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2MR_SPEC>;
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
impl From<crate::W<C2MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH6FM` reader - processor 2 Transmit channel 6 free interrupt mask"]
pub struct CH6FM_R(crate::FieldReader<bool, bool>);
impl CH6FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH6FM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6FM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6FM` writer - processor 2 Transmit channel 6 free interrupt mask"]
pub struct CH6FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6FM_W<'a> {
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
#[doc = "Field `CH5FM` reader - processor 2 Transmit channel 5 free interrupt mask"]
pub struct CH5FM_R(crate::FieldReader<bool, bool>);
impl CH5FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5FM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5FM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5FM` writer - processor 2 Transmit channel 5 free interrupt mask"]
pub struct CH5FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5FM_W<'a> {
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
#[doc = "Field `CH4FM` reader - processor 2 Transmit channel 4 free interrupt mask"]
pub struct CH4FM_R(crate::FieldReader<bool, bool>);
impl CH4FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4FM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4FM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4FM` writer - processor 2 Transmit channel 4 free interrupt mask"]
pub struct CH4FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4FM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CH3FM` reader - processor 2 Transmit channel 3 free interrupt mask"]
pub struct CH3FM_R(crate::FieldReader<bool, bool>);
impl CH3FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3FM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3FM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3FM` writer - processor 2 Transmit channel 3 free interrupt mask"]
pub struct CH3FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3FM_W<'a> {
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
#[doc = "Field `CH2FM` reader - processor 2 Transmit channel 2 free interrupt mask"]
pub struct CH2FM_R(crate::FieldReader<bool, bool>);
impl CH2FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2FM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2FM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2FM` writer - processor 2 Transmit channel 2 free interrupt mask"]
pub struct CH2FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2FM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CH1FM` reader - processor 2 Transmit channel 1 free interrupt mask"]
pub struct CH1FM_R(crate::FieldReader<bool, bool>);
impl CH1FM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1FM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1FM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1FM` writer - processor 2 Transmit channel 1 free interrupt mask"]
pub struct CH1FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1FM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CH6OM` reader - processor 2 Receive channel 6 occupied interrupt enable"]
pub struct CH6OM_R(crate::FieldReader<bool, bool>);
impl CH6OM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH6OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6OM` writer - processor 2 Receive channel 6 occupied interrupt enable"]
pub struct CH6OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6OM_W<'a> {
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
#[doc = "Field `CH5OM` reader - processor 2 Receive channel 5 occupied interrupt enable"]
pub struct CH5OM_R(crate::FieldReader<bool, bool>);
impl CH5OM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5OM` writer - processor 2 Receive channel 5 occupied interrupt enable"]
pub struct CH5OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5OM_W<'a> {
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
#[doc = "Field `CH4OM` reader - processor 2 Receive channel 4 occupied interrupt enable"]
pub struct CH4OM_R(crate::FieldReader<bool, bool>);
impl CH4OM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4OM` writer - processor 2 Receive channel 4 occupied interrupt enable"]
pub struct CH4OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4OM_W<'a> {
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
#[doc = "Field `CH3OM` reader - processor 2 Receive channel 3 occupied interrupt enable"]
pub struct CH3OM_R(crate::FieldReader<bool, bool>);
impl CH3OM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3OM` writer - processor 2 Receive channel 3 occupied interrupt enable"]
pub struct CH3OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OM_W<'a> {
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
#[doc = "Field `CH2OM` reader - processor 2 Receive channel 2 occupied interrupt enable"]
pub struct CH2OM_R(crate::FieldReader<bool, bool>);
impl CH2OM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2OM` writer - processor 2 Receive channel 2 occupied interrupt enable"]
pub struct CH2OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OM_W<'a> {
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
#[doc = "Field `CH1OM` reader - processor 2 Receive channel 1 occupied interrupt enable"]
pub struct CH1OM_R(crate::FieldReader<bool, bool>);
impl CH1OM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1OM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1OM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1OM` writer - processor 2 Receive channel 1 occupied interrupt enable"]
pub struct CH1OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OM_W<'a> {
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
    #[doc = "Bit 21 - processor 2 Transmit channel 6 free interrupt mask"]
    #[inline(always)]
    pub fn ch6fm(&self) -> CH6FM_R {
        CH6FM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - processor 2 Transmit channel 5 free interrupt mask"]
    #[inline(always)]
    pub fn ch5fm(&self) -> CH5FM_R {
        CH5FM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - processor 2 Transmit channel 4 free interrupt mask"]
    #[inline(always)]
    pub fn ch4fm(&self) -> CH4FM_R {
        CH4FM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - processor 2 Transmit channel 3 free interrupt mask"]
    #[inline(always)]
    pub fn ch3fm(&self) -> CH3FM_R {
        CH3FM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - processor 2 Transmit channel 2 free interrupt mask"]
    #[inline(always)]
    pub fn ch2fm(&self) -> CH2FM_R {
        CH2FM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - processor 2 Transmit channel 1 free interrupt mask"]
    #[inline(always)]
    pub fn ch1fm(&self) -> CH1FM_R {
        CH1FM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 5 - processor 2 Receive channel 6 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch6om(&self) -> CH6OM_R {
        CH6OM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - processor 2 Receive channel 5 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch5om(&self) -> CH5OM_R {
        CH5OM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - processor 2 Receive channel 4 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch4om(&self) -> CH4OM_R {
        CH4OM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - processor 2 Receive channel 3 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - processor 2 Receive channel 2 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - processor 2 Receive channel 1 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - processor 2 Transmit channel 6 free interrupt mask"]
    #[inline(always)]
    pub fn ch6fm(&mut self) -> CH6FM_W {
        CH6FM_W { w: self }
    }
    #[doc = "Bit 20 - processor 2 Transmit channel 5 free interrupt mask"]
    #[inline(always)]
    pub fn ch5fm(&mut self) -> CH5FM_W {
        CH5FM_W { w: self }
    }
    #[doc = "Bit 19 - processor 2 Transmit channel 4 free interrupt mask"]
    #[inline(always)]
    pub fn ch4fm(&mut self) -> CH4FM_W {
        CH4FM_W { w: self }
    }
    #[doc = "Bit 18 - processor 2 Transmit channel 3 free interrupt mask"]
    #[inline(always)]
    pub fn ch3fm(&mut self) -> CH3FM_W {
        CH3FM_W { w: self }
    }
    #[doc = "Bit 17 - processor 2 Transmit channel 2 free interrupt mask"]
    #[inline(always)]
    pub fn ch2fm(&mut self) -> CH2FM_W {
        CH2FM_W { w: self }
    }
    #[doc = "Bit 16 - processor 2 Transmit channel 1 free interrupt mask"]
    #[inline(always)]
    pub fn ch1fm(&mut self) -> CH1FM_W {
        CH1FM_W { w: self }
    }
    #[doc = "Bit 5 - processor 2 Receive channel 6 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch6om(&mut self) -> CH6OM_W {
        CH6OM_W { w: self }
    }
    #[doc = "Bit 4 - processor 2 Receive channel 5 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch5om(&mut self) -> CH5OM_W {
        CH5OM_W { w: self }
    }
    #[doc = "Bit 3 - processor 2 Receive channel 4 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch4om(&mut self) -> CH4OM_W {
        CH4OM_W { w: self }
    }
    #[doc = "Bit 2 - processor 2 Receive channel 3 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch3om(&mut self) -> CH3OM_W {
        CH3OM_W { w: self }
    }
    #[doc = "Bit 1 - processor 2 Receive channel 2 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch2om(&mut self) -> CH2OM_W {
        CH2OM_W { w: self }
    }
    #[doc = "Bit 0 - processor 2 Receive channel 1 occupied interrupt enable"]
    #[inline(always)]
    pub fn ch1om(&mut self) -> CH1OM_W {
        CH1OM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask register CPU2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2mr](index.html) module"]
pub struct C2MR_SPEC;
impl crate::RegisterSpec for C2MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2mr::R](R) reader structure"]
impl crate::Readable for C2MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2mr::W](W) writer structure"]
impl crate::Writable for C2MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2MR to value 0xffff_ffff"]
impl crate::Resettable for C2MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
