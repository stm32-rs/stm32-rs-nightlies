///Register `FDCAN_DBTP` reader
pub type R = crate::R<FDCAN_DBTPrs>;
///Field `DSJW` reader - Synchronization Jump Width
pub type DSJW_R = crate::FieldReader;
///Field `DTSEG2` reader - Data time segment after sample point
pub type DTSEG2_R = crate::FieldReader;
///Field `DTSEG1` reader - Data time segment after sample point
pub type DTSEG1_R = crate::FieldReader;
///Field `DBRP` reader - Data BIt Rate Prescaler
pub type DBRP_R = crate::FieldReader;
///Field `TDC` reader - Transceiver Delay Compensation
pub type TDC_R = crate::BitReader;
impl R {
    ///Bits 0:3 - Synchronization Jump Width
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Data time segment after sample point
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:12 - Data time segment after sample point
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - Data BIt Rate Prescaler
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bit 23 - Transceiver Delay Compensation
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_DBTP")
            .field("dsjw", &self.dsjw())
            .field("dtseg2", &self.dtseg2())
            .field("dtseg1", &self.dtseg1())
            .field("dbrp", &self.dbrp())
            .field("tdc", &self.tdc())
            .finish()
    }
}
/**FDCAN Data Bit Timing and Prescaler Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_dbtp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_DBTP)*/
pub struct FDCAN_DBTPrs;
impl crate::RegisterSpec for FDCAN_DBTPrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_dbtp::R`](R) reader structure
impl crate::Readable for FDCAN_DBTPrs {}
///`reset()` method sets FDCAN_DBTP to value 0x0a33
impl crate::Resettable for FDCAN_DBTPrs {
    const RESET_VALUE: u32 = 0x0a33;
}
