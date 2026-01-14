///Register `CRCPARSTAT` reader
pub type R = crate::R<CRCPARSTATrs>;
///Field `DFI_ALERT_ERR_CNT` reader - DFI_ALERT_ERR_CNT
pub type DFI_ALERT_ERR_CNT_R = crate::FieldReader<u16>;
///Field `DFI_ALERT_ERR_INT` reader - DFI_ALERT_ERR_INT
pub type DFI_ALERT_ERR_INT_R = crate::BitReader;
impl R {
    ///Bits 0:15 - DFI_ALERT_ERR_CNT
    #[inline(always)]
    pub fn dfi_alert_err_cnt(&self) -> DFI_ALERT_ERR_CNT_R {
        DFI_ALERT_ERR_CNT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - DFI_ALERT_ERR_INT
    #[inline(always)]
    pub fn dfi_alert_err_int(&self) -> DFI_ALERT_ERR_INT_R {
        DFI_ALERT_ERR_INT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCPARSTAT")
            .field("dfi_alert_err_cnt", &self.dfi_alert_err_cnt())
            .field("dfi_alert_err_int", &self.dfi_alert_err_int())
            .finish()
    }
}
/**DDRCTRL CRC parity status register

You can [`read`](crate::Reg::read) this register and get [`crcparstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:CRCPARSTAT)*/
pub struct CRCPARSTATrs;
impl crate::RegisterSpec for CRCPARSTATrs {
    type Ux = u32;
}
///`read()` method returns [`crcparstat::R`](R) reader structure
impl crate::Readable for CRCPARSTATrs {}
///`reset()` method sets CRCPARSTAT to value 0
impl crate::Resettable for CRCPARSTATrs {}
