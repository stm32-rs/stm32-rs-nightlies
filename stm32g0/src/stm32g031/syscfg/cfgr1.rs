#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_PAx_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits"]
pub struct I2C_PAX_FMP_R(crate::FieldReader<u8, u8>);
impl I2C_PAX_FMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C_PAX_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_PAX_FMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_PAx_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits"]
pub struct I2C_PAX_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PAX_FMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `I2C2_FMP` reader - FM+ driving capability activation for I2C2"]
pub struct I2C2_FMP_R(crate::FieldReader<bool, bool>);
impl I2C2_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2_FMP` writer - FM+ driving capability activation for I2C2"]
pub struct I2C2_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_FMP_W<'a> {
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
#[doc = "Field `I2C1_FMP` reader - FM+ driving capability activation for I2C1"]
pub struct I2C1_FMP_R(crate::FieldReader<bool, bool>);
impl I2C1_FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1_FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1_FMP` writer - FM+ driving capability activation for I2C1"]
pub struct I2C1_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_FMP_W<'a> {
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
#[doc = "Field `I2C_PBx_FMP` reader - Fast Mode Plus (FM+) driving capability activation bits"]
pub struct I2C_PBX_FMP_R(crate::FieldReader<u8, u8>);
impl I2C_PBX_FMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C_PBX_FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_PBX_FMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_PBx_FMP` writer - Fast Mode Plus (FM+) driving capability activation bits"]
pub struct I2C_PBX_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_PBX_FMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `BOOSTEN` reader - I/O analog switch voltage booster enable"]
pub struct BOOSTEN_R(crate::FieldReader<bool, bool>);
impl BOOSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOSTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOSTEN` writer - I/O analog switch voltage booster enable"]
pub struct BOOSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOSTEN_W<'a> {
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
#[doc = "Field `IR_MOD` reader - IR Modulation Envelope signal selection."]
pub struct IR_MOD_R(crate::FieldReader<u8, u8>);
impl IR_MOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        IR_MOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IR_MOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IR_MOD` writer - IR Modulation Envelope signal selection."]
pub struct IR_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `IR_POL` reader - IR output polarity selection"]
pub struct IR_POL_R(crate::FieldReader<bool, bool>);
impl IR_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        IR_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IR_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IR_POL` writer - IR output polarity selection"]
pub struct IR_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_POL_W<'a> {
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
#[doc = "Field `PA11_PA12_RMP` reader - PA11 and PA12 remapping bit."]
pub struct PA11_PA12_RMP_R(crate::FieldReader<bool, bool>);
impl PA11_PA12_RMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA11_PA12_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA11_PA12_RMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA11_PA12_RMP` writer - PA11 and PA12 remapping bit."]
pub struct PA11_PA12_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PA11_PA12_RMP_W<'a> {
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
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub struct MEM_MODE_R(crate::FieldReader<u8, u8>);
impl MEM_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MEM_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MEM_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub struct MEM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:23 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pax_fmp(&self) -> I2C_PAX_FMP_R {
        I2C_PAX_FMP_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 21 - FM+ driving capability activation for I2C2"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - FM+ driving capability activation for I2C1"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pbx_fmp(&self) -> I2C_PBX_FMP_R {
        I2C_PBX_FMP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection."]
    #[inline(always)]
    pub fn ir_mod(&self) -> IR_MOD_R {
        IR_MOD_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    pub fn ir_pol(&self) -> IR_POL_R {
        IR_POL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit."]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&self) -> PA11_PA12_RMP_R {
        PA11_PA12_RMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 22:23 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pax_fmp(&mut self) -> I2C_PAX_FMP_W {
        I2C_PAX_FMP_W { w: self }
    }
    #[doc = "Bit 21 - FM+ driving capability activation for I2C2"]
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W {
        I2C2_FMP_W { w: self }
    }
    #[doc = "Bit 20 - FM+ driving capability activation for I2C1"]
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W {
        I2C1_FMP_W { w: self }
    }
    #[doc = "Bits 16:19 - Fast Mode Plus (FM+) driving capability activation bits"]
    #[inline(always)]
    pub fn i2c_pbx_fmp(&mut self) -> I2C_PBX_FMP_W {
        I2C_PBX_FMP_W { w: self }
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W {
        BOOSTEN_W { w: self }
    }
    #[doc = "Bits 6:7 - IR Modulation Envelope signal selection."]
    #[inline(always)]
    pub fn ir_mod(&mut self) -> IR_MOD_W {
        IR_MOD_W { w: self }
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    pub fn ir_pol(&mut self) -> IR_POL_W {
        IR_POL_W { w: self }
    }
    #[doc = "Bit 4 - PA11 and PA12 remapping bit."]
    #[inline(always)]
    pub fn pa11_pa12_rmp(&mut self) -> PA11_PA12_RMP_W {
        PA11_PA12_RMP_W { w: self }
    }
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W {
        MEM_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
