#[doc = "Register `DAC_SR` reader"]
pub struct R(crate::R<DAC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_SR` writer"]
pub struct W(crate::W<DAC_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_SR_SPEC>;
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
impl From<crate::W<DAC_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAUDR1` reader - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub struct DMAUDR1_R(crate::FieldReader<bool, bool>);
impl DMAUDR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAUDR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAUDR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAUDR1` writer - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub struct DMAUDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR1_W<'a> {
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
#[doc = "Field `CAL_FLAG1` reader - DAC Channel 1 calibration offset status This bit is set and cleared by hardware"]
pub struct CAL_FLAG1_R(crate::FieldReader<bool, bool>);
impl CAL_FLAG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAL_FLAG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_FLAG1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BWST1` reader - DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization)."]
pub struct BWST1_R(crate::FieldReader<bool, bool>);
impl BWST1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BWST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BWST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAUDR2` reader - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub struct DMAUDR2_R(crate::FieldReader<bool, bool>);
impl DMAUDR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAUDR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAUDR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAUDR2` writer - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
pub struct DMAUDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAUDR2_W<'a> {
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
#[doc = "Field `CAL_FLAG2` reader - DAC Channel 2 calibration offset status This bit is set and cleared by hardware"]
pub struct CAL_FLAG2_R(crate::FieldReader<bool, bool>);
impl CAL_FLAG2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAL_FLAG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_FLAG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BWST2` reader - DAC Channel 2 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization)."]
pub struct BWST2_R(crate::FieldReader<bool, bool>);
impl BWST2_R {
    pub(crate) fn new(bits: bool) -> Self {
        BWST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BWST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization)."]
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DAC Channel 2 calibration offset status This bit is set and cleared by hardware"]
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DAC Channel 2 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization)."]
    #[inline(always)]
    pub fn bwst2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W {
        DMAUDR1_W { w: self }
    }
    #[doc = "Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1)."]
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W {
        DMAUDR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_sr](index.html) module"]
pub struct DAC_SR_SPEC;
impl crate::RegisterSpec for DAC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_sr::R](R) reader structure"]
impl crate::Readable for DAC_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_sr::W](W) writer structure"]
impl crate::Writable for DAC_SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_SR to value 0"]
impl crate::Resettable for DAC_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
