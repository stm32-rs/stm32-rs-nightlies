#[doc = "Register `I2C_CR2` reader"]
pub struct R(crate::R<I2C_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CR2` writer"]
pub struct W(crate::W<I2C_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CR2_SPEC>;
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
impl From<crate::W<I2C_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADD` reader - SADD"]
pub struct SADD_R(crate::FieldReader<u16, u16>);
impl SADD_R {
    pub(crate) fn new(bits: u16) -> Self {
        SADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADD` writer - SADD"]
pub struct SADD_W<'a> {
    w: &'a mut W,
}
impl<'a> SADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `RD_WRN` reader - RD_WRN"]
pub struct RD_WRN_R(crate::FieldReader<bool, bool>);
impl RD_WRN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RD_WRN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_WRN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_WRN` writer - RD_WRN"]
pub struct RD_WRN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_WRN_W<'a> {
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
#[doc = "Field `ADD10` reader - ADD10"]
pub struct ADD10_R(crate::FieldReader<bool, bool>);
impl ADD10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD10` writer - ADD10"]
pub struct ADD10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD10_W<'a> {
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
#[doc = "Field `HEAD10R` reader - HEAD10R"]
pub struct HEAD10R_R(crate::FieldReader<bool, bool>);
impl HEAD10R_R {
    pub(crate) fn new(bits: bool) -> Self {
        HEAD10R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HEAD10R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HEAD10R` writer - HEAD10R"]
pub struct HEAD10R_W<'a> {
    w: &'a mut W,
}
impl<'a> HEAD10R_W<'a> {
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
#[doc = "Field `START` reader - START"]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - START"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Field `STOP` reader - STOP"]
pub struct STOP_R(crate::FieldReader<bool, bool>);
impl STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - STOP"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
#[doc = "Field `NACK` reader - NACK"]
pub struct NACK_R(crate::FieldReader<bool, bool>);
impl NACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NACK` writer - NACK"]
pub struct NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> NACK_W<'a> {
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
#[doc = "Field `NBYTES` reader - NBYTES"]
pub struct NBYTES_R(crate::FieldReader<u8, u8>);
impl NBYTES_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBYTES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBYTES` writer - NBYTES"]
pub struct NBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `RELOAD` reader - RELOAD"]
pub struct RELOAD_R(crate::FieldReader<bool, bool>);
impl RELOAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RELOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RELOAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RELOAD` writer - RELOAD"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
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
#[doc = "Field `AUTOEND` reader - AUTOEND"]
pub struct AUTOEND_R(crate::FieldReader<bool, bool>);
impl AUTOEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUTOEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOEND` writer - AUTOEND"]
pub struct AUTOEND_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOEND_W<'a> {
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
#[doc = "Field `PECBYTE` reader - PECBYTE"]
pub struct PECBYTE_R(crate::FieldReader<bool, bool>);
impl PECBYTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PECBYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PECBYTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PECBYTE` writer - PECBYTE"]
pub struct PECBYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PECBYTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - SADD"]
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - RD_WRN"]
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADD10"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HEAD10R"]
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - STOP"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - NACK"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - NBYTES"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - RELOAD"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - AUTOEND"]
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PECBYTE"]
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - SADD"]
    #[inline(always)]
    pub fn sadd(&mut self) -> SADD_W {
        SADD_W { w: self }
    }
    #[doc = "Bit 10 - RD_WRN"]
    #[inline(always)]
    pub fn rd_wrn(&mut self) -> RD_WRN_W {
        RD_WRN_W { w: self }
    }
    #[doc = "Bit 11 - ADD10"]
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W {
        ADD10_W { w: self }
    }
    #[doc = "Bit 12 - HEAD10R"]
    #[inline(always)]
    pub fn head10r(&mut self) -> HEAD10R_W {
        HEAD10R_W { w: self }
    }
    #[doc = "Bit 13 - START"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 14 - STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 15 - NACK"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W {
        NACK_W { w: self }
    }
    #[doc = "Bits 16:23 - NBYTES"]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W {
        NBYTES_W { w: self }
    }
    #[doc = "Bit 24 - RELOAD"]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
    #[doc = "Bit 25 - AUTOEND"]
    #[inline(always)]
    pub fn autoend(&mut self) -> AUTOEND_W {
        AUTOEND_W { w: self }
    }
    #[doc = "Bit 26 - PECBYTE"]
    #[inline(always)]
    pub fn pecbyte(&mut self) -> PECBYTE_W {
        PECBYTE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x i2c_pclk + 6 x i2c_ker_ck.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_cr2](index.html) module"]
pub struct I2C_CR2_SPEC;
impl crate::RegisterSpec for I2C_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_cr2::R](R) reader structure"]
impl crate::Readable for I2C_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_cr2::W](W) writer structure"]
impl crate::Writable for I2C_CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CR2 to value 0"]
impl crate::Resettable for I2C_CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
