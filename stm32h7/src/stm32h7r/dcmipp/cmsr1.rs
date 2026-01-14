///Register `CMSR1` reader
pub type R = crate::R<CMSR1rs>;
///Field `PRHSYNC` reader - This bit gives the state of the HSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in HPOL bit of the DCMIPP_PRCR register, and cleared otherwise. When embedded synchronization codes are used the meaning of this bit is the following: In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in the DCMIPP_PRCR register is set.
pub type PRHSYNC_R = crate::BitReader;
///Field `PRVSYNC` reader - This bit gives the state of the VSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in VPOL bit of the DCMIPP_PRCR register, and cleared otherwise. When embedded synchronization codes are used, the meaning of this bit is the following: In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in the DCMIPP_PRCR register is set.
pub type PRVSYNC_R = crate::BitReader;
///Field `P0CPTACT` reader - Active frame capture (active from start-of-frame to frame complete) for Pipe0
pub type P0CPTACT_R = crate::BitReader;
impl R {
    ///Bit 0 - This bit gives the state of the HSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in HPOL bit of the DCMIPP_PRCR register, and cleared otherwise. When embedded synchronization codes are used the meaning of this bit is the following: In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in the DCMIPP_PRCR register is set.
    #[inline(always)]
    pub fn prhsync(&self) -> PRHSYNC_R {
        PRHSYNC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - This bit gives the state of the VSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in VPOL bit of the DCMIPP_PRCR register, and cleared otherwise. When embedded synchronization codes are used, the meaning of this bit is the following: In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in the DCMIPP_PRCR register is set.
    #[inline(always)]
    pub fn prvsync(&self) -> PRVSYNC_R {
        PRVSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 15 - Active frame capture (active from start-of-frame to frame complete) for Pipe0
    #[inline(always)]
    pub fn p0cptact(&self) -> P0CPTACT_R {
        P0CPTACT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMSR1")
            .field("prhsync", &self.prhsync())
            .field("prvsync", &self.prvsync())
            .field("p0cptact", &self.p0cptact())
            .finish()
    }
}
/**DCMIPP common status register 1

You can [`read`](crate::Reg::read) this register and get [`cmsr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:CMSR1)*/
pub struct CMSR1rs;
impl crate::RegisterSpec for CMSR1rs {
    type Ux = u32;
}
///`read()` method returns [`cmsr1::R`](R) reader structure
impl crate::Readable for CMSR1rs {}
///`reset()` method sets CMSR1 to value 0x03
impl crate::Resettable for CMSR1rs {
    const RESET_VALUE: u32 = 0x03;
}
