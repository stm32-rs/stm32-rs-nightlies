#[doc = "Register `DDRCTRL_CRCPARSTAT` reader"]
pub struct R(crate::R<DDRCTRL_CRCPARSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_CRCPARSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_CRCPARSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_CRCPARSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DFI_ALERT_ERR_CNT` reader - DFI_ALERT_ERR_CNT"]
pub struct DFI_ALERT_ERR_CNT_R(crate::FieldReader<u16, u16>);
impl DFI_ALERT_ERR_CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        DFI_ALERT_ERR_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_ALERT_ERR_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_ALERT_ERR_INT` reader - DFI_ALERT_ERR_INT"]
pub struct DFI_ALERT_ERR_INT_R(crate::FieldReader<bool, bool>);
impl DFI_ALERT_ERR_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFI_ALERT_ERR_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_ALERT_ERR_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - DFI_ALERT_ERR_CNT"]
    #[inline(always)]
    pub fn dfi_alert_err_cnt(&self) -> DFI_ALERT_ERR_CNT_R {
        DFI_ALERT_ERR_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - DFI_ALERT_ERR_INT"]
    #[inline(always)]
    pub fn dfi_alert_err_int(&self) -> DFI_ALERT_ERR_INT_R {
        DFI_ALERT_ERR_INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "DDRCTRL CRC parity status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_crcparstat](index.html) module"]
pub struct DDRCTRL_CRCPARSTAT_SPEC;
impl crate::RegisterSpec for DDRCTRL_CRCPARSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_crcparstat::R](R) reader structure"]
impl crate::Readable for DDRCTRL_CRCPARSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DDRCTRL_CRCPARSTAT to value 0"]
impl crate::Resettable for DDRCTRL_CRCPARSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
