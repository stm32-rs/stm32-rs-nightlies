///Register `LPUART_ICR` writer
pub type W = crate::W<LPUART_ICRrs>;
///Field `PECF` writer - Parity error clear flag Writing 1 to this bit clears the PE flag in the LPUART_ISR register.
pub type PECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FECF` writer - Framing error clear flag Writing 1 to this bit clears the FE flag in the LPUART_ISR register.
pub type FECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NECF` writer - Noise detected clear flag Writing 1 to this bit clears the NE flag in the LPUART_ISR register.
pub type NECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ORECF` writer - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the LPUART_ISR register.
pub type ORECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLECF` writer - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the LPUART_ISR register.
pub type IDLECF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCCF` writer - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the LPUART_ISR register.
pub type TCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTSCF` writer - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the LPUART_ISR register.
pub type CTSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMCF` writer - Character match clear flag Writing 1 to this bit clears the CMF flag in the LPUART_ISR register.
pub type CMCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUCF` writer - Wake-up from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section132.3: LPUART implementation on page1914.
pub type WUCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<LPUART_ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Parity error clear flag Writing 1 to this bit clears the PE flag in the LPUART_ISR register.
    #[inline(always)]
    pub fn pecf(&mut self) -> PECF_W<LPUART_ICRrs> {
        PECF_W::new(self, 0)
    }
    ///Bit 1 - Framing error clear flag Writing 1 to this bit clears the FE flag in the LPUART_ISR register.
    #[inline(always)]
    pub fn fecf(&mut self) -> FECF_W<LPUART_ICRrs> {
        FECF_W::new(self, 1)
    }
    ///Bit 2 - Noise detected clear flag Writing 1 to this bit clears the NE flag in the LPUART_ISR register.
    #[inline(always)]
    pub fn necf(&mut self) -> NECF_W<LPUART_ICRrs> {
        NECF_W::new(self, 2)
    }
    ///Bit 3 - Overrun error clear flag Writing 1 to this bit clears the ORE flag in the LPUART_ISR register.
    #[inline(always)]
    pub fn orecf(&mut self) -> ORECF_W<LPUART_ICRrs> {
        ORECF_W::new(self, 3)
    }
    ///Bit 4 - Idle line detected clear flag Writing 1 to this bit clears the IDLE flag in the LPUART_ISR register.
    #[inline(always)]
    pub fn idlecf(&mut self) -> IDLECF_W<LPUART_ICRrs> {
        IDLECF_W::new(self, 4)
    }
    ///Bit 6 - Transmission complete clear flag Writing 1 to this bit clears the TC flag in the LPUART_ISR register.
    #[inline(always)]
    pub fn tccf(&mut self) -> TCCF_W<LPUART_ICRrs> {
        TCCF_W::new(self, 6)
    }
    ///Bit 9 - CTS clear flag Writing 1 to this bit clears the CTSIF flag in the LPUART_ISR register.
    #[inline(always)]
    pub fn ctscf(&mut self) -> CTSCF_W<LPUART_ICRrs> {
        CTSCF_W::new(self, 9)
    }
    ///Bit 17 - Character match clear flag Writing 1 to this bit clears the CMF flag in the LPUART_ISR register.
    #[inline(always)]
    pub fn cmcf(&mut self) -> CMCF_W<LPUART_ICRrs> {
        CMCF_W::new(self, 17)
    }
    ///Bit 20 - Wake-up from low-power mode clear flag Writing 1 to this bit clears the WUF flag in the USART_ISR register. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section132.3: LPUART implementation on page1914.
    #[inline(always)]
    pub fn wucf(&mut self) -> WUCF_W<LPUART_ICRrs> {
        WUCF_W::new(self, 20)
    }
}
/**LPUART interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#LPUART1:LPUART_ICR)*/
pub struct LPUART_ICRrs;
impl crate::RegisterSpec for LPUART_ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lpuart_icr::W`](W) writer structure
impl crate::Writable for LPUART_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPUART_ICR to value 0
impl crate::Resettable for LPUART_ICRrs {
    const RESET_VALUE: u32 = 0;
}
