///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `ADDM7` reader - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)
pub type ADDM7_R = crate::BitReader;
///Field `ADDM7` writer - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)
pub type ADDM7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` reader - STOP\[1:0\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0).
pub type STOP_R = crate::FieldReader;
///Field `STOP` writer - STOP\[1:0\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0).
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SWAP` reader - SWAP: Swap TX/RX pins This bit is set and cleared by software. -0: TX/RX pins are used as defined in standard pinout -1: The TX and RX pins functions are swapped. This allows to work in the case of a cross-wired connection to another UART. This bit field can only be written when the USART is disabled (UE=0).
pub type SWAP_R = crate::BitReader;
///Field `SWAP` writer - SWAP: Swap TX/RX pins This bit is set and cleared by software. -0: TX/RX pins are used as defined in standard pinout -1: The TX and RX pins functions are swapped. This allows to work in the case of a cross-wired connection to another UART. This bit field can only be written when the USART is disabled (UE=0).
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXINV` reader - RXINV: RX pin active level inversion This bit is set and cleared by software. -0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the RX line. This bit field can only be written when the USART is disabled (UE=0).
pub type RXINV_R = crate::BitReader;
///Field `RXINV` writer - RXINV: RX pin active level inversion This bit is set and cleared by software. -0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the RX line. This bit field can only be written when the USART is disabled (UE=0).
pub type RXINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXINV` reader - TXINV: TX pin active level inversion This bit is set and cleared by software. -0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the TX line. This bit field can only be written when the USART is disabled (UE=0).
pub type TXINV_R = crate::BitReader;
///Field `TXINV` writer - TXINV: TX pin active level inversion This bit is set and cleared by software. -0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the TX line. This bit field can only be written when the USART is disabled (UE=0).
pub type TXINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DATAINV` reader - DATAINV: Binary data inversion This bit is set and cleared by software. -0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L) -1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted. This bit field can only be written when the USART is disabled (UE=0).
pub type DATAINV_R = crate::BitReader;
///Field `DATAINV` writer - DATAINV: Binary data inversion This bit is set and cleared by software. -0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L) -1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted. This bit field can only be written when the USART is disabled (UE=0).
pub type DATAINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSBFIRST` reader - MSBFIRST: Most significant bit first This bit is set and cleared by software. -0: data is transmitted/received with data bit 0 first, following the start bit. -1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit. This bit field can only be written when the USART is disabled (UE=0).
pub type MSBFIRST_R = crate::BitReader;
///Field `MSBFIRST` writer - MSBFIRST: Most significant bit first This bit is set and cleared by software. -0: data is transmitted/received with data bit 0 first, following the start bit. -1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit. This bit field can only be written when the USART is disabled (UE=0).
pub type MSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD` reader - ADD\[7:0\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\[7:0\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)
pub type ADD_R = crate::FieldReader;
///Field `ADD` writer - ADD\[7:0\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\[7:0\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 4 - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 12:13 - STOP\[1:0\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 15 - SWAP: Swap TX/RX pins This bit is set and cleared by software. -0: TX/RX pins are used as defined in standard pinout -1: The TX and RX pins functions are swapped. This allows to work in the case of a cross-wired connection to another UART. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RXINV: RX pin active level inversion This bit is set and cleared by software. -0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the RX line. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TXINV: TX pin active level inversion This bit is set and cleared by software. -0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the TX line. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DATAINV: Binary data inversion This bit is set and cleared by software. -0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L) -1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - MSBFIRST: Most significant bit first This bit is set and cleared by software. -0: data is transmitted/received with data bit 0 first, following the start bit. -1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 24:31 - ADD\[7:0\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\[7:0\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("addm7", &self.addm7())
            .field("stop", &self.stop())
            .field("swap", &self.swap())
            .field("rxinv", &self.rxinv())
            .field("txinv", &self.txinv())
            .field("datainv", &self.datainv())
            .field("msbfirst", &self.msbfirst())
            .field("add", &self.add())
            .finish()
    }
}
impl W {
    ///Bit 4 - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)
    #[inline(always)]
    pub fn addm7(&mut self) -> ADDM7_W<'_, CR2rs> {
        ADDM7_W::new(self, 4)
    }
    ///Bits 12:13 - STOP\[1:0\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<'_, CR2rs> {
        STOP_W::new(self, 12)
    }
    ///Bit 15 - SWAP: Swap TX/RX pins This bit is set and cleared by software. -0: TX/RX pins are used as defined in standard pinout -1: The TX and RX pins functions are swapped. This allows to work in the case of a cross-wired connection to another UART. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W<'_, CR2rs> {
        SWAP_W::new(self, 15)
    }
    ///Bit 16 - RXINV: RX pin active level inversion This bit is set and cleared by software. -0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the RX line. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W<'_, CR2rs> {
        RXINV_W::new(self, 16)
    }
    ///Bit 17 - TXINV: TX pin active level inversion This bit is set and cleared by software. -0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the TX line. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W<'_, CR2rs> {
        TXINV_W::new(self, 17)
    }
    ///Bit 18 - DATAINV: Binary data inversion This bit is set and cleared by software. -0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L) -1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn datainv(&mut self) -> DATAINV_W<'_, CR2rs> {
        DATAINV_W::new(self, 18)
    }
    ///Bit 19 - MSBFIRST: Most significant bit first This bit is set and cleared by software. -0: data is transmitted/received with data bit 0 first, following the start bit. -1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<'_, CR2rs> {
        MSBFIRST_W::new(self, 19)
    }
    ///Bits 24:31 - ADD\[7:0\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\[7:0\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, CR2rs> {
        ADD_W::new(self, 24)
    }
}
/**CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPUART:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
