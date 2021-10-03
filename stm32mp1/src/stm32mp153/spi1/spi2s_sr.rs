#[doc = "Register `SPI2S_SR` reader"]
pub struct R(crate::R<SPI2S_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI2S_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI2S_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI2S_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXP` reader - RXP"]
pub struct RXP_R(crate::FieldReader<bool, bool>);
impl RXP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXP` reader - TXP"]
pub struct TXP_R(crate::FieldReader<bool, bool>);
impl TXP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DXP` reader - DXP"]
pub struct DXP_R(crate::FieldReader<bool, bool>);
impl DXP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DXP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DXP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOT` reader - EOT"]
pub struct EOT_R(crate::FieldReader<bool, bool>);
impl EOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXTF` reader - TXTF"]
pub struct TXTF_R(crate::FieldReader<bool, bool>);
impl TXTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDR` reader - UDR"]
pub struct UDR_R(crate::FieldReader<bool, bool>);
impl UDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        UDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR` reader - OVR"]
pub struct OVR_R(crate::FieldReader<bool, bool>);
impl OVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCE` reader - CRCE"]
pub struct CRCE_R(crate::FieldReader<bool, bool>);
impl CRCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIFRE` reader - TIFRE"]
pub struct TIFRE_R(crate::FieldReader<bool, bool>);
impl TIFRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIFRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIFRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODF` reader - MODF"]
pub struct MODF_R(crate::FieldReader<bool, bool>);
impl MODF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSERF` reader - TSERF"]
pub struct TSERF_R(crate::FieldReader<bool, bool>);
impl TSERF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP` reader - SUSP"]
pub struct SUSP_R(crate::FieldReader<bool, bool>);
impl SUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXC` reader - TXC"]
pub struct TXC_R(crate::FieldReader<bool, bool>);
impl TXC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXPLVL` reader - RXPLVL"]
pub struct RXPLVL_R(crate::FieldReader<u8, u8>);
impl RXPLVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXPLVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXPLVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXWNE` reader - RXWNE"]
pub struct RXWNE_R(crate::FieldReader<bool, bool>);
impl RXWNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXWNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXWNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSIZE` reader - CTSIZE"]
pub struct CTSIZE_R(crate::FieldReader<u16, u16>);
impl CTSIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RXP"]
    #[inline(always)]
    pub fn rxp(&self) -> RXP_R {
        RXP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXP"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DXP"]
    #[inline(always)]
    pub fn dxp(&self) -> DXP_R {
        DXP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EOT"]
    #[inline(always)]
    pub fn eot(&self) -> EOT_R {
        EOT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXTF"]
    #[inline(always)]
    pub fn txtf(&self) -> TXTF_R {
        TXTF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UDR"]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRCE"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIFRE"]
    #[inline(always)]
    pub fn tifre(&self) -> TIFRE_R {
        TIFRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MODF"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TSERF"]
    #[inline(always)]
    pub fn tserf(&self) -> TSERF_R {
        TSERF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SUSP"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TXC"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - RXPLVL"]
    #[inline(always)]
    pub fn rxplvl(&self) -> RXPLVL_R {
        RXPLVL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - RXWNE"]
    #[inline(always)]
    pub fn rxwne(&self) -> RXWNE_R {
        RXWNE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - CTSIZE"]
    #[inline(always)]
    pub fn ctsize(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SPI/I2S status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2s_sr](index.html) module"]
pub struct SPI2S_SR_SPEC;
impl crate::RegisterSpec for SPI2S_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi2s_sr::R](R) reader structure"]
impl crate::Readable for SPI2S_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI2S_SR to value 0x1002"]
impl crate::Resettable for SPI2S_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1002
    }
}
