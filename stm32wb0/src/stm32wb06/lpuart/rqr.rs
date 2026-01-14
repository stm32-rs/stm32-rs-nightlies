///Register `RQR` reader
pub type R = crate::R<RQRrs>;
///Register `RQR` writer
pub type W = crate::W<RQRrs>;
/**SBKRQ: Send break request Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBKRQ {
    ///1: sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available
    Break = 1,
}
impl From<SBKRQ> for bool {
    #[inline(always)]
    fn from(variant: SBKRQ) -> Self {
        variant as u8 != 0
    }
}
///Field `SBKRQ` writer - SBKRQ: Send break request Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available.
pub type SBKRQ_W<'a, REG> = crate::BitWriter<'a, REG, SBKRQ>;
impl<'a, REG> SBKRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available
    #[inline(always)]
    pub fn break_(self) -> &'a mut crate::W<REG> {
        self.variant(SBKRQ::Break)
    }
}
/**MMRQ: Mute mode request Writing 1 to this bit puts the USART in mute mode and resets the RWU flag.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMRQ {
    ///1: Puts the USART in mute mode and sets the RWU flag
    Mute = 1,
}
impl From<MMRQ> for bool {
    #[inline(always)]
    fn from(variant: MMRQ) -> Self {
        variant as u8 != 0
    }
}
///Field `MMRQ` writer - MMRQ: Mute mode request Writing 1 to this bit puts the USART in mute mode and resets the RWU flag.
pub type MMRQ_W<'a, REG> = crate::BitWriter<'a, REG, MMRQ>;
impl<'a, REG> MMRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Puts the USART in mute mode and sets the RWU flag
    #[inline(always)]
    pub fn mute(self) -> &'a mut crate::W<REG> {
        self.variant(MMRQ::Mute)
    }
}
/**RXFRQ: Receive data flush request Writing 1 to this bit empties the entire receive FIFO i.e. clears the bit RXFNE. This allows to discard the received data without reading them, and avoid an overrun condition.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFRQ {
    ///1: clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition
    Discard = 1,
}
impl From<RXFRQ> for bool {
    #[inline(always)]
    fn from(variant: RXFRQ) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFRQ` writer - RXFRQ: Receive data flush request Writing 1 to this bit empties the entire receive FIFO i.e. clears the bit RXFNE. This allows to discard the received data without reading them, and avoid an overrun condition.
pub type RXFRQ_W<'a, REG> = crate::BitWriter<'a, REG, RXFRQ>;
impl<'a, REG> RXFRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(RXFRQ::Discard)
    }
}
/**TXFRQ: Transmit data flush request When FIFO mode is disabled, Writing 1 to this bit sets the TXE flag. This allows to discard the transmit data. This bit must be used only in Smartcard mode, when data has not been sent due to errors (NACK) and the FE flag is active in the USART_ISR register. If the USART does not support Smartcard mode, this bit is reserved and forced by hardware to 0 When FIFO is enabled, TXFRQ bit is set to flush the whole FIFO . This will set the flag TXFE (Transmit FIFO empty, bit 23 in the USART_ISR register). Flushing the Transmit FIFO is supported in both UART and Smartcard modes.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFRQ {
    ///1: Set the TXE flags. This allows to discard the transmit data
    Discard = 1,
}
impl From<TXFRQ> for bool {
    #[inline(always)]
    fn from(variant: TXFRQ) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFRQ` writer - TXFRQ: Transmit data flush request When FIFO mode is disabled, Writing 1 to this bit sets the TXE flag. This allows to discard the transmit data. This bit must be used only in Smartcard mode, when data has not been sent due to errors (NACK) and the FE flag is active in the USART_ISR register. If the USART does not support Smartcard mode, this bit is reserved and forced by hardware to 0 When FIFO is enabled, TXFRQ bit is set to flush the whole FIFO . This will set the flag TXFE (Transmit FIFO empty, bit 23 in the USART_ISR register). Flushing the Transmit FIFO is supported in both UART and Smartcard modes.
pub type TXFRQ_W<'a, REG> = crate::BitWriter<'a, REG, TXFRQ>;
impl<'a, REG> TXFRQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Set the TXE flags. This allows to discard the transmit data
    #[inline(always)]
    pub fn discard(self) -> &'a mut crate::W<REG> {
        self.variant(TXFRQ::Discard)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RQR").finish()
    }
}
impl W {
    ///Bit 1 - SBKRQ: Send break request Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available.
    #[inline(always)]
    pub fn sbkrq(&mut self) -> SBKRQ_W<'_, RQRrs> {
        SBKRQ_W::new(self, 1)
    }
    ///Bit 2 - MMRQ: Mute mode request Writing 1 to this bit puts the USART in mute mode and resets the RWU flag.
    #[inline(always)]
    pub fn mmrq(&mut self) -> MMRQ_W<'_, RQRrs> {
        MMRQ_W::new(self, 2)
    }
    ///Bit 3 - RXFRQ: Receive data flush request Writing 1 to this bit empties the entire receive FIFO i.e. clears the bit RXFNE. This allows to discard the received data without reading them, and avoid an overrun condition.
    #[inline(always)]
    pub fn rxfrq(&mut self) -> RXFRQ_W<'_, RQRrs> {
        RXFRQ_W::new(self, 3)
    }
    ///Bit 4 - TXFRQ: Transmit data flush request When FIFO mode is disabled, Writing 1 to this bit sets the TXE flag. This allows to discard the transmit data. This bit must be used only in Smartcard mode, when data has not been sent due to errors (NACK) and the FE flag is active in the USART_ISR register. If the USART does not support Smartcard mode, this bit is reserved and forced by hardware to 0 When FIFO is enabled, TXFRQ bit is set to flush the whole FIFO . This will set the flag TXFE (Transmit FIFO empty, bit 23 in the USART_ISR register). Flushing the Transmit FIFO is supported in both UART and Smartcard modes.
    #[inline(always)]
    pub fn txfrq(&mut self) -> TXFRQ_W<'_, RQRrs> {
        TXFRQ_W::new(self, 4)
    }
}
/**RQR register

You can [`read`](crate::Reg::read) this register and get [`rqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#LPUART:RQR)*/
pub struct RQRrs;
impl crate::RegisterSpec for RQRrs {
    type Ux = u32;
}
///`read()` method returns [`rqr::R`](R) reader structure
impl crate::Readable for RQRrs {}
///`write(|w| ..)` method takes [`rqr::W`](W) writer structure
impl crate::Writable for RQRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RQR to value 0
impl crate::Resettable for RQRrs {}
