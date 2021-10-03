#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EOCALIE` reader - End of calibration interrupt enable"]
pub struct EOCALIE_R(crate::FieldReader<bool, bool>);
impl EOCALIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOCALIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOCALIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOCALIE` writer - End of calibration interrupt enable"]
pub struct EOCALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCALIE_W<'a> {
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
#[doc = "Field `JEOCIE` reader - Injected end of conversion interrupt enable"]
pub struct JEOCIE_R(crate::FieldReader<bool, bool>);
impl JEOCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEOCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEOCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEOCIE` writer - Injected end of conversion interrupt enable"]
pub struct JEOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JEOCIE_W<'a> {
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
#[doc = "Field `JOVRIE` reader - Injected data overrun interrupt enable"]
pub struct JOVRIE_R(crate::FieldReader<bool, bool>);
impl JOVRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JOVRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JOVRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JOVRIE` writer - Injected data overrun interrupt enable"]
pub struct JOVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JOVRIE_W<'a> {
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
#[doc = "Field `REOCIE` reader - Regular end of conversion interrupt enable"]
pub struct REOCIE_R(crate::FieldReader<bool, bool>);
impl REOCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REOCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REOCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REOCIE` writer - Regular end of conversion interrupt enable"]
pub struct REOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> REOCIE_W<'a> {
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
#[doc = "Field `ROVRIE` reader - Regular data overrun interrupt enable"]
pub struct ROVRIE_R(crate::FieldReader<bool, bool>);
impl ROVRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROVRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROVRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROVRIE` writer - Regular data overrun interrupt enable"]
pub struct ROVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVRIE_W<'a> {
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
#[doc = "Field `REFV` reader - Reference voltage selection"]
pub struct REFV_R(crate::FieldReader<u8, u8>);
impl REFV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFV` writer - Reference voltage selection"]
pub struct REFV_W<'a> {
    w: &'a mut W,
}
impl<'a> REFV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `SLOWCK` reader - Slow clock mode enable"]
pub struct SLOWCK_R(crate::FieldReader<bool, bool>);
impl SLOWCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOWCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOWCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOWCK` writer - Slow clock mode enable"]
pub struct SLOWCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOWCK_W<'a> {
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
#[doc = "Field `SBI` reader - Enter Standby mode when idle"]
pub struct SBI_R(crate::FieldReader<bool, bool>);
impl SBI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBI` writer - Enter Standby mode when idle"]
pub struct SBI_W<'a> {
    w: &'a mut W,
}
impl<'a> SBI_W<'a> {
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
#[doc = "Field `PDI` reader - Enter power down mode when idle"]
pub struct PDI_R(crate::FieldReader<bool, bool>);
impl PDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDI` writer - Enter power down mode when idle"]
pub struct PDI_W<'a> {
    w: &'a mut W,
}
impl<'a> PDI_W<'a> {
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
#[doc = "Field `JSYNC` reader - Launch a injected conversion synchronously with SDADC1"]
pub struct JSYNC_R(crate::FieldReader<bool, bool>);
impl JSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        JSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JSYNC` writer - Launch a injected conversion synchronously with SDADC1"]
pub struct JSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> JSYNC_W<'a> {
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
#[doc = "Field `RSYNC` reader - Launch regular conversion synchronously with SDADC1"]
pub struct RSYNC_R(crate::FieldReader<bool, bool>);
impl RSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSYNC` writer - Launch regular conversion synchronously with SDADC1"]
pub struct RSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSYNC_W<'a> {
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
#[doc = "Field `JDMAEN` reader - DMA channel enabled to read data for the injected channel group"]
pub struct JDMAEN_R(crate::FieldReader<bool, bool>);
impl JDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        JDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JDMAEN` writer - DMA channel enabled to read data for the injected channel group"]
pub struct JDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JDMAEN_W<'a> {
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
#[doc = "Field `RDMAEN` reader - DMA channel enabled to read data for the regular channel"]
pub struct RDMAEN_R(crate::FieldReader<bool, bool>);
impl RDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDMAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDMAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDMAEN` writer - DMA channel enabled to read data for the regular channel"]
pub struct RDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMAEN_W<'a> {
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
#[doc = "Field `INIT` reader - Initialization mode request"]
pub struct INIT_R(crate::FieldReader<bool, bool>);
impl INIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT` writer - Initialization mode request"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - End of calibration interrupt enable"]
    #[inline(always)]
    pub fn eocalie(&self) -> EOCALIE_R {
        EOCALIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Regular data overrun interrupt enable"]
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Reference voltage selection"]
    #[inline(always)]
    pub fn refv(&self) -> REFV_R {
        REFV_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Slow clock mode enable"]
    #[inline(always)]
    pub fn slowck(&self) -> SLOWCK_R {
        SLOWCK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enter Standby mode when idle"]
    #[inline(always)]
    pub fn sbi(&self) -> SBI_R {
        SBI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enter power down mode when idle"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Launch a injected conversion synchronously with SDADC1"]
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Launch regular conversion synchronously with SDADC1"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DMA channel enabled to read data for the regular channel"]
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Initialization mode request"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of calibration interrupt enable"]
    #[inline(always)]
    pub fn eocalie(&mut self) -> EOCALIE_W {
        EOCALIE_W { w: self }
    }
    #[doc = "Bit 1 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W {
        JEOCIE_W { w: self }
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    pub fn jovrie(&mut self) -> JOVRIE_W {
        JOVRIE_W { w: self }
    }
    #[doc = "Bit 3 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    pub fn reocie(&mut self) -> REOCIE_W {
        REOCIE_W { w: self }
    }
    #[doc = "Bit 4 - Regular data overrun interrupt enable"]
    #[inline(always)]
    pub fn rovrie(&mut self) -> ROVRIE_W {
        ROVRIE_W { w: self }
    }
    #[doc = "Bits 8:9 - Reference voltage selection"]
    #[inline(always)]
    pub fn refv(&mut self) -> REFV_W {
        REFV_W { w: self }
    }
    #[doc = "Bit 10 - Slow clock mode enable"]
    #[inline(always)]
    pub fn slowck(&mut self) -> SLOWCK_W {
        SLOWCK_W { w: self }
    }
    #[doc = "Bit 11 - Enter Standby mode when idle"]
    #[inline(always)]
    pub fn sbi(&mut self) -> SBI_W {
        SBI_W { w: self }
    }
    #[doc = "Bit 12 - Enter power down mode when idle"]
    #[inline(always)]
    pub fn pdi(&mut self) -> PDI_W {
        PDI_W { w: self }
    }
    #[doc = "Bit 14 - Launch a injected conversion synchronously with SDADC1"]
    #[inline(always)]
    pub fn jsync(&mut self) -> JSYNC_W {
        JSYNC_W { w: self }
    }
    #[doc = "Bit 15 - Launch regular conversion synchronously with SDADC1"]
    #[inline(always)]
    pub fn rsync(&mut self) -> RSYNC_W {
        RSYNC_W { w: self }
    }
    #[doc = "Bit 16 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    pub fn jdmaen(&mut self) -> JDMAEN_W {
        JDMAEN_W { w: self }
    }
    #[doc = "Bit 17 - DMA channel enabled to read data for the regular channel"]
    #[inline(always)]
    pub fn rdmaen(&mut self) -> RDMAEN_W {
        RDMAEN_W { w: self }
    }
    #[doc = "Bit 31 - Initialization mode request"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
