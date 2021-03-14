#[doc = "Reader of register DDRCTRL_CRCPARSTAT"]
pub type R = crate::R<u32, super::DDRCTRL_CRCPARSTAT>;
#[doc = "Reader of field `DFI_ALERT_ERR_CNT`"]
pub type DFI_ALERT_ERR_CNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `DFI_ALERT_ERR_INT`"]
pub type DFI_ALERT_ERR_INT_R = crate::R<bool, bool>;
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
