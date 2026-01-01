///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `INSTRUCTION` reader - Instruction Instruction to be send to the external SPI device. This field can be written only when BUSY = 0.
pub type INSTRUCTION_R = crate::FieldReader;
///Field `INSTRUCTION` writer - Instruction Instruction to be send to the external SPI device. This field can be written only when BUSY = 0.
pub type INSTRUCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
/**Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IMODE {
    ///0: No instruction
    NoInstruction = 0,
    ///1: Instruction on a single line
    SingleLine = 1,
    ///2: Instruction on two lines
    TwoLines = 2,
    ///3: Instruction on four lines
    FourLines = 3,
}
impl From<IMODE> for u8 {
    #[inline(always)]
    fn from(variant: IMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IMODE {
    type Ux = u8;
}
impl crate::IsEnum for IMODE {}
///Field `IMODE` reader - Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0.
pub type IMODE_R = crate::FieldReader<IMODE>;
impl IMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IMODE {
        match self.bits {
            0 => IMODE::NoInstruction,
            1 => IMODE::SingleLine,
            2 => IMODE::TwoLines,
            3 => IMODE::FourLines,
            _ => unreachable!(),
        }
    }
    ///No instruction
    #[inline(always)]
    pub fn is_no_instruction(&self) -> bool {
        *self == IMODE::NoInstruction
    }
    ///Instruction on a single line
    #[inline(always)]
    pub fn is_single_line(&self) -> bool {
        *self == IMODE::SingleLine
    }
    ///Instruction on two lines
    #[inline(always)]
    pub fn is_two_lines(&self) -> bool {
        *self == IMODE::TwoLines
    }
    ///Instruction on four lines
    #[inline(always)]
    pub fn is_four_lines(&self) -> bool {
        *self == IMODE::FourLines
    }
}
///Field `IMODE` writer - Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0.
pub type IMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IMODE, crate::Safe>;
impl<'a, REG> IMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No instruction
    #[inline(always)]
    pub fn no_instruction(self) -> &'a mut crate::W<REG> {
        self.variant(IMODE::NoInstruction)
    }
    ///Instruction on a single line
    #[inline(always)]
    pub fn single_line(self) -> &'a mut crate::W<REG> {
        self.variant(IMODE::SingleLine)
    }
    ///Instruction on two lines
    #[inline(always)]
    pub fn two_lines(self) -> &'a mut crate::W<REG> {
        self.variant(IMODE::TwoLines)
    }
    ///Instruction on four lines
    #[inline(always)]
    pub fn four_lines(self) -> &'a mut crate::W<REG> {
        self.variant(IMODE::FourLines)
    }
}
/**Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADMODE {
    ///0: No address
    NoAddress = 0,
    ///1: Address on a single line
    SingleLine = 1,
    ///2: Address on two lines
    TwoLines = 2,
    ///3: Address on four lines
    FourLines = 3,
}
impl From<ADMODE> for u8 {
    #[inline(always)]
    fn from(variant: ADMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADMODE {
    type Ux = u8;
}
impl crate::IsEnum for ADMODE {}
///Field `ADMODE` reader - Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0.
pub type ADMODE_R = crate::FieldReader<ADMODE>;
impl ADMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADMODE {
        match self.bits {
            0 => ADMODE::NoAddress,
            1 => ADMODE::SingleLine,
            2 => ADMODE::TwoLines,
            3 => ADMODE::FourLines,
            _ => unreachable!(),
        }
    }
    ///No address
    #[inline(always)]
    pub fn is_no_address(&self) -> bool {
        *self == ADMODE::NoAddress
    }
    ///Address on a single line
    #[inline(always)]
    pub fn is_single_line(&self) -> bool {
        *self == ADMODE::SingleLine
    }
    ///Address on two lines
    #[inline(always)]
    pub fn is_two_lines(&self) -> bool {
        *self == ADMODE::TwoLines
    }
    ///Address on four lines
    #[inline(always)]
    pub fn is_four_lines(&self) -> bool {
        *self == ADMODE::FourLines
    }
}
///Field `ADMODE` writer - Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0.
pub type ADMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADMODE, crate::Safe>;
impl<'a, REG> ADMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No address
    #[inline(always)]
    pub fn no_address(self) -> &'a mut crate::W<REG> {
        self.variant(ADMODE::NoAddress)
    }
    ///Address on a single line
    #[inline(always)]
    pub fn single_line(self) -> &'a mut crate::W<REG> {
        self.variant(ADMODE::SingleLine)
    }
    ///Address on two lines
    #[inline(always)]
    pub fn two_lines(self) -> &'a mut crate::W<REG> {
        self.variant(ADMODE::TwoLines)
    }
    ///Address on four lines
    #[inline(always)]
    pub fn four_lines(self) -> &'a mut crate::W<REG> {
        self.variant(ADMODE::FourLines)
    }
}
/**Address size This bit defines address size: This field can be written only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADSIZE {
    ///0: 8-bit address
    Bit8 = 0,
    ///1: 16-bit address
    Bit16 = 1,
    ///2: 24-bit address
    Bit24 = 2,
    ///3: 32-bit address
    Bit32 = 3,
}
impl From<ADSIZE> for u8 {
    #[inline(always)]
    fn from(variant: ADSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADSIZE {
    type Ux = u8;
}
impl crate::IsEnum for ADSIZE {}
///Field `ADSIZE` reader - Address size This bit defines address size: This field can be written only when BUSY = 0.
pub type ADSIZE_R = crate::FieldReader<ADSIZE>;
impl ADSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADSIZE {
        match self.bits {
            0 => ADSIZE::Bit8,
            1 => ADSIZE::Bit16,
            2 => ADSIZE::Bit24,
            3 => ADSIZE::Bit32,
            _ => unreachable!(),
        }
    }
    ///8-bit address
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == ADSIZE::Bit8
    }
    ///16-bit address
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == ADSIZE::Bit16
    }
    ///24-bit address
    #[inline(always)]
    pub fn is_bit24(&self) -> bool {
        *self == ADSIZE::Bit24
    }
    ///32-bit address
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == ADSIZE::Bit32
    }
}
///Field `ADSIZE` writer - Address size This bit defines address size: This field can be written only when BUSY = 0.
pub type ADSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ADSIZE, crate::Safe>;
impl<'a, REG> ADSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8-bit address
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(ADSIZE::Bit8)
    }
    ///16-bit address
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(ADSIZE::Bit16)
    }
    ///24-bit address
    #[inline(always)]
    pub fn bit24(self) -> &'a mut crate::W<REG> {
        self.variant(ADSIZE::Bit24)
    }
    ///32-bit address
    #[inline(always)]
    pub fn bit32(self) -> &'a mut crate::W<REG> {
        self.variant(ADSIZE::Bit32)
    }
}
/**Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABMODE {
    ///0: No alternate bytes
    NoAlternateBytes = 0,
    ///1: Alternate bytes on a single line
    SingleLine = 1,
    ///2: Alternate bytes on two lines
    TwoLines = 2,
    ///3: Alternate bytes on four lines
    FourLines = 3,
}
impl From<ABMODE> for u8 {
    #[inline(always)]
    fn from(variant: ABMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ABMODE {
    type Ux = u8;
}
impl crate::IsEnum for ABMODE {}
///Field `ABMODE` reader - Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0.
pub type ABMODE_R = crate::FieldReader<ABMODE>;
impl ABMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ABMODE {
        match self.bits {
            0 => ABMODE::NoAlternateBytes,
            1 => ABMODE::SingleLine,
            2 => ABMODE::TwoLines,
            3 => ABMODE::FourLines,
            _ => unreachable!(),
        }
    }
    ///No alternate bytes
    #[inline(always)]
    pub fn is_no_alternate_bytes(&self) -> bool {
        *self == ABMODE::NoAlternateBytes
    }
    ///Alternate bytes on a single line
    #[inline(always)]
    pub fn is_single_line(&self) -> bool {
        *self == ABMODE::SingleLine
    }
    ///Alternate bytes on two lines
    #[inline(always)]
    pub fn is_two_lines(&self) -> bool {
        *self == ABMODE::TwoLines
    }
    ///Alternate bytes on four lines
    #[inline(always)]
    pub fn is_four_lines(&self) -> bool {
        *self == ABMODE::FourLines
    }
}
///Field `ABMODE` writer - Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0.
pub type ABMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ABMODE, crate::Safe>;
impl<'a, REG> ABMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No alternate bytes
    #[inline(always)]
    pub fn no_alternate_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(ABMODE::NoAlternateBytes)
    }
    ///Alternate bytes on a single line
    #[inline(always)]
    pub fn single_line(self) -> &'a mut crate::W<REG> {
        self.variant(ABMODE::SingleLine)
    }
    ///Alternate bytes on two lines
    #[inline(always)]
    pub fn two_lines(self) -> &'a mut crate::W<REG> {
        self.variant(ABMODE::TwoLines)
    }
    ///Alternate bytes on four lines
    #[inline(always)]
    pub fn four_lines(self) -> &'a mut crate::W<REG> {
        self.variant(ABMODE::FourLines)
    }
}
/**Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABSIZE {
    ///0: 8-bit alternate byte
    Bit8 = 0,
    ///1: 16-bit alternate bytes
    Bit16 = 1,
    ///2: 24-bit alternate bytes
    Bit24 = 2,
    ///3: 32-bit alternate bytes
    Bit32 = 3,
}
impl From<ABSIZE> for u8 {
    #[inline(always)]
    fn from(variant: ABSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ABSIZE {
    type Ux = u8;
}
impl crate::IsEnum for ABSIZE {}
///Field `ABSIZE` reader - Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0.
pub type ABSIZE_R = crate::FieldReader<ABSIZE>;
impl ABSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ABSIZE {
        match self.bits {
            0 => ABSIZE::Bit8,
            1 => ABSIZE::Bit16,
            2 => ABSIZE::Bit24,
            3 => ABSIZE::Bit32,
            _ => unreachable!(),
        }
    }
    ///8-bit alternate byte
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == ABSIZE::Bit8
    }
    ///16-bit alternate bytes
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == ABSIZE::Bit16
    }
    ///24-bit alternate bytes
    #[inline(always)]
    pub fn is_bit24(&self) -> bool {
        *self == ABSIZE::Bit24
    }
    ///32-bit alternate bytes
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == ABSIZE::Bit32
    }
}
///Field `ABSIZE` writer - Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0.
pub type ABSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ABSIZE, crate::Safe>;
impl<'a, REG> ABSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8-bit alternate byte
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(ABSIZE::Bit8)
    }
    ///16-bit alternate bytes
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(ABSIZE::Bit16)
    }
    ///24-bit alternate bytes
    #[inline(always)]
    pub fn bit24(self) -> &'a mut crate::W<REG> {
        self.variant(ABSIZE::Bit24)
    }
    ///32-bit alternate bytes
    #[inline(always)]
    pub fn bit32(self) -> &'a mut crate::W<REG> {
        self.variant(ABSIZE::Bit32)
    }
}
///Field `DCYC` reader - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DDR modes, it specifies a number of CLK cycles (0-31). This field can be written only when BUSY = 0.
pub type DCYC_R = crate::FieldReader;
///Field `DCYC` writer - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DDR modes, it specifies a number of CLK cycles (0-31). This field can be written only when BUSY = 0.
pub type DCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
/**Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMODE {
    ///0: No data
    NoData = 0,
    ///1: Data on a single line
    SingleLine = 1,
    ///2: Data on two lines
    TwoLines = 2,
    ///3: Data on four lines
    FourLines = 3,
}
impl From<DMODE> for u8 {
    #[inline(always)]
    fn from(variant: DMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMODE {
    type Ux = u8;
}
impl crate::IsEnum for DMODE {}
///Field `DMODE` reader - Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0.
pub type DMODE_R = crate::FieldReader<DMODE>;
impl DMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMODE {
        match self.bits {
            0 => DMODE::NoData,
            1 => DMODE::SingleLine,
            2 => DMODE::TwoLines,
            3 => DMODE::FourLines,
            _ => unreachable!(),
        }
    }
    ///No data
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == DMODE::NoData
    }
    ///Data on a single line
    #[inline(always)]
    pub fn is_single_line(&self) -> bool {
        *self == DMODE::SingleLine
    }
    ///Data on two lines
    #[inline(always)]
    pub fn is_two_lines(&self) -> bool {
        *self == DMODE::TwoLines
    }
    ///Data on four lines
    #[inline(always)]
    pub fn is_four_lines(&self) -> bool {
        *self == DMODE::FourLines
    }
}
///Field `DMODE` writer - Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0.
pub type DMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DMODE, crate::Safe>;
impl<'a, REG> DMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No data
    #[inline(always)]
    pub fn no_data(self) -> &'a mut crate::W<REG> {
        self.variant(DMODE::NoData)
    }
    ///Data on a single line
    #[inline(always)]
    pub fn single_line(self) -> &'a mut crate::W<REG> {
        self.variant(DMODE::SingleLine)
    }
    ///Data on two lines
    #[inline(always)]
    pub fn two_lines(self) -> &'a mut crate::W<REG> {
        self.variant(DMODE::TwoLines)
    }
    ///Data on four lines
    #[inline(always)]
    pub fn four_lines(self) -> &'a mut crate::W<REG> {
        self.variant(DMODE::FourLines)
    }
}
/**Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FMODE {
    ///0: Indirect write mode
    IndirectWrite = 0,
    ///1: Indirect read mode
    IndirectRead = 1,
    ///2: Automatic polling mode
    AutomaticPolling = 2,
    ///3: Memory-mapped mode
    MemoryMapped = 3,
}
impl From<FMODE> for u8 {
    #[inline(always)]
    fn from(variant: FMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FMODE {
    type Ux = u8;
}
impl crate::IsEnum for FMODE {}
///Field `FMODE` reader - Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0.
pub type FMODE_R = crate::FieldReader<FMODE>;
impl FMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMODE {
        match self.bits {
            0 => FMODE::IndirectWrite,
            1 => FMODE::IndirectRead,
            2 => FMODE::AutomaticPolling,
            3 => FMODE::MemoryMapped,
            _ => unreachable!(),
        }
    }
    ///Indirect write mode
    #[inline(always)]
    pub fn is_indirect_write(&self) -> bool {
        *self == FMODE::IndirectWrite
    }
    ///Indirect read mode
    #[inline(always)]
    pub fn is_indirect_read(&self) -> bool {
        *self == FMODE::IndirectRead
    }
    ///Automatic polling mode
    #[inline(always)]
    pub fn is_automatic_polling(&self) -> bool {
        *self == FMODE::AutomaticPolling
    }
    ///Memory-mapped mode
    #[inline(always)]
    pub fn is_memory_mapped(&self) -> bool {
        *self == FMODE::MemoryMapped
    }
}
///Field `FMODE` writer - Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0.
pub type FMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FMODE, crate::Safe>;
impl<'a, REG> FMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Indirect write mode
    #[inline(always)]
    pub fn indirect_write(self) -> &'a mut crate::W<REG> {
        self.variant(FMODE::IndirectWrite)
    }
    ///Indirect read mode
    #[inline(always)]
    pub fn indirect_read(self) -> &'a mut crate::W<REG> {
        self.variant(FMODE::IndirectRead)
    }
    ///Automatic polling mode
    #[inline(always)]
    pub fn automatic_polling(self) -> &'a mut crate::W<REG> {
        self.variant(FMODE::AutomaticPolling)
    }
    ///Memory-mapped mode
    #[inline(always)]
    pub fn memory_mapped(self) -> &'a mut crate::W<REG> {
        self.variant(FMODE::MemoryMapped)
    }
}
/**Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIOO {
    ///0: Send instruction on every transaction
    SendEveryTransaction = 0,
    ///1: Send instruction only for the first command
    SendFirstCommand = 1,
}
impl From<SIOO> for bool {
    #[inline(always)]
    fn from(variant: SIOO) -> Self {
        variant as u8 != 0
    }
}
///Field `SIOO` reader - Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0.
pub type SIOO_R = crate::BitReader<SIOO>;
impl SIOO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SIOO {
        match self.bits {
            false => SIOO::SendEveryTransaction,
            true => SIOO::SendFirstCommand,
        }
    }
    ///Send instruction on every transaction
    #[inline(always)]
    pub fn is_send_every_transaction(&self) -> bool {
        *self == SIOO::SendEveryTransaction
    }
    ///Send instruction only for the first command
    #[inline(always)]
    pub fn is_send_first_command(&self) -> bool {
        *self == SIOO::SendFirstCommand
    }
}
///Field `SIOO` writer - Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0.
pub type SIOO_W<'a, REG> = crate::BitWriter<'a, REG, SIOO>;
impl<'a, REG> SIOO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Send instruction on every transaction
    #[inline(always)]
    pub fn send_every_transaction(self) -> &'a mut crate::W<REG> {
        self.variant(SIOO::SendEveryTransaction)
    }
    ///Send instruction only for the first command
    #[inline(always)]
    pub fn send_first_command(self) -> &'a mut crate::W<REG> {
        self.variant(SIOO::SendFirstCommand)
    }
}
/**DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DHHC {
    ///0: Delay the data output using analog delay
    NoDelay = 0,
    ///1: Delay the data output by 1/4 of a QUADSPI output clock cycle.
    Delayed = 1,
}
impl From<DHHC> for bool {
    #[inline(always)]
    fn from(variant: DHHC) -> Self {
        variant as u8 != 0
    }
}
///Field `DHHC` reader - DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0.
pub type DHHC_R = crate::BitReader<DHHC>;
impl DHHC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DHHC {
        match self.bits {
            false => DHHC::NoDelay,
            true => DHHC::Delayed,
        }
    }
    ///Delay the data output using analog delay
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == DHHC::NoDelay
    }
    ///Delay the data output by 1/4 of a QUADSPI output clock cycle.
    #[inline(always)]
    pub fn is_delayed(&self) -> bool {
        *self == DHHC::Delayed
    }
}
///Field `DHHC` writer - DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0.
pub type DHHC_W<'a, REG> = crate::BitWriter<'a, REG, DHHC>;
impl<'a, REG> DHHC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Delay the data output using analog delay
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut crate::W<REG> {
        self.variant(DHHC::NoDelay)
    }
    ///Delay the data output by 1/4 of a QUADSPI output clock cycle.
    #[inline(always)]
    pub fn delayed(self) -> &'a mut crate::W<REG> {
        self.variant(DHHC::Delayed)
    }
}
/**Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDRM {
    ///0: DDR Mode disabled
    Disabled = 0,
    ///1: DDR Mode enabled
    Enabled = 1,
}
impl From<DDRM> for bool {
    #[inline(always)]
    fn from(variant: DDRM) -> Self {
        variant as u8 != 0
    }
}
///Field `DDRM` reader - Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0.
pub type DDRM_R = crate::BitReader<DDRM>;
impl DDRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DDRM {
        match self.bits {
            false => DDRM::Disabled,
            true => DDRM::Enabled,
        }
    }
    ///DDR Mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDRM::Disabled
    }
    ///DDR Mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DDRM::Enabled
    }
}
///Field `DDRM` writer - Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0.
pub type DDRM_W<'a, REG> = crate::BitWriter<'a, REG, DDRM>;
impl<'a, REG> DDRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DDR Mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDRM::Disabled)
    }
    ///DDR Mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DDRM::Enabled)
    }
}
impl R {
    ///Bits 0:7 - Instruction Instruction to be send to the external SPI device. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Address size This bit defines address size: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:22 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DDR modes, it specifies a number of CLK cycles (0-31). This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn dcyc(&self) -> DCYC_R {
        DCYC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 24:25 - Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn fmode(&self) -> FMODE_R {
        FMODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bit 28 - Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn sioo(&self) -> SIOO_R {
        SIOO_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn dhhc(&self) -> DHHC_R {
        DHHC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn ddrm(&self) -> DDRM_R {
        DDRM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("instruction", &self.instruction())
            .field("imode", &self.imode())
            .field("admode", &self.admode())
            .field("adsize", &self.adsize())
            .field("abmode", &self.abmode())
            .field("absize", &self.absize())
            .field("dcyc", &self.dcyc())
            .field("dmode", &self.dmode())
            .field("fmode", &self.fmode())
            .field("sioo", &self.sioo())
            .field("dhhc", &self.dhhc())
            .field("ddrm", &self.ddrm())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Instruction Instruction to be send to the external SPI device. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn instruction(&mut self) -> INSTRUCTION_W<'_, CCRrs> {
        INSTRUCTION_W::new(self, 0)
    }
    ///Bits 8:9 - Instruction mode This field defines the instruction phase mode of operation: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn imode(&mut self) -> IMODE_W<'_, CCRrs> {
        IMODE_W::new(self, 8)
    }
    ///Bits 10:11 - Address mode This field defines the address phase mode of operation: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn admode(&mut self) -> ADMODE_W<'_, CCRrs> {
        ADMODE_W::new(self, 10)
    }
    ///Bits 12:13 - Address size This bit defines address size: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn adsize(&mut self) -> ADSIZE_W<'_, CCRrs> {
        ADSIZE_W::new(self, 12)
    }
    ///Bits 14:15 - Alternate bytes mode This field defines the alternate-bytes phase mode of operation: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn abmode(&mut self) -> ABMODE_W<'_, CCRrs> {
        ABMODE_W::new(self, 14)
    }
    ///Bits 16:17 - Alternate bytes size This bit defines alternate bytes size: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn absize(&mut self) -> ABSIZE_W<'_, CCRrs> {
        ABSIZE_W::new(self, 16)
    }
    ///Bits 18:22 - Number of dummy cycles This field defines the duration of the dummy phase. In both SDR and DDR modes, it specifies a number of CLK cycles (0-31). This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn dcyc(&mut self) -> DCYC_W<'_, CCRrs> {
        DCYC_W::new(self, 18)
    }
    ///Bits 24:25 - Data mode This field defines the data phases mode of operation: This field also determines the dummy phase mode of operation. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn dmode(&mut self) -> DMODE_W<'_, CCRrs> {
        DMODE_W::new(self, 24)
    }
    ///Bits 26:27 - Functional mode This field defines the QUADSPI functional mode of operation. If DMAEN = 1 already, then the DMA controller for the corresponding channel must be disabled before changing the FMODE value. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn fmode(&mut self) -> FMODE_W<'_, CCRrs> {
        FMODE_W::new(self, 26)
    }
    ///Bit 28 - Send instruction only once mode See Section15.3.11: Sending the instruction only once on page13. This bit has no effect when IMODE = 00. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn sioo(&mut self) -> SIOO_W<'_, CCRrs> {
        SIOO_W::new(self, 28)
    }
    ///Bit 30 - DDR hold Delay the data output by 1/4 of the QUADSPI output clock cycle in DDR mode: This feature is only active in DDR mode. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn dhhc(&mut self) -> DHHC_W<'_, CCRrs> {
        DHHC_W::new(self, 30)
    }
    ///Bit 31 - Double data rate mode This bit sets the DDR mode for the address, alternate byte and data phase: This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn ddrm(&mut self) -> DDRM_W<'_, CCRrs> {
        DDRM_W::new(self, 31)
    }
}
/**QUADSPI communication configuration register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#QUADSPI:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCRrs {}
