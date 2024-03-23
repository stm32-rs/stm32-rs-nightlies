#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Synchronous Slave mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVEN {
    #[doc = "0: Slave mode disabled"]
    Disabled = 0,
    #[doc = "1: Slave mode enabled"]
    Enabled = 1,
}
impl From<SLVEN> for bool {
    #[inline(always)]
    fn from(variant: SLVEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVEN` reader - Synchronous Slave mode enable"]
pub type SLVEN_R = crate::BitReader<SLVEN>;
impl SLVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLVEN {
        match self.bits {
            false => SLVEN::Disabled,
            true => SLVEN::Enabled,
        }
    }
    #[doc = "Slave mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVEN::Disabled
    }
    #[doc = "Slave mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVEN::Enabled
    }
}
#[doc = "Field `SLVEN` writer - Synchronous Slave mode enable"]
pub type SLVEN_W<'a, REG> = crate::BitWriter<'a, REG, SLVEN>;
impl<'a, REG> SLVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SLVEN::Disabled)
    }
    #[doc = "Slave mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SLVEN::Enabled)
    }
}
#[doc = "DIS_NSS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIS_NSS {
    #[doc = "0: SPI slave selection depends on NSS input pin"]
    Disabled = 0,
    #[doc = "1: SPI slave is always selected and NSS input pin is ignored"]
    Enabled = 1,
}
impl From<DIS_NSS> for bool {
    #[inline(always)]
    fn from(variant: DIS_NSS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_NSS` reader - DIS_NSS"]
pub type DIS_NSS_R = crate::BitReader<DIS_NSS>;
impl DIS_NSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIS_NSS {
        match self.bits {
            false => DIS_NSS::Disabled,
            true => DIS_NSS::Enabled,
        }
    }
    #[doc = "SPI slave selection depends on NSS input pin"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIS_NSS::Disabled
    }
    #[doc = "SPI slave is always selected and NSS input pin is ignored"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIS_NSS::Enabled
    }
}
#[doc = "Field `DIS_NSS` writer - DIS_NSS"]
pub type DIS_NSS_W<'a, REG> = crate::BitWriter<'a, REG, DIS_NSS>;
impl<'a, REG> DIS_NSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI slave selection depends on NSS input pin"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIS_NSS::Disabled)
    }
    #[doc = "SPI slave is always selected and NSS input pin is ignored"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIS_NSS::Enabled)
    }
}
#[doc = "7-bit Address Detection/4-bit Address Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDM7 {
    #[doc = "0: 4-bit address detection"]
    Bit4 = 0,
    #[doc = "1: 7-bit address detection"]
    Bit7 = 1,
}
impl From<ADDM7> for bool {
    #[inline(always)]
    fn from(variant: ADDM7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection"]
pub type ADDM7_R = crate::BitReader<ADDM7>;
impl ADDM7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADDM7 {
        match self.bits {
            false => ADDM7::Bit4,
            true => ADDM7::Bit7,
        }
    }
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn is_bit4(&self) -> bool {
        *self == ADDM7::Bit4
    }
    #[doc = "7-bit address detection"]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADDM7::Bit7
    }
}
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection"]
pub type ADDM7_W<'a, REG> = crate::BitWriter<'a, REG, ADDM7>;
impl<'a, REG> ADDM7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4-bit address detection"]
    #[inline(always)]
    pub fn bit4(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM7::Bit4)
    }
    #[doc = "7-bit address detection"]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut crate::W<REG> {
        self.variant(ADDM7::Bit7)
    }
}
#[doc = "LIN break detection length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDL {
    #[doc = "0: 10-bit break detection"]
    Bit10 = 0,
    #[doc = "1: 11-bit break detection"]
    Bit11 = 1,
}
impl From<LBDL> for bool {
    #[inline(always)]
    fn from(variant: LBDL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDL` reader - LIN break detection length"]
pub type LBDL_R = crate::BitReader<LBDL>;
impl LBDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBDL {
        match self.bits {
            false => LBDL::Bit10,
            true => LBDL::Bit11,
        }
    }
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == LBDL::Bit10
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn is_bit11(&self) -> bool {
        *self == LBDL::Bit11
    }
}
#[doc = "Field `LBDL` writer - LIN break detection length"]
pub type LBDL_W<'a, REG> = crate::BitWriter<'a, REG, LBDL>;
impl<'a, REG> LBDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "10-bit break detection"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut crate::W<REG> {
        self.variant(LBDL::Bit10)
    }
    #[doc = "11-bit break detection"]
    #[inline(always)]
    pub fn bit11(self) -> &'a mut crate::W<REG> {
        self.variant(LBDL::Bit11)
    }
}
#[doc = "LIN break detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDIE {
    #[doc = "0: Interrupt is inhibited"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated whenever LBDF=1 in the ISR register"]
    Enabled = 1,
}
impl From<LBDIE> for bool {
    #[inline(always)]
    fn from(variant: LBDIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable"]
pub type LBDIE_R = crate::BitReader<LBDIE>;
impl LBDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBDIE {
        match self.bits {
            false => LBDIE::Disabled,
            true => LBDIE::Enabled,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBDIE::Disabled
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the ISR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBDIE::Enabled
    }
}
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable"]
pub type LBDIE_W<'a, REG> = crate::BitWriter<'a, REG, LBDIE>;
impl<'a, REG> LBDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBDIE::Disabled)
    }
    #[doc = "An interrupt is generated whenever LBDF=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LBDIE::Enabled)
    }
}
#[doc = "Last bit clock pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBCL {
    #[doc = "0: The clock pulse of the last data bit is not output to the CK pin"]
    NotOutput = 0,
    #[doc = "1: The clock pulse of the last data bit is output to the CK pin"]
    Output = 1,
}
impl From<LBCL> for bool {
    #[inline(always)]
    fn from(variant: LBCL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBCL` reader - Last bit clock pulse"]
pub type LBCL_R = crate::BitReader<LBCL>;
impl LBCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LBCL {
        match self.bits {
            false => LBCL::NotOutput,
            true => LBCL::Output,
        }
    }
    #[doc = "The clock pulse of the last data bit is not output to the CK pin"]
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        *self == LBCL::NotOutput
    }
    #[doc = "The clock pulse of the last data bit is output to the CK pin"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == LBCL::Output
    }
}
#[doc = "Field `LBCL` writer - Last bit clock pulse"]
pub type LBCL_W<'a, REG> = crate::BitWriter<'a, REG, LBCL>;
impl<'a, REG> LBCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The clock pulse of the last data bit is not output to the CK pin"]
    #[inline(always)]
    pub fn not_output(self) -> &'a mut crate::W<REG> {
        self.variant(LBCL::NotOutput)
    }
    #[doc = "The clock pulse of the last data bit is output to the CK pin"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(LBCL::Output)
    }
}
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA {
    #[doc = "0: The first clock transition is the first data capture edge"]
    First = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    Second = 1,
}
impl From<CPHA> for bool {
    #[inline(always)]
    fn from(variant: CPHA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CPHA_R = crate::BitReader<CPHA>;
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPHA {
        match self.bits {
            false => CPHA::First,
            true => CPHA::Second,
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_first(&self) -> bool {
        *self == CPHA::First
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == CPHA::Second
    }
}
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::First)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::Second)
    }
}
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL {
    #[doc = "0: Steady low value on CK pin outside transmission window"]
    Low = 0,
    #[doc = "1: Steady high value on CK pin outside transmission window"]
    High = 1,
}
impl From<CPOL> for bool {
    #[inline(always)]
    fn from(variant: CPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CPOL_R = crate::BitReader<CPOL>;
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPOL {
        match self.bits {
            false => CPOL::Low,
            true => CPOL::High,
        }
    }
    #[doc = "Steady low value on CK pin outside transmission window"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOL::Low
    }
    #[doc = "Steady high value on CK pin outside transmission window"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOL::High
    }
}
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Steady low value on CK pin outside transmission window"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::Low)
    }
    #[doc = "Steady high value on CK pin outside transmission window"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::High)
    }
}
#[doc = "Clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN {
    #[doc = "0: CK pin disabled"]
    Disabled = 0,
    #[doc = "1: CK pin enabled"]
    Enabled = 1,
}
impl From<CLKEN> for bool {
    #[inline(always)]
    fn from(variant: CLKEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type CLKEN_R = crate::BitReader<CLKEN>;
impl CLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKEN {
        match self.bits {
            false => CLKEN::Disabled,
            true => CLKEN::Enabled,
        }
    }
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN::Disabled
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN::Enabled
    }
}
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type CLKEN_W<'a, REG> = crate::BitWriter<'a, REG, CLKEN>;
impl<'a, REG> CLKEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CK pin disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN::Disabled)
    }
    #[doc = "CK pin enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CLKEN::Enabled)
    }
}
#[doc = "stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOP {
    #[doc = "0: 1 stop bit"]
    Stop1 = 0,
    #[doc = "1: 0.5 stop bit"]
    Stop0p5 = 1,
    #[doc = "2: 2 stop bit"]
    Stop2 = 2,
    #[doc = "3: 1.5 stop bit"]
    Stop1p5 = 3,
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
#[doc = "Field `STOP` reader - stop bits"]
pub type STOP_R = crate::FieldReader<STOP>;
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP {
        match self.bits {
            0 => STOP::Stop1,
            1 => STOP::Stop0p5,
            2 => STOP::Stop2,
            3 => STOP::Stop1p5,
            _ => unreachable!(),
        }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == STOP::Stop1
    }
    #[doc = "0.5 stop bit"]
    #[inline(always)]
    pub fn is_stop0p5(&self) -> bool {
        *self == STOP::Stop0p5
    }
    #[doc = "2 stop bit"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == STOP::Stop2
    }
    #[doc = "1.5 stop bit"]
    #[inline(always)]
    pub fn is_stop1p5(&self) -> bool {
        *self == STOP::Stop1p5
    }
}
#[doc = "Field `STOP` writer - stop bits"]
pub type STOP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, STOP>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop1)
    }
    #[doc = "0.5 stop bit"]
    #[inline(always)]
    pub fn stop0p5(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop0p5)
    }
    #[doc = "2 stop bit"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop2)
    }
    #[doc = "1.5 stop bit"]
    #[inline(always)]
    pub fn stop1p5(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop1p5)
    }
}
#[doc = "LIN mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEN {
    #[doc = "0: LIN mode disabled"]
    Disabled = 0,
    #[doc = "1: LIN mode enabled"]
    Enabled = 1,
}
impl From<LINEN> for bool {
    #[inline(always)]
    fn from(variant: LINEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LINEN_R = crate::BitReader<LINEN>;
impl LINEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LINEN {
        match self.bits {
            false => LINEN::Disabled,
            true => LINEN::Enabled,
        }
    }
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINEN::Disabled
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINEN::Enabled
    }
}
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LINEN_W<'a, REG> = crate::BitWriter<'a, REG, LINEN>;
impl<'a, REG> LINEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIN mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LINEN::Disabled)
    }
    #[doc = "LIN mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LINEN::Enabled)
    }
}
#[doc = "Swap TX/RX pins\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP {
    #[doc = "0: TX/RX pins are used as defined in standard pinout"]
    Standard = 0,
    #[doc = "1: The TX and RX pins functions are swapped"]
    Swapped = 1,
}
impl From<SWAP> for bool {
    #[inline(always)]
    fn from(variant: SWAP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP` reader - Swap TX/RX pins"]
pub type SWAP_R = crate::BitReader<SWAP>;
impl SWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWAP {
        match self.bits {
            false => SWAP::Standard,
            true => SWAP::Swapped,
        }
    }
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SWAP::Standard
    }
    #[doc = "The TX and RX pins functions are swapped"]
    #[inline(always)]
    pub fn is_swapped(&self) -> bool {
        *self == SWAP::Swapped
    }
}
#[doc = "Field `SWAP` writer - Swap TX/RX pins"]
pub type SWAP_W<'a, REG> = crate::BitWriter<'a, REG, SWAP>;
impl<'a, REG> SWAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP::Standard)
    }
    #[doc = "The TX and RX pins functions are swapped"]
    #[inline(always)]
    pub fn swapped(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP::Swapped)
    }
}
#[doc = "RX pin active level inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXINV {
    #[doc = "0: RX pin signal works using the standard logic levels"]
    Standard = 0,
    #[doc = "1: RX pin signal values are inverted"]
    Inverted = 1,
}
impl From<RXINV> for bool {
    #[inline(always)]
    fn from(variant: RXINV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - RX pin active level inversion"]
pub type RXINV_R = crate::BitReader<RXINV>;
impl RXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXINV {
        match self.bits {
            false => RXINV::Standard,
            true => RXINV::Inverted,
        }
    }
    #[doc = "RX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == RXINV::Standard
    }
    #[doc = "RX pin signal values are inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RXINV::Inverted
    }
}
#[doc = "Field `RXINV` writer - RX pin active level inversion"]
pub type RXINV_W<'a, REG> = crate::BitWriter<'a, REG, RXINV>;
impl<'a, REG> RXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(RXINV::Standard)
    }
    #[doc = "RX pin signal values are inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(RXINV::Inverted)
    }
}
#[doc = "TX pin active level inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXINV {
    #[doc = "0: TX pin signal works using the standard logic levels"]
    Standard = 0,
    #[doc = "1: TX pin signal values are inverted"]
    Inverted = 1,
}
impl From<TXINV> for bool {
    #[inline(always)]
    fn from(variant: TXINV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - TX pin active level inversion"]
pub type TXINV_R = crate::BitReader<TXINV>;
impl TXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXINV {
        match self.bits {
            false => TXINV::Standard,
            true => TXINV::Inverted,
        }
    }
    #[doc = "TX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TXINV::Standard
    }
    #[doc = "TX pin signal values are inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TXINV::Inverted
    }
}
#[doc = "Field `TXINV` writer - TX pin active level inversion"]
pub type TXINV_W<'a, REG> = crate::BitWriter<'a, REG, TXINV>;
impl<'a, REG> TXINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX pin signal works using the standard logic levels"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(TXINV::Standard)
    }
    #[doc = "TX pin signal values are inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(TXINV::Inverted)
    }
}
#[doc = "Binary data inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATAINV {
    #[doc = "0: Logical data from the data register are send/received in positive/direct logic"]
    Positive = 0,
    #[doc = "1: Logical data from the data register are send/received in negative/inverse logic"]
    Negative = 1,
}
impl From<DATAINV> for bool {
    #[inline(always)]
    fn from(variant: DATAINV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAINV` reader - Binary data inversion"]
pub type DATAINV_R = crate::BitReader<DATAINV>;
impl DATAINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATAINV {
        match self.bits {
            false => DATAINV::Positive,
            true => DATAINV::Negative,
        }
    }
    #[doc = "Logical data from the data register are send/received in positive/direct logic"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == DATAINV::Positive
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == DATAINV::Negative
    }
}
#[doc = "Field `DATAINV` writer - Binary data inversion"]
pub type DATAINV_W<'a, REG> = crate::BitWriter<'a, REG, DATAINV>;
impl<'a, REG> DATAINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Logical data from the data register are send/received in positive/direct logic"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(DATAINV::Positive)
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(DATAINV::Negative)
    }
}
#[doc = "Most significant bit first\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBFIRST {
    #[doc = "0: data is transmitted/received with data bit 0 first, following the start bit"]
    Lsb = 0,
    #[doc = "1: data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    Msb = 1,
}
impl From<MSBFIRST> for bool {
    #[inline(always)]
    fn from(variant: MSBFIRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBFIRST` reader - Most significant bit first"]
pub type MSBFIRST_R = crate::BitReader<MSBFIRST>;
impl MSBFIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSBFIRST {
        match self.bits {
            false => MSBFIRST::Lsb,
            true => MSBFIRST::Msb,
        }
    }
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == MSBFIRST::Lsb
    }
    #[doc = "data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == MSBFIRST::Msb
    }
}
#[doc = "Field `MSBFIRST` writer - Most significant bit first"]
pub type MSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG, MSBFIRST>;
impl<'a, REG> MSBFIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(MSBFIRST::Lsb)
    }
    #[doc = "data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(MSBFIRST::Msb)
    }
}
#[doc = "Auto baud rate enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABREN {
    #[doc = "0: Auto baud rate detection is disabled"]
    Disabled = 0,
    #[doc = "1: Auto baud rate detection is enabled"]
    Enabled = 1,
}
impl From<ABREN> for bool {
    #[inline(always)]
    fn from(variant: ABREN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABREN` reader - Auto baud rate enable"]
pub type ABREN_R = crate::BitReader<ABREN>;
impl ABREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABREN {
        match self.bits {
            false => ABREN::Disabled,
            true => ABREN::Enabled,
        }
    }
    #[doc = "Auto baud rate detection is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABREN::Disabled
    }
    #[doc = "Auto baud rate detection is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABREN::Enabled
    }
}
#[doc = "Field `ABREN` writer - Auto baud rate enable"]
pub type ABREN_W<'a, REG> = crate::BitWriter<'a, REG, ABREN>;
impl<'a, REG> ABREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto baud rate detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ABREN::Disabled)
    }
    #[doc = "Auto baud rate detection is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ABREN::Enabled)
    }
}
#[doc = "Auto baud rate mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ABRMOD {
    #[doc = "0: Measurement of the start bit is used to detect the baud rate"]
    Start = 0,
    #[doc = "1: Falling edge to falling edge measurement"]
    Edge = 1,
    #[doc = "2: 0x7F frame detection"]
    Frame7f = 2,
    #[doc = "3: 0x55 frame detection"]
    Frame55 = 3,
}
impl From<ABRMOD> for u8 {
    #[inline(always)]
    fn from(variant: ABRMOD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ABRMOD {
    type Ux = u8;
}
#[doc = "Field `ABRMOD` reader - Auto baud rate mode"]
pub type ABRMOD_R = crate::FieldReader<ABRMOD>;
impl ABRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ABRMOD {
        match self.bits {
            0 => ABRMOD::Start,
            1 => ABRMOD::Edge,
            2 => ABRMOD::Frame7f,
            3 => ABRMOD::Frame55,
            _ => unreachable!(),
        }
    }
    #[doc = "Measurement of the start bit is used to detect the baud rate"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ABRMOD::Start
    }
    #[doc = "Falling edge to falling edge measurement"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ABRMOD::Edge
    }
    #[doc = "0x7F frame detection"]
    #[inline(always)]
    pub fn is_frame7f(&self) -> bool {
        *self == ABRMOD::Frame7f
    }
    #[doc = "0x55 frame detection"]
    #[inline(always)]
    pub fn is_frame55(&self) -> bool {
        *self == ABRMOD::Frame55
    }
}
#[doc = "Field `ABRMOD` writer - Auto baud rate mode"]
pub type ABRMOD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ABRMOD>;
impl<'a, REG> ABRMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Measurement of the start bit is used to detect the baud rate"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(ABRMOD::Start)
    }
    #[doc = "Falling edge to falling edge measurement"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(ABRMOD::Edge)
    }
    #[doc = "0x7F frame detection"]
    #[inline(always)]
    pub fn frame7f(self) -> &'a mut crate::W<REG> {
        self.variant(ABRMOD::Frame7f)
    }
    #[doc = "0x55 frame detection"]
    #[inline(always)]
    pub fn frame55(self) -> &'a mut crate::W<REG> {
        self.variant(ABRMOD::Frame55)
    }
}
#[doc = "Receiver timeout enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOEN {
    #[doc = "0: Receiver timeout feature disabled"]
    Disabled = 0,
    #[doc = "1: Receiver timeout feature enabled"]
    Enabled = 1,
}
impl From<RTOEN> for bool {
    #[inline(always)]
    fn from(variant: RTOEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOEN` reader - Receiver timeout enable"]
pub type RTOEN_R = crate::BitReader<RTOEN>;
impl RTOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTOEN {
        match self.bits {
            false => RTOEN::Disabled,
            true => RTOEN::Enabled,
        }
    }
    #[doc = "Receiver timeout feature disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTOEN::Disabled
    }
    #[doc = "Receiver timeout feature enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTOEN::Enabled
    }
}
#[doc = "Field `RTOEN` writer - Receiver timeout enable"]
pub type RTOEN_W<'a, REG> = crate::BitWriter<'a, REG, RTOEN>;
impl<'a, REG> RTOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver timeout feature disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTOEN::Disabled)
    }
    #[doc = "Receiver timeout feature enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTOEN::Enabled)
    }
}
#[doc = "Field `ADD` reader - Address of the USART node"]
pub type ADD_R = crate::FieldReader;
#[doc = "Field `ADD` writer - Address of the USART node"]
pub type ADD_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Synchronous Slave mode enable"]
    #[inline(always)]
    pub fn slven(&self) -> SLVEN_R {
        SLVEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - DIS_NSS"]
    #[inline(always)]
    pub fn dis_nss(&self) -> DIS_NSS_R {
        DIS_NSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - stop bits"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    pub fn abrmod(&self) -> ABRMOD_R {
        ABRMOD_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    pub fn rtoen(&self) -> RTOEN_R {
        RTOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address of the USART node"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Slave mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn slven(&mut self) -> SLVEN_W<CR2rs> {
        SLVEN_W::new(self, 0)
    }
    #[doc = "Bit 3 - DIS_NSS"]
    #[inline(always)]
    #[must_use]
    pub fn dis_nss(&mut self) -> DIS_NSS_W<CR2rs> {
        DIS_NSS_W::new(self, 3)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline(always)]
    #[must_use]
    pub fn addm7(&mut self) -> ADDM7_W<CR2rs> {
        ADDM7_W::new(self, 4)
    }
    #[doc = "Bit 5 - LIN break detection length"]
    #[inline(always)]
    #[must_use]
    pub fn lbdl(&mut self) -> LBDL_W<CR2rs> {
        LBDL_W::new(self, 5)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbdie(&mut self) -> LBDIE_W<CR2rs> {
        LBDIE_W::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    #[must_use]
    pub fn lbcl(&mut self) -> LBCL_W<CR2rs> {
        LBCL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CR2rs> {
        CPHA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CR2rs> {
        CPOL_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CR2rs> {
        CLKEN_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - stop bits"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR2rs> {
        STOP_W::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LINEN_W<CR2rs> {
        LINEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline(always)]
    #[must_use]
    pub fn swap(&mut self) -> SWAP_W<CR2rs> {
        SWAP_W::new(self, 15)
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<CR2rs> {
        RXINV_W::new(self, 16)
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<CR2rs> {
        TXINV_W::new(self, 17)
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline(always)]
    #[must_use]
    pub fn datainv(&mut self) -> DATAINV_W<CR2rs> {
        DATAINV_W::new(self, 18)
    }
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline(always)]
    #[must_use]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<CR2rs> {
        MSBFIRST_W::new(self, 19)
    }
    #[doc = "Bit 20 - Auto baud rate enable"]
    #[inline(always)]
    #[must_use]
    pub fn abren(&mut self) -> ABREN_W<CR2rs> {
        ABREN_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode"]
    #[inline(always)]
    #[must_use]
    pub fn abrmod(&mut self) -> ABRMOD_W<CR2rs> {
        ABRMOD_W::new(self, 21)
    }
    #[doc = "Bit 23 - Receiver timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtoen(&mut self) -> RTOEN_W<CR2rs> {
        RTOEN_W::new(self, 23)
    }
    #[doc = "Bits 24:31 - Address of the USART node"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<CR2rs> {
        ADD_W::new(self, 24)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
