#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXBFF` reader - Receive buffer full flag"]
pub struct RXBFF_R(crate::FieldReader<bool, bool>);
impl RXBFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBEF` reader - Transmit buffer empty flag"]
pub struct TXBEF_R(crate::FieldReader<bool, bool>);
impl TXBEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBERF` reader - Receive CRC error flag"]
pub struct RXBERF_R(crate::FieldReader<bool, bool>);
impl RXBERF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVRF` reader - Receive overrun error flag"]
pub struct RXOVRF_R(crate::FieldReader<bool, bool>);
impl RXOVRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNRF` reader - Transmit underrun error flag"]
pub struct TXUNRF_R(crate::FieldReader<bool, bool>);
impl TXUNRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXNE` reader - Receive data register not empty"]
pub struct RXNE_R(crate::FieldReader<bool, bool>);
impl RXNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXE` reader - Transmit data register empty"]
pub struct TXE_R(crate::FieldReader<bool, bool>);
impl TXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCF` reader - Transfer complete flag"]
pub struct TCF_R(crate::FieldReader<bool, bool>);
impl TCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRF` reader - Slave resume flag"]
pub struct SRF_R(crate::FieldReader<bool, bool>);
impl SRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSP` reader - SUSPEND flag"]
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
#[doc = "Field `DEACTF` reader - DEACTIVATED flag"]
pub struct DEACTF_R(crate::FieldReader<bool, bool>);
impl DEACTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEACTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEACTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receive buffer full flag"]
    #[inline(always)]
    pub fn rxbff(&self) -> RXBFF_R {
        RXBFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty flag"]
    #[inline(always)]
    pub fn txbef(&self) -> TXBEF_R {
        TXBEF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive CRC error flag"]
    #[inline(always)]
    pub fn rxberf(&self) -> RXBERF_R {
        RXBERF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive overrun error flag"]
    #[inline(always)]
    pub fn rxovrf(&self) -> RXOVRF_R {
        RXOVRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit underrun error flag"]
    #[inline(always)]
    pub fn txunrf(&self) -> TXUNRF_R {
        TXUNRF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive data register not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit data register empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transfer complete flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Slave resume flag"]
    #[inline(always)]
    pub fn srf(&self) -> SRF_R {
        SRF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SUSPEND flag"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DEACTIVATED flag"]
    #[inline(always)]
    pub fn deactf(&self) -> DEACTF_R {
        DEACTF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "SWPMI Interrupt and Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISR to value 0x02c2"]
impl crate::Resettable for ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02c2
    }
}
