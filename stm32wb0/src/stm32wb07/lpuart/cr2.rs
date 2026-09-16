///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDM7 {
    ///0: 4-bit address detection
    Bit4 = 0,
    ///1: 7-bit address detection
    Bit7 = 1,
}
impl From<ADDM7> for bool {
    #[inline(always)]
    fn from(variant: ADDM7) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDM7` reader - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)
pub type ADDM7_R = crate::BitReader<ADDM7>;
impl ADDM7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDM7 {
        match self.bits {
            false => ADDM7::Bit4,
            true => ADDM7::Bit7,
        }
    }
    ///4-bit address detection
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == ADDM7::Bit4
    }
    ///7-bit address detection
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADDM7::Bit7
    }
}
///Field `ADDM7` writer - ADDM7:7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. -0: 4-bit address detection -1: 7-bit address detection (in 8-bit data mode) This bit can only be written when the USART is disabled (UE=0)
pub type ADDM7_W<'a, REG> = crate::BitWriter<'a, REG, ADDM7>;
impl<'a, REG> ADDM7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///4-bit address detection
    #[inline(always)]
    pub fn bit4(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM7::Bit4)
    }
    ///7-bit address detection
    #[inline(always)]
    pub fn bit7(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM7::Bit7)
    }
}
/**STOP\[1:0\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP {
    ///0: 1 stop bit
    Stop1 = 0,
    ///2: 2 stop bit
    Stop2 = 2,
}
impl From<STOP> for u8 {
    #[inline(always)]
    fn from(variant: STOP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STOP {
    type Ux = u8;
}
impl crate::IsEnum for STOP {}
///Field `STOP` reader - STOP\[1:0\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0).
pub type STOP_R = crate::FieldReader<STOP>;
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STOP> {
        match self.bits {
            0 => Some(STOP::Stop1),
            2 => Some(STOP::Stop2),
            _ => None,
        }
    }
    ///1 stop bit
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STOP::Stop1
    }
    ///2 stop bit
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STOP::Stop2
    }
}
///Field `STOP` writer - STOP\[1:0\]: STOP bits These bits are used for programming the stop bits. -00: 1 stop bit -01: 0.5 stop bit. -10: 2 stop bits -11: 1.5 stop bits This bit field can only be written when the USART is disabled (UE=0).
pub type STOP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, STOP>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 stop bit
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop1)
    }
    ///2 stop bit
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop2)
    }
}
/**SWAP: Swap TX/RX pins This bit is set and cleared by software. -0: TX/RX pins are used as defined in standard pinout -1: The TX and RX pins functions are swapped. This allows to work in the case of a cross-wired connection to another UART. This bit field can only be written when the USART is disabled (UE=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP {
    ///0: TX/RX pins are used as defined in standard pinout
    Standard = 0,
    ///1: The TX and RX pins functions are swapped
    Swapped = 1,
}
impl From<SWAP> for bool {
    #[inline(always)]
    fn from(variant: SWAP) -> Self {
        variant as u8 != 0
    }
}
///Field `SWAP` reader - SWAP: Swap TX/RX pins This bit is set and cleared by software. -0: TX/RX pins are used as defined in standard pinout -1: The TX and RX pins functions are swapped. This allows to work in the case of a cross-wired connection to another UART. This bit field can only be written when the USART is disabled (UE=0).
pub type SWAP_R = crate::BitReader<SWAP>;
impl SWAP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWAP {
        match self.bits {
            false => SWAP::Standard,
            true => SWAP::Swapped,
        }
    }
    ///TX/RX pins are used as defined in standard pinout
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SWAP::Standard
    }
    ///The TX and RX pins functions are swapped
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == SWAP::Swapped
    }
}
///Field `SWAP` writer - SWAP: Swap TX/RX pins This bit is set and cleared by software. -0: TX/RX pins are used as defined in standard pinout -1: The TX and RX pins functions are swapped. This allows to work in the case of a cross-wired connection to another UART. This bit field can only be written when the USART is disabled (UE=0).
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG, SWAP>;
impl<'a, REG> SWAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX/RX pins are used as defined in standard pinout
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP::Standard)
    }
    ///The TX and RX pins functions are swapped
    #[inline(always)]
    pub fn swapped(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP::Swapped)
    }
}
/**RXINV: RX pin active level inversion This bit is set and cleared by software. -0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the RX line. This bit field can only be written when the USART is disabled (UE=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXINV {
    ///0: RX pin signal works using the standard logic levels
    Standard = 0,
    ///1: RX pin signal values are inverted
    Inverted = 1,
}
impl From<RXINV> for bool {
    #[inline(always)]
    fn from(variant: RXINV) -> Self {
        variant as u8 != 0
    }
}
///Field `RXINV` reader - RXINV: RX pin active level inversion This bit is set and cleared by software. -0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the RX line. This bit field can only be written when the USART is disabled (UE=0).
pub type RXINV_R = crate::BitReader<RXINV>;
impl RXINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXINV {
        match self.bits {
            false => RXINV::Standard,
            true => RXINV::Inverted,
        }
    }
    ///RX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == RXINV::Standard
    }
    ///RX pin signal values are inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RXINV::Inverted
    }
}
///Field `RXINV` writer - RXINV: RX pin active level inversion This bit is set and cleared by software. -0: RX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: RX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the RX line. This bit field can only be written when the USART is disabled (UE=0).
pub type RXINV_W<'a, REG> = crate::BitWriter<'a, REG, RXINV>;
impl<'a, REG> RXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(RXINV::Standard)
    }
    ///RX pin signal values are inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(RXINV::Inverted)
    }
}
/**TXINV: TX pin active level inversion This bit is set and cleared by software. -0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the TX line. This bit field can only be written when the USART is disabled (UE=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXINV {
    ///0: TX pin signal works using the standard logic levels
    Standard = 0,
    ///1: TX pin signal values are inverted
    Inverted = 1,
}
impl From<TXINV> for bool {
    #[inline(always)]
    fn from(variant: TXINV) -> Self {
        variant as u8 != 0
    }
}
///Field `TXINV` reader - TXINV: TX pin active level inversion This bit is set and cleared by software. -0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the TX line. This bit field can only be written when the USART is disabled (UE=0).
pub type TXINV_R = crate::BitReader<TXINV>;
impl TXINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXINV {
        match self.bits {
            false => TXINV::Standard,
            true => TXINV::Inverted,
        }
    }
    ///TX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TXINV::Standard
    }
    ///TX pin signal values are inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TXINV::Inverted
    }
}
///Field `TXINV` writer - TXINV: TX pin active level inversion This bit is set and cleared by software. -0: TX pin signal works using the standard logic levels (VDD =1/idle, Gnd=0/mark) -1: TX pin signal values are inverted. ((VDD =0/mark, Gnd=1/idle). This allows the use of an external inverter on the TX line. This bit field can only be written when the USART is disabled (UE=0).
pub type TXINV_W<'a, REG> = crate::BitWriter<'a, REG, TXINV>;
impl<'a, REG> TXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TX pin signal works using the standard logic levels
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(TXINV::Standard)
    }
    ///TX pin signal values are inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(TXINV::Inverted)
    }
}
/**DATAINV: Binary data inversion This bit is set and cleared by software. -0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L) -1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted. This bit field can only be written when the USART is disabled (UE=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAINV {
    ///0: Logical data from the data register are send/received in positive/direct logic
    Positive = 0,
    ///1: Logical data from the data register are send/received in negative/inverse logic
    Negative = 1,
}
impl From<DATAINV> for bool {
    #[inline(always)]
    fn from(variant: DATAINV) -> Self {
        variant as u8 != 0
    }
}
///Field `DATAINV` reader - DATAINV: Binary data inversion This bit is set and cleared by software. -0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L) -1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted. This bit field can only be written when the USART is disabled (UE=0).
pub type DATAINV_R = crate::BitReader<DATAINV>;
impl DATAINV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DATAINV {
        match self.bits {
            false => DATAINV::Positive,
            true => DATAINV::Negative,
        }
    }
    ///Logical data from the data register are send/received in positive/direct logic
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DATAINV::Positive
    }
    ///Logical data from the data register are send/received in negative/inverse logic
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DATAINV::Negative
    }
}
///Field `DATAINV` writer - DATAINV: Binary data inversion This bit is set and cleared by software. -0: Logical data from the data register are send/received in positive/direct logic. (1=H, 0=L) -1: Logical data from the data register are send/received in negative/inverse logic. (1=L, 0=H). The parity bit is also inverted. This bit field can only be written when the USART is disabled (UE=0).
pub type DATAINV_W<'a, REG> = crate::BitWriter<'a, REG, DATAINV>;
impl<'a, REG> DATAINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Logical data from the data register are send/received in positive/direct logic
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(DATAINV::Positive)
    }
    ///Logical data from the data register are send/received in negative/inverse logic
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(DATAINV::Negative)
    }
}
/**MSBFIRST: Most significant bit first This bit is set and cleared by software. -0: data is transmitted/received with data bit 0 first, following the start bit. -1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit. This bit field can only be written when the USART is disabled (UE=0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBFIRST {
    ///0: data is transmitted/received with data bit 0 first, following the start bit
    Lsb = 0,
    ///1: data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
    Msb = 1,
}
impl From<MSBFIRST> for bool {
    #[inline(always)]
    fn from(variant: MSBFIRST) -> Self {
        variant as u8 != 0
    }
}
///Field `MSBFIRST` reader - MSBFIRST: Most significant bit first This bit is set and cleared by software. -0: data is transmitted/received with data bit 0 first, following the start bit. -1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit. This bit field can only be written when the USART is disabled (UE=0).
pub type MSBFIRST_R = crate::BitReader<MSBFIRST>;
impl MSBFIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSBFIRST {
        match self.bits {
            false => MSBFIRST::Lsb,
            true => MSBFIRST::Msb,
        }
    }
    ///data is transmitted/received with data bit 0 first, following the start bit
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == MSBFIRST::Lsb
    }
    ///data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == MSBFIRST::Msb
    }
}
///Field `MSBFIRST` writer - MSBFIRST: Most significant bit first This bit is set and cleared by software. -0: data is transmitted/received with data bit 0 first, following the start bit. -1: data is transmitted/received with the MSB (bit 7/8) first, following the start bit. This bit field can only be written when the USART is disabled (UE=0).
pub type MSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG, MSBFIRST>;
impl<'a, REG> MSBFIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///data is transmitted/received with data bit 0 first, following the start bit
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(MSBFIRST::Lsb)
    }
    ///data is transmitted/received with MSB (bit 7/8/9) first, following the start bit
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(MSBFIRST::Msb)
    }
}
///Field `ADD` reader - ADD\[7:0\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\[7:0\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)
pub type ADD_R = crate::FieldReader;
///Field `ADD` writer - ADD\[7:0\]: Address of the USART node This bit-field gives the address of the USART node or a character code to be recognized. This is used in multiprocessor communication during Mute mode or Stop mode, for wakeup with 7- bit address mark detection. The MSB of the character sent by the transmitter should be equal to 1. It may also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8- bit) is compared to the ADD\[7:0\] value and CMF flag is set on match. This bit field can only be written when reception is disabled (RE = 0) or the USART is disabled (UE=0)
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#LPUART:CR2)*/
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
