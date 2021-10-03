#[doc = "Register `ADC_CFGR2` reader"]
pub struct R(crate::R<ADC_CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CFGR2` writer"]
pub struct W(crate::W<ADC_CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CFGR2_SPEC>;
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
impl From<crate::W<ADC_CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVSE` reader - OVSE"]
pub struct OVSE_R(crate::FieldReader<bool, bool>);
impl OVSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSE` writer - OVSE"]
pub struct OVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSE_W<'a> {
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
#[doc = "Field `OVSR0` reader - OVSR0"]
pub struct OVSR0_R(crate::FieldReader<bool, bool>);
impl OVSR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVSR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVSR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSR0` writer - OVSR0"]
pub struct OVSR0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSR0_W<'a> {
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
#[doc = "Field `OVSR1` reader - OVSR1"]
pub struct OVSR1_R(crate::FieldReader<bool, bool>);
impl OVSR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVSR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVSR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSR1` writer - OVSR1"]
pub struct OVSR1_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSR1_W<'a> {
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
#[doc = "Field `OVSR2` reader - OVSR2"]
pub struct OVSR2_R(crate::FieldReader<bool, bool>);
impl OVSR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVSR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVSR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSR2` writer - OVSR2"]
pub struct OVSR2_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSR2_W<'a> {
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
#[doc = "Field `OVSS0` reader - OVSS0"]
pub struct OVSS0_R(crate::FieldReader<bool, bool>);
impl OVSS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVSS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVSS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSS0` writer - OVSS0"]
pub struct OVSS0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS0_W<'a> {
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
#[doc = "Field `OVSS1` reader - OVSS1"]
pub struct OVSS1_R(crate::FieldReader<bool, bool>);
impl OVSS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVSS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVSS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSS1` writer - OVSS1"]
pub struct OVSS1_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS1_W<'a> {
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
#[doc = "Field `OVSS2` reader - OVSS2"]
pub struct OVSS2_R(crate::FieldReader<bool, bool>);
impl OVSS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVSS2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVSS2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSS2` writer - OVSS2"]
pub struct OVSS2_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS2_W<'a> {
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
#[doc = "Field `OVSS3` reader - OVSS3"]
pub struct OVSS3_R(crate::FieldReader<bool, bool>);
impl OVSS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVSS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVSS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSS3` writer - OVSS3"]
pub struct OVSS3_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS3_W<'a> {
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
#[doc = "Field `TOVS` reader - TOVS"]
pub struct TOVS_R(crate::FieldReader<bool, bool>);
impl TOVS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOVS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOVS` writer - TOVS"]
pub struct TOVS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOVS_W<'a> {
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
#[doc = "Field `LFTRIG` reader - LFTRIG"]
pub struct LFTRIG_R(crate::FieldReader<bool, bool>);
impl LFTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFTRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFTRIG` writer - LFTRIG"]
pub struct LFTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> LFTRIG_W<'a> {
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
#[doc = "Field `CKMODE` reader - CKMODE"]
pub struct CKMODE_R(crate::FieldReader<u8, u8>);
impl CKMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKMODE` writer - CKMODE"]
pub struct CKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OVSE"]
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - OVSR0"]
    #[inline(always)]
    pub fn ovsr0(&self) -> OVSR0_R {
        OVSR0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OVSR1"]
    #[inline(always)]
    pub fn ovsr1(&self) -> OVSR1_R {
        OVSR1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OVSR2"]
    #[inline(always)]
    pub fn ovsr2(&self) -> OVSR2_R {
        OVSR2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OVSS0"]
    #[inline(always)]
    pub fn ovss0(&self) -> OVSS0_R {
        OVSS0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OVSS1"]
    #[inline(always)]
    pub fn ovss1(&self) -> OVSS1_R {
        OVSS1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - OVSS2"]
    #[inline(always)]
    pub fn ovss2(&self) -> OVSS2_R {
        OVSS2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - OVSS3"]
    #[inline(always)]
    pub fn ovss3(&self) -> OVSS3_R {
        OVSS3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TOVS"]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LFTRIG"]
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - CKMODE"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OVSE"]
    #[inline(always)]
    pub fn ovse(&mut self) -> OVSE_W {
        OVSE_W { w: self }
    }
    #[doc = "Bit 2 - OVSR0"]
    #[inline(always)]
    pub fn ovsr0(&mut self) -> OVSR0_W {
        OVSR0_W { w: self }
    }
    #[doc = "Bit 3 - OVSR1"]
    #[inline(always)]
    pub fn ovsr1(&mut self) -> OVSR1_W {
        OVSR1_W { w: self }
    }
    #[doc = "Bit 4 - OVSR2"]
    #[inline(always)]
    pub fn ovsr2(&mut self) -> OVSR2_W {
        OVSR2_W { w: self }
    }
    #[doc = "Bit 5 - OVSS0"]
    #[inline(always)]
    pub fn ovss0(&mut self) -> OVSS0_W {
        OVSS0_W { w: self }
    }
    #[doc = "Bit 6 - OVSS1"]
    #[inline(always)]
    pub fn ovss1(&mut self) -> OVSS1_W {
        OVSS1_W { w: self }
    }
    #[doc = "Bit 7 - OVSS2"]
    #[inline(always)]
    pub fn ovss2(&mut self) -> OVSS2_W {
        OVSS2_W { w: self }
    }
    #[doc = "Bit 8 - OVSS3"]
    #[inline(always)]
    pub fn ovss3(&mut self) -> OVSS3_W {
        OVSS3_W { w: self }
    }
    #[doc = "Bit 9 - TOVS"]
    #[inline(always)]
    pub fn tovs(&mut self) -> TOVS_W {
        TOVS_W { w: self }
    }
    #[doc = "Bit 29 - LFTRIG"]
    #[inline(always)]
    pub fn lftrig(&mut self) -> LFTRIG_W {
        LFTRIG_W { w: self }
    }
    #[doc = "Bits 30:31 - CKMODE"]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CKMODE_W {
        CKMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cfgr2](index.html) module"]
pub struct ADC_CFGR2_SPEC;
impl crate::RegisterSpec for ADC_CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_cfgr2::R](R) reader structure"]
impl crate::Readable for ADC_CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_cfgr2::W](W) writer structure"]
impl crate::Writable for ADC_CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CFGR2 to value 0"]
impl crate::Resettable for ADC_CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
