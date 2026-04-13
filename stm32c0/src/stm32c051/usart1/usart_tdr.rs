///Register `USART_TDR` reader
pub type R = crate::R<USART_TDRrs>;
///Register `USART_TDR` writer
pub type W = crate::W<USART_TDRrs>;
///Field `TDR` reader - Transmit data value Contains the data character to be transmitted. The USART_TDR register provides the parallel interface between the internal bus and the output shift register (see Figure 243). When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF = 1.
pub type TDR_R = crate::FieldReader<u16>;
///Field `TDR` writer - Transmit data value Contains the data character to be transmitted. The USART_TDR register provides the parallel interface between the internal bus and the output shift register (see Figure 243). When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF = 1.
pub type TDR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Transmit data value Contains the data character to be transmitted. The USART_TDR register provides the parallel interface between the internal bus and the output shift register (see Figure 243). When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF = 1.
    #[inline(always)]
    pub fn tdr(&self) -> TDR_R {
        TDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_TDR")
            .field("tdr", &self.tdr())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Transmit data value Contains the data character to be transmitted. The USART_TDR register provides the parallel interface between the internal bus and the output shift register (see Figure 243). When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register), the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect because it is replaced by the parity. Note: This register must be written only when TXE/TXFNF = 1.
    #[inline(always)]
    pub fn tdr(&mut self) -> TDR_W<'_, USART_TDRrs> {
        TDR_W::new(self, 0)
    }
}
/**USART transmit data register

You can [`read`](crate::Reg::read) this register and get [`usart_tdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_tdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#USART1:USART_TDR)*/
pub struct USART_TDRrs;
impl crate::RegisterSpec for USART_TDRrs {
    type Ux = u32;
}
///`read()` method returns [`usart_tdr::R`](R) reader structure
impl crate::Readable for USART_TDRrs {}
///`write(|w| ..)` method takes [`usart_tdr::W`](W) writer structure
impl crate::Writable for USART_TDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USART_TDR to value 0
impl crate::Resettable for USART_TDRrs {}
