///Register `USART_ICR` writer
pub type W = crate::W<USART_ICRrs>;
///Field `PECF` writer - Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register.
pub type PECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FECF` writer - Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register.
pub type FECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NECF` writer - Noise detected clear flag Writing 1 to this bit clears the NE flag in the USART_ISR register.
pub type NECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ORECF` writer - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register.
pub type ORECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLECF` writer - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register.
pub type IDLECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFECF` writer - TXFIFO empty clear flag Writing 1 to this bit clears the TXFE flag in the USART_ISR register.
pub type TXFECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCCF` writer - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register.
pub type TCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCBGTCF` writer - Transmission complete before Guard time clear flag Writing 1 to this bit clears the TCBGT flag in the USART_ISR register.
pub type TCBGTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBDCF` writer - LIN break detection clear flag Writing 1 to this bit clears the LBDF flag in the USART_ISR register. Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
pub type LBDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSCF` writer - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register. Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTOCF` writer - Receiver timeout clear flag Writing 1 to this bit clears the RTOF flag in the USART_ISR register. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
pub type RTOCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOBCF` writer - End of block clear flag Writing 1 to this bit clears the EOBF flag in the USART_ISR register. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
pub type EOBCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDRCF` writer - SPI slave underrun clear flag Writing 1 to this bit clears the UDRF flag in the USART_ISR register. Note: If the USART does not support SPI slave mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826
pub type UDRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMCF` writer - Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register.
pub type CMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUCF` writer - Wake-up from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
pub type WUCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<USART_ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register.
    #[inline(always)]
    pub fn pecf(&mut self) -> PECF_W<USART_ICRrs> {
        PECF_W::new(self, 0)
    }
    ///Bit 1 - Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register.
    #[inline(always)]
    pub fn fecf(&mut self) -> FECF_W<USART_ICRrs> {
        FECF_W::new(self, 1)
    }
    ///Bit 2 - Noise detected clear flag Writing 1 to this bit clears the NE flag in the USART_ISR register.
    #[inline(always)]
    pub fn necf(&mut self) -> NECF_W<USART_ICRrs> {
        NECF_W::new(self, 2)
    }
    ///Bit 3 - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register.
    #[inline(always)]
    pub fn orecf(&mut self) -> ORECF_W<USART_ICRrs> {
        ORECF_W::new(self, 3)
    }
    ///Bit 4 - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register.
    #[inline(always)]
    pub fn idlecf(&mut self) -> IDLECF_W<USART_ICRrs> {
        IDLECF_W::new(self, 4)
    }
    ///Bit 5 - TXFIFO empty clear flag Writing 1 to this bit clears the TXFE flag in the USART_ISR register.
    #[inline(always)]
    pub fn txfecf(&mut self) -> TXFECF_W<USART_ICRrs> {
        TXFECF_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register.
    #[inline(always)]
    pub fn tccf(&mut self) -> TCCF_W<USART_ICRrs> {
        TCCF_W::new(self, 6)
    }
    ///Bit 7 - Transmission complete before Guard time clear flag Writing 1 to this bit clears the TCBGT flag in the USART_ISR register.
    #[inline(always)]
    pub fn tcbgtcf(&mut self) -> TCBGTCF_W<USART_ICRrs> {
        TCBGTCF_W::new(self, 7)
    }
    ///Bit 8 - LIN break detection clear flag Writing 1 to this bit clears the LBDF flag in the USART_ISR register. Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
    #[inline(always)]
    pub fn lbdcf(&mut self) -> LBDCF_W<USART_ICRrs> {
        LBDCF_W::new(self, 8)
    }
    ///Bit 9 - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register. Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<USART_ICRrs> {
        CTSCF_W::new(self, 9)
    }
    ///Bit 11 - Receiver timeout clear flag Writing 1 to this bit clears the RTOF flag in the USART_ISR register. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
    #[inline(always)]
    pub fn rtocf(&mut self) -> RTOCF_W<USART_ICRrs> {
        RTOCF_W::new(self, 11)
    }
    ///Bit 12 - End of block clear flag Writing 1 to this bit clears the EOBF flag in the USART_ISR register. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
    #[inline(always)]
    pub fn eobcf(&mut self) -> EOBCF_W<USART_ICRrs> {
        EOBCF_W::new(self, 12)
    }
    ///Bit 13 - SPI slave underrun clear flag Writing 1 to this bit clears the UDRF flag in the USART_ISR register. Note: If the USART does not support SPI slave mode, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826
    #[inline(always)]
    pub fn udrcf(&mut self) -> UDRCF_W<USART_ICRrs> {
        UDRCF_W::new(self, 13)
    }
    ///Bit 17 - Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register.
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W<USART_ICRrs> {
        CMCF_W::new(self, 17)
    }
    ///Bit 20 - Wake-up from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section131.4: USART implementation on page1826.
    #[inline(always)]
    pub fn wucf(&mut self) -> WUCF_W<USART_ICRrs> {
        WUCF_W::new(self, 20)
    }
}
/**USART interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USART1:USART_ICR)*/
pub struct USART_ICRrs;
impl crate::RegisterSpec for USART_ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`usart_icr::W`](W) writer structure
impl crate::Writable for USART_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets USART_ICR to value 0
impl crate::Resettable for USART_ICRrs {
    const RESET_VALUE: u32 = 0;
}
