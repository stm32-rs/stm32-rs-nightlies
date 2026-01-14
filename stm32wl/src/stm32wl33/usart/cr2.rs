///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `SLVEN` reader - SLVEN: Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. -0: Slave mode disabled. -1: Slave mode enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value
pub type SLVEN_R = crate::BitReader;
///Field `SLVEN` writer - SLVEN: Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. -0: Slave mode disabled. -1: Slave mode enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value
pub type SLVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIS_NSS` reader - DIS_NSS When the DSI_NSS bit is set, the NSS pin input will be ignored. -0: SPI slave selection depends on NSS input pin. -1: SPI slave will be always selected and NSS input pin will be ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value
pub type DIS_NSS_R = crate::BitReader;
///Field `DIS_NSS` writer - DIS_NSS When the DSI_NSS bit is set, the NSS pin input will be ignored. -0: SPI slave selection depends on NSS input pin. -1: SPI slave will be always selected and NSS input pin will be ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value
pub type DIS_NSS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDM7` reader - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)
pub type ADDM7_R = crate::BitReader;
///Field `ADDM7` writer - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)
pub type ADDM7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBDL` reader - LBDL: LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. -0: 10-bit break detection -1: 11-bit break detection This bit can only be written when the USART is disabled (UE=0).
pub type LBDL_R = crate::BitReader;
///Field `LBDL` writer - LBDL: LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. -0: 10-bit break detection -1: 11-bit break detection This bit can only be written when the USART is disabled (UE=0).
pub type LBDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBDIE` reader - LBDIE: LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). -0: Interrupt is inhibited -1: An interrupt is generated whenever LBDF=1 in the USART_ISR register
pub type LBDIE_R = crate::BitReader;
///Field `LBDIE` writer - LBDIE: LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). -0: Interrupt is inhibited -1: An interrupt is generated whenever LBDF=1 in the USART_ISR register
pub type LBDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBCL` reader - LBCL: Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. -0: The clock pulse of the last data bit is not output to the SCLK pin -1: The clock pulse of the last data bit is output to the SCLK pin Caution: The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0).
pub type LBCL_R = crate::BitReader;
///Field `LBCL` writer - LBCL: Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. -0: The clock pulse of the last data bit is not output to the SCLK pin -1: The clock pulse of the last data bit is output to the SCLK pin Caution: The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0).
pub type LBCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPHA` reader - CPHA: Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see Figure 137 and Figure 138) -0: The first clock transition is the first data capture edge -1: The second clock transition is the first data capture edge This bit can only be written when the USART is disabled (UE=0).
pub type CPHA_R = crate::BitReader;
///Field `CPHA` writer - CPHA: Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see Figure 137 and Figure 138) -0: The first clock transition is the first data capture edge -1: The second clock transition is the first data capture edge This bit can only be written when the USART is disabled (UE=0).
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPOL` reader - CPOL: Clock polarity This bit allows the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship -0: Steady low value on SCLK pin outside transmission window -1: Steady high value on SCLK pin outside transmission window This bit can only be written when the USART is disabled (UE=0).
pub type CPOL_R = crate::BitReader;
///Field `CPOL` writer - CPOL: Clock polarity This bit allows the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship -0: Steady low value on SCLK pin outside transmission window -1: Steady high value on SCLK pin outside transmission window This bit can only be written when the USART is disabled (UE=0).
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKEN` reader - CLKEN: Clock enable This bit allows the user to enable the SCLK pin. -0: SCLK pin disabled -1: SCLK pin enabled This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and forced by hardware to 0. Please refer to Section 23.4: USART implementation on page 483. Note: In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: - UE = 0 - SCEN = 1 - GTPR configuration - CLKEN= 1 - UE = 1
pub type CLKEN_R = crate::BitReader;
///Field `CLKEN` writer - CLKEN: Clock enable This bit allows the user to enable the SCLK pin. -0: SCLK pin disabled -1: SCLK pin enabled This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and forced by hardware to 0. Please refer to Section 23.4: USART implementation on page 483. Note: In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: - UE = 0 - SCEN = 1 - GTPR configuration - CLKEN= 1 - UE = 1
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` reader - STOP\[1:0\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0).
pub type STOP_R = crate::FieldReader;
///Field `STOP` writer - STOP\[1:0\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0).
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LINEN` reader - LINEN: LIN mode enable This bit is set and cleared by software. -0: LIN mode disabled -1: LIN mode enabled The LIN mode enables the capability to send LIN Synch Breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bit field can only be written when the USART is disabled (UE=0).
pub type LINEN_R = crate::BitReader;
///Field `LINEN` writer - LINEN: LIN mode enable This bit is set and cleared by software. -0: LIN mode disabled -1: LIN mode enabled The LIN mode enables the capability to send LIN Synch Breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bit field can only be written when the USART is disabled (UE=0).
pub type LINEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
///Field `ABREN` reader - ABREN: Auto baud rate enable This bit is set and cleared by software. -0: Auto baud rate detection is disabled. -1: Auto baud rate detection is enabled.
pub type ABREN_R = crate::BitReader;
///Field `ABREN` writer - ABREN: Auto baud rate enable This bit is set and cleared by software. -0: Auto baud rate detection is disabled. -1: Auto baud rate detection is enabled.
pub type ABREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABRMOD` reader - ABRMOD\[1:0\]: Auto baud rate mode These bits are set and cleared by software. -00: Measurement of the start bit is used to detect the baud rate. -01: Falling edge to falling edge measurement. (the received frame must start with a single bit = 1 -> Frame = Start10xxxxxx) -10: 0x7F frame detection. -11: 0x55 frame detection This bit field can only be written when ABREN = 0 or the USART is disabled (UE=0).
pub type ABRMOD_R = crate::FieldReader;
///Field `ABRMOD` writer - ABRMOD\[1:0\]: Auto baud rate mode These bits are set and cleared by software. -00: Measurement of the start bit is used to detect the baud rate. -01: Falling edge to falling edge measurement. (the received frame must start with a single bit = 1 -> Frame = Start10xxxxxx) -10: 0x7F frame detection. -11: 0x55 frame detection This bit field can only be written when ABREN = 0 or the USART is disabled (UE=0).
pub type ABRMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RTOEN` reader - RTOEN: Receiver timeout enable This bit is set and cleared by software. -0: Receiver timeout feature disabled. -1: Receiver timeout feature enabled. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register).
pub type RTOEN_R = crate::BitReader;
///Field `RTOEN` writer - RTOEN: Receiver timeout enable This bit is set and cleared by software. -0: Receiver timeout feature disabled. -1: Receiver timeout feature enabled. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register).
pub type RTOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADD` reader - ADD\[7:0\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\[7:0\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)
pub type ADD_R = crate::FieldReader;
///Field `ADD` writer - ADD\[7:0\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\[7:0\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bit 0 - SLVEN: Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. -0: Slave mode disabled. -1: Slave mode enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value
    #[inline(always)]
    pub fn slven(&self) -> SLVEN_R {
        SLVEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - DIS_NSS When the DSI_NSS bit is set, the NSS pin input will be ignored. -0: SPI slave selection depends on NSS input pin. -1: SPI slave will be always selected and NSS input pin will be ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value
    #[inline(always)]
    pub fn dis_nss(&self) -> DIS_NSS_R {
        DIS_NSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LBDL: LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. -0: 10-bit break detection -1: 11-bit break detection This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LBDIE: LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). -0: Interrupt is inhibited -1: An interrupt is generated whenever LBDF=1 in the USART_ISR register
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - LBCL: Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. -0: The clock pulse of the last data bit is not output to the SCLK pin -1: The clock pulse of the last data bit is output to the SCLK pin Caution: The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPHA: Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see Figure 137 and Figure 138) -0: The first clock transition is the first data capture edge -1: The second clock transition is the first data capture edge This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPOL: Clock polarity This bit allows the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship -0: Steady low value on SCLK pin outside transmission window -1: Steady high value on SCLK pin outside transmission window This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CLKEN: Clock enable This bit allows the user to enable the SCLK pin. -0: SCLK pin disabled -1: SCLK pin enabled This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and forced by hardware to 0. Please refer to Section 23.4: USART implementation on page 483. Note: In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: - UE = 0 - SCEN = 1 - GTPR configuration - CLKEN= 1 - UE = 1
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - STOP\[1:0\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - LINEN: LIN mode enable This bit is set and cleared by software. -0: LIN mode disabled -1: LIN mode enabled The LIN mode enables the capability to send LIN Synch Breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
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
    ///Bit 20 - ABREN: Auto baud rate enable This bit is set and cleared by software. -0: Auto baud rate detection is disabled. -1: Auto baud rate detection is enabled.
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - ABRMOD\[1:0\]: Auto baud rate mode These bits are set and cleared by software. -00: Measurement of the start bit is used to detect the baud rate. -01: Falling edge to falling edge measurement. (the received frame must start with a single bit = 1 -> Frame = Start10xxxxxx) -10: 0x7F frame detection. -11: 0x55 frame detection This bit field can only be written when ABREN = 0 or the USART is disabled (UE=0).
    #[inline(always)]
    pub fn abrmod(&self) -> ABRMOD_R {
        ABRMOD_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - RTOEN: Receiver timeout enable This bit is set and cleared by software. -0: Receiver timeout feature disabled. -1: Receiver timeout feature enabled. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register).
    #[inline(always)]
    pub fn rtoen(&self) -> RTOEN_R {
        RTOEN_R::new(((self.bits >> 23) & 1) != 0)
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
            .field("slven", &self.slven())
            .field("dis_nss", &self.dis_nss())
            .field("addm7", &self.addm7())
            .field("lbdl", &self.lbdl())
            .field("lbdie", &self.lbdie())
            .field("lbcl", &self.lbcl())
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .field("clken", &self.clken())
            .field("stop", &self.stop())
            .field("linen", &self.linen())
            .field("swap", &self.swap())
            .field("rxinv", &self.rxinv())
            .field("txinv", &self.txinv())
            .field("datainv", &self.datainv())
            .field("msbfirst", &self.msbfirst())
            .field("abren", &self.abren())
            .field("abrmod", &self.abrmod())
            .field("rtoen", &self.rtoen())
            .field("add", &self.add())
            .finish()
    }
}
impl W {
    ///Bit 0 - SLVEN: Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. -0: Slave mode disabled. -1: Slave mode enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value
    #[inline(always)]
    pub fn slven(&mut self) -> SLVEN_W<'_, CR2rs> {
        SLVEN_W::new(self, 0)
    }
    ///Bit 3 - DIS_NSS When the DSI_NSS bit is set, the NSS pin input will be ignored. -0: SPI slave selection depends on NSS input pin. -1: SPI slave will be always selected and NSS input pin will be ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value
    #[inline(always)]
    pub fn dis_nss(&mut self) -> DIS_NSS_W<'_, CR2rs> {
        DIS_NSS_W::new(self, 3)
    }
    ///Bit 4 - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)
    #[inline(always)]
    pub fn addm7(&mut self) -> ADDM7_W<'_, CR2rs> {
        ADDM7_W::new(self, 4)
    }
    ///Bit 5 - LBDL: LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. -0: 10-bit break detection -1: 11-bit break detection This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn lbdl(&mut self) -> LBDL_W<'_, CR2rs> {
        LBDL_W::new(self, 5)
    }
    ///Bit 6 - LBDIE: LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). -0: Interrupt is inhibited -1: An interrupt is generated whenever LBDF=1 in the USART_ISR register
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W<'_, CR2rs> {
        LBDIE_W::new(self, 6)
    }
    ///Bit 8 - LBCL: Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. -0: The clock pulse of the last data bit is not output to the SCLK pin -1: The clock pulse of the last data bit is output to the SCLK pin Caution: The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn lbcl(&mut self) -> LBCL_W<'_, CR2rs> {
        LBCL_W::new(self, 8)
    }
    ///Bit 9 - CPHA: Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see Figure 137 and Figure 138) -0: The first clock transition is the first data capture edge -1: The second clock transition is the first data capture edge This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<'_, CR2rs> {
        CPHA_W::new(self, 9)
    }
    ///Bit 10 - CPOL: Clock polarity This bit allows the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship -0: Steady low value on SCLK pin outside transmission window -1: Steady high value on SCLK pin outside transmission window This bit can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<'_, CR2rs> {
        CPOL_W::new(self, 10)
    }
    ///Bit 11 - CLKEN: Clock enable This bit allows the user to enable the SCLK pin. -0: SCLK pin disabled -1: SCLK pin enabled This bit can only be written when the USART is disabled (UE=0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and forced by hardware to 0. Please refer to Section 23.4: USART implementation on page 483. Note: In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: - UE = 0 - SCEN = 1 - GTPR configuration - CLKEN= 1 - UE = 1
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<'_, CR2rs> {
        CLKEN_W::new(self, 11)
    }
    ///Bits 12:13 - STOP\[1:0\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<'_, CR2rs> {
        STOP_W::new(self, 12)
    }
    ///Bit 14 - LINEN: LIN mode enable This bit is set and cleared by software. -0: LIN mode disabled -1: LIN mode enabled The LIN mode enables the capability to send LIN Synch Breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bit field can only be written when the USART is disabled (UE=0).
    #[inline(always)]
    pub fn linen(&mut self) -> LINEN_W<'_, CR2rs> {
        LINEN_W::new(self, 14)
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
    ///Bit 20 - ABREN: Auto baud rate enable This bit is set and cleared by software. -0: Auto baud rate detection is disabled. -1: Auto baud rate detection is enabled.
    #[inline(always)]
    pub fn abren(&mut self) -> ABREN_W<'_, CR2rs> {
        ABREN_W::new(self, 20)
    }
    ///Bits 21:22 - ABRMOD\[1:0\]: Auto baud rate mode These bits are set and cleared by software. -00: Measurement of the start bit is used to detect the baud rate. -01: Falling edge to falling edge measurement. (the received frame must start with a single bit = 1 -> Frame = Start10xxxxxx) -10: 0x7F frame detection. -11: 0x55 frame detection This bit field can only be written when ABREN = 0 or the USART is disabled (UE=0).
    #[inline(always)]
    pub fn abrmod(&mut self) -> ABRMOD_W<'_, CR2rs> {
        ABRMOD_W::new(self, 21)
    }
    ///Bit 23 - RTOEN: Receiver timeout enable This bit is set and cleared by software. -0: Receiver timeout feature disabled. -1: Receiver timeout feature enabled. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register).
    #[inline(always)]
    pub fn rtoen(&mut self) -> RTOEN_W<'_, CR2rs> {
        RTOEN_W::new(self, 23)
    }
    ///Bits 24:31 - ADD\[7:0\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\[7:0\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, CR2rs> {
        ADD_W::new(self, 24)
    }
}
/**CR2 register

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#USART:CR2)*/
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
