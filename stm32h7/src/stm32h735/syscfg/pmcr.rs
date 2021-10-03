#[doc = "Register `PMCR` reader"]
pub struct R(crate::R<PMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMCR` writer"]
pub struct W(crate::W<PMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCR_SPEC>;
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
impl From<crate::W<PMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C1FMP` reader - I2C1 Fm+"]
pub struct I2C1FMP_R(crate::FieldReader<bool, bool>);
impl I2C1FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1FMP` writer - I2C1 Fm+"]
pub struct I2C1FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1FMP_W<'a> {
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
#[doc = "Field `I2C2FMP` reader - I2C2 Fm+"]
pub struct I2C2FMP_R(crate::FieldReader<bool, bool>);
impl I2C2FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2FMP` writer - I2C2 Fm+"]
pub struct I2C2FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2FMP_W<'a> {
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
#[doc = "Field `I2C3FMP` reader - I2C3 Fm+"]
pub struct I2C3FMP_R(crate::FieldReader<bool, bool>);
impl I2C3FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3FMP` writer - I2C3 Fm+"]
pub struct I2C3FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3FMP_W<'a> {
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
#[doc = "Field `I2C4FMP` reader - I2C4 Fm+"]
pub struct I2C4FMP_R(crate::FieldReader<bool, bool>);
impl I2C4FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C4FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4FMP` writer - I2C4 Fm+"]
pub struct I2C4FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4FMP_W<'a> {
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
#[doc = "Field `PB6FMP` reader - PB(6) Fm+"]
pub struct PB6FMP_R(crate::FieldReader<bool, bool>);
impl PB6FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB6FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB6FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB6FMP` writer - PB(6) Fm+"]
pub struct PB6FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB6FMP_W<'a> {
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
#[doc = "Field `PB7FMP` reader - PB(7) Fast Mode Plus"]
pub struct PB7FMP_R(crate::FieldReader<bool, bool>);
impl PB7FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB7FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB7FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB7FMP` writer - PB(7) Fast Mode Plus"]
pub struct PB7FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB7FMP_W<'a> {
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
#[doc = "Field `PB8FMP` reader - PB(8) Fast Mode Plus"]
pub struct PB8FMP_R(crate::FieldReader<bool, bool>);
impl PB8FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB8FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB8FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB8FMP` writer - PB(8) Fast Mode Plus"]
pub struct PB8FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB8FMP_W<'a> {
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
#[doc = "Field `PB9FMP` reader - PB(9) Fm+"]
pub struct PB9FMP_R(crate::FieldReader<bool, bool>);
impl PB9FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PB9FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PB9FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PB9FMP` writer - PB(9) Fm+"]
pub struct PB9FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB9FMP_W<'a> {
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
#[doc = "Field `BOOSTE` reader - Booster Enable"]
pub struct BOOSTE_R(crate::FieldReader<bool, bool>);
impl BOOSTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOSTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOSTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOSTE` writer - Booster Enable"]
pub struct BOOSTE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOSTE_W<'a> {
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
#[doc = "Field `EPIS` reader - Ethernet PHY Interface Selection"]
pub struct EPIS_R(crate::FieldReader<u8, u8>);
impl EPIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPIS` writer - Ethernet PHY Interface Selection"]
pub struct EPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `PA0SO` reader - PA0 Switch Open"]
pub struct PA0SO_R(crate::FieldReader<bool, bool>);
impl PA0SO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA0SO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA0SO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA0SO` writer - PA0 Switch Open"]
pub struct PA0SO_W<'a> {
    w: &'a mut W,
}
impl<'a> PA0SO_W<'a> {
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
#[doc = "Field `PA1SO` reader - PA1 Switch Open"]
pub struct PA1SO_R(crate::FieldReader<bool, bool>);
impl PA1SO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PA1SO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA1SO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA1SO` writer - PA1 Switch Open"]
pub struct PA1SO_W<'a> {
    w: &'a mut W,
}
impl<'a> PA1SO_W<'a> {
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
#[doc = "Field `PC2SO` reader - PC2 Switch Open"]
pub struct PC2SO_R(crate::FieldReader<bool, bool>);
impl PC2SO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PC2SO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC2SO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC2SO` writer - PC2 Switch Open"]
pub struct PC2SO_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2SO_W<'a> {
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
#[doc = "Field `PC3SO` reader - PC3 Switch Open"]
pub struct PC3SO_R(crate::FieldReader<bool, bool>);
impl PC3SO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PC3SO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC3SO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC3SO` writer - PC3 Switch Open"]
pub struct PC3SO_W<'a> {
    w: &'a mut W,
}
impl<'a> PC3SO_W<'a> {
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
#[doc = "Field `BOOSTVDDSEL` reader - Analog switch supply voltage selection"]
pub struct BOOSTVDDSEL_R(crate::FieldReader<bool, bool>);
impl BOOSTVDDSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOSTVDDSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOSTVDDSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOSTVDDSEL` writer - Analog switch supply voltage selection"]
pub struct BOOSTVDDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOSTVDDSEL_W<'a> {
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
#[doc = "Field `I2C5FMP` reader - I2C5 Fm+"]
pub struct I2C5FMP_R(crate::FieldReader<bool, bool>);
impl I2C5FMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C5FMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C5FMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C5FMP` writer - I2C5 Fm+"]
pub struct I2C5FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C5FMP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C1 Fm+"]
    #[inline(always)]
    pub fn i2c1fmp(&self) -> I2C1FMP_R {
        I2C1FMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C2 Fm+"]
    #[inline(always)]
    pub fn i2c2fmp(&self) -> I2C2FMP_R {
        I2C2FMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C3 Fm+"]
    #[inline(always)]
    pub fn i2c3fmp(&self) -> I2C3FMP_R {
        I2C3FMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C4 Fm+"]
    #[inline(always)]
    pub fn i2c4fmp(&self) -> I2C4FMP_R {
        I2C4FMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PB(6) Fm+"]
    #[inline(always)]
    pub fn pb6fmp(&self) -> PB6FMP_R {
        PB6FMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PB(7) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb7fmp(&self) -> PB7FMP_R {
        PB7FMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PB(8) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb8fmp(&self) -> PB8FMP_R {
        PB8FMP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PB(9) Fm+"]
    #[inline(always)]
    pub fn pb9fmp(&self) -> PB9FMP_R {
        PB9FMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Booster Enable"]
    #[inline(always)]
    pub fn booste(&self) -> BOOSTE_R {
        BOOSTE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 21:23 - Ethernet PHY Interface Selection"]
    #[inline(always)]
    pub fn epis(&self) -> EPIS_R {
        EPIS_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 24 - PA0 Switch Open"]
    #[inline(always)]
    pub fn pa0so(&self) -> PA0SO_R {
        PA0SO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PA1 Switch Open"]
    #[inline(always)]
    pub fn pa1so(&self) -> PA1SO_R {
        PA1SO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PC2 Switch Open"]
    #[inline(always)]
    pub fn pc2so(&self) -> PC2SO_R {
        PC2SO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PC3 Switch Open"]
    #[inline(always)]
    pub fn pc3so(&self) -> PC3SO_R {
        PC3SO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Analog switch supply voltage selection"]
    #[inline(always)]
    pub fn boostvddsel(&self) -> BOOSTVDDSEL_R {
        BOOSTVDDSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - I2C5 Fm+"]
    #[inline(always)]
    pub fn i2c5fmp(&self) -> I2C5FMP_R {
        I2C5FMP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1 Fm+"]
    #[inline(always)]
    pub fn i2c1fmp(&mut self) -> I2C1FMP_W {
        I2C1FMP_W { w: self }
    }
    #[doc = "Bit 1 - I2C2 Fm+"]
    #[inline(always)]
    pub fn i2c2fmp(&mut self) -> I2C2FMP_W {
        I2C2FMP_W { w: self }
    }
    #[doc = "Bit 2 - I2C3 Fm+"]
    #[inline(always)]
    pub fn i2c3fmp(&mut self) -> I2C3FMP_W {
        I2C3FMP_W { w: self }
    }
    #[doc = "Bit 3 - I2C4 Fm+"]
    #[inline(always)]
    pub fn i2c4fmp(&mut self) -> I2C4FMP_W {
        I2C4FMP_W { w: self }
    }
    #[doc = "Bit 4 - PB(6) Fm+"]
    #[inline(always)]
    pub fn pb6fmp(&mut self) -> PB6FMP_W {
        PB6FMP_W { w: self }
    }
    #[doc = "Bit 5 - PB(7) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb7fmp(&mut self) -> PB7FMP_W {
        PB7FMP_W { w: self }
    }
    #[doc = "Bit 6 - PB(8) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb8fmp(&mut self) -> PB8FMP_W {
        PB8FMP_W { w: self }
    }
    #[doc = "Bit 7 - PB(9) Fm+"]
    #[inline(always)]
    pub fn pb9fmp(&mut self) -> PB9FMP_W {
        PB9FMP_W { w: self }
    }
    #[doc = "Bit 8 - Booster Enable"]
    #[inline(always)]
    pub fn booste(&mut self) -> BOOSTE_W {
        BOOSTE_W { w: self }
    }
    #[doc = "Bits 21:23 - Ethernet PHY Interface Selection"]
    #[inline(always)]
    pub fn epis(&mut self) -> EPIS_W {
        EPIS_W { w: self }
    }
    #[doc = "Bit 24 - PA0 Switch Open"]
    #[inline(always)]
    pub fn pa0so(&mut self) -> PA0SO_W {
        PA0SO_W { w: self }
    }
    #[doc = "Bit 25 - PA1 Switch Open"]
    #[inline(always)]
    pub fn pa1so(&mut self) -> PA1SO_W {
        PA1SO_W { w: self }
    }
    #[doc = "Bit 26 - PC2 Switch Open"]
    #[inline(always)]
    pub fn pc2so(&mut self) -> PC2SO_W {
        PC2SO_W { w: self }
    }
    #[doc = "Bit 27 - PC3 Switch Open"]
    #[inline(always)]
    pub fn pc3so(&mut self) -> PC3SO_W {
        PC3SO_W { w: self }
    }
    #[doc = "Bit 9 - Analog switch supply voltage selection"]
    #[inline(always)]
    pub fn boostvddsel(&mut self) -> BOOSTVDDSEL_W {
        BOOSTVDDSEL_W { w: self }
    }
    #[doc = "Bit 10 - I2C5 Fm+"]
    #[inline(always)]
    pub fn i2c5fmp(&mut self) -> I2C5FMP_W {
        I2C5FMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmcr](index.html) module"]
pub struct PMCR_SPEC;
impl crate::RegisterSpec for PMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmcr::R](R) reader structure"]
impl crate::Readable for PMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmcr::W](W) writer structure"]
impl crate::Writable for PMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMCR to value 0"]
impl crate::Resettable for PMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
