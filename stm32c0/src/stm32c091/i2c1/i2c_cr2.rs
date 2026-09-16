///Register `I2C_CR2` reader
pub type R = crate::R<I2C_CR2rs>;
///Register `I2C_CR2` writer
pub type W = crate::W<I2C_CR2rs>;
///Field `SADD` reader - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\[7:1\] must be written with the 7-bit slave address to be sent. Bits SADD\[9\], SADD\[8\] and SADD\[0\] are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\[9:0\] must be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed.
pub type SADD_R = crate::FieldReader<u16>;
///Field `SADD` writer - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\[7:1\] must be written with the 7-bit slave address to be sent. Bits SADD\[9\], SADD\[8\] and SADD\[0\] are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\[9:0\] must be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed.
pub type SADD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
/**Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_WRN {
    ///0: Master requests a write transfer
    B0x0 = 0,
    ///1: Master requests a read transfer
    B0x1 = 1,
}
impl From<RD_WRN> for bool {
    #[inline(always)]
    fn from(variant: RD_WRN) -> Self {
        variant as u8 != 0
    }
}
///Field `RD_WRN` reader - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.
pub type RD_WRN_R = crate::BitReader<RD_WRN>;
impl RD_WRN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RD_WRN {
        match self.bits {
            false => RD_WRN::B0x0,
            true => RD_WRN::B0x1,
        }
    }
    ///Master requests a write transfer
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RD_WRN::B0x0
    }
    ///Master requests a read transfer
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RD_WRN::B0x1
    }
}
///Field `RD_WRN` writer - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.
pub type RD_WRN_W<'a, REG> = crate::BitWriter<'a, REG, RD_WRN>;
impl<'a, REG> RD_WRN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Master requests a write transfer
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RD_WRN::B0x0)
    }
    ///Master requests a read transfer
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RD_WRN::B0x1)
    }
}
/**10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD10 {
    ///0: The master operates in 7-bit addressing mode
    B0x0 = 0,
    ///1: The master operates in 10-bit addressing mode
    B0x1 = 1,
}
impl From<ADD10> for bool {
    #[inline(always)]
    fn from(variant: ADD10) -> Self {
        variant as u8 != 0
    }
}
///Field `ADD10` reader - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.
pub type ADD10_R = crate::BitReader<ADD10>;
impl ADD10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADD10 {
        match self.bits {
            false => ADD10::B0x0,
            true => ADD10::B0x1,
        }
    }
    ///The master operates in 7-bit addressing mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADD10::B0x0
    }
    ///The master operates in 10-bit addressing mode
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADD10::B0x1
    }
}
///Field `ADD10` writer - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.
pub type ADD10_W<'a, REG> = crate::BitWriter<'a, REG, ADD10>;
impl<'a, REG> ADD10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The master operates in 7-bit addressing mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADD10::B0x0)
    }
    ///The master operates in 10-bit addressing mode
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADD10::B0x1)
    }
}
/**10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEAD10R {
    ///0: The master sends the complete 10-bit slave address read sequence: Start + 2 bytes 10-bit address in write direction + restart + first seven bits of the 10-bit address in read direction.
    B0x0 = 0,
    ///1: The master sends only the first seven bits of the 10-bit address, followed by read direction.
    B0x1 = 1,
}
impl From<HEAD10R> for bool {
    #[inline(always)]
    fn from(variant: HEAD10R) -> Self {
        variant as u8 != 0
    }
}
///Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.
pub type HEAD10R_R = crate::BitReader<HEAD10R>;
impl HEAD10R_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HEAD10R {
        match self.bits {
            false => HEAD10R::B0x0,
            true => HEAD10R::B0x1,
        }
    }
    ///The master sends the complete 10-bit slave address read sequence: Start + 2 bytes 10-bit address in write direction + restart + first seven bits of the 10-bit address in read direction.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HEAD10R::B0x0
    }
    ///The master sends only the first seven bits of the 10-bit address, followed by read direction.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HEAD10R::B0x1
    }
}
///Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.
pub type HEAD10R_W<'a, REG> = crate::BitWriter<'a, REG, HEAD10R>;
impl<'a, REG> HEAD10R_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The master sends the complete 10-bit slave address read sequence: Start + 2 bytes 10-bit address in write direction + restart + first seven bits of the 10-bit address in read direction.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HEAD10R::B0x0)
    }
    ///The master sends only the first seven bits of the 10-bit address, followed by read direction.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HEAD10R::B0x1)
    }
}
/**Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START {
    ///0: No Start generation
    B0x0 = 0,
    ///1: Restart/Start generation:
    B0x1 = 1,
}
impl From<START> for bool {
    #[inline(always)]
    fn from(variant: START) -> Self {
        variant as u8 != 0
    }
}
///Field `START` reader - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set.
pub type START_R = crate::BitReader<START>;
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> START {
        match self.bits {
            false => START::B0x0,
            true => START::B0x1,
        }
    }
    ///No Start generation
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == START::B0x0
    }
    ///Restart/Start generation:
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == START::B0x1
    }
}
///Field `START` writer - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set.
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, START>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Start generation
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(START::B0x0)
    }
    ///Restart/Start generation:
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(START::B0x1)
    }
}
/**Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP {
    ///0: No Stop generation
    B0x0 = 0,
    ///1: Stop generation after current byte transfer
    B0x1 = 1,
}
impl From<STOP> for bool {
    #[inline(always)]
    fn from(variant: STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `STOP` reader - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect.
pub type STOP_R = crate::BitReader<STOP>;
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOP {
        match self.bits {
            false => STOP::B0x0,
            true => STOP::B0x1,
        }
    }
    ///No Stop generation
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == STOP::B0x0
    }
    ///Stop generation after current byte transfer
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == STOP::B0x1
    }
}
///Field `STOP` writer - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect.
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG, STOP>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Stop generation
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::B0x0)
    }
    ///Stop generation after current byte transfer
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::B0x1)
    }
}
/**NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used only in slave mode: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACK {
    ///0: an ACK is sent after current received byte.
    B0x0 = 0,
    ///1: a NACK is sent after current received byte.
    B0x1 = 1,
}
impl From<NACK> for bool {
    #[inline(always)]
    fn from(variant: NACK) -> Self {
        variant as u8 != 0
    }
}
///Field `NACK` reader - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used only in slave mode: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value.
pub type NACK_R = crate::BitReader<NACK>;
impl NACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NACK {
        match self.bits {
            false => NACK::B0x0,
            true => NACK::B0x1,
        }
    }
    ///an ACK is sent after current received byte.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NACK::B0x0
    }
    ///a NACK is sent after current received byte.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NACK::B0x1
    }
}
///Field `NACK` writer - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used only in slave mode: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value.
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG, NACK>;
impl<'a, REG> NACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///an ACK is sent after current received byte.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NACK::B0x0)
    }
    ///a NACK is sent after current received byte.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NACK::B0x1)
    }
}
///Field `NBYTES` reader - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is don t care in slave mode with SBC = 0. Note: Changing these bits when the START bit is set is not allowed.
pub type NBYTES_R = crate::FieldReader;
///Field `NBYTES` writer - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is don t care in slave mode with SBC = 0. Note: Changing these bits when the START bit is set is not allowed.
pub type NBYTES_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
/**NBYTES reload mode This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RELOAD {
    ///0: The transfer is completed after the NBYTES data transfer (STOP or RESTART follows).
    B0x0 = 0,
    ///1: The transfer is not completed after the NBYTES data transfer (NBYTES is reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low.
    B0x1 = 1,
}
impl From<RELOAD> for bool {
    #[inline(always)]
    fn from(variant: RELOAD) -> Self {
        variant as u8 != 0
    }
}
///Field `RELOAD` reader - NBYTES reload mode This bit is set and cleared by software.
pub type RELOAD_R = crate::BitReader<RELOAD>;
impl RELOAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RELOAD {
        match self.bits {
            false => RELOAD::B0x0,
            true => RELOAD::B0x1,
        }
    }
    ///The transfer is completed after the NBYTES data transfer (STOP or RESTART follows).
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RELOAD::B0x0
    }
    ///The transfer is not completed after the NBYTES data transfer (NBYTES is reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RELOAD::B0x1
    }
}
///Field `RELOAD` writer - NBYTES reload mode This bit is set and cleared by software.
pub type RELOAD_W<'a, REG> = crate::BitWriter<'a, REG, RELOAD>;
impl<'a, REG> RELOAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The transfer is completed after the NBYTES data transfer (STOP or RESTART follows).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RELOAD::B0x0)
    }
    ///The transfer is not completed after the NBYTES data transfer (NBYTES is reloaded). TCR flag is set when NBYTES data are transferred, stretching SCL low.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RELOAD::B0x1)
    }
}
/**Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOEND {
    ///0: software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low.
    B0x0 = 0,
    ///1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred.
    B0x1 = 1,
}
impl From<AUTOEND> for bool {
    #[inline(always)]
    fn from(variant: AUTOEND) -> Self {
        variant as u8 != 0
    }
}
///Field `AUTOEND` reader - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.
pub type AUTOEND_R = crate::BitReader<AUTOEND>;
impl AUTOEND_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AUTOEND {
        match self.bits {
            false => AUTOEND::B0x0,
            true => AUTOEND::B0x1,
        }
    }
    ///software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AUTOEND::B0x0
    }
    ///Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AUTOEND::B0x1
    }
}
///Field `AUTOEND` writer - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.
pub type AUTOEND_W<'a, REG> = crate::BitWriter<'a, REG, AUTOEND>;
impl<'a, REG> AUTOEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOEND::B0x0)
    }
    ///Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AUTOEND::B0x1)
    }
}
/**Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit has no effect when RELOAD is set, and in slave mode when SBC = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECBYTE {
    ///0: No PEC transfer
    B0x0 = 0,
    ///1: PEC transmission/reception is requested
    B0x1 = 1,
}
impl From<PECBYTE> for bool {
    #[inline(always)]
    fn from(variant: PECBYTE) -> Self {
        variant as u8 != 0
    }
}
///Field `PECBYTE` reader - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit has no effect when RELOAD is set, and in slave mode when SBC = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type PECBYTE_R = crate::BitReader<PECBYTE>;
impl PECBYTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PECBYTE {
        match self.bits {
            false => PECBYTE::B0x0,
            true => PECBYTE::B0x1,
        }
    }
    ///No PEC transfer
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PECBYTE::B0x0
    }
    ///PEC transmission/reception is requested
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PECBYTE::B0x1
    }
}
///Field `PECBYTE` writer - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit has no effect when RELOAD is set, and in slave mode when SBC = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type PECBYTE_W<'a, REG> = crate::BitWriter<'a, REG, PECBYTE>;
impl<'a, REG> PECBYTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No PEC transfer
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PECBYTE::B0x0)
    }
    ///PEC transmission/reception is requested
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PECBYTE::B0x1)
    }
}
impl R {
    ///Bits 0:9 - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\[7:1\] must be written with the 7-bit slave address to be sent. Bits SADD\[9\], SADD\[8\] and SADD\[0\] are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\[9:0\] must be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed.
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set.
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect.
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used only in slave mode: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value.
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is don t care in slave mode with SBC = 0. Note: Changing these bits when the START bit is set is not allowed.
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - NBYTES reload mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit has no effect when RELOAD is set, and in slave mode when SBC = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_CR2")
            .field("sadd", &self.sadd())
            .field("rd_wrn", &self.rd_wrn())
            .field("add10", &self.add10())
            .field("head10r", &self.head10r())
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("nack", &self.nack())
            .field("nbytes", &self.nbytes())
            .field("reload", &self.reload())
            .field("autoend", &self.autoend())
            .field("pecbyte", &self.pecbyte())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Slave address (master mode) In 7-bit addressing mode (ADD10 = 0): SADD\[7:1\] must be written with the 7-bit slave address to be sent. Bits SADD\[9\], SADD\[8\] and SADD\[0\] are don't care. In 10-bit addressing mode (ADD10 = 1): SADD\[9:0\] must be written with the 10-bit slave address to be sent. Note: Changing these bits when the START bit is set is not allowed.
    #[inline(always)]
    pub fn sadd(&mut self) -> SADD_W<'_, I2C_CR2rs> {
        SADD_W::new(self, 0)
    }
    ///Bit 10 - Transfer direction (master mode) Note: Changing this bit when the START bit is set is not allowed.
    #[inline(always)]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<'_, I2C_CR2rs> {
        RD_WRN_W::new(self, 10)
    }
    ///Bit 11 - 10-bit addressing mode (master mode) Note: Changing this bit when the START bit is set is not allowed.
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W<'_, I2C_CR2rs> {
        ADD10_W::new(self, 11)
    }
    ///Bit 12 - 10-bit address header only read direction (master receiver mode) Note: Changing this bit when the START bit is set is not allowed.
    #[inline(always)]
    pub fn head10r(&mut self) -> HEAD10R_W<'_, I2C_CR2rs> {
        HEAD10R_W::new(self, 12)
    }
    ///Bit 13 - Start generation This bit is set by software, and cleared by hardware after the Start followed by the address sequence is sent, by an arbitration loss, by an address matched in slave mode, by a timeout error detection, or when PE = 0. If the I2C is already in master mode with AUTOEND = 0, setting this bit generates a Repeated start condition when RELOAD = 0, after the end of the NBYTES transfer. Otherwise, setting this bit generates a START condition once the bus is free. Note: Writing 0 to this bit has no effect. Note: The START bit can be set even if the bus is BUSY or I2C is in slave mode. Note: This bit has no effect when RELOAD is set.
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, I2C_CR2rs> {
        START_W::new(self, 13)
    }
    ///Bit 14 - Stop generation (master mode) The bit is set by software, cleared by hardware when a STOP condition is detected, or when PE = 0. In master mode: Note: Writing 0 to this bit has no effect.
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<'_, I2C_CR2rs> {
        STOP_W::new(self, 14)
    }
    ///Bit 15 - NACK generation (slave mode) The bit is set by software, cleared by hardware when the NACK is sent, or when a STOP condition or an Address matched is received, or when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit is used only in slave mode: in master receiver mode, NACK is automatically generated after last byte preceding STOP or RESTART condition, whatever the NACK bit value. Note: When an overrun occurs in slave receiver NOSTRETCH mode, a NACK is automatically generated, whatever the NACK bit value. Note: When hardware PEC checking is enabled (PECBYTE = 1), the PEC acknowledge value does not depend on the NACK value.
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<'_, I2C_CR2rs> {
        NACK_W::new(self, 15)
    }
    ///Bits 16:23 - Number of bytes The number of bytes to be transmitted/received is programmed there. This field is don t care in slave mode with SBC = 0. Note: Changing these bits when the START bit is set is not allowed.
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W<'_, I2C_CR2rs> {
        NBYTES_W::new(self, 16)
    }
    ///Bit 24 - NBYTES reload mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<'_, I2C_CR2rs> {
        RELOAD_W::new(self, 24)
    }
    ///Bit 25 - Automatic end mode (master mode) This bit is set and cleared by software. Note: This bit has no effect in slave mode or when the RELOAD bit is set.
    #[inline(always)]
    pub fn autoend(&mut self) -> AUTOEND_W<'_, I2C_CR2rs> {
        AUTOEND_W::new(self, 25)
    }
    ///Bit 26 - Packet error checking byte This bit is set by software, and cleared by hardware when the PEC is transferred, or when a STOP condition or an Address matched is received, also when PE = 0. Note: Writing 0 to this bit has no effect. Note: This bit has no effect when RELOAD is set, and in slave mode when SBC = 0. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn pecbyte(&mut self) -> PECBYTE_W<'_, I2C_CR2rs> {
        PECBYTE_W::new(self, 26)
    }
}
/**I2C control register 2

You can [`read`](crate::Reg::read) this register and get [`i2c_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#I2C1:I2C_CR2)*/
pub struct I2C_CR2rs;
impl crate::RegisterSpec for I2C_CR2rs {
    type Ux = u32;
}
///`read()` method returns [`i2c_cr2::R`](R) reader structure
impl crate::Readable for I2C_CR2rs {}
///`write(|w| ..)` method takes [`i2c_cr2::W`](W) writer structure
impl crate::Writable for I2C_CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2C_CR2 to value 0
impl crate::Resettable for I2C_CR2rs {}
