///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `PECF` writer - Parity error clear flag
pub type PECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FECF` writer - Framing error clear flag
pub type FECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NCF` writer - Noise detected clear flag
pub type NCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ORECF` writer - Overrun error clear flag
pub type ORECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLECF` writer - Idle line detected clear flag
pub type IDLECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFECF` writer - TXFECF
pub type TXFECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCCF` writer - Transmission complete clear flag
pub type TCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCBGTCF` writer - TCBGTCF
pub type TCBGTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBDCF` writer - LIN break detection clear flag
pub type LBDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSCF` writer - CTS clear flag
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTOCF` writer - Receiver timeout clear flag
pub type RTOCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOBCF` writer - End of block clear flag
pub type EOBCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDRCF` writer - UDRCF
pub type UDRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMCF` writer - Character match clear flag
pub type CMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUCF` writer - Wakeup from Stop mode clear flag
pub type WUCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Parity error clear flag
    #[inline(always)]
    pub fn pecf(&mut self) -> PECF_W<ICRrs> {
        PECF_W::new(self, 0)
    }
    ///Bit 1 - Framing error clear flag
    #[inline(always)]
    pub fn fecf(&mut self) -> FECF_W<ICRrs> {
        FECF_W::new(self, 1)
    }
    ///Bit 2 - Noise detected clear flag
    #[inline(always)]
    pub fn ncf(&mut self) -> NCF_W<ICRrs> {
        NCF_W::new(self, 2)
    }
    ///Bit 3 - Overrun error clear flag
    #[inline(always)]
    pub fn orecf(&mut self) -> ORECF_W<ICRrs> {
        ORECF_W::new(self, 3)
    }
    ///Bit 4 - Idle line detected clear flag
    #[inline(always)]
    pub fn idlecf(&mut self) -> IDLECF_W<ICRrs> {
        IDLECF_W::new(self, 4)
    }
    ///Bit 5 - TXFECF
    #[inline(always)]
    pub fn txfecf(&mut self) -> TXFECF_W<ICRrs> {
        TXFECF_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete clear flag
    #[inline(always)]
    pub fn tccf(&mut self) -> TCCF_W<ICRrs> {
        TCCF_W::new(self, 6)
    }
    ///Bit 7 - TCBGTCF
    #[inline(always)]
    pub fn tcbgtcf(&mut self) -> TCBGTCF_W<ICRrs> {
        TCBGTCF_W::new(self, 7)
    }
    ///Bit 8 - LIN break detection clear flag
    #[inline(always)]
    pub fn lbdcf(&mut self) -> LBDCF_W<ICRrs> {
        LBDCF_W::new(self, 8)
    }
    ///Bit 9 - CTS clear flag
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<ICRrs> {
        CTSCF_W::new(self, 9)
    }
    ///Bit 11 - Receiver timeout clear flag
    #[inline(always)]
    pub fn rtocf(&mut self) -> RTOCF_W<ICRrs> {
        RTOCF_W::new(self, 11)
    }
    ///Bit 12 - End of block clear flag
    #[inline(always)]
    pub fn eobcf(&mut self) -> EOBCF_W<ICRrs> {
        EOBCF_W::new(self, 12)
    }
    ///Bit 13 - UDRCF
    #[inline(always)]
    pub fn udrcf(&mut self) -> UDRCF_W<ICRrs> {
        UDRCF_W::new(self, 13)
    }
    ///Bit 17 - Character match clear flag
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W<ICRrs> {
        CMCF_W::new(self, 17)
    }
    ///Bit 20 - Wakeup from Stop mode clear flag
    #[inline(always)]
    pub fn wucf(&mut self) -> WUCF_W<ICRrs> {
        WUCF_W::new(self, 20)
    }
}
/**Interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G484.html#UART4:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
