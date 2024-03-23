#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE {
    #[doc = "0: Peripheral disabled"]
    Disabled = 0,
    #[doc = "1: Peripheral enabled"]
    Enabled = 1,
}
impl From<PE> for bool {
    #[inline(always)]
    fn from(variant: PE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
pub type PE_R = crate::BitReader<PE>;
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PE {
        match self.bits {
            false => PE::Disabled,
            true => PE::Enabled,
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PE::Disabled
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PE::Enabled
    }
}
#[doc = "Field `PE` writer - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG, PE>;
impl<'a, REG> PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PE::Disabled)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PE::Enabled)
    }
}
#[doc = "TX Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXIE {
    #[doc = "0: Transmit (TXIS) interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Transmit (TXIS) interrupt enabled"]
    Enabled = 1,
}
impl From<TXIE> for bool {
    #[inline(always)]
    fn from(variant: TXIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIE` reader - TX Interrupt enable"]
pub type TXIE_R = crate::BitReader<TXIE>;
impl TXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXIE {
        match self.bits {
            false => TXIE::Disabled,
            true => TXIE::Enabled,
        }
    }
    #[doc = "Transmit (TXIS) interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXIE::Disabled
    }
    #[doc = "Transmit (TXIS) interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXIE::Enabled
    }
}
#[doc = "Field `TXIE` writer - TX Interrupt enable"]
pub type TXIE_W<'a, REG> = crate::BitWriter<'a, REG, TXIE>;
impl<'a, REG> TXIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit (TXIS) interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXIE::Disabled)
    }
    #[doc = "Transmit (TXIS) interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXIE::Enabled)
    }
}
#[doc = "RX Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXIE {
    #[doc = "0: Receive (RXNE) interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Receive (RXNE) interrupt enabled"]
    Enabled = 1,
}
impl From<RXIE> for bool {
    #[inline(always)]
    fn from(variant: RXIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIE` reader - RX Interrupt enable"]
pub type RXIE_R = crate::BitReader<RXIE>;
impl RXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXIE {
        match self.bits {
            false => RXIE::Disabled,
            true => RXIE::Enabled,
        }
    }
    #[doc = "Receive (RXNE) interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXIE::Disabled
    }
    #[doc = "Receive (RXNE) interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXIE::Enabled
    }
}
#[doc = "Field `RXIE` writer - RX Interrupt enable"]
pub type RXIE_W<'a, REG> = crate::BitWriter<'a, REG, RXIE>;
impl<'a, REG> RXIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive (RXNE) interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXIE::Disabled)
    }
    #[doc = "Receive (RXNE) interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXIE::Enabled)
    }
}
#[doc = "Address match Interrupt enable (slave only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRIE {
    #[doc = "0: Address match (ADDR) interrupts disabled"]
    Disabled = 0,
    #[doc = "1: Address match (ADDR) interrupts enabled"]
    Enabled = 1,
}
impl From<ADDRIE> for bool {
    #[inline(always)]
    fn from(variant: ADDRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRIE` reader - Address match Interrupt enable (slave only)"]
pub type ADDRIE_R = crate::BitReader<ADDRIE>;
impl ADDRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDRIE {
        match self.bits {
            false => ADDRIE::Disabled,
            true => ADDRIE::Enabled,
        }
    }
    #[doc = "Address match (ADDR) interrupts disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRIE::Disabled
    }
    #[doc = "Address match (ADDR) interrupts enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRIE::Enabled
    }
}
#[doc = "Field `ADDRIE` writer - Address match Interrupt enable (slave only)"]
pub type ADDRIE_W<'a, REG> = crate::BitWriter<'a, REG, ADDRIE>;
impl<'a, REG> ADDRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address match (ADDR) interrupts disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRIE::Disabled)
    }
    #[doc = "Address match (ADDR) interrupts enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRIE::Enabled)
    }
}
#[doc = "Not acknowledge received Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKIE {
    #[doc = "0: Not acknowledge (NACKF) received interrupts disabled"]
    Disabled = 0,
    #[doc = "1: Not acknowledge (NACKF) received interrupts enabled"]
    Enabled = 1,
}
impl From<NACKIE> for bool {
    #[inline(always)]
    fn from(variant: NACKIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKIE` reader - Not acknowledge received Interrupt enable"]
pub type NACKIE_R = crate::BitReader<NACKIE>;
impl NACKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NACKIE {
        match self.bits {
            false => NACKIE::Disabled,
            true => NACKIE::Enabled,
        }
    }
    #[doc = "Not acknowledge (NACKF) received interrupts disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACKIE::Disabled
    }
    #[doc = "Not acknowledge (NACKF) received interrupts enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACKIE::Enabled
    }
}
#[doc = "Field `NACKIE` writer - Not acknowledge received Interrupt enable"]
pub type NACKIE_W<'a, REG> = crate::BitWriter<'a, REG, NACKIE>;
impl<'a, REG> NACKIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not acknowledge (NACKF) received interrupts disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NACKIE::Disabled)
    }
    #[doc = "Not acknowledge (NACKF) received interrupts enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NACKIE::Enabled)
    }
}
#[doc = "Stop detection Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPIE {
    #[doc = "0: Stop detection (STOPF) interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Stop detection (STOPF) interrupt enabled"]
    Enabled = 1,
}
impl From<STOPIE> for bool {
    #[inline(always)]
    fn from(variant: STOPIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPIE` reader - Stop detection Interrupt enable"]
pub type STOPIE_R = crate::BitReader<STOPIE>;
impl STOPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPIE {
        match self.bits {
            false => STOPIE::Disabled,
            true => STOPIE::Enabled,
        }
    }
    #[doc = "Stop detection (STOPF) interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOPIE::Disabled
    }
    #[doc = "Stop detection (STOPF) interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOPIE::Enabled
    }
}
#[doc = "Field `STOPIE` writer - Stop detection Interrupt enable"]
pub type STOPIE_W<'a, REG> = crate::BitWriter<'a, REG, STOPIE>;
impl<'a, REG> STOPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop detection (STOPF) interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(STOPIE::Disabled)
    }
    #[doc = "Stop detection (STOPF) interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(STOPIE::Enabled)
    }
}
#[doc = "Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    #[doc = "0: Transfer Complete interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Transfer Complete interrupt enabled"]
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::Disabled,
            true => TCIE::Enabled,
        }
    }
    #[doc = "Transfer Complete interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    #[doc = "Transfer Complete interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
#[doc = "Field `TCIE` writer - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transfer Complete interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    #[doc = "Transfer Complete interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
#[doc = "Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    #[doc = "0: Error detection interrupts disabled"]
    Disabled = 0,
    #[doc = "1: Error detection interrupts enabled"]
    Enabled = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Disabled,
            true => ERRIE::Enabled,
        }
    }
    #[doc = "Error detection interrupts disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE::Disabled
    }
    #[doc = "Error detection interrupts enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE::Enabled
    }
}
#[doc = "Field `ERRIE` writer - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error detection interrupts disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Disabled)
    }
    #[doc = "Error detection interrupts enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Enabled)
    }
}
#[doc = "Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DNF {
    #[doc = "0: Digital filter disabled"]
    NoFilter = 0,
    #[doc = "1: Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    Filter1 = 1,
    #[doc = "2: Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    Filter2 = 2,
    #[doc = "3: Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    Filter3 = 3,
    #[doc = "4: Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    Filter4 = 4,
    #[doc = "5: Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    Filter5 = 5,
    #[doc = "6: Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    Filter6 = 6,
    #[doc = "7: Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    Filter7 = 7,
    #[doc = "8: Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    Filter8 = 8,
    #[doc = "9: Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    Filter9 = 9,
    #[doc = "10: Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    Filter10 = 10,
    #[doc = "11: Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    Filter11 = 11,
    #[doc = "12: Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    Filter12 = 12,
    #[doc = "13: Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    Filter13 = 13,
    #[doc = "14: Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    Filter14 = 14,
    #[doc = "15: Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    Filter15 = 15,
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
#[doc = "Field `DNF` reader - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
pub type DNF_R = crate::FieldReader<DNF>;
impl DNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DNF {
        match self.bits {
            0 => DNF::NoFilter,
            1 => DNF::Filter1,
            2 => DNF::Filter2,
            3 => DNF::Filter3,
            4 => DNF::Filter4,
            5 => DNF::Filter5,
            6 => DNF::Filter6,
            7 => DNF::Filter7,
            8 => DNF::Filter8,
            9 => DNF::Filter9,
            10 => DNF::Filter10,
            11 => DNF::Filter11,
            12 => DNF::Filter12,
            13 => DNF::Filter13,
            14 => DNF::Filter14,
            15 => DNF::Filter15,
            _ => unreachable!(),
        }
    }
    #[doc = "Digital filter disabled"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == DNF::NoFilter
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        *self == DNF::Filter1
    }
    #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == DNF::Filter2
    }
    #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        *self == DNF::Filter3
    }
    #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == DNF::Filter4
    }
    #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter5(&self) -> bool {
        *self == DNF::Filter5
    }
    #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter6(&self) -> bool {
        *self == DNF::Filter6
    }
    #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter7(&self) -> bool {
        *self == DNF::Filter7
    }
    #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == DNF::Filter8
    }
    #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter9(&self) -> bool {
        *self == DNF::Filter9
    }
    #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter10(&self) -> bool {
        *self == DNF::Filter10
    }
    #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter11(&self) -> bool {
        *self == DNF::Filter11
    }
    #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter12(&self) -> bool {
        *self == DNF::Filter12
    }
    #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter13(&self) -> bool {
        *self == DNF::Filter13
    }
    #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter14(&self) -> bool {
        *self == DNF::Filter14
    }
    #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    #[inline(always)]
    pub fn is_filter15(&self) -> bool {
        *self == DNF::Filter15
    }
}
#[doc = "Field `DNF` writer - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
pub type DNF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, DNF>;
impl<'a, REG> DNF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Digital filter disabled"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::NoFilter)
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    #[inline(always)]
    pub fn filter1(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter1)
    }
    #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    #[inline(always)]
    pub fn filter2(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter2)
    }
    #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    #[inline(always)]
    pub fn filter3(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter3)
    }
    #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    #[inline(always)]
    pub fn filter4(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter4)
    }
    #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    #[inline(always)]
    pub fn filter5(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter5)
    }
    #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    #[inline(always)]
    pub fn filter6(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter6)
    }
    #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    #[inline(always)]
    pub fn filter7(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter7)
    }
    #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    #[inline(always)]
    pub fn filter8(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter8)
    }
    #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    #[inline(always)]
    pub fn filter9(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter9)
    }
    #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    #[inline(always)]
    pub fn filter10(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter10)
    }
    #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    #[inline(always)]
    pub fn filter11(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter11)
    }
    #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    #[inline(always)]
    pub fn filter12(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter12)
    }
    #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    #[inline(always)]
    pub fn filter13(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter13)
    }
    #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    #[inline(always)]
    pub fn filter14(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter14)
    }
    #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    #[inline(always)]
    pub fn filter15(self) -> &'a mut crate::W<REG> {
        self.variant(DNF::Filter15)
    }
}
#[doc = "Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANFOFF {
    #[doc = "0: Analog noise filter enabled"]
    Enabled = 0,
    #[doc = "1: Analog noise filter disabled"]
    Disabled = 1,
}
impl From<ANFOFF> for bool {
    #[inline(always)]
    fn from(variant: ANFOFF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANFOFF` reader - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type ANFOFF_R = crate::BitReader<ANFOFF>;
impl ANFOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ANFOFF {
        match self.bits {
            false => ANFOFF::Enabled,
            true => ANFOFF::Disabled,
        }
    }
    #[doc = "Analog noise filter enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ANFOFF::Enabled
    }
    #[doc = "Analog noise filter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ANFOFF::Disabled
    }
}
#[doc = "Field `ANFOFF` writer - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type ANFOFF_W<'a, REG> = crate::BitWriter<'a, REG, ANFOFF>;
impl<'a, REG> ANFOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog noise filter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ANFOFF::Enabled)
    }
    #[doc = "Analog noise filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ANFOFF::Disabled)
    }
}
#[doc = "DMA transmission requests enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN {
    #[doc = "0: DMA mode disabled for transmission"]
    Disabled = 0,
    #[doc = "1: DMA mode enabled for transmission"]
    Enabled = 1,
}
impl From<TXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - DMA transmission requests enable"]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN>;
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN {
        match self.bits {
            false => TXDMAEN::Disabled,
            true => TXDMAEN::Enabled,
        }
    }
    #[doc = "DMA mode disabled for transmission"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN::Disabled
    }
    #[doc = "DMA mode enabled for transmission"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN::Enabled
    }
}
#[doc = "Field `TXDMAEN` writer - DMA transmission requests enable"]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled for transmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Disabled)
    }
    #[doc = "DMA mode enabled for transmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Enabled)
    }
}
#[doc = "DMA reception requests enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN {
    #[doc = "0: DMA mode disabled for reception"]
    Disabled = 0,
    #[doc = "1: DMA mode enabled for reception"]
    Enabled = 1,
}
impl From<RXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - DMA reception requests enable"]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN>;
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN {
        match self.bits {
            false => RXDMAEN::Disabled,
            true => RXDMAEN::Enabled,
        }
    }
    #[doc = "DMA mode disabled for reception"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN::Disabled
    }
    #[doc = "DMA mode enabled for reception"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN::Enabled
    }
}
#[doc = "Field `RXDMAEN` writer - DMA reception requests enable"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode disabled for reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Disabled)
    }
    #[doc = "DMA mode enabled for reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Enabled)
    }
}
#[doc = "Slave byte control This bit is used to enable hardware byte control in slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBC {
    #[doc = "0: Slave byte control disabled"]
    Disabled = 0,
    #[doc = "1: Slave byte control enabled"]
    Enabled = 1,
}
impl From<SBC> for bool {
    #[inline(always)]
    fn from(variant: SBC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBC` reader - Slave byte control This bit is used to enable hardware byte control in slave mode."]
pub type SBC_R = crate::BitReader<SBC>;
impl SBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBC {
        match self.bits {
            false => SBC::Disabled,
            true => SBC::Enabled,
        }
    }
    #[doc = "Slave byte control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SBC::Disabled
    }
    #[doc = "Slave byte control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SBC::Enabled
    }
}
#[doc = "Field `SBC` writer - Slave byte control This bit is used to enable hardware byte control in slave mode."]
pub type SBC_W<'a, REG> = crate::BitWriter<'a, REG, SBC>;
impl<'a, REG> SBC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave byte control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBC::Disabled)
    }
    #[doc = "Slave byte control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SBC::Enabled)
    }
}
#[doc = "Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOSTRETCH {
    #[doc = "0: Clock stretching enabled"]
    Enabled = 0,
    #[doc = "1: Clock stretching disabled"]
    Disabled = 1,
}
impl From<NOSTRETCH> for bool {
    #[inline(always)]
    fn from(variant: NOSTRETCH) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type NOSTRETCH_R = crate::BitReader<NOSTRETCH>;
impl NOSTRETCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NOSTRETCH {
        match self.bits {
            false => NOSTRETCH::Enabled,
            true => NOSTRETCH::Disabled,
        }
    }
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NOSTRETCH::Enabled
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NOSTRETCH::Disabled
    }
}
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
pub type NOSTRETCH_W<'a, REG> = crate::BitWriter<'a, REG, NOSTRETCH>;
impl<'a, REG> NOSTRETCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NOSTRETCH::Enabled)
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NOSTRETCH::Disabled)
    }
}
#[doc = "Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to . Note: WUPEN can be set only when DNF = ‘0000’\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN {
    #[doc = "0: Wakeup from Stop mode disabled"]
    Disabled = 0,
    #[doc = "1: Wakeup from Stop mode enabled"]
    Enabled = 1,
}
impl From<WUPEN> for bool {
    #[inline(always)]
    fn from(variant: WUPEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUPEN` reader - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to . Note: WUPEN can be set only when DNF = ‘0000’"]
pub type WUPEN_R = crate::BitReader<WUPEN>;
impl WUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUPEN {
        match self.bits {
            false => WUPEN::Disabled,
            true => WUPEN::Enabled,
        }
    }
    #[doc = "Wakeup from Stop mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUPEN::Disabled
    }
    #[doc = "Wakeup from Stop mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUPEN::Enabled
    }
}
#[doc = "Field `WUPEN` writer - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to . Note: WUPEN can be set only when DNF = ‘0000’"]
pub type WUPEN_W<'a, REG> = crate::BitWriter<'a, REG, WUPEN>;
impl<'a, REG> WUPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup from Stop mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN::Disabled)
    }
    #[doc = "Wakeup from Stop mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUPEN::Enabled)
    }
}
#[doc = "General call enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCEN {
    #[doc = "0: General call disabled. Address 0b00000000 is NACKed"]
    Disabled = 0,
    #[doc = "1: General call enabled. Address 0b00000000 is ACKed"]
    Enabled = 1,
}
impl From<GCEN> for bool {
    #[inline(always)]
    fn from(variant: GCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCEN` reader - General call enable"]
pub type GCEN_R = crate::BitReader<GCEN>;
impl GCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GCEN {
        match self.bits {
            false => GCEN::Disabled,
            true => GCEN::Enabled,
        }
    }
    #[doc = "General call disabled. Address 0b00000000 is NACKed"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCEN::Disabled
    }
    #[doc = "General call enabled. Address 0b00000000 is ACKed"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCEN::Enabled
    }
}
#[doc = "Field `GCEN` writer - General call enable"]
pub type GCEN_W<'a, REG> = crate::BitWriter<'a, REG, GCEN>;
impl<'a, REG> GCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "General call disabled. Address 0b00000000 is NACKed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GCEN::Disabled)
    }
    #[doc = "General call enabled. Address 0b00000000 is ACKed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(GCEN::Enabled)
    }
}
#[doc = "SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBHEN {
    #[doc = "0: Host address disabled. Address 0b0001000x is NACKed"]
    Disabled = 0,
    #[doc = "1: Host address enabled. Address 0b0001000x is ACKed"]
    Enabled = 1,
}
impl From<SMBHEN> for bool {
    #[inline(always)]
    fn from(variant: SMBHEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBHEN` reader - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type SMBHEN_R = crate::BitReader<SMBHEN>;
impl SMBHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMBHEN {
        match self.bits {
            false => SMBHEN::Disabled,
            true => SMBHEN::Enabled,
        }
    }
    #[doc = "Host address disabled. Address 0b0001000x is NACKed"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMBHEN::Disabled
    }
    #[doc = "Host address enabled. Address 0b0001000x is ACKed"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMBHEN::Enabled
    }
}
#[doc = "Field `SMBHEN` writer - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type SMBHEN_W<'a, REG> = crate::BitWriter<'a, REG, SMBHEN>;
impl<'a, REG> SMBHEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Host address disabled. Address 0b0001000x is NACKed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMBHEN::Disabled)
    }
    #[doc = "Host address enabled. Address 0b0001000x is ACKed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMBHEN::Enabled)
    }
}
#[doc = "SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBDEN {
    #[doc = "0: Device default address disabled. Address 0b1100001x is NACKed"]
    Disabled = 0,
    #[doc = "1: Device default address enabled. Address 0b1100001x is ACKed"]
    Enabled = 1,
}
impl From<SMBDEN> for bool {
    #[inline(always)]
    fn from(variant: SMBDEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBDEN` reader - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type SMBDEN_R = crate::BitReader<SMBDEN>;
impl SMBDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMBDEN {
        match self.bits {
            false => SMBDEN::Disabled,
            true => SMBDEN::Enabled,
        }
    }
    #[doc = "Device default address disabled. Address 0b1100001x is NACKed"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMBDEN::Disabled
    }
    #[doc = "Device default address enabled. Address 0b1100001x is ACKed"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMBDEN::Enabled
    }
}
#[doc = "Field `SMBDEN` writer - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type SMBDEN_W<'a, REG> = crate::BitWriter<'a, REG, SMBDEN>;
impl<'a, REG> SMBDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device default address disabled. Address 0b1100001x is NACKed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMBDEN::Disabled)
    }
    #[doc = "Device default address enabled. Address 0b1100001x is ACKed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SMBDEN::Enabled)
    }
}
#[doc = "SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERTEN {
    #[doc = "0: In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported"]
    Disabled = 0,
    #[doc = "1: In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported"]
    Enabled = 1,
}
impl From<ALERTEN> for bool {
    #[inline(always)]
    fn from(variant: ALERTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTEN` reader - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type ALERTEN_R = crate::BitReader<ALERTEN>;
impl ALERTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALERTEN {
        match self.bits {
            false => ALERTEN::Disabled,
            true => ALERTEN::Enabled,
        }
    }
    #[doc = "In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALERTEN::Disabled
    }
    #[doc = "In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALERTEN::Enabled
    }
}
#[doc = "Field `ALERTEN` writer - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type ALERTEN_W<'a, REG> = crate::BitWriter<'a, REG, ALERTEN>;
impl<'a, REG> ALERTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALERTEN::Disabled)
    }
    #[doc = "In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALERTEN::Enabled)
    }
}
#[doc = "PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECEN {
    #[doc = "0: PEC calculation disabled"]
    Disabled = 0,
    #[doc = "1: PEC calculation enabled"]
    Enabled = 1,
}
impl From<PECEN> for bool {
    #[inline(always)]
    fn from(variant: PECEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECEN` reader - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type PECEN_R = crate::BitReader<PECEN>;
impl PECEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PECEN {
        match self.bits {
            false => PECEN::Disabled,
            true => PECEN::Enabled,
        }
    }
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PECEN::Disabled
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PECEN::Enabled
    }
}
#[doc = "Field `PECEN` writer - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
pub type PECEN_W<'a, REG> = crate::BitWriter<'a, REG, PECEN>;
impl<'a, REG> PECEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PECEN::Disabled)
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PECEN::Enabled)
    }
}
#[doc = "Fast-mode Plus 20 mA drive enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMP {
    #[doc = "0: 20 mA I/O drive disabled"]
    Disabled = 0,
    #[doc = "1: 20 mA I/O drive enabled"]
    Enabled = 1,
}
impl From<FMP> for bool {
    #[inline(always)]
    fn from(variant: FMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMP` reader - Fast-mode Plus 20 mA drive enable"]
pub type FMP_R = crate::BitReader<FMP>;
impl FMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMP {
        match self.bits {
            false => FMP::Disabled,
            true => FMP::Enabled,
        }
    }
    #[doc = "20 mA I/O drive disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FMP::Disabled
    }
    #[doc = "20 mA I/O drive enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FMP::Enabled
    }
}
#[doc = "Field `FMP` writer - Fast-mode Plus 20 mA drive enable"]
pub type FMP_W<'a, REG> = crate::BitWriter<'a, REG, FMP>;
impl<'a, REG> FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "20 mA I/O drive disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMP::Disabled)
    }
    #[doc = "20 mA I/O drive enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FMP::Enabled)
    }
}
#[doc = "Address match flag (ADDR) automatic clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRACLR {
    #[doc = "0: ADDR flag is set by hardware, cleared by software"]
    Disabled = 0,
    #[doc = "1: ADDR flag remains cleared by hardware"]
    Enabled = 1,
}
impl From<ADDRACLR> for bool {
    #[inline(always)]
    fn from(variant: ADDRACLR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRACLR` reader - Address match flag (ADDR) automatic clear"]
pub type ADDRACLR_R = crate::BitReader<ADDRACLR>;
impl ADDRACLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDRACLR {
        match self.bits {
            false => ADDRACLR::Disabled,
            true => ADDRACLR::Enabled,
        }
    }
    #[doc = "ADDR flag is set by hardware, cleared by software"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRACLR::Disabled
    }
    #[doc = "ADDR flag remains cleared by hardware"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRACLR::Enabled
    }
}
#[doc = "Field `ADDRACLR` writer - Address match flag (ADDR) automatic clear"]
pub type ADDRACLR_W<'a, REG> = crate::BitWriter<'a, REG, ADDRACLR>;
impl<'a, REG> ADDRACLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADDR flag is set by hardware, cleared by software"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRACLR::Disabled)
    }
    #[doc = "ADDR flag remains cleared by hardware"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRACLR::Enabled)
    }
}
#[doc = "STOP detection flag (STOPF) automatic clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPFACLR {
    #[doc = "0: STOPF flag is set by hardware, cleared by software"]
    Disabled = 0,
    #[doc = "1: STOPF flag remains cleared by hardware"]
    Enabled = 1,
}
impl From<STOPFACLR> for bool {
    #[inline(always)]
    fn from(variant: STOPFACLR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPFACLR` reader - STOP detection flag (STOPF) automatic clear"]
pub type STOPFACLR_R = crate::BitReader<STOPFACLR>;
impl STOPFACLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOPFACLR {
        match self.bits {
            false => STOPFACLR::Disabled,
            true => STOPFACLR::Enabled,
        }
    }
    #[doc = "STOPF flag is set by hardware, cleared by software"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOPFACLR::Disabled
    }
    #[doc = "STOPF flag remains cleared by hardware"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOPFACLR::Enabled
    }
}
#[doc = "Field `STOPFACLR` writer - STOP detection flag (STOPF) automatic clear"]
pub type STOPFACLR_W<'a, REG> = crate::BitWriter<'a, REG, STOPFACLR>;
impl<'a, REG> STOPFACLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STOPF flag is set by hardware, cleared by software"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(STOPFACLR::Disabled)
    }
    #[doc = "STOPF flag remains cleared by hardware"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(STOPFACLR::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match Interrupt enable (slave only)"]
    #[inline(always)]
    pub fn addrie(&self) -> ADDRIE_R {
        ADDRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received Interrupt enable"]
    #[inline(always)]
    pub fn nackie(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop detection Interrupt enable"]
    #[inline(always)]
    pub fn stopie(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn anfoff(&self) -> ANFOFF_R {
        ANFOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode."]
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to . Note: WUPEN can be set only when DNF = ‘0000’"]
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    pub fn smbhen(&self) -> SMBHEN_R {
        SMBHEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    pub fn smbden(&self) -> SMBDEN_R {
        SMBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Fast-mode Plus 20 mA drive enable"]
    #[inline(always)]
    pub fn fmp(&self) -> FMP_R {
        FMP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Address match flag (ADDR) automatic clear"]
    #[inline(always)]
    pub fn addraclr(&self) -> ADDRACLR_R {
        ADDRACLR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - STOP detection flag (STOPF) automatic clear"]
    #[inline(always)]
    pub fn stopfaclr(&self) -> STOPFACLR_R {
        STOPFACLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable Note: When PE=0, the I2C SCL and SDA lines are released. Internal state machines and status bits are put back to their reset value. When cleared, PE must be kept low for at least 3 APB clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<CR1rs> {
        PE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TXIE_W<CR1rs> {
        TXIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<CR1rs> {
        RXIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Address match Interrupt enable (slave only)"]
    #[inline(always)]
    #[must_use]
    pub fn addrie(&mut self) -> ADDRIE_W<CR1rs> {
        ADDRIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Not acknowledge received Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nackie(&mut self) -> NACKIE_W<CR1rs> {
        NACKIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stop detection Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stopie(&mut self) -> STOPIE_W<CR1rs> {
        STOPIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable Note: Any of these events generate an interrupt: Transfer Complete (TC) Transfer Complete Reload (TCR)"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CR1rs> {
        TCIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Error interrupts enable Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CR1rs> {
        ERRIE_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter, filters spikes with a length of up to DNF\\[3:0\\]
* tI2CCLK ... Note: If the analog filter is also enabled, the digital filter is added to the analog filter. This filter can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn dnf(&mut self) -> DNF_W<CR1rs> {
        DNF_W::new(self, 8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn anfoff(&mut self) -> ANFOFF_W<CR1rs> {
        ANFOFF_W::new(self, 12)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CR1rs> {
        TXDMAEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CR1rs> {
        RXDMAEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn sbc(&mut self) -> SBC_W<CR1rs> {
        SBC_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. Note: This bit can only be programmed when the I2C is disabled (PE = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<CR1rs> {
        NOSTRETCH_W::new(self, 17)
    }
    #[doc = "Bit 18 - Wakeup from Stop mode enable Note: If the Wakeup from Stop mode feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to . Note: WUPEN can be set only when DNF = ‘0000’"]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WUPEN_W<CR1rs> {
        WUPEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<CR1rs> {
        GCEN_W::new(self, 19)
    }
    #[doc = "Bit 20 - SMBus host address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn smbhen(&mut self) -> SMBHEN_W<CR1rs> {
        SMBHEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - SMBus device default address enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn smbden(&mut self) -> SMBDEN_W<CR1rs> {
        SMBDEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - SMBus alert enable Note: When ALERTEN=0, the SMBA pin can be used as a standard GPIO. If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn alerten(&mut self) -> ALERTEN_W<CR1rs> {
        ALERTEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - PEC enable Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to ‘0’. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<CR1rs> {
        PECEN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Fast-mode Plus 20 mA drive enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmp(&mut self) -> FMP_W<CR1rs> {
        FMP_W::new(self, 24)
    }
    #[doc = "Bit 30 - Address match flag (ADDR) automatic clear"]
    #[inline(always)]
    #[must_use]
    pub fn addraclr(&mut self) -> ADDRACLR_W<CR1rs> {
        ADDRACLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - STOP detection flag (STOPF) automatic clear"]
    #[inline(always)]
    #[must_use]
    pub fn stopfaclr(&mut self) -> STOPFACLR_W<CR1rs> {
        STOPFACLR_W::new(self, 31)
    }
}
#[doc = "I2C control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
