#[doc = "Register `MACHWF1R` reader"]
pub struct R(crate::R<MACHWF1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACHWF1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACHWF1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACHWF1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFOSIZE` reader - MTL Receive FIFO Size"]
pub struct RXFIFOSIZE_R(crate::FieldReader<u8, u8>);
impl RXFIFOSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXFIFOSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFOSIZE` reader - MTL Transmit FIFO Size"]
pub struct TXFIFOSIZE_R(crate::FieldReader<u8, u8>);
impl TXFIFOSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFOSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSTEN` reader - One-Step Timestamping Enable"]
pub struct OSTEN_R(crate::FieldReader<bool, bool>);
impl OSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTOEN` reader - PTP Offload Enable"]
pub struct PTOEN_R(crate::FieldReader<bool, bool>);
impl PTOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADVTHWORD` reader - IEEE 1588 High Word Register Enable"]
pub struct ADVTHWORD_R(crate::FieldReader<bool, bool>);
impl ADVTHWORD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADVTHWORD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADVTHWORD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCBEN` reader - DCB Feature Enable"]
pub struct DCBEN_R(crate::FieldReader<bool, bool>);
impl DCBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPHEN` reader - Split Header Feature Enable"]
pub struct SPHEN_R(crate::FieldReader<bool, bool>);
impl SPHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSOEN` reader - TCP Segmentation Offload Enable"]
pub struct TSOEN_R(crate::FieldReader<bool, bool>);
impl TSOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGMEMA` reader - DMA Debug Registers Enable"]
pub struct DBGMEMA_R(crate::FieldReader<bool, bool>);
impl DBGMEMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBGMEMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBGMEMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVSEL` reader - AV Feature Enable"]
pub struct AVSEL_R(crate::FieldReader<bool, bool>);
impl AVSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASHTBLSZ` reader - Hash Table Size"]
pub struct HASHTBLSZ_R(crate::FieldReader<u8, u8>);
impl HASHTBLSZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        HASHTBLSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASHTBLSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L3L4FNUM` reader - Total number of L3 or L4 Filters"]
pub struct L3L4FNUM_R(crate::FieldReader<u8, u8>);
impl L3L4FNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        L3L4FNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L3L4FNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - MTL Receive FIFO Size"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - MTL Transmit FIFO Size"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - One-Step Timestamping Enable"]
    #[inline(always)]
    pub fn osten(&self) -> OSTEN_R {
        OSTEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PTP Offload Enable"]
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - IEEE 1588 High Word Register Enable"]
    #[inline(always)]
    pub fn advthword(&self) -> ADVTHWORD_R {
        ADVTHWORD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DCB Feature Enable"]
    #[inline(always)]
    pub fn dcben(&self) -> DCBEN_R {
        DCBEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Split Header Feature Enable"]
    #[inline(always)]
    pub fn sphen(&self) -> SPHEN_R {
        SPHEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TCP Segmentation Offload Enable"]
    #[inline(always)]
    pub fn tsoen(&self) -> TSOEN_R {
        TSOEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DMA Debug Registers Enable"]
    #[inline(always)]
    pub fn dbgmema(&self) -> DBGMEMA_R {
        DBGMEMA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - AV Feature Enable"]
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Hash Table Size"]
    #[inline(always)]
    pub fn hashtblsz(&self) -> HASHTBLSZ_R {
        HASHTBLSZ_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 27:30 - Total number of L3 or L4 Filters"]
    #[inline(always)]
    pub fn l3l4fnum(&self) -> L3L4FNUM_R {
        L3L4FNUM_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
#[doc = "HW feature 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [machwf1r](index.html) module"]
pub struct MACHWF1R_SPEC;
impl crate::RegisterSpec for MACHWF1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [machwf1r::R](R) reader structure"]
impl crate::Readable for MACHWF1R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACHWF1R to value 0x1184_1904"]
impl crate::Resettable for MACHWF1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1184_1904
    }
}
