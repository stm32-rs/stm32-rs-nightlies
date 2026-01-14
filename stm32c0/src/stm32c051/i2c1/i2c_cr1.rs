///Register `I2C_CR1` reader
pub type R = crate::R<I2C_CR1rs>;
///Register `I2C_CR1` writer
pub type W = crate::W<I2C_CR1rs>;
/**Peripheral enable Note: When PE = 0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least three APB clock cycles.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE {
    ///0: Peripheral disabled
    B0x0 = 0,
    ///1: Peripheral enabled
    B0x1 = 1,
}
impl From<PE> for bool {
    #[inline(always)]
    fn from(variant: PE) -> Self {
        variant as u8 != 0
    }
}
///Field `PE` reader - Peripheral enable Note: When PE = 0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least three APB clock cycles.
pub type PE_R = crate::BitReader<PE>;
impl PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PE {
        match self.bits {
            false => PE::B0x0,
            true => PE::B0x1,
        }
    }
    ///Peripheral disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PE::B0x0
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PE::B0x1
    }
}
///Field `PE` writer - Peripheral enable Note: When PE = 0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least three APB clock cycles.
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG, PE>;
impl<'a, REG> PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PE::B0x0)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PE::B0x1)
    }
}
/**TX interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXIE {
    ///0: Transmit (TXIS) interrupt disabled
    B0x0 = 0,
    ///1: Transmit (TXIS) interrupt enabled
    B0x1 = 1,
}
impl From<TXIE> for bool {
    #[inline(always)]
    fn from(variant: TXIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXIE` reader - TX interrupt enable
pub type TXIE_R = crate::BitReader<TXIE>;
impl TXIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXIE {
        match self.bits {
            false => TXIE::B0x0,
            true => TXIE::B0x1,
        }
    }
    ///Transmit (TXIS) interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXIE::B0x0
    }
    ///Transmit (TXIS) interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXIE::B0x1
    }
}
///Field `TXIE` writer - TX interrupt enable
pub type TXIE_W<'a, REG> = crate::BitWriter<'a, REG, TXIE>;
impl<'a, REG> TXIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmit (TXIS) interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXIE::B0x0)
    }
    ///Transmit (TXIS) interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXIE::B0x1)
    }
}
/**RX interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXIE {
    ///0: Receive (RXNE) interrupt disabled
    B0x0 = 0,
    ///1: Receive (RXNE) interrupt enabled
    B0x1 = 1,
}
impl From<RXIE> for bool {
    #[inline(always)]
    fn from(variant: RXIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXIE` reader - RX interrupt enable
pub type RXIE_R = crate::BitReader<RXIE>;
impl RXIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXIE {
        match self.bits {
            false => RXIE::B0x0,
            true => RXIE::B0x1,
        }
    }
    ///Receive (RXNE) interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXIE::B0x0
    }
    ///Receive (RXNE) interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXIE::B0x1
    }
}
///Field `RXIE` writer - RX interrupt enable
pub type RXIE_W<'a, REG> = crate::BitWriter<'a, REG, RXIE>;
impl<'a, REG> RXIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receive (RXNE) interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXIE::B0x0)
    }
    ///Receive (RXNE) interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXIE::B0x1)
    }
}
/**Address match interrupt enable (slave only)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRIE {
    ///0: Address match (ADDR) interrupts disabled
    B0x0 = 0,
    ///1: Address match (ADDR) interrupts enabled
    B0x1 = 1,
}
impl From<ADDRIE> for bool {
    #[inline(always)]
    fn from(variant: ADDRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDRIE` reader - Address match interrupt enable (slave only)
pub type ADDRIE_R = crate::BitReader<ADDRIE>;
impl ADDRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDRIE {
        match self.bits {
            false => ADDRIE::B0x0,
            true => ADDRIE::B0x1,
        }
    }
    ///Address match (ADDR) interrupts disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADDRIE::B0x0
    }
    ///Address match (ADDR) interrupts enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADDRIE::B0x1
    }
}
///Field `ADDRIE` writer - Address match interrupt enable (slave only)
pub type ADDRIE_W<'a, REG> = crate::BitWriter<'a, REG, ADDRIE>;
impl<'a, REG> ADDRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Address match (ADDR) interrupts disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRIE::B0x0)
    }
    ///Address match (ADDR) interrupts enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRIE::B0x1)
    }
}
/**Not acknowledge received interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKIE {
    ///0: Not acknowledge (NACKF) received interrupts disabled
    B0x0 = 0,
    ///1: Not acknowledge (NACKF) received interrupts enabled
    B0x1 = 1,
}
impl From<NACKIE> for bool {
    #[inline(always)]
    fn from(variant: NACKIE) -> Self {
        variant as u8 != 0
    }
}
///Field `NACKIE` reader - Not acknowledge received interrupt enable
pub type NACKIE_R = crate::BitReader<NACKIE>;
impl NACKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NACKIE {
        match self.bits {
            false => NACKIE::B0x0,
            true => NACKIE::B0x1,
        }
    }
    ///Not acknowledge (NACKF) received interrupts disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NACKIE::B0x0
    }
    ///Not acknowledge (NACKF) received interrupts enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NACKIE::B0x1
    }
}
///Field `NACKIE` writer - Not acknowledge received interrupt enable
pub type NACKIE_W<'a, REG> = crate::BitWriter<'a, REG, NACKIE>;
impl<'a, REG> NACKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Not acknowledge (NACKF) received interrupts disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NACKIE::B0x0)
    }
    ///Not acknowledge (NACKF) received interrupts enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NACKIE::B0x1)
    }
}
/**Stop detection interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPIE {
    ///0: Stop detection (STOPF) interrupt disabled
    B0x0 = 0,
    ///1: Stop detection (STOPF) interrupt enabled
    B0x1 = 1,
}
impl From<STOPIE> for bool {
    #[inline(always)]
    fn from(variant: STOPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `STOPIE` reader - Stop detection interrupt enable
pub type STOPIE_R = crate::BitReader<STOPIE>;
impl STOPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOPIE {
        match self.bits {
            false => STOPIE::B0x0,
            true => STOPIE::B0x1,
        }
    }
    ///Stop detection (STOPF) interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == STOPIE::B0x0
    }
    ///Stop detection (STOPF) interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == STOPIE::B0x1
    }
}
///Field `STOPIE` writer - Stop detection interrupt enable
pub type STOPIE_W<'a, REG> = crate::BitWriter<'a, REG, STOPIE>;
impl<'a, REG> STOPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stop detection (STOPF) interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(STOPIE::B0x0)
    }
    ///Stop detection (STOPF) interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(STOPIE::B0x1)
    }
}
/**Transfer complete interrupt enable Note: Any of these events generates an interrupt: Note: Transfer complete (TC) Note: Transfer complete reload (TCR)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    ///0: Transfer complete interrupt disabled
    B0x0 = 0,
    ///1: Transfer complete interrupt enabled
    B0x1 = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIE` reader - Transfer complete interrupt enable Note: Any of these events generates an interrupt: Note: Transfer complete (TC) Note: Transfer complete reload (TCR)
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::B0x0,
            true => TCIE::B0x1,
        }
    }
    ///Transfer complete interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIE::B0x0
    }
    ///Transfer complete interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIE::B0x1
    }
}
///Field `TCIE` writer - Transfer complete interrupt enable Note: Any of these events generates an interrupt: Note: Transfer complete (TC) Note: Transfer complete reload (TCR)
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer complete interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::B0x0)
    }
    ///Transfer complete interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::B0x1)
    }
}
/**Error interrupts enable Note: Any of these errors generates an interrupt: Note: Arbitration loss (ARLO) Note: Bus error detection (BERR) Note: Overrun/underrun (OVR) Note: Timeout detection (TIMEOUT) Note: PEC error detection (PECERR) Note: Alert pin event detection (ALERT)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    ///0: Error detection interrupts disabled
    B0x0 = 0,
    ///1: Error detection interrupts enabled
    B0x1 = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRIE` reader - Error interrupts enable Note: Any of these errors generates an interrupt: Note: Arbitration loss (ARLO) Note: Bus error detection (BERR) Note: Overrun/underrun (OVR) Note: Timeout detection (TIMEOUT) Note: PEC error detection (PECERR) Note: Alert pin event detection (ALERT)
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::B0x0,
            true => ERRIE::B0x1,
        }
    }
    ///Error detection interrupts disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ERRIE::B0x0
    }
    ///Error detection interrupts enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ERRIE::B0x1
    }
}
///Field `ERRIE` writer - Error interrupts enable Note: Any of these errors generates an interrupt: Note: Arbitration loss (ARLO) Note: Bus error detection (BERR) Note: Overrun/underrun (OVR) Note: Timeout detection (TIMEOUT) Note: PEC error detection (PECERR) Note: Alert pin event detection (ALERT)
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error detection interrupts disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::B0x0)
    }
    ///Error detection interrupts enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::B0x1)
    }
}
/**Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\[3:0\] * t<sub>I2CCLK</sub> ... Note: If the analog filter is enabled, the digital filter is added to it. This filter can be programmed only when the I2C is disabled (PE = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DNF {
    ///0: Digital filter disabled
    B0x0 = 0,
    ///1: Digital filter enabled and filtering capability up to one t<sub>I2CCLK</sub>
    B0x1 = 1,
    ///15: digital filter enabled and filtering capability up to fifteen t<sub>I2CCLK</sub>
    B0xF = 15,
}
impl From<DNF> for u8 {
    #[inline(always)]
    fn from(variant: DNF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DNF {
    type Ux = u8;
}
impl crate::IsEnum for DNF {}
///Field `DNF` reader - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\[3:0\] * t<sub>I2CCLK</sub> ... Note: If the analog filter is enabled, the digital filter is added to it. This filter can be programmed only when the I2C is disabled (PE = 0).
pub type DNF_R = crate::FieldReader<DNF>;
impl DNF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DNF> {
        match self.bits {
            0 => Some(DNF::B0x0),
            1 => Some(DNF::B0x1),
            15 => Some(DNF::B0xF),
            _ => None,
        }
    }
    ///Digital filter disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DNF::B0x0
    }
    ///Digital filter enabled and filtering capability up to one t<sub>I2CCLK</sub>
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DNF::B0x1
    }
    ///digital filter enabled and filtering capability up to fifteen t<sub>I2CCLK</sub>
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == DNF::B0xF
    }
}
///Field `DNF` writer - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\[3:0\] * t<sub>I2CCLK</sub> ... Note: If the analog filter is enabled, the digital filter is added to it. This filter can be programmed only when the I2C is disabled (PE = 0).
pub type DNF_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DNF>;
impl<'a, REG> DNF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Digital filter disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::B0x0)
    }
    ///Digital filter enabled and filtering capability up to one t<sub>I2CCLK</sub>
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::B0x1)
    }
    ///digital filter enabled and filtering capability up to fifteen t<sub>I2CCLK</sub>
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::B0xF)
    }
}
/**Analog noise filter OFF Note: This bit can be programmed only when the I2C is disabled (PE = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANFOFF {
    ///0: Analog noise filter enabled
    B0x0 = 0,
    ///1: Analog noise filter disabled
    B0x1 = 1,
}
impl From<ANFOFF> for bool {
    #[inline(always)]
    fn from(variant: ANFOFF) -> Self {
        variant as u8 != 0
    }
}
///Field `ANFOFF` reader - Analog noise filter OFF Note: This bit can be programmed only when the I2C is disabled (PE = 0).
pub type ANFOFF_R = crate::BitReader<ANFOFF>;
impl ANFOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ANFOFF {
        match self.bits {
            false => ANFOFF::B0x0,
            true => ANFOFF::B0x1,
        }
    }
    ///Analog noise filter enabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ANFOFF::B0x0
    }
    ///Analog noise filter disabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ANFOFF::B0x1
    }
}
///Field `ANFOFF` writer - Analog noise filter OFF Note: This bit can be programmed only when the I2C is disabled (PE = 0).
pub type ANFOFF_W<'a, REG> = crate::BitWriter<'a, REG, ANFOFF>;
impl<'a, REG> ANFOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Analog noise filter enabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ANFOFF::B0x0)
    }
    ///Analog noise filter disabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ANFOFF::B0x1)
    }
}
/**DMA transmission requests enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN {
    ///0: DMA mode disabled for transmission
    B0x0 = 0,
    ///1: DMA mode enabled for transmission
    B0x1 = 1,
}
impl From<TXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDMAEN` reader - DMA transmission requests enable
pub type TXDMAEN_R = crate::BitReader<TXDMAEN>;
impl TXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN {
        match self.bits {
            false => TXDMAEN::B0x0,
            true => TXDMAEN::B0x1,
        }
    }
    ///DMA mode disabled for transmission
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXDMAEN::B0x0
    }
    ///DMA mode enabled for transmission
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXDMAEN::B0x1
    }
}
///Field `TXDMAEN` writer - DMA transmission requests enable
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode disabled for transmission
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::B0x0)
    }
    ///DMA mode enabled for transmission
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::B0x1)
    }
}
/**DMA reception requests enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN {
    ///0: DMA mode disabled for reception
    B0x0 = 0,
    ///1: DMA mode enabled for reception
    B0x1 = 1,
}
impl From<RXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDMAEN` reader - DMA reception requests enable
pub type RXDMAEN_R = crate::BitReader<RXDMAEN>;
impl RXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN {
        match self.bits {
            false => RXDMAEN::B0x0,
            true => RXDMAEN::B0x1,
        }
    }
    ///DMA mode disabled for reception
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXDMAEN::B0x0
    }
    ///DMA mode enabled for reception
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXDMAEN::B0x1
    }
}
///Field `RXDMAEN` writer - DMA reception requests enable
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode disabled for reception
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::B0x0)
    }
    ///DMA mode enabled for reception
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::B0x1)
    }
}
/**Slave byte control This bit is used to enable hardware byte control in slave mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBC {
    ///0: Slave byte control disabled
    B0x0 = 0,
    ///1: Slave byte control enabled
    B0x1 = 1,
}
impl From<SBC> for bool {
    #[inline(always)]
    fn from(variant: SBC) -> Self {
        variant as u8 != 0
    }
}
///Field `SBC` reader - Slave byte control This bit is used to enable hardware byte control in slave mode.
pub type SBC_R = crate::BitReader<SBC>;
impl SBC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBC {
        match self.bits {
            false => SBC::B0x0,
            true => SBC::B0x1,
        }
    }
    ///Slave byte control disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SBC::B0x0
    }
    ///Slave byte control enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SBC::B0x1
    }
}
///Field `SBC` writer - Slave byte control This bit is used to enable hardware byte control in slave mode.
pub type SBC_W<'a, REG> = crate::BitWriter<'a, REG, SBC>;
impl<'a, REG> SBC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave byte control disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SBC::B0x0)
    }
    ///Slave byte control enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SBC::B0x1)
    }
}
/**Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can be programmed only when the I2C is disabled (PE = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOSTRETCH {
    ///0: Clock stretching enabled
    B0x0 = 0,
    ///1: Clock stretching disabled
    B0x1 = 1,
}
impl From<NOSTRETCH> for bool {
    #[inline(always)]
    fn from(variant: NOSTRETCH) -> Self {
        variant as u8 != 0
    }
}
///Field `NOSTRETCH` reader - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can be programmed only when the I2C is disabled (PE = 0).
pub type NOSTRETCH_R = crate::BitReader<NOSTRETCH>;
impl NOSTRETCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NOSTRETCH {
        match self.bits {
            false => NOSTRETCH::B0x0,
            true => NOSTRETCH::B0x1,
        }
    }
    ///Clock stretching enabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NOSTRETCH::B0x0
    }
    ///Clock stretching disabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NOSTRETCH::B0x1
    }
}
///Field `NOSTRETCH` writer - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can be programmed only when the I2C is disabled (PE = 0).
pub type NOSTRETCH_W<'a, REG> = crate::BitWriter<'a, REG, NOSTRETCH>;
impl<'a, REG> NOSTRETCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock stretching enabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NOSTRETCH::B0x0)
    }
    ///Clock stretching disabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NOSTRETCH::B0x1)
    }
}
/**Wake-up from Stop mode enable Note: If the wake-up from Stop mode feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3. Note: WUPEN can be set only when DNF = 0000.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN {
    ///0: Wake-up from Stop mode disabled.
    B0x0 = 0,
    ///1: Wake-up from Stop mode enabled.
    B0x1 = 1,
}
impl From<WUPEN> for bool {
    #[inline(always)]
    fn from(variant: WUPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `WUPEN` reader - Wake-up from Stop mode enable Note: If the wake-up from Stop mode feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3. Note: WUPEN can be set only when DNF = 0000.
pub type WUPEN_R = crate::BitReader<WUPEN>;
impl WUPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUPEN {
        match self.bits {
            false => WUPEN::B0x0,
            true => WUPEN::B0x1,
        }
    }
    ///Wake-up from Stop mode disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WUPEN::B0x0
    }
    ///Wake-up from Stop mode enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WUPEN::B0x1
    }
}
///Field `WUPEN` writer - Wake-up from Stop mode enable Note: If the wake-up from Stop mode feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3. Note: WUPEN can be set only when DNF = 0000.
pub type WUPEN_W<'a, REG> = crate::BitWriter<'a, REG, WUPEN>;
impl<'a, REG> WUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Wake-up from Stop mode disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN::B0x0)
    }
    ///Wake-up from Stop mode enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN::B0x1)
    }
}
/**General call enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCEN {
    ///0: General call disabled. Address 0b00000000 is NACKed.
    B0x0 = 0,
    ///1: General call enabled. Address 0b00000000 is ACKed.
    B0x1 = 1,
}
impl From<GCEN> for bool {
    #[inline(always)]
    fn from(variant: GCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `GCEN` reader - General call enable
pub type GCEN_R = crate::BitReader<GCEN>;
impl GCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GCEN {
        match self.bits {
            false => GCEN::B0x0,
            true => GCEN::B0x1,
        }
    }
    ///General call disabled. Address 0b00000000 is NACKed.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GCEN::B0x0
    }
    ///General call enabled. Address 0b00000000 is ACKed.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GCEN::B0x1
    }
}
///Field `GCEN` writer - General call enable
pub type GCEN_W<'a, REG> = crate::BitWriter<'a, REG, GCEN>;
impl<'a, REG> GCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///General call disabled. Address 0b00000000 is NACKed.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GCEN::B0x0)
    }
    ///General call enabled. Address 0b00000000 is ACKed.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GCEN::B0x1)
    }
}
/**SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBHEN {
    ///0: Host address disabled. Address 0b0001000x is NACKed.
    B0x0 = 0,
    ///1: Host address enabled. Address 0b0001000x is ACKed.
    B0x1 = 1,
}
impl From<SMBHEN> for bool {
    #[inline(always)]
    fn from(variant: SMBHEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SMBHEN` reader - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type SMBHEN_R = crate::BitReader<SMBHEN>;
impl SMBHEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMBHEN {
        match self.bits {
            false => SMBHEN::B0x0,
            true => SMBHEN::B0x1,
        }
    }
    ///Host address disabled. Address 0b0001000x is NACKed.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMBHEN::B0x0
    }
    ///Host address enabled. Address 0b0001000x is ACKed.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMBHEN::B0x1
    }
}
///Field `SMBHEN` writer - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type SMBHEN_W<'a, REG> = crate::BitWriter<'a, REG, SMBHEN>;
impl<'a, REG> SMBHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Host address disabled. Address 0b0001000x is NACKed.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMBHEN::B0x0)
    }
    ///Host address enabled. Address 0b0001000x is ACKed.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMBHEN::B0x1)
    }
}
/**SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBDEN {
    ///0: Device default address disabled. Address 0b1100001x is NACKed.
    B0x0 = 0,
    ///1: Device default address enabled. Address 0b1100001x is ACKed.
    B0x1 = 1,
}
impl From<SMBDEN> for bool {
    #[inline(always)]
    fn from(variant: SMBDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SMBDEN` reader - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type SMBDEN_R = crate::BitReader<SMBDEN>;
impl SMBDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMBDEN {
        match self.bits {
            false => SMBDEN::B0x0,
            true => SMBDEN::B0x1,
        }
    }
    ///Device default address disabled. Address 0b1100001x is NACKed.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SMBDEN::B0x0
    }
    ///Device default address enabled. Address 0b1100001x is ACKed.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SMBDEN::B0x1
    }
}
///Field `SMBDEN` writer - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type SMBDEN_W<'a, REG> = crate::BitWriter<'a, REG, SMBDEN>;
impl<'a, REG> SMBDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Device default address disabled. Address 0b1100001x is NACKed.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SMBDEN::B0x0)
    }
    ///Device default address enabled. Address 0b1100001x is ACKed.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SMBDEN::B0x1)
    }
}
/**SMBus alert enable Note: When ALERTEN = 0, the SMBA pin can be used as a standard GPIO. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERTEN {
    ///0: The SMBus alert pin (SMBA) is not supported in host mode (SMBHEN = 1). In device mode (SMBHEN = 0), the SMBA pin is released and the Alert Response Address header is disabled (0001100x followed by NACK).
    B0x0 = 0,
    ///1: The SMBus alert pin is supported in host mode (SMBHEN = 1). In device mode (SMBHEN = 0), the SMBA pin is driven low and the Alert Response Address header is enabled (0001100x followed by ACK).
    B0x1 = 1,
}
impl From<ALERTEN> for bool {
    #[inline(always)]
    fn from(variant: ALERTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `ALERTEN` reader - SMBus alert enable Note: When ALERTEN = 0, the SMBA pin can be used as a standard GPIO. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type ALERTEN_R = crate::BitReader<ALERTEN>;
impl ALERTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALERTEN {
        match self.bits {
            false => ALERTEN::B0x0,
            true => ALERTEN::B0x1,
        }
    }
    ///The SMBus alert pin (SMBA) is not supported in host mode (SMBHEN = 1). In device mode (SMBHEN = 0), the SMBA pin is released and the Alert Response Address header is disabled (0001100x followed by NACK).
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ALERTEN::B0x0
    }
    ///The SMBus alert pin is supported in host mode (SMBHEN = 1). In device mode (SMBHEN = 0), the SMBA pin is driven low and the Alert Response Address header is enabled (0001100x followed by ACK).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ALERTEN::B0x1
    }
}
///Field `ALERTEN` writer - SMBus alert enable Note: When ALERTEN = 0, the SMBA pin can be used as a standard GPIO. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type ALERTEN_W<'a, REG> = crate::BitWriter<'a, REG, ALERTEN>;
impl<'a, REG> ALERTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The SMBus alert pin (SMBA) is not supported in host mode (SMBHEN = 1). In device mode (SMBHEN = 0), the SMBA pin is released and the Alert Response Address header is disabled (0001100x followed by NACK).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALERTEN::B0x0)
    }
    ///The SMBus alert pin is supported in host mode (SMBHEN = 1). In device mode (SMBHEN = 0), the SMBA pin is driven low and the Alert Response Address header is enabled (0001100x followed by ACK).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALERTEN::B0x1)
    }
}
/**PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECEN {
    ///0: PEC calculation disabled
    B0x0 = 0,
    ///1: PEC calculation enabled
    B0x1 = 1,
}
impl From<PECEN> for bool {
    #[inline(always)]
    fn from(variant: PECEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PECEN` reader - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type PECEN_R = crate::BitReader<PECEN>;
impl PECEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PECEN {
        match self.bits {
            false => PECEN::B0x0,
            true => PECEN::B0x1,
        }
    }
    ///PEC calculation disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PECEN::B0x0
    }
    ///PEC calculation enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PECEN::B0x1
    }
}
///Field `PECEN` writer - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type PECEN_W<'a, REG> = crate::BitWriter<'a, REG, PECEN>;
impl<'a, REG> PECEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PEC calculation disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PECEN::B0x0)
    }
    ///PEC calculation enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PECEN::B0x1)
    }
}
impl R {
    ///Bit 0 - Peripheral enable Note: When PE = 0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least three APB clock cycles.
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX interrupt enable
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RX interrupt enable
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Address match interrupt enable (slave only)
    #[inline(always)]
    pub fn addrie(&self) -> ADDRIE_R {
        ADDRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Not acknowledge received interrupt enable
    #[inline(always)]
    pub fn nackie(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Stop detection interrupt enable
    #[inline(always)]
    pub fn stopie(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transfer complete interrupt enable Note: Any of these events generates an interrupt: Note: Transfer complete (TC) Note: Transfer complete reload (TCR)
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Error interrupts enable Note: Any of these errors generates an interrupt: Note: Arbitration loss (ARLO) Note: Bus error detection (BERR) Note: Overrun/underrun (OVR) Note: Timeout detection (TIMEOUT) Note: PEC error detection (PECERR) Note: Alert pin event detection (ALERT)
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\[3:0\] * t<sub>I2CCLK</sub> ... Note: If the analog filter is enabled, the digital filter is added to it. This filter can be programmed only when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Analog noise filter OFF Note: This bit can be programmed only when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn anfoff(&self) -> ANFOFF_R {
        ANFOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - DMA transmission requests enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DMA reception requests enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode.
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can be programmed only when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Wake-up from Stop mode enable Note: If the wake-up from Stop mode feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3. Note: WUPEN can be set only when DNF = 0000.
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - General call enable
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn smbhen(&self) -> SMBHEN_R {
        SMBHEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn smbden(&self) -> SMBDEN_R {
        SMBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SMBus alert enable Note: When ALERTEN = 0, the SMBA pin can be used as a standard GPIO. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_CR1")
            .field("pe", &self.pe())
            .field("txie", &self.txie())
            .field("rxie", &self.rxie())
            .field("addrie", &self.addrie())
            .field("nackie", &self.nackie())
            .field("stopie", &self.stopie())
            .field("tcie", &self.tcie())
            .field("errie", &self.errie())
            .field("dnf", &self.dnf())
            .field("anfoff", &self.anfoff())
            .field("txdmaen", &self.txdmaen())
            .field("rxdmaen", &self.rxdmaen())
            .field("sbc", &self.sbc())
            .field("nostretch", &self.nostretch())
            .field("wupen", &self.wupen())
            .field("gcen", &self.gcen())
            .field("smbhen", &self.smbhen())
            .field("smbden", &self.smbden())
            .field("alerten", &self.alerten())
            .field("pecen", &self.pecen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Peripheral enable Note: When PE = 0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least three APB clock cycles.
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<'_, I2C_CR1rs> {
        PE_W::new(self, 0)
    }
    ///Bit 1 - TX interrupt enable
    #[inline(always)]
    pub fn txie(&mut self) -> TXIE_W<'_, I2C_CR1rs> {
        TXIE_W::new(self, 1)
    }
    ///Bit 2 - RX interrupt enable
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W<'_, I2C_CR1rs> {
        RXIE_W::new(self, 2)
    }
    ///Bit 3 - Address match interrupt enable (slave only)
    #[inline(always)]
    pub fn addrie(&mut self) -> ADDRIE_W<'_, I2C_CR1rs> {
        ADDRIE_W::new(self, 3)
    }
    ///Bit 4 - Not acknowledge received interrupt enable
    #[inline(always)]
    pub fn nackie(&mut self) -> NACKIE_W<'_, I2C_CR1rs> {
        NACKIE_W::new(self, 4)
    }
    ///Bit 5 - Stop detection interrupt enable
    #[inline(always)]
    pub fn stopie(&mut self) -> STOPIE_W<'_, I2C_CR1rs> {
        STOPIE_W::new(self, 5)
    }
    ///Bit 6 - Transfer complete interrupt enable Note: Any of these events generates an interrupt: Note: Transfer complete (TC) Note: Transfer complete reload (TCR)
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, I2C_CR1rs> {
        TCIE_W::new(self, 6)
    }
    ///Bit 7 - Error interrupts enable Note: Any of these errors generates an interrupt: Note: Arbitration loss (ARLO) Note: Bus error detection (BERR) Note: Overrun/underrun (OVR) Note: Timeout detection (TIMEOUT) Note: PEC error detection (PECERR) Note: Alert pin event detection (ALERT)
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, I2C_CR1rs> {
        ERRIE_W::new(self, 7)
    }
    ///Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\[3:0\] * t<sub>I2CCLK</sub> ... Note: If the analog filter is enabled, the digital filter is added to it. This filter can be programmed only when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn dnf(&mut self) -> DNF_W<'_, I2C_CR1rs> {
        DNF_W::new(self, 8)
    }
    ///Bit 12 - Analog noise filter OFF Note: This bit can be programmed only when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn anfoff(&mut self) -> ANFOFF_W<'_, I2C_CR1rs> {
        ANFOFF_W::new(self, 12)
    }
    ///Bit 14 - DMA transmission requests enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<'_, I2C_CR1rs> {
        TXDMAEN_W::new(self, 14)
    }
    ///Bit 15 - DMA reception requests enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<'_, I2C_CR1rs> {
        RXDMAEN_W::new(self, 15)
    }
    ///Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode.
    #[inline(always)]
    pub fn sbc(&mut self) -> SBC_W<'_, I2C_CR1rs> {
        SBC_W::new(self, 16)
    }
    ///Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can be programmed only when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<'_, I2C_CR1rs> {
        NOSTRETCH_W::new(self, 17)
    }
    ///Bit 18 - Wake-up from Stop mode enable Note: If the wake-up from Stop mode feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3. Note: WUPEN can be set only when DNF = 0000.
    #[inline(always)]
    pub fn wupen(&mut self) -> WUPEN_W<'_, I2C_CR1rs> {
        WUPEN_W::new(self, 18)
    }
    ///Bit 19 - General call enable
    #[inline(always)]
    pub fn gcen(&mut self) -> GCEN_W<'_, I2C_CR1rs> {
        GCEN_W::new(self, 19)
    }
    ///Bit 20 - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn smbhen(&mut self) -> SMBHEN_W<'_, I2C_CR1rs> {
        SMBHEN_W::new(self, 20)
    }
    ///Bit 21 - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn smbden(&mut self) -> SMBDEN_W<'_, I2C_CR1rs> {
        SMBDEN_W::new(self, 21)
    }
    ///Bit 22 - SMBus alert enable Note: When ALERTEN = 0, the SMBA pin can be used as a standard GPIO. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn alerten(&mut self) -> ALERTEN_W<'_, I2C_CR1rs> {
        ALERTEN_W::new(self, 22)
    }
    ///Bit 23 - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W<'_, I2C_CR1rs> {
        PECEN_W::new(self, 23)
    }
}
/**I2C control register 1

You can [`read`](crate::Reg::read) this register and get [`i2c_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#I2C1:I2C_CR1)*/
pub struct I2C_CR1rs;
impl crate::RegisterSpec for I2C_CR1rs {
    type Ux = u32;
}
///`read()` method returns [`i2c_cr1::R`](R) reader structure
impl crate::Readable for I2C_CR1rs {}
///`write(|w| ..)` method takes [`i2c_cr1::W`](W) writer structure
impl crate::Writable for I2C_CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2C_CR1 to value 0
impl crate::Resettable for I2C_CR1rs {}
