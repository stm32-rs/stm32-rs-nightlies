#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `PECF` writer - Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register."]
pub type PECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECF` writer - Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register."]
pub type FECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NECF` writer - Noise detected clear flag Writing 1 to this bit clears the NE flag in the USART_ISR register."]
pub type NECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORECF` writer - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register."]
pub type ORECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLECF` writer - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register."]
pub type IDLECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFECF` writer - TXFIFO empty clear flag Writing 1 to this bit clears the TXFE flag in the USART_ISR register."]
pub type TXFECF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCCF` writer - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register."]
pub type TCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCBGTCF` writer - Transmission complete before Guard time clear flag Writing 1 to this bit clears the TCBGT flag in the USART_ISR register."]
pub type TCBGTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDCF` writer - LIN break detection clear flag Writing 1 to this bit clears the LBDF flag in the USART_ISR register. Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCF` writer - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register. Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOCF` writer - Receiver timeout clear flag Writing 1 to this bit clears the RTOF flag in the USART_ISR register. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to page 1985."]
pub type RTOCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOBCF` writer - End of block clear flag Writing 1 to this bit clears the EOBF flag in the USART_ISR register. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type EOBCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRCF` writer - SPI slave underrun clear flag Writing 1 to this bit clears the UDRF flag in the USART_ISR register. Note: If the USART does not support SPI slave mode, this bit is reserved and must be kept at reset value. Refer to"]
pub type UDRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMCF` writer - Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register."]
pub type CMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUCF` writer - Wakeup from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 1985."]
pub type WUCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Parity error clear flag Writing 1 to this bit clears the PE flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn pecf(&mut self) -> PECF_W<ICRrs> {
        PECF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Framing error clear flag Writing 1 to this bit clears the FE flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn fecf(&mut self) -> FECF_W<ICRrs> {
        FECF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Noise detected clear flag Writing 1 to this bit clears the NE flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn necf(&mut self) -> NECF_W<ICRrs> {
        NECF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn orecf(&mut self) -> ORECF_W<ICRrs> {
        ORECF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn idlecf(&mut self) -> IDLECF_W<ICRrs> {
        IDLECF_W::new(self, 4)
    }
    #[doc = "Bit 5 - TXFIFO empty clear flag Writing 1 to this bit clears the TXFE flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn txfecf(&mut self) -> TXFECF_W<ICRrs> {
        TXFECF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn tccf(&mut self) -> TCCF_W<ICRrs> {
        TCCF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transmission complete before Guard time clear flag Writing 1 to this bit clears the TCBGT flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn tcbgtcf(&mut self) -> TCBGTCF_W<ICRrs> {
        TCBGTCF_W::new(self, 7)
    }
    #[doc = "Bit 8 - LIN break detection clear flag Writing 1 to this bit clears the LBDF flag in the USART_ISR register. Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn lbdcf(&mut self) -> LBDCF_W<ICRrs> {
        LBDCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the USART_ISR register. Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CTSCF_W<ICRrs> {
        CTSCF_W::new(self, 9)
    }
    #[doc = "Bit 11 - Receiver timeout clear flag Writing 1 to this bit clears the RTOF flag in the USART_ISR register. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to page 1985."]
    #[inline(always)]
    #[must_use]
    pub fn rtocf(&mut self) -> RTOCF_W<ICRrs> {
        RTOCF_W::new(self, 11)
    }
    #[doc = "Bit 12 - End of block clear flag Writing 1 to this bit clears the EOBF flag in the USART_ISR register. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn eobcf(&mut self) -> EOBCF_W<ICRrs> {
        EOBCF_W::new(self, 12)
    }
    #[doc = "Bit 13 - SPI slave underrun clear flag Writing 1 to this bit clears the UDRF flag in the USART_ISR register. Note: If the USART does not support SPI slave mode, this bit is reserved and must be kept at reset value. Refer to"]
    #[inline(always)]
    #[must_use]
    pub fn udrcf(&mut self) -> UDRCF_W<ICRrs> {
        UDRCF_W::new(self, 13)
    }
    #[doc = "Bit 17 - Character match clear flag Writing 1 to this bit clears the CMF flag in the USART_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn cmcf(&mut self) -> CMCF_W<ICRrs> {
        CMCF_W::new(self, 17)
    }
    #[doc = "Bit 20 - Wakeup from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to page 1985."]
    #[inline(always)]
    #[must_use]
    pub fn wucf(&mut self) -> WUCF_W<ICRrs> {
        WUCF_W::new(self, 20)
    }
}
#[doc = "USART interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
