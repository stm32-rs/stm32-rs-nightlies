#[doc = "Register `CCMR1_Input` reader"]
pub struct R(crate::R<CCMR1_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR1_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR1_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR1_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMR1_Input` writer"]
pub struct W(crate::W<CCMR1_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR1_INPUT_SPEC>;
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
impl From<crate::W<CCMR1_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR1_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection"]
pub struct CC1S_R(crate::FieldReader<u8, u8>);
impl CC1S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC1S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC1S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection"]
pub struct CC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `OC1FE` reader - Output Compare 1 fast enable"]
pub struct OC1FE_R(crate::FieldReader<bool, bool>);
impl OC1FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC1FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC1FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC1FE` writer - Output Compare 1 fast enable"]
pub struct OC1FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1FE_W<'a> {
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
#[doc = "Field `OC1PE` reader - Output Compare 1 preload enable"]
pub struct OC1PE_R(crate::FieldReader<bool, bool>);
impl OC1PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC1PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC1PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC1PE` writer - Output Compare 1 preload enable"]
pub struct OC1PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1PE_W<'a> {
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
#[doc = "Field `OC1M` reader - Output Compare 1 mode"]
pub struct OC1M_R(crate::FieldReader<u8, u8>);
impl OC1M_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC1M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC1M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC1M` writer - Output Compare 1 mode"]
pub struct OC1M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `OC1CE` reader - Output Compare 1 clear enable"]
pub struct OC1CE_R(crate::FieldReader<bool, bool>);
impl OC1CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC1CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC1CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC1CE` writer - Output Compare 1 clear enable"]
pub struct OC1CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1CE_W<'a> {
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
#[doc = "Field `CC2S` reader - Capture/Compare 2 selection"]
pub struct CC2S_R(crate::FieldReader<u8, u8>);
impl CC2S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2S` writer - Capture/Compare 2 selection"]
pub struct CC2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `OC2FE` reader - Output Compare 2 fast enable"]
pub struct OC2FE_R(crate::FieldReader<bool, bool>);
impl OC2FE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC2FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC2FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC2FE` writer - Output Compare 2 fast enable"]
pub struct OC2FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2FE_W<'a> {
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
#[doc = "Field `OC2PE` reader - Output Compare 2 preload enable"]
pub struct OC2PE_R(crate::FieldReader<bool, bool>);
impl OC2PE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC2PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC2PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC2PE` writer - Output Compare 2 preload enable"]
pub struct OC2PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2PE_W<'a> {
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
#[doc = "Field `OC2M` reader - Output Compare 2 mode"]
pub struct OC2M_R(crate::FieldReader<u8, u8>);
impl OC2M_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC2M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC2M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC2M` writer - Output Compare 2 mode"]
pub struct OC2M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `OC2CE` reader - Output Compare 2 clear enable"]
pub struct OC2CE_R(crate::FieldReader<bool, bool>);
impl OC2CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OC2CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC2CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC2CE` writer - Output Compare 2 clear enable"]
pub struct OC2CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2CE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fe(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pe(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode"]
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Output Compare 1 clear enable"]
    #[inline(always)]
    pub fn oc1ce(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Output Compare 2 fast enable"]
    #[inline(always)]
    pub fn oc2fe(&self) -> OC2FE_R {
        OC2FE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Output Compare 2 preload enable"]
    #[inline(always)]
    pub fn oc2pe(&self) -> OC2PE_R {
        OC2PE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Output Compare 2 mode"]
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Output Compare 2 clear enable"]
    #[inline(always)]
    pub fn oc2ce(&self) -> OC2CE_R {
        OC2CE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W { w: self }
    }
    #[doc = "Bit 2 - Output Compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fe(&mut self) -> OC1FE_W {
        OC1FE_W { w: self }
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pe(&mut self) -> OC1PE_W {
        OC1PE_W { w: self }
    }
    #[doc = "Bits 4:6 - Output Compare 1 mode"]
    #[inline(always)]
    pub fn oc1m(&mut self) -> OC1M_W {
        OC1M_W { w: self }
    }
    #[doc = "Bit 7 - Output Compare 1 clear enable"]
    #[inline(always)]
    pub fn oc1ce(&mut self) -> OC1CE_W {
        OC1CE_W { w: self }
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&mut self) -> CC2S_W {
        CC2S_W { w: self }
    }
    #[doc = "Bit 10 - Output Compare 2 fast enable"]
    #[inline(always)]
    pub fn oc2fe(&mut self) -> OC2FE_W {
        OC2FE_W { w: self }
    }
    #[doc = "Bit 11 - Output Compare 2 preload enable"]
    #[inline(always)]
    pub fn oc2pe(&mut self) -> OC2PE_W {
        OC2PE_W { w: self }
    }
    #[doc = "Bits 12:14 - Output Compare 2 mode"]
    #[inline(always)]
    pub fn oc2m(&mut self) -> OC2M_W {
        OC2M_W { w: self }
    }
    #[doc = "Bit 15 - Output Compare 2 clear enable"]
    #[inline(always)]
    pub fn oc2ce(&mut self) -> OC2CE_W {
        OC2CE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 1 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr1_input](index.html) module"]
pub struct CCMR1_INPUT_SPEC;
impl crate::RegisterSpec for CCMR1_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccmr1_input::R](R) reader structure"]
impl crate::Readable for CCMR1_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmr1_input::W](W) writer structure"]
impl crate::Writable for CCMR1_INPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCMR1_Input to value 0"]
impl crate::Resettable for CCMR1_INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
