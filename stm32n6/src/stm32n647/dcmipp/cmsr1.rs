///Register `CMSR1` reader
pub type R = crate::R<CMSR1rs>;
///Field `PRHSYNC` reader - This bit gives the state of the HSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in HPOL bit of the DCMIPP_PRCR register, and cleared otherwise.
pub type PRHSYNC_R = crate::BitReader;
///Field `PRVSYNC` reader - This bit gives the state of the VSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in VPOL bit of the DCMIPP_PRCR register, and cleared otherwise.
pub type PRVSYNC_R = crate::BitReader;
///Field `P0LSTLINE` reader - Last line LSB bit, sampled at Frame capture complete event for Pipe0
pub type P0LSTLINE_R = crate::BitReader;
///Field `P0LSTFRM` reader - Last frame LSB bit, sampled at Frame capture complete event for Pipe0
pub type P0LSTFRM_R = crate::BitReader;
///Field `P0CPTACT` reader - Active frame capture (active from start-of-frame to frame complete) for Pipe0
pub type P0CPTACT_R = crate::BitReader;
///Field `P1LSTLINE` reader - Last line LSB bit, sampled at Frame capture complete event for Pipe1
pub type P1LSTLINE_R = crate::BitReader;
///Field `P1LSTFRM` reader - Last frame LSB bit, sampled at frame capture complete event for Pipe1
pub type P1LSTFRM_R = crate::BitReader;
///Field `P1CPTACT` reader - Active frame capture (active from start-of-frame to frame complete) for Pipe1
pub type P1CPTACT_R = crate::BitReader;
///Field `P2LSTLINE` reader - Last line LSB bit, sampled at frame capture complete event for Pipe2
pub type P2LSTLINE_R = crate::BitReader;
///Field `P2LSTFRM` reader - Last frame LSB bit, sampled at frame capture complete event for Pipe2
pub type P2LSTFRM_R = crate::BitReader;
///Field `P2CPTACT` reader - Active frame capture (active from start-of-frame to frame complete) for Pipe2
pub type P2CPTACT_R = crate::BitReader;
impl R {
    ///Bit 0 - This bit gives the state of the HSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in HPOL bit of the DCMIPP_PRCR register, and cleared otherwise.
    #[inline(always)]
    pub fn prhsync(&self) -> PRHSYNC_R {
        PRHSYNC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - This bit gives the state of the VSYNC pin with the correct programmed polarity on the parallel interface if ENABLE bit is set into the DCMIPP_PRCR register and if the pixel clock is received. It is set during the blanking period whatever the polarity selected in VPOL bit of the DCMIPP_PRCR register, and cleared otherwise.
    #[inline(always)]
    pub fn prvsync(&self) -> PRVSYNC_R {
        PRVSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Last line LSB bit, sampled at Frame capture complete event for Pipe0
    #[inline(always)]
    pub fn p0lstline(&self) -> P0LSTLINE_R {
        P0LSTLINE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Last frame LSB bit, sampled at Frame capture complete event for Pipe0
    #[inline(always)]
    pub fn p0lstfrm(&self) -> P0LSTFRM_R {
        P0LSTFRM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - Active frame capture (active from start-of-frame to frame complete) for Pipe0
    #[inline(always)]
    pub fn p0cptact(&self) -> P0CPTACT_R {
        P0CPTACT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Last line LSB bit, sampled at Frame capture complete event for Pipe1
    #[inline(always)]
    pub fn p1lstline(&self) -> P1LSTLINE_R {
        P1LSTLINE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Last frame LSB bit, sampled at frame capture complete event for Pipe1
    #[inline(always)]
    pub fn p1lstfrm(&self) -> P1LSTFRM_R {
        P1LSTFRM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 23 - Active frame capture (active from start-of-frame to frame complete) for Pipe1
    #[inline(always)]
    pub fn p1cptact(&self) -> P1CPTACT_R {
        P1CPTACT_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Last line LSB bit, sampled at frame capture complete event for Pipe2
    #[inline(always)]
    pub fn p2lstline(&self) -> P2LSTLINE_R {
        P2LSTLINE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Last frame LSB bit, sampled at frame capture complete event for Pipe2
    #[inline(always)]
    pub fn p2lstfrm(&self) -> P2LSTFRM_R {
        P2LSTFRM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 31 - Active frame capture (active from start-of-frame to frame complete) for Pipe2
    #[inline(always)]
    pub fn p2cptact(&self) -> P2CPTACT_R {
        P2CPTACT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMSR1")
            .field("prhsync", &self.prhsync())
            .field("prvsync", &self.prvsync())
            .field("p0lstline", &self.p0lstline())
            .field("p0lstfrm", &self.p0lstfrm())
            .field("p0cptact", &self.p0cptact())
            .field("p1lstline", &self.p1lstline())
            .field("p1lstfrm", &self.p1lstfrm())
            .field("p1cptact", &self.p1cptact())
            .field("p2lstline", &self.p2lstline())
            .field("p2lstfrm", &self.p2lstfrm())
            .field("p2cptact", &self.p2cptact())
            .finish()
    }
}
/**DCMIPP common status register 1

You can [`read`](crate::Reg::read) this register and get [`cmsr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#DCMIPP:CMSR1)*/
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
