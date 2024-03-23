#[doc = "Register `DDRCTRL_CRCPARSTAT` reader"]
pub type R = crate::R<DDRCTRL_CRCPARSTATrs>;
#[doc = "Field `DFI_ALERT_ERR_CNT` reader - DFI_ALERT_ERR_CNT"]
pub type DFI_ALERT_ERR_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `DFI_ALERT_ERR_INT` reader - DFI_ALERT_ERR_INT"]
pub type DFI_ALERT_ERR_INT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - DFI_ALERT_ERR_CNT"]
    #[inline(always)]
    pub fn dfi_alert_err_cnt(&self) -> DFI_ALERT_ERR_CNT_R {
        DFI_ALERT_ERR_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - DFI_ALERT_ERR_INT"]
    #[inline(always)]
    pub fn dfi_alert_err_int(&self) -> DFI_ALERT_ERR_INT_R {
        DFI_ALERT_ERR_INT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "DDRCTRL CRC parity status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_crcparstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_CRCPARSTATrs;
impl crate::RegisterSpec for DDRCTRL_CRCPARSTATrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_crcparstat::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_CRCPARSTATrs {}
#[doc = "`reset()` method sets DDRCTRL_CRCPARSTAT to value 0"]
impl crate::Resettable for DDRCTRL_CRCPARSTATrs {
    const RESET_VALUE: u32 = 0;
}
