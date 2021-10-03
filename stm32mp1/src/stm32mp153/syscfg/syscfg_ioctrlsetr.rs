#[doc = "Register `SYSCFG_IOCTRLSETR` reader"]
pub struct R(crate::R<SYSCFG_IOCTRLSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_IOCTRLSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_IOCTRLSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_IOCTRLSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_IOCTRLSETR` writer"]
pub struct W(crate::W<SYSCFG_IOCTRLSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_IOCTRLSETR_SPEC>;
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
impl From<crate::W<SYSCFG_IOCTRLSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_IOCTRLSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSLVEN_TRACE` reader - HSLVEN_TRACE"]
pub struct HSLVEN_TRACE_R(crate::FieldReader<bool, bool>);
impl HSLVEN_TRACE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSLVEN_TRACE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSLVEN_TRACE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSLVEN_TRACE` writer - HSLVEN_TRACE"]
pub struct HSLVEN_TRACE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSLVEN_TRACE_W<'a> {
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
#[doc = "Field `HSLVEN_QUADSPI` reader - HSLVEN_QUADSPI"]
pub struct HSLVEN_QUADSPI_R(crate::FieldReader<bool, bool>);
impl HSLVEN_QUADSPI_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSLVEN_QUADSPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSLVEN_QUADSPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSLVEN_QUADSPI` writer - HSLVEN_QUADSPI"]
pub struct HSLVEN_QUADSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> HSLVEN_QUADSPI_W<'a> {
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
#[doc = "Field `HSLVEN_ETH` reader - HSLVEN_ETH"]
pub struct HSLVEN_ETH_R(crate::FieldReader<bool, bool>);
impl HSLVEN_ETH_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSLVEN_ETH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSLVEN_ETH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSLVEN_ETH` writer - HSLVEN_ETH"]
pub struct HSLVEN_ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> HSLVEN_ETH_W<'a> {
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
#[doc = "Field `HSLVEN_SDMMC` reader - HSLVEN_SDMMC"]
pub struct HSLVEN_SDMMC_R(crate::FieldReader<bool, bool>);
impl HSLVEN_SDMMC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSLVEN_SDMMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSLVEN_SDMMC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSLVEN_SDMMC` writer - HSLVEN_SDMMC"]
pub struct HSLVEN_SDMMC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSLVEN_SDMMC_W<'a> {
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
#[doc = "Field `HSLVEN_SPI` reader - HSLVEN_SPI"]
pub struct HSLVEN_SPI_R(crate::FieldReader<bool, bool>);
impl HSLVEN_SPI_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSLVEN_SPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSLVEN_SPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSLVEN_SPI` writer - HSLVEN_SPI"]
pub struct HSLVEN_SPI_W<'a> {
    w: &'a mut W,
}
impl<'a> HSLVEN_SPI_W<'a> {
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
impl R {
    #[doc = "Bit 0 - HSLVEN_TRACE"]
    #[inline(always)]
    pub fn hslven_trace(&self) -> HSLVEN_TRACE_R {
        HSLVEN_TRACE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HSLVEN_QUADSPI"]
    #[inline(always)]
    pub fn hslven_quadspi(&self) -> HSLVEN_QUADSPI_R {
        HSLVEN_QUADSPI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSLVEN_ETH"]
    #[inline(always)]
    pub fn hslven_eth(&self) -> HSLVEN_ETH_R {
        HSLVEN_ETH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSLVEN_SDMMC"]
    #[inline(always)]
    pub fn hslven_sdmmc(&self) -> HSLVEN_SDMMC_R {
        HSLVEN_SDMMC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HSLVEN_SPI"]
    #[inline(always)]
    pub fn hslven_spi(&self) -> HSLVEN_SPI_R {
        HSLVEN_SPI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSLVEN_TRACE"]
    #[inline(always)]
    pub fn hslven_trace(&mut self) -> HSLVEN_TRACE_W {
        HSLVEN_TRACE_W { w: self }
    }
    #[doc = "Bit 1 - HSLVEN_QUADSPI"]
    #[inline(always)]
    pub fn hslven_quadspi(&mut self) -> HSLVEN_QUADSPI_W {
        HSLVEN_QUADSPI_W { w: self }
    }
    #[doc = "Bit 2 - HSLVEN_ETH"]
    #[inline(always)]
    pub fn hslven_eth(&mut self) -> HSLVEN_ETH_W {
        HSLVEN_ETH_W { w: self }
    }
    #[doc = "Bit 3 - HSLVEN_SDMMC"]
    #[inline(always)]
    pub fn hslven_sdmmc(&mut self) -> HSLVEN_SDMMC_W {
        HSLVEN_SDMMC_W { w: self }
    }
    #[doc = "Bit 4 - HSLVEN_SPI"]
    #[inline(always)]
    pub fn hslven_spi(&mut self) -> HSLVEN_SPI_W {
        HSLVEN_SPI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG IO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_ioctrlsetr](index.html) module"]
pub struct SYSCFG_IOCTRLSETR_SPEC;
impl crate::RegisterSpec for SYSCFG_IOCTRLSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_ioctrlsetr::R](R) reader structure"]
impl crate::Readable for SYSCFG_IOCTRLSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_ioctrlsetr::W](W) writer structure"]
impl crate::Writable for SYSCFG_IOCTRLSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCFG_IOCTRLSETR to value 0"]
impl crate::Resettable for SYSCFG_IOCTRLSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
