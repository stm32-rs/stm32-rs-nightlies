///Register `MACTxQPMR` reader
pub type R = crate::R<MACTX_QPMRrs>;
///Field `PSTQ0` reader - PSTQ0
pub type PSTQ0_R = crate::FieldReader;
///Field `PSTQ1` reader - PSTQ1
pub type PSTQ1_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PSTQ0
    #[inline(always)]
    pub fn pstq0(&self) -> PSTQ0_R {
        PSTQ0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - PSTQ1
    #[inline(always)]
    pub fn pstq1(&self) -> PSTQ1_R {
        PSTQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACTxQPMR")
            .field("pstq0", &self.pstq0())
            .field("pstq1", &self.pstq1())
            .finish()
    }
}
/**The transmit queue priority mapping 0 register contains the priority values assigned to Tx queue 0 and tx queue 1.

You can [`read`](crate::Reg::read) this register and get [`mactx_qpmr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACTxQPMR)*/
pub struct MACTX_QPMRrs;
impl crate::RegisterSpec for MACTX_QPMRrs {
    type Ux = u32;
}
///`read()` method returns [`mactx_qpmr::R`](R) reader structure
impl crate::Readable for MACTX_QPMRrs {}
///`reset()` method sets MACTxQPMR to value 0
impl crate::Resettable for MACTX_QPMRrs {}
