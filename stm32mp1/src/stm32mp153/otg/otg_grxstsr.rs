#[doc = "Register `OTG_GRXSTSR` reader"]
pub struct R(crate::R<OTG_GRXSTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_GRXSTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_GRXSTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_GRXSTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPNUM` reader - EPNUM"]
pub struct EPNUM_R(crate::FieldReader<u8, u8>);
impl EPNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCNT` reader - BCNT"]
pub struct BCNT_R(crate::FieldReader<u16, u16>);
impl BCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        BCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPID` reader - DPID"]
pub struct DPID_R(crate::FieldReader<u8, u8>);
impl DPID_R {
    pub(crate) fn new(bits: u8) -> Self {
        DPID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKTSTS` reader - PKTSTS"]
pub struct PKTSTS_R(crate::FieldReader<u8, u8>);
impl PKTSTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PKTSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKTSTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRMNUM` reader - FRMNUM"]
pub struct FRMNUM_R(crate::FieldReader<u8, u8>);
impl FRMNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRMNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STSPHST` reader - STSPHST"]
pub struct STSPHST_R(crate::FieldReader<bool, bool>);
impl STSPHST_R {
    pub(crate) fn new(bits: bool) -> Self {
        STSPHST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSPHST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - EPNUM"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - BCNT"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - DPID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 17:20 - PKTSTS"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - FRMNUM"]
    #[inline(always)]
    pub fn frmnum(&self) -> FRMNUM_R {
        FRMNUM_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - STSPHST"]
    #[inline(always)]
    pub fn stsphst(&self) -> STSPHST_R {
        STSPHST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "This description is for register OTG_GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_grxstsr](index.html) module"]
pub struct OTG_GRXSTSR_SPEC;
impl crate::RegisterSpec for OTG_GRXSTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_grxstsr::R](R) reader structure"]
impl crate::Readable for OTG_GRXSTSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_GRXSTSR to value 0"]
impl crate::Resettable for OTG_GRXSTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
