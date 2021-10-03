#[doc = "Register `WTCR` reader"]
pub struct R(crate::R<WTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WTCR` writer"]
pub struct W(crate::W<WTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WTCR_SPEC>;
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
impl From<crate::W<WTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMODE` reader - IMODE"]
pub struct IMODE_R(crate::FieldReader<u8, u8>);
impl IMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        IMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMODE` writer - IMODE"]
pub struct IMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `IDTR` reader - IDTR"]
pub struct IDTR_R(crate::FieldReader<bool, bool>);
impl IDTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDTR` writer - IDTR"]
pub struct IDTR_W<'a> {
    w: &'a mut W,
}
impl<'a> IDTR_W<'a> {
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
#[doc = "Field `ISIZE` reader - ISIZE"]
pub struct ISIZE_R(crate::FieldReader<u8, u8>);
impl ISIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ISIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISIZE` writer - ISIZE"]
pub struct ISIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `ADMODE` reader - ADMODE"]
pub struct ADMODE_R(crate::FieldReader<u8, u8>);
impl ADMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADMODE` writer - ADMODE"]
pub struct ADMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `ADDTR` reader - ADDTR"]
pub struct ADDTR_R(crate::FieldReader<bool, bool>);
impl ADDTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDTR` writer - ADDTR"]
pub struct ADDTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDTR_W<'a> {
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
#[doc = "Field `ADSIZE` reader - ADSIZE"]
pub struct ADSIZE_R(crate::FieldReader<u8, u8>);
impl ADSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADSIZE` writer - ADSIZE"]
pub struct ADSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `ABMODE` reader - ABMODE"]
pub struct ABMODE_R(crate::FieldReader<u8, u8>);
impl ABMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ABMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABMODE` writer - ABMODE"]
pub struct ABMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `ABDTR` reader - ABDTR"]
pub struct ABDTR_R(crate::FieldReader<bool, bool>);
impl ABDTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABDTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABDTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABDTR` writer - ABDTR"]
pub struct ABDTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ABDTR_W<'a> {
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
#[doc = "Field `ABSIZE` reader - ABSIZE"]
pub struct ABSIZE_R(crate::FieldReader<u8, u8>);
impl ABSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ABSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABSIZE` writer - ABSIZE"]
pub struct ABSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `DMODE` reader - DMODE"]
pub struct DMODE_R(crate::FieldReader<u8, u8>);
impl DMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMODE` writer - DMODE"]
pub struct DMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `DDTR` reader - DDTR"]
pub struct DDTR_R(crate::FieldReader<bool, bool>);
impl DDTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDTR` writer - DDTR"]
pub struct DDTR_W<'a> {
    w: &'a mut W,
}
impl<'a> DDTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `DQSE` reader - DQSE"]
pub struct DQSE_R(crate::FieldReader<bool, bool>);
impl DQSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DQSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSE` writer - DQSE"]
pub struct DQSE_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSE_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - IMODE"]
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - IDTR"]
    #[inline(always)]
    pub fn idtr(&self) -> IDTR_R {
        IDTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - ISIZE"]
    #[inline(always)]
    pub fn isize(&self) -> ISIZE_R {
        ISIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - ADMODE"]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - ADDTR"]
    #[inline(always)]
    pub fn addtr(&self) -> ADDTR_R {
        ADDTR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - ADSIZE"]
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - ABMODE"]
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - ABDTR"]
    #[inline(always)]
    pub fn abdtr(&self) -> ABDTR_R {
        ABDTR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - ABSIZE"]
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - DMODE"]
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - DDTR"]
    #[inline(always)]
    pub fn ddtr(&self) -> DDTR_R {
        DDTR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DQSE"]
    #[inline(always)]
    pub fn dqse(&self) -> DQSE_R {
        DQSE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - IMODE"]
    #[inline(always)]
    pub fn imode(&mut self) -> IMODE_W {
        IMODE_W { w: self }
    }
    #[doc = "Bit 3 - IDTR"]
    #[inline(always)]
    pub fn idtr(&mut self) -> IDTR_W {
        IDTR_W { w: self }
    }
    #[doc = "Bits 4:5 - ISIZE"]
    #[inline(always)]
    pub fn isize(&mut self) -> ISIZE_W {
        ISIZE_W { w: self }
    }
    #[doc = "Bits 8:10 - ADMODE"]
    #[inline(always)]
    pub fn admode(&mut self) -> ADMODE_W {
        ADMODE_W { w: self }
    }
    #[doc = "Bit 11 - ADDTR"]
    #[inline(always)]
    pub fn addtr(&mut self) -> ADDTR_W {
        ADDTR_W { w: self }
    }
    #[doc = "Bits 12:13 - ADSIZE"]
    #[inline(always)]
    pub fn adsize(&mut self) -> ADSIZE_W {
        ADSIZE_W { w: self }
    }
    #[doc = "Bits 16:18 - ABMODE"]
    #[inline(always)]
    pub fn abmode(&mut self) -> ABMODE_W {
        ABMODE_W { w: self }
    }
    #[doc = "Bit 19 - ABDTR"]
    #[inline(always)]
    pub fn abdtr(&mut self) -> ABDTR_W {
        ABDTR_W { w: self }
    }
    #[doc = "Bits 20:21 - ABSIZE"]
    #[inline(always)]
    pub fn absize(&mut self) -> ABSIZE_W {
        ABSIZE_W { w: self }
    }
    #[doc = "Bits 24:26 - DMODE"]
    #[inline(always)]
    pub fn dmode(&mut self) -> DMODE_W {
        DMODE_W { w: self }
    }
    #[doc = "Bit 27 - DDTR"]
    #[inline(always)]
    pub fn ddtr(&mut self) -> DDTR_W {
        DDTR_W { w: self }
    }
    #[doc = "Bit 29 - DQSE"]
    #[inline(always)]
    pub fn dqse(&mut self) -> DQSE_W {
        DQSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WTCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wtcr](index.html) module"]
pub struct WTCR_SPEC;
impl crate::RegisterSpec for WTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wtcr::R](R) reader structure"]
impl crate::Readable for WTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wtcr::W](W) writer structure"]
impl crate::Writable for WTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WTCR to value 0"]
impl crate::Resettable for WTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
