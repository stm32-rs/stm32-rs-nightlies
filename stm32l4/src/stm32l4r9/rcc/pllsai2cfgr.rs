#[doc = "Register `PLLSAI2CFGR` reader"]
pub struct R(crate::R<PLLSAI2CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSAI2CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSAI2CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSAI2CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLSAI2CFGR` writer"]
pub struct W(crate::W<PLLSAI2CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLSAI2CFGR_SPEC>;
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
impl From<crate::W<PLLSAI2CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLSAI2CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSAI2PDIV` reader - PLLSAI2 division factor for PLLSAI2CLK"]
pub struct PLLSAI2PDIV_R(crate::FieldReader<u8, u8>);
impl PLLSAI2PDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSAI2PDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2PDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2PDIV` writer - PLLSAI2 division factor for PLLSAI2CLK"]
pub struct PLLSAI2PDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2PDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
#[doc = "Field `PLLSAI2R` reader - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
pub struct PLLSAI2R_R(crate::FieldReader<u8, u8>);
impl PLLSAI2R_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSAI2R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2R` writer - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
pub struct PLLSAI2R_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `PLLSAI2REN` reader - PLLSAI2 PLLADC2CLK output enable"]
pub struct PLLSAI2REN_R(crate::FieldReader<bool, bool>);
impl PLLSAI2REN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSAI2REN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2REN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2REN` writer - PLLSAI2 PLLADC2CLK output enable"]
pub struct PLLSAI2REN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2REN_W<'a> {
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
#[doc = "Field `PLLSAI2Q` reader - SAI2PLL PLLSAI2CLK output enable"]
pub struct PLLSAI2Q_R(crate::FieldReader<u8, u8>);
impl PLLSAI2Q_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSAI2Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2Q_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2Q` writer - SAI2PLL PLLSAI2CLK output enable"]
pub struct PLLSAI2Q_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `PLLSAI2QEN` reader - PLLSAI2 division factor for PLLDISCLK"]
pub struct PLLSAI2QEN_R(crate::FieldReader<bool, bool>);
impl PLLSAI2QEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSAI2QEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2QEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2QEN` writer - PLLSAI2 division factor for PLLDISCLK"]
pub struct PLLSAI2QEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2QEN_W<'a> {
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
#[doc = "Field `PLLSAI2P` reader - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
pub struct PLLSAI2P_R(crate::FieldReader<bool, bool>);
impl PLLSAI2P_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSAI2P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2P_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2P` writer - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
pub struct PLLSAI2P_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2P_W<'a> {
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
#[doc = "Field `PLLSAI2PEN` reader - SAI2PLL PLLSAI2CLK output enable"]
pub struct PLLSAI2PEN_R(crate::FieldReader<bool, bool>);
impl PLLSAI2PEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSAI2PEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2PEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2PEN` writer - SAI2PLL PLLSAI2CLK output enable"]
pub struct PLLSAI2PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2PEN_W<'a> {
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
#[doc = "Field `PLLSAI2N` reader - SAI2PLL multiplication factor for VCO"]
pub struct PLLSAI2N_R(crate::FieldReader<u8, u8>);
impl PLLSAI2N_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSAI2N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2N` writer - SAI2PLL multiplication factor for VCO"]
pub struct PLLSAI2N_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `PLLSAI2M` reader - Division factor for PLLSAI2 input clock"]
pub struct PLLSAI2M_R(crate::FieldReader<u8, u8>);
impl PLLSAI2M_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSAI2M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAI2M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAI2M` writer - Division factor for PLLSAI2 input clock"]
pub struct PLLSAI2M_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAI2M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllsai2pdiv(&self) -> PLLSAI2PDIV_R {
        PLLSAI2PDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai2r(&self) -> PLLSAI2R_R {
        PLLSAI2R_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2ren(&self) -> PLLSAI2REN_R {
        PLLSAI2REN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2q(&self) -> PLLSAI2Q_R {
        PLLSAI2Q_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - PLLSAI2 division factor for PLLDISCLK"]
    #[inline(always)]
    pub fn pllsai2qen(&self) -> PLLSAI2QEN_R {
        PLLSAI2QEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai2p(&self) -> PLLSAI2P_R {
        PLLSAI2P_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2pen(&self) -> PLLSAI2PEN_R {
        PLLSAI2PEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai2n(&self) -> PLLSAI2N_R {
        PLLSAI2N_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 4:7 - Division factor for PLLSAI2 input clock"]
    #[inline(always)]
    pub fn pllsai2m(&self) -> PLLSAI2M_R {
        PLLSAI2M_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31 - PLLSAI2 division factor for PLLSAI2CLK"]
    #[inline(always)]
    pub fn pllsai2pdiv(&mut self) -> PLLSAI2PDIV_W {
        PLLSAI2PDIV_W { w: self }
    }
    #[doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)"]
    #[inline(always)]
    pub fn pllsai2r(&mut self) -> PLLSAI2R_W {
        PLLSAI2R_W { w: self }
    }
    #[doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2ren(&mut self) -> PLLSAI2REN_W {
        PLLSAI2REN_W { w: self }
    }
    #[doc = "Bits 21:22 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2q(&mut self) -> PLLSAI2Q_W {
        PLLSAI2Q_W { w: self }
    }
    #[doc = "Bit 20 - PLLSAI2 division factor for PLLDISCLK"]
    #[inline(always)]
    pub fn pllsai2qen(&mut self) -> PLLSAI2QEN_W {
        PLLSAI2QEN_W { w: self }
    }
    #[doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)"]
    #[inline(always)]
    pub fn pllsai2p(&mut self) -> PLLSAI2P_W {
        PLLSAI2P_W { w: self }
    }
    #[doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable"]
    #[inline(always)]
    pub fn pllsai2pen(&mut self) -> PLLSAI2PEN_W {
        PLLSAI2PEN_W { w: self }
    }
    #[doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO"]
    #[inline(always)]
    pub fn pllsai2n(&mut self) -> PLLSAI2N_W {
        PLLSAI2N_W { w: self }
    }
    #[doc = "Bits 4:7 - Division factor for PLLSAI2 input clock"]
    #[inline(always)]
    pub fn pllsai2m(&mut self) -> PLLSAI2M_W {
        PLLSAI2M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLSAI2 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllsai2cfgr](index.html) module"]
pub struct PLLSAI2CFGR_SPEC;
impl crate::RegisterSpec for PLLSAI2CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllsai2cfgr::R](R) reader structure"]
impl crate::Readable for PLLSAI2CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllsai2cfgr::W](W) writer structure"]
impl crate::Writable for PLLSAI2CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLSAI2CFGR to value 0x1000"]
impl crate::Resettable for PLLSAI2CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
