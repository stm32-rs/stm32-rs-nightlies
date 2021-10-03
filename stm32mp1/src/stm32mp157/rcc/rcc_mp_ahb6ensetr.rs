#[doc = "Register `RCC_MP_AHB6ENSETR` reader"]
pub struct R(crate::R<RCC_MP_AHB6ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_AHB6ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_AHB6ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_AHB6ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_AHB6ENSETR` writer"]
pub struct W(crate::W<RCC_MP_AHB6ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_AHB6ENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_AHB6ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_AHB6ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDMAEN` reader - MDMAEN"]
pub struct MDMAEN_R(crate::FieldReader<bool, bool>);
impl MDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDMAEN` writer - MDMAEN"]
pub struct MDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMAEN_W<'a> {
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
#[doc = "Field `GPUEN` reader - GPUEN"]
pub struct GPUEN_R(crate::FieldReader<bool, bool>);
impl GPUEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPUEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPUEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPUEN` writer - GPUEN"]
pub struct GPUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPUEN_W<'a> {
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
#[doc = "Field `ETHCKEN` reader - ETHCKEN"]
pub struct ETHCKEN_R(crate::FieldReader<bool, bool>);
impl ETHCKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETHCKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHCKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHCKEN` writer - ETHCKEN"]
pub struct ETHCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHCKEN_W<'a> {
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
#[doc = "Field `ETHTXEN` reader - ETHTXEN"]
pub struct ETHTXEN_R(crate::FieldReader<bool, bool>);
impl ETHTXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETHTXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHTXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHTXEN` writer - ETHTXEN"]
pub struct ETHTXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHTXEN_W<'a> {
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
#[doc = "Field `ETHRXEN` reader - ETHRXEN"]
pub struct ETHRXEN_R(crate::FieldReader<bool, bool>);
impl ETHRXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETHRXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHRXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHRXEN` writer - ETHRXEN"]
pub struct ETHRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHRXEN_W<'a> {
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
#[doc = "Field `ETHMACEN` reader - ETHMACEN"]
pub struct ETHMACEN_R(crate::FieldReader<bool, bool>);
impl ETHMACEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETHMACEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETHMACEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETHMACEN` writer - ETHMACEN"]
pub struct ETHMACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHMACEN_W<'a> {
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
#[doc = "Field `FMCEN` reader - FMCEN"]
pub struct FMCEN_R(crate::FieldReader<bool, bool>);
impl FMCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FMCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMCEN` writer - FMCEN"]
pub struct FMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCEN_W<'a> {
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
#[doc = "Field `QSPIEN` reader - QSPIEN"]
pub struct QSPIEN_R(crate::FieldReader<bool, bool>);
impl QSPIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        QSPIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QSPIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPIEN` writer - QSPIEN"]
pub struct QSPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIEN_W<'a> {
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
#[doc = "Field `SDMMC1EN` reader - SDMMC1EN"]
pub struct SDMMC1EN_R(crate::FieldReader<bool, bool>);
impl SDMMC1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC1EN` writer - SDMMC1EN"]
pub struct SDMMC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1EN_W<'a> {
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
#[doc = "Field `SDMMC2EN` reader - SDMMC2EN"]
pub struct SDMMC2EN_R(crate::FieldReader<bool, bool>);
impl SDMMC2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC2EN` writer - SDMMC2EN"]
pub struct SDMMC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2EN_W<'a> {
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
#[doc = "Field `CRC1EN` reader - CRC1EN"]
pub struct CRC1EN_R(crate::FieldReader<bool, bool>);
impl CRC1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC1EN` writer - CRC1EN"]
pub struct CRC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC1EN_W<'a> {
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
#[doc = "Field `USBHEN` reader - USBHEN"]
pub struct USBHEN_R(crate::FieldReader<bool, bool>);
impl USBHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBHEN` writer - USBHEN"]
pub struct USBHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHEN_W<'a> {
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
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPUEN"]
    #[inline(always)]
    pub fn gpuen(&self) -> GPUEN_R {
        GPUEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ETHCKEN"]
    #[inline(always)]
    pub fn ethcken(&self) -> ETHCKEN_R {
        ETHCKEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ETHTXEN"]
    #[inline(always)]
    pub fn ethtxen(&self) -> ETHTXEN_R {
        ETHTXEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ETHRXEN"]
    #[inline(always)]
    pub fn ethrxen(&self) -> ETHRXEN_R {
        ETHRXEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ETHMACEN"]
    #[inline(always)]
    pub fn ethmacen(&self) -> ETHMACEN_R {
        ETHMACEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - QSPIEN"]
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1EN"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SDMMC2EN"]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CRC1EN"]
    #[inline(always)]
    pub fn crc1en(&self) -> CRC1EN_R {
        CRC1EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - USBHEN"]
    #[inline(always)]
    pub fn usbhen(&self) -> USBHEN_R {
        USBHEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&mut self) -> MDMAEN_W {
        MDMAEN_W { w: self }
    }
    #[doc = "Bit 5 - GPUEN"]
    #[inline(always)]
    pub fn gpuen(&mut self) -> GPUEN_W {
        GPUEN_W { w: self }
    }
    #[doc = "Bit 7 - ETHCKEN"]
    #[inline(always)]
    pub fn ethcken(&mut self) -> ETHCKEN_W {
        ETHCKEN_W { w: self }
    }
    #[doc = "Bit 8 - ETHTXEN"]
    #[inline(always)]
    pub fn ethtxen(&mut self) -> ETHTXEN_W {
        ETHTXEN_W { w: self }
    }
    #[doc = "Bit 9 - ETHRXEN"]
    #[inline(always)]
    pub fn ethrxen(&mut self) -> ETHRXEN_W {
        ETHRXEN_W { w: self }
    }
    #[doc = "Bit 10 - ETHMACEN"]
    #[inline(always)]
    pub fn ethmacen(&mut self) -> ETHMACEN_W {
        ETHMACEN_W { w: self }
    }
    #[doc = "Bit 12 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W {
        FMCEN_W { w: self }
    }
    #[doc = "Bit 14 - QSPIEN"]
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W {
        QSPIEN_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1EN"]
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W {
        SDMMC1EN_W { w: self }
    }
    #[doc = "Bit 17 - SDMMC2EN"]
    #[inline(always)]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W {
        SDMMC2EN_W { w: self }
    }
    #[doc = "Bit 20 - CRC1EN"]
    #[inline(always)]
    pub fn crc1en(&mut self) -> CRC1EN_W {
        CRC1EN_W { w: self }
    }
    #[doc = "Bit 24 - USBHEN"]
    #[inline(always)]
    pub fn usbhen(&mut self) -> USBHEN_W {
        USBHEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_ahb6ensetr](index.html) module"]
pub struct RCC_MP_AHB6ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_AHB6ENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_ahb6ensetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_AHB6ENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_ahb6ensetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_AHB6ENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_AHB6ENSETR to value 0"]
impl crate::Resettable for RCC_MP_AHB6ENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
