#[doc = "Register `RCC_MC_AHB6LPENSETR` reader"]
pub struct R(crate::R<RCC_MC_AHB6LPENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AHB6LPENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AHB6LPENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AHB6LPENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MC_AHB6LPENSETR` writer"]
pub struct W(crate::W<RCC_MC_AHB6LPENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AHB6LPENSETR_SPEC>;
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
impl From<crate::W<RCC_MC_AHB6LPENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AHB6LPENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDMALPEN` reader - MDMALPEN"]
pub struct MDMALPEN_R(crate::FieldReader<bool, bool>);
impl MDMALPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMALPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDMALPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMALPEN` writer - MDMALPEN"]
pub struct MDMALPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMALPEN_W<'a> {
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
#[doc = "Field `GPULPEN` reader - GPULPEN"]
pub struct GPULPEN_R(crate::FieldReader<bool, bool>);
impl GPULPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPULPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPULPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPULPEN` writer - GPULPEN"]
pub struct GPULPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPULPEN_W<'a> {
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
#[doc = "Field `ETHCKLPEN` reader - ETHCKLPEN"]
pub struct ETHCKLPEN_R(crate::FieldReader<bool, bool>);
impl ETHCKLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETHCKLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHCKLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHCKLPEN` writer - ETHCKLPEN"]
pub struct ETHCKLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHCKLPEN_W<'a> {
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
#[doc = "Field `ETHTXLPEN` reader - ETHTXLPEN"]
pub struct ETHTXLPEN_R(crate::FieldReader<bool, bool>);
impl ETHTXLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETHTXLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHTXLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHTXLPEN` writer - ETHTXLPEN"]
pub struct ETHTXLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHTXLPEN_W<'a> {
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
#[doc = "Field `ETHRXLPEN` reader - ETHRXLPEN"]
pub struct ETHRXLPEN_R(crate::FieldReader<bool, bool>);
impl ETHRXLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETHRXLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHRXLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHRXLPEN` writer - ETHRXLPEN"]
pub struct ETHRXLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHRXLPEN_W<'a> {
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
#[doc = "Field `ETHMACLPEN` reader - ETHMACLPEN"]
pub struct ETHMACLPEN_R(crate::FieldReader<bool, bool>);
impl ETHMACLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETHMACLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHMACLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHMACLPEN` writer - ETHMACLPEN"]
pub struct ETHMACLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHMACLPEN_W<'a> {
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
#[doc = "Field `ETHSTPEN` reader - ETHSTPEN"]
pub struct ETHSTPEN_R(crate::FieldReader<bool, bool>);
impl ETHSTPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETHSTPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHSTPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHSTPEN` writer - ETHSTPEN"]
pub struct ETHSTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHSTPEN_W<'a> {
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
#[doc = "Field `FMCLPEN` reader - FMCLPEN"]
pub struct FMCLPEN_R(crate::FieldReader<bool, bool>);
impl FMCLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMCLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMCLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMCLPEN` writer - FMCLPEN"]
pub struct FMCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCLPEN_W<'a> {
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
#[doc = "Field `QSPILPEN` reader - QSPILPEN"]
pub struct QSPILPEN_R(crate::FieldReader<bool, bool>);
impl QSPILPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        QSPILPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QSPILPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPILPEN` writer - QSPILPEN"]
pub struct QSPILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPILPEN_W<'a> {
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
#[doc = "Field `SDMMC1LPEN` reader - SDMMC1LPEN"]
pub struct SDMMC1LPEN_R(crate::FieldReader<bool, bool>);
impl SDMMC1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC1LPEN` writer - SDMMC1LPEN"]
pub struct SDMMC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1LPEN_W<'a> {
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
#[doc = "Field `SDMMC2LPEN` reader - SDMMC2LPEN"]
pub struct SDMMC2LPEN_R(crate::FieldReader<bool, bool>);
impl SDMMC2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC2LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC2LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC2LPEN` writer - SDMMC2LPEN"]
pub struct SDMMC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2LPEN_W<'a> {
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
#[doc = "Field `CRC1LPEN` reader - CRC1LPEN"]
pub struct CRC1LPEN_R(crate::FieldReader<bool, bool>);
impl CRC1LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC1LPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC1LPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC1LPEN` writer - CRC1LPEN"]
pub struct CRC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC1LPEN_W<'a> {
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
#[doc = "Field `USBHLPEN` reader - USBHLPEN"]
pub struct USBHLPEN_R(crate::FieldReader<bool, bool>);
impl USBHLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBHLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBHLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBHLPEN` writer - USBHLPEN"]
pub struct USBHLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHLPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPULPEN"]
    #[inline(always)]
    pub fn gpulpen(&self) -> GPULPEN_R {
        GPULPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ETHCKLPEN"]
    #[inline(always)]
    pub fn ethcklpen(&self) -> ETHCKLPEN_R {
        ETHCKLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ETHTXLPEN"]
    #[inline(always)]
    pub fn ethtxlpen(&self) -> ETHTXLPEN_R {
        ETHTXLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ETHRXLPEN"]
    #[inline(always)]
    pub fn ethrxlpen(&self) -> ETHRXLPEN_R {
        ETHRXLPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ETHMACLPEN"]
    #[inline(always)]
    pub fn ethmaclpen(&self) -> ETHMACLPEN_R {
        ETHMACLPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ETHSTPEN"]
    #[inline(always)]
    pub fn ethstpen(&self) -> ETHSTPEN_R {
        ETHSTPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FMCLPEN"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - QSPILPEN"]
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1LPEN"]
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SDMMC2LPEN"]
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CRC1LPEN"]
    #[inline(always)]
    pub fn crc1lpen(&self) -> CRC1LPEN_R {
        CRC1LPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - USBHLPEN"]
    #[inline(always)]
    pub fn usbhlpen(&self) -> USBHLPEN_R {
        USBHLPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W {
        MDMALPEN_W { w: self }
    }
    #[doc = "Bit 5 - GPULPEN"]
    #[inline(always)]
    pub fn gpulpen(&mut self) -> GPULPEN_W {
        GPULPEN_W { w: self }
    }
    #[doc = "Bit 7 - ETHCKLPEN"]
    #[inline(always)]
    pub fn ethcklpen(&mut self) -> ETHCKLPEN_W {
        ETHCKLPEN_W { w: self }
    }
    #[doc = "Bit 8 - ETHTXLPEN"]
    #[inline(always)]
    pub fn ethtxlpen(&mut self) -> ETHTXLPEN_W {
        ETHTXLPEN_W { w: self }
    }
    #[doc = "Bit 9 - ETHRXLPEN"]
    #[inline(always)]
    pub fn ethrxlpen(&mut self) -> ETHRXLPEN_W {
        ETHRXLPEN_W { w: self }
    }
    #[doc = "Bit 10 - ETHMACLPEN"]
    #[inline(always)]
    pub fn ethmaclpen(&mut self) -> ETHMACLPEN_W {
        ETHMACLPEN_W { w: self }
    }
    #[doc = "Bit 11 - ETHSTPEN"]
    #[inline(always)]
    pub fn ethstpen(&mut self) -> ETHSTPEN_W {
        ETHSTPEN_W { w: self }
    }
    #[doc = "Bit 12 - FMCLPEN"]
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W {
        FMCLPEN_W { w: self }
    }
    #[doc = "Bit 14 - QSPILPEN"]
    #[inline(always)]
    pub fn qspilpen(&mut self) -> QSPILPEN_W {
        QSPILPEN_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1LPEN"]
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W {
        SDMMC1LPEN_W { w: self }
    }
    #[doc = "Bit 17 - SDMMC2LPEN"]
    #[inline(always)]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W {
        SDMMC2LPEN_W { w: self }
    }
    #[doc = "Bit 20 - CRC1LPEN"]
    #[inline(always)]
    pub fn crc1lpen(&mut self) -> CRC1LPEN_W {
        CRC1LPEN_W { w: self }
    }
    #[doc = "Bit 24 - USBHLPEN"]
    #[inline(always)]
    pub fn usbhlpen(&mut self) -> USBHLPEN_W {
        USBHLPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to set the PERxLPEN bit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mc_ahb6lpensetr](index.html) module"]
pub struct RCC_MC_AHB6LPENSETR_SPEC;
impl crate::RegisterSpec for RCC_MC_AHB6LPENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mc_ahb6lpensetr::R](R) reader structure"]
impl crate::Readable for RCC_MC_AHB6LPENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mc_ahb6lpensetr::W](W) writer structure"]
impl crate::Writable for RCC_MC_AHB6LPENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MC_AHB6LPENSETR to value 0x0113_57a1"]
impl crate::Resettable for RCC_MC_AHB6LPENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0113_57a1
    }
}
