#[doc = "Register `WPCR0` reader"]
pub struct R(crate::R<WPCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR0` writer"]
pub struct W(crate::W<WPCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR0_SPEC>;
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
impl From<crate::W<WPCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIX4` reader - UIX4"]
pub struct UIX4_R(crate::FieldReader<u8, u8>);
impl UIX4_R {
    pub(crate) fn new(bits: u8) -> Self {
        UIX4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UIX4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UIX4` writer - UIX4"]
pub struct UIX4_W<'a> {
    w: &'a mut W,
}
impl<'a> UIX4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `SWCL` reader - SWCL"]
pub struct SWCL_R(crate::FieldReader<bool, bool>);
impl SWCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWCL` writer - SWCL"]
pub struct SWCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCL_W<'a> {
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
#[doc = "Field `SWDL0` reader - SWDL0"]
pub struct SWDL0_R(crate::FieldReader<bool, bool>);
impl SWDL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWDL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWDL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWDL0` writer - SWDL0"]
pub struct SWDL0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDL0_W<'a> {
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
#[doc = "Field `SWDL1` reader - SWDL1"]
pub struct SWDL1_R(crate::FieldReader<bool, bool>);
impl SWDL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWDL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWDL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWDL1` writer - SWDL1"]
pub struct SWDL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDL1_W<'a> {
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
#[doc = "Field `HSICL` reader - HSICL"]
pub struct HSICL_R(crate::FieldReader<bool, bool>);
impl HSICL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSICL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSICL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSICL` writer - HSICL"]
pub struct HSICL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSICL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `HSIDL0` reader - HSIDL0"]
pub struct HSIDL0_R(crate::FieldReader<bool, bool>);
impl HSIDL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIDL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIDL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIDL0` writer - HSIDL0"]
pub struct HSIDL0_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDL0_W<'a> {
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
#[doc = "Field `HSIDL1` reader - HSIDL1"]
pub struct HSIDL1_R(crate::FieldReader<bool, bool>);
impl HSIDL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSIDL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIDL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIDL1` writer - HSIDL1"]
pub struct HSIDL1_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDL1_W<'a> {
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
#[doc = "Field `FTXSMCL` reader - FTXSMCL"]
pub struct FTXSMCL_R(crate::FieldReader<bool, bool>);
impl FTXSMCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTXSMCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTXSMCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTXSMCL` writer - FTXSMCL"]
pub struct FTXSMCL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTXSMCL_W<'a> {
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
#[doc = "Field `FTXSMDL` reader - FTXSMDL"]
pub struct FTXSMDL_R(crate::FieldReader<bool, bool>);
impl FTXSMDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTXSMDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTXSMDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTXSMDL` writer - FTXSMDL"]
pub struct FTXSMDL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTXSMDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CDOFFDL` reader - CDOFFDL"]
pub struct CDOFFDL_R(crate::FieldReader<bool, bool>);
impl CDOFFDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDOFFDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDOFFDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDOFFDL` writer - CDOFFDL"]
pub struct CDOFFDL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDOFFDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `TDDL` reader - TDDL"]
pub struct TDDL_R(crate::FieldReader<bool, bool>);
impl TDDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDDL` writer - TDDL"]
pub struct TDDL_W<'a> {
    w: &'a mut W,
}
impl<'a> TDDL_W<'a> {
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
impl R {
    #[doc = "Bits 0:5 - UIX4"]
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - SWCL"]
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SWDL0"]
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SWDL1"]
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HSICL"]
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSIDL0"]
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSIDL1"]
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FTXSMCL"]
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - FTXSMDL"]
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CDOFFDL"]
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TDDL"]
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - UIX4"]
    #[inline(always)]
    pub fn uix4(&mut self) -> UIX4_W {
        UIX4_W { w: self }
    }
    #[doc = "Bit 6 - SWCL"]
    #[inline(always)]
    pub fn swcl(&mut self) -> SWCL_W {
        SWCL_W { w: self }
    }
    #[doc = "Bit 7 - SWDL0"]
    #[inline(always)]
    pub fn swdl0(&mut self) -> SWDL0_W {
        SWDL0_W { w: self }
    }
    #[doc = "Bit 8 - SWDL1"]
    #[inline(always)]
    pub fn swdl1(&mut self) -> SWDL1_W {
        SWDL1_W { w: self }
    }
    #[doc = "Bit 9 - HSICL"]
    #[inline(always)]
    pub fn hsicl(&mut self) -> HSICL_W {
        HSICL_W { w: self }
    }
    #[doc = "Bit 10 - HSIDL0"]
    #[inline(always)]
    pub fn hsidl0(&mut self) -> HSIDL0_W {
        HSIDL0_W { w: self }
    }
    #[doc = "Bit 11 - HSIDL1"]
    #[inline(always)]
    pub fn hsidl1(&mut self) -> HSIDL1_W {
        HSIDL1_W { w: self }
    }
    #[doc = "Bit 12 - FTXSMCL"]
    #[inline(always)]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W {
        FTXSMCL_W { w: self }
    }
    #[doc = "Bit 13 - FTXSMDL"]
    #[inline(always)]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W {
        FTXSMDL_W { w: self }
    }
    #[doc = "Bit 14 - CDOFFDL"]
    #[inline(always)]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W {
        CDOFFDL_W { w: self }
    }
    #[doc = "Bit 16 - TDDL"]
    #[inline(always)]
    pub fn tddl(&mut self) -> TDDL_W {
        TDDL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI wrapper PHY configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr0](index.html) module"]
pub struct WPCR0_SPEC;
impl crate::RegisterSpec for WPCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr0::R](R) reader structure"]
impl crate::Readable for WPCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr0::W](W) writer structure"]
impl crate::Writable for WPCR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WPCR0 to value 0"]
impl crate::Resettable for WPCR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
