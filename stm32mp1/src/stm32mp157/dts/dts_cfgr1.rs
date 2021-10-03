#[doc = "Register `DTS_CFGR1` reader"]
pub struct R(crate::R<DTS_CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTS_CFGR1` writer"]
pub struct W(crate::W<DTS_CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTS_CFGR1_SPEC>;
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
impl From<crate::W<DTS_CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTS_CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS1_EN` reader - TS1_EN"]
pub struct TS1_EN_R(crate::FieldReader<bool, bool>);
impl TS1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_EN` writer - TS1_EN"]
pub struct TS1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_EN_W<'a> {
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
#[doc = "Field `TS1_START` reader - TS1_START"]
pub struct TS1_START_R(crate::FieldReader<bool, bool>);
impl TS1_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_START` writer - TS1_START"]
pub struct TS1_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_START_W<'a> {
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
#[doc = "Field `TS1_INTRIG_SEL` reader - TS1_INTRIG_SEL"]
pub struct TS1_INTRIG_SEL_R(crate::FieldReader<u8, u8>);
impl TS1_INTRIG_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TS1_INTRIG_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_INTRIG_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_INTRIG_SEL` writer - TS1_INTRIG_SEL"]
pub struct TS1_INTRIG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_INTRIG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TS1_SMP_TIME` reader - TS1_SMP_TIME"]
pub struct TS1_SMP_TIME_R(crate::FieldReader<u8, u8>);
impl TS1_SMP_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        TS1_SMP_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_SMP_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1_SMP_TIME` writer - TS1_SMP_TIME"]
pub struct TS1_SMP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_SMP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `REFCLK_SEL` reader - REFCLK_SEL"]
pub struct REFCLK_SEL_R(crate::FieldReader<bool, bool>);
impl REFCLK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFCLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFCLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFCLK_SEL` writer - REFCLK_SEL"]
pub struct REFCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCLK_SEL_W<'a> {
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
#[doc = "Field `Q_MEAS_opt` reader - Q_MEAS_opt"]
pub struct Q_MEAS_OPT_R(crate::FieldReader<bool, bool>);
impl Q_MEAS_OPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        Q_MEAS_OPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Q_MEAS_OPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Q_MEAS_opt` writer - Q_MEAS_opt"]
pub struct Q_MEAS_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> Q_MEAS_OPT_W<'a> {
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
#[doc = "Field `HSREF_CLK_DIV` reader - HSREF_CLK_DIV"]
pub struct HSREF_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl HSREF_CLK_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSREF_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSREF_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSREF_CLK_DIV` writer - HSREF_CLK_DIV"]
pub struct HSREF_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREF_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TS1_EN"]
    #[inline(always)]
    pub fn ts1_en(&self) -> TS1_EN_R {
        TS1_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - TS1_START"]
    #[inline(always)]
    pub fn ts1_start(&self) -> TS1_START_R {
        TS1_START_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - TS1_INTRIG_SEL"]
    #[inline(always)]
    pub fn ts1_intrig_sel(&self) -> TS1_INTRIG_SEL_R {
        TS1_INTRIG_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TS1_SMP_TIME"]
    #[inline(always)]
    pub fn ts1_smp_time(&self) -> TS1_SMP_TIME_R {
        TS1_SMP_TIME_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - REFCLK_SEL"]
    #[inline(always)]
    pub fn refclk_sel(&self) -> REFCLK_SEL_R {
        REFCLK_SEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Q_MEAS_opt"]
    #[inline(always)]
    pub fn q_meas_opt(&self) -> Q_MEAS_OPT_R {
        Q_MEAS_OPT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 24:30 - HSREF_CLK_DIV"]
    #[inline(always)]
    pub fn hsref_clk_div(&self) -> HSREF_CLK_DIV_R {
        HSREF_CLK_DIV_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TS1_EN"]
    #[inline(always)]
    pub fn ts1_en(&mut self) -> TS1_EN_W {
        TS1_EN_W { w: self }
    }
    #[doc = "Bit 4 - TS1_START"]
    #[inline(always)]
    pub fn ts1_start(&mut self) -> TS1_START_W {
        TS1_START_W { w: self }
    }
    #[doc = "Bits 8:11 - TS1_INTRIG_SEL"]
    #[inline(always)]
    pub fn ts1_intrig_sel(&mut self) -> TS1_INTRIG_SEL_W {
        TS1_INTRIG_SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - TS1_SMP_TIME"]
    #[inline(always)]
    pub fn ts1_smp_time(&mut self) -> TS1_SMP_TIME_W {
        TS1_SMP_TIME_W { w: self }
    }
    #[doc = "Bit 20 - REFCLK_SEL"]
    #[inline(always)]
    pub fn refclk_sel(&mut self) -> REFCLK_SEL_W {
        REFCLK_SEL_W { w: self }
    }
    #[doc = "Bit 21 - Q_MEAS_opt"]
    #[inline(always)]
    pub fn q_meas_opt(&mut self) -> Q_MEAS_OPT_W {
        Q_MEAS_OPT_W { w: self }
    }
    #[doc = "Bits 24:30 - HSREF_CLK_DIV"]
    #[inline(always)]
    pub fn hsref_clk_div(&mut self) -> HSREF_CLK_DIV_W {
        HSREF_CLK_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTS_CFGR1 is the configuration register for temperature sensor 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_cfgr1](index.html) module"]
pub struct DTS_CFGR1_SPEC;
impl crate::RegisterSpec for DTS_CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_cfgr1::R](R) reader structure"]
impl crate::Readable for DTS_CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dts_cfgr1::W](W) writer structure"]
impl crate::Writable for DTS_CFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTS_CFGR1 to value 0"]
impl crate::Resettable for DTS_CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
