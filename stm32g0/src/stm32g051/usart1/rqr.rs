#[doc = "Register `RQR` writer"]
pub type W = crate::W<RQRrs>;
#[doc = "Field `ABRRQ` writer - Auto baud rate request Writing 1 to this bit resets the ABRF flag in the USART_ISR and requests an automatic baud rate measurement on the next received data frame. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type ABRRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBKRQ` writer - Send break request Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available. Note: When the application needs to send the break character following all previously inserted data, including the ones not yet transmitted, the software should wait for the TXE flag assertion before setting the SBKRQ bit."]
pub type SBKRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMRQ` writer - Mute mode request Writing 1 to this bit puts the USART in Mute mode and resets the RWU flag."]
pub type MMRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFRQ` writer - Receive data flush request Writing 1 to this bit empties the entire receive FIFO i.e. clears the bit RXFNE. This enables to discard the received data without reading them, and avoid an overrun condition."]
pub type RXFRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFRQ` writer - Transmit data flush request When FIFO mode is disabled, writing '1â\u{80}\u{99} to this bit sets the TXE flag. This enables to discard the transmit data. This bit must be used only in Smartcard mode, when data have not been sent due to errors (NACK) and the FE flag is active in the USART_ISR register. If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. When FIFO is enabled, TXFRQ bit is set to flush the whole FIFO. This sets the TXFE flag (Transmit FIFO empty, bit 23 in the USART_ISR register). Flushing the Transmit FIFO is supported in both UART and Smartcard modes. Note: In FIFO mode, the TXFNF flag is reset during the flush request until TxFIFO is empty in order to ensure that no data are written in the data register."]
pub type TXFRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Auto baud rate request Writing 1 to this bit resets the ABRF flag in the USART_ISR and requests an automatic baud rate measurement on the next received data frame. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn abrrq(&mut self) -> ABRRQ_W<RQRrs> {
        ABRRQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - Send break request Writing 1 to this bit sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available. Note: When the application needs to send the break character following all previously inserted data, including the ones not yet transmitted, the software should wait for the TXE flag assertion before setting the SBKRQ bit."]
    #[inline(always)]
    #[must_use]
    pub fn sbkrq(&mut self) -> SBKRQ_W<RQRrs> {
        SBKRQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mute mode request Writing 1 to this bit puts the USART in Mute mode and resets the RWU flag."]
    #[inline(always)]
    #[must_use]
    pub fn mmrq(&mut self) -> MMRQ_W<RQRrs> {
        MMRQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive data flush request Writing 1 to this bit empties the entire receive FIFO i.e. clears the bit RXFNE. This enables to discard the received data without reading them, and avoid an overrun condition."]
    #[inline(always)]
    #[must_use]
    pub fn rxfrq(&mut self) -> RXFRQ_W<RQRrs> {
        RXFRQ_W::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit data flush request When FIFO mode is disabled, writing '1â\u{80}\u{99} to this bit sets the TXE flag. This enables to discard the transmit data. This bit must be used only in Smartcard mode, when data have not been sent due to errors (NACK) and the FE flag is active in the USART_ISR register. If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. When FIFO is enabled, TXFRQ bit is set to flush the whole FIFO. This sets the TXFE flag (Transmit FIFO empty, bit 23 in the USART_ISR register). Flushing the Transmit FIFO is supported in both UART and Smartcard modes. Note: In FIFO mode, the TXFNF flag is reset during the flush request until TxFIFO is empty in order to ensure that no data are written in the data register."]
    #[inline(always)]
    #[must_use]
    pub fn txfrq(&mut self) -> TXFRQ_W<RQRrs> {
        TXFRQ_W::new(self, 4)
    }
}
#[doc = "Request register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rqr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RQRrs;
impl crate::RegisterSpec for RQRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rqr::W`](W) writer structure"]
impl crate::Writable for RQRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RQR to value 0"]
impl crate::Resettable for RQRrs {
    const RESET_VALUE: u32 = 0;
}
