#[doc = "Register `AHBRSTR` reader"]
pub struct R(crate::R<AHBRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBRSTR` writer"]
pub struct W(crate::W<AHBRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBRSTR_SPEC>;
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
impl From<crate::W<AHBRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSMCRST` reader - FSMC reset"]
pub struct FSMCRST_R(crate::FieldReader<bool, bool>);
impl FSMCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSMCRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSMCRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSMCRST` writer - FSMC reset"]
pub struct FSMCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMCRST_W<'a> {
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
#[doc = "Field `DMA2RST` reader - DMA2 reset"]
pub struct DMA2RST_R(crate::FieldReader<bool, bool>);
impl DMA2RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2RST` writer - DMA2 reset"]
pub struct DMA2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2RST_W<'a> {
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
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub struct DMA1RST_R(crate::FieldReader<bool, bool>);
impl DMA1RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub struct DMA1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1RST_W<'a> {
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
#[doc = "Field `FLITFRST` reader - FLITF reset"]
pub struct FLITFRST_R(crate::FieldReader<bool, bool>);
impl FLITFRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLITFRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLITFRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLITFRST` writer - FLITF reset"]
pub struct FLITFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FLITFRST_W<'a> {
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
#[doc = "Field `CRCRST` reader - CRC reset"]
pub struct CRCRST_R(crate::FieldReader<bool, bool>);
impl CRCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCRST` writer - CRC reset"]
pub struct CRCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCRST_W<'a> {
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
#[doc = "Field `GPIOGRST` reader - IO port G reset"]
pub struct GPIOGRST_R(crate::FieldReader<bool, bool>);
impl GPIOGRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOGRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOGRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOGRST` writer - IO port G reset"]
pub struct GPIOGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGRST_W<'a> {
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
#[doc = "Field `GPIOFRST` reader - IO port F reset"]
pub struct GPIOFRST_R(crate::FieldReader<bool, bool>);
impl GPIOFRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOFRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOFRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOFRST` writer - IO port F reset"]
pub struct GPIOFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFRST_W<'a> {
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
#[doc = "Field `GPIOHRST` reader - IO port H reset"]
pub struct GPIOHRST_R(crate::FieldReader<bool, bool>);
impl GPIOHRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOHRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOHRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOHRST` writer - IO port H reset"]
pub struct GPIOHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHRST_W<'a> {
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
#[doc = "Field `GPIOERST` reader - IO port E reset"]
pub struct GPIOERST_R(crate::FieldReader<bool, bool>);
impl GPIOERST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOERST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOERST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOERST` writer - IO port E reset"]
pub struct GPIOERST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOERST_W<'a> {
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
#[doc = "Field `GPIODRST` reader - IO port D reset"]
pub struct GPIODRST_R(crate::FieldReader<bool, bool>);
impl GPIODRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIODRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIODRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIODRST` writer - IO port D reset"]
pub struct GPIODRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODRST_W<'a> {
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
#[doc = "Field `GPIOCRST` reader - IO port C reset"]
pub struct GPIOCRST_R(crate::FieldReader<bool, bool>);
impl GPIOCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOCRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOCRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOCRST` writer - IO port C reset"]
pub struct GPIOCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCRST_W<'a> {
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
#[doc = "Field `GPIOBRST` reader - IO port B reset"]
pub struct GPIOBRST_R(crate::FieldReader<bool, bool>);
impl GPIOBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOBRST` writer - IO port B reset"]
pub struct GPIOBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBRST_W<'a> {
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
#[doc = "Field `GPIOARST` reader - IO port A reset"]
pub struct GPIOARST_R(crate::FieldReader<bool, bool>);
impl GPIOARST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIOARST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIOARST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIOARST` writer - IO port A reset"]
pub struct GPIOARST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOARST_W<'a> {
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
    #[doc = "Bit 30 - FSMC reset"]
    #[inline(always)]
    pub fn fsmcrst(&self) -> FSMCRST_R {
        FSMCRST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FLITF reset"]
    #[inline(always)]
    pub fn flitfrst(&self) -> FLITFRST_R {
        FLITFRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IO port G reset"]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IO port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IO port H reset"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - FSMC reset"]
    #[inline(always)]
    pub fn fsmcrst(&mut self) -> FSMCRST_W {
        FSMCRST_W { w: self }
    }
    #[doc = "Bit 25 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W {
        DMA2RST_W { w: self }
    }
    #[doc = "Bit 24 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W {
        DMA1RST_W { w: self }
    }
    #[doc = "Bit 15 - FLITF reset"]
    #[inline(always)]
    pub fn flitfrst(&mut self) -> FLITFRST_W {
        FLITFRST_W { w: self }
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W {
        CRCRST_W { w: self }
    }
    #[doc = "Bit 7 - IO port G reset"]
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W {
        GPIOGRST_W { w: self }
    }
    #[doc = "Bit 6 - IO port F reset"]
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W {
        GPIOFRST_W { w: self }
    }
    #[doc = "Bit 5 - IO port H reset"]
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W {
        GPIOHRST_W { w: self }
    }
    #[doc = "Bit 4 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W {
        GPIOERST_W { w: self }
    }
    #[doc = "Bit 3 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W {
        GPIODRST_W { w: self }
    }
    #[doc = "Bit 2 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W {
        GPIOCRST_W { w: self }
    }
    #[doc = "Bit 1 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W {
        GPIOBRST_W { w: self }
    }
    #[doc = "Bit 0 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W {
        GPIOARST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrstr](index.html) module"]
pub struct AHBRSTR_SPEC;
impl crate::RegisterSpec for AHBRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbrstr::R](R) reader structure"]
impl crate::Readable for AHBRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbrstr::W](W) writer structure"]
impl crate::Writable for AHBRSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
