///Register `WPCR0` reader
pub type R = crate::R<WPCR0rs>;
///Register `WPCR0` writer
pub type W = crate::W<WPCR0rs>;
///Field `UIX4` reader - Unit interval multiplied by 4 This field defines the bit period in high-speed mode in unit of 0.25 ns. As an example, if the unit interval is 3 ns, a value of twelve (0x0C) must be driven to this input. This value is used to generate delays. If the period is not a multiple of 0.25 ns, the value driven must be rounded down. For example, a 600 Mbit/s link uses a unit interval of 1.667 ns, which, multiplied by four gives 6.667 ns. In this case a value of 6 (not 7) must be driven onto the ui_x4 input.
pub type UIX4_R = crate::FieldReader;
///Field `UIX4` writer - Unit interval multiplied by 4 This field defines the bit period in high-speed mode in unit of 0.25 ns. As an example, if the unit interval is 3 ns, a value of twelve (0x0C) must be driven to this input. This value is used to generate delays. If the period is not a multiple of 0.25 ns, the value driven must be rounded down. For example, a 600 Mbit/s link uses a unit interval of 1.667 ns, which, multiplied by four gives 6.667 ns. In this case a value of 6 (not 7) must be driven onto the ui_x4 input.
pub type UIX4_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
/**Swap clock lane pins This bit swaps the pins on clock lane.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWCL {
    ///0: Regular clock lane pin configuration
    B0x0 = 0,
    ///1: Swapped clock lane pin
    B0x1 = 1,
}
impl From<SWCL> for bool {
    #[inline(always)]
    fn from(variant: SWCL) -> Self {
        variant as u8 != 0
    }
}
///Field `SWCL` reader - Swap clock lane pins This bit swaps the pins on clock lane.
pub type SWCL_R = crate::BitReader<SWCL>;
impl SWCL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWCL {
        match self.bits {
            false => SWCL::B0x0,
            true => SWCL::B0x1,
        }
    }
    ///Regular clock lane pin configuration
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWCL::B0x0
    }
    ///Swapped clock lane pin
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWCL::B0x1
    }
}
///Field `SWCL` writer - Swap clock lane pins This bit swaps the pins on clock lane.
pub type SWCL_W<'a, REG> = crate::BitWriter<'a, REG, SWCL>;
impl<'a, REG> SWCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular clock lane pin configuration
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWCL::B0x0)
    }
    ///Swapped clock lane pin
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWCL::B0x1)
    }
}
/**Swap data lane 0 pins This bit swaps the pins on data lane 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWDL0 {
    ///0: Regular clock lane pin configuration
    B0x0 = 0,
    ///1: Swapped clock lane pin
    B0x1 = 1,
}
impl From<SWDL0> for bool {
    #[inline(always)]
    fn from(variant: SWDL0) -> Self {
        variant as u8 != 0
    }
}
///Field `SWDL0` reader - Swap data lane 0 pins This bit swaps the pins on data lane 0.
pub type SWDL0_R = crate::BitReader<SWDL0>;
impl SWDL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWDL0 {
        match self.bits {
            false => SWDL0::B0x0,
            true => SWDL0::B0x1,
        }
    }
    ///Regular clock lane pin configuration
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWDL0::B0x0
    }
    ///Swapped clock lane pin
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWDL0::B0x1
    }
}
///Field `SWDL0` writer - Swap data lane 0 pins This bit swaps the pins on data lane 0.
pub type SWDL0_W<'a, REG> = crate::BitWriter<'a, REG, SWDL0>;
impl<'a, REG> SWDL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular clock lane pin configuration
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWDL0::B0x0)
    }
    ///Swapped clock lane pin
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWDL0::B0x1)
    }
}
/**Swap data lane 1 pins This bit swaps the pins on clock lane.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWDL1 {
    ///0: Regular clock lane pin configuration
    B0x0 = 0,
    ///1: Swapped clock lane pin
    B0x1 = 1,
}
impl From<SWDL1> for bool {
    #[inline(always)]
    fn from(variant: SWDL1) -> Self {
        variant as u8 != 0
    }
}
///Field `SWDL1` reader - Swap data lane 1 pins This bit swaps the pins on clock lane.
pub type SWDL1_R = crate::BitReader<SWDL1>;
impl SWDL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWDL1 {
        match self.bits {
            false => SWDL1::B0x0,
            true => SWDL1::B0x1,
        }
    }
    ///Regular clock lane pin configuration
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWDL1::B0x0
    }
    ///Swapped clock lane pin
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWDL1::B0x1
    }
}
///Field `SWDL1` writer - Swap data lane 1 pins This bit swaps the pins on clock lane.
pub type SWDL1_W<'a, REG> = crate::BitWriter<'a, REG, SWDL1>;
impl<'a, REG> SWDL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Regular clock lane pin configuration
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SWDL1::B0x0)
    }
    ///Swapped clock lane pin
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SWDL1::B0x1)
    }
}
/**Invert high-speed data signal on clock lane This bit inverts the high-speed data signal on clock lane.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSICL {
    ///0: Normal data configuration
    B0x0 = 0,
    ///1: Inverted data configuration
    B0x1 = 1,
}
impl From<HSICL> for bool {
    #[inline(always)]
    fn from(variant: HSICL) -> Self {
        variant as u8 != 0
    }
}
///Field `HSICL` reader - Invert high-speed data signal on clock lane This bit inverts the high-speed data signal on clock lane.
pub type HSICL_R = crate::BitReader<HSICL>;
impl HSICL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSICL {
        match self.bits {
            false => HSICL::B0x0,
            true => HSICL::B0x1,
        }
    }
    ///Normal data configuration
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSICL::B0x0
    }
    ///Inverted data configuration
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSICL::B0x1
    }
}
///Field `HSICL` writer - Invert high-speed data signal on clock lane This bit inverts the high-speed data signal on clock lane.
pub type HSICL_W<'a, REG> = crate::BitWriter<'a, REG, HSICL>;
impl<'a, REG> HSICL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal data configuration
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSICL::B0x0)
    }
    ///Inverted data configuration
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSICL::B0x1)
    }
}
/**Invert the high-speed data signal on data lane 0 This bit inverts the high-speed data signal on clock lane.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIDL0 {
    ///0: Normal data signal configuration
    B0x0 = 0,
    ///1: Inverted data signal configuration
    B0x1 = 1,
}
impl From<HSIDL0> for bool {
    #[inline(always)]
    fn from(variant: HSIDL0) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIDL0` reader - Invert the high-speed data signal on data lane 0 This bit inverts the high-speed data signal on clock lane.
pub type HSIDL0_R = crate::BitReader<HSIDL0>;
impl HSIDL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIDL0 {
        match self.bits {
            false => HSIDL0::B0x0,
            true => HSIDL0::B0x1,
        }
    }
    ///Normal data signal configuration
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIDL0::B0x0
    }
    ///Inverted data signal configuration
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIDL0::B0x1
    }
}
///Field `HSIDL0` writer - Invert the high-speed data signal on data lane 0 This bit inverts the high-speed data signal on clock lane.
pub type HSIDL0_W<'a, REG> = crate::BitWriter<'a, REG, HSIDL0>;
impl<'a, REG> HSIDL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal data signal configuration
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDL0::B0x0)
    }
    ///Inverted data signal configuration
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDL0::B0x1)
    }
}
/**Invert the high-speed data signal on data lane 1 This bit inverts the high-speed data signal on data lane 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIDL1 {
    ///0: Normal data signal configuration
    B0x0 = 0,
    ///1: Inverted data signal configuration
    B0x1 = 1,
}
impl From<HSIDL1> for bool {
    #[inline(always)]
    fn from(variant: HSIDL1) -> Self {
        variant as u8 != 0
    }
}
///Field `HSIDL1` reader - Invert the high-speed data signal on data lane 1 This bit inverts the high-speed data signal on data lane 1.
pub type HSIDL1_R = crate::BitReader<HSIDL1>;
impl HSIDL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HSIDL1 {
        match self.bits {
            false => HSIDL1::B0x0,
            true => HSIDL1::B0x1,
        }
    }
    ///Normal data signal configuration
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIDL1::B0x0
    }
    ///Inverted data signal configuration
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIDL1::B0x1
    }
}
///Field `HSIDL1` writer - Invert the high-speed data signal on data lane 1 This bit inverts the high-speed data signal on data lane 1.
pub type HSIDL1_W<'a, REG> = crate::BitWriter<'a, REG, HSIDL1>;
impl<'a, REG> HSIDL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal data signal configuration
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDL1::B0x0)
    }
    ///Inverted data signal configuration
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HSIDL1::B0x1)
    }
}
/**Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTXSMCL {
    ///0: No effect
    B0x0 = 0,
    ///1: Force the clock lane in TX Stop mode
    B0x1 = 1,
}
impl From<FTXSMCL> for bool {
    #[inline(always)]
    fn from(variant: FTXSMCL) -> Self {
        variant as u8 != 0
    }
}
///Field `FTXSMCL` reader - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
pub type FTXSMCL_R = crate::BitReader<FTXSMCL>;
impl FTXSMCL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FTXSMCL {
        match self.bits {
            false => FTXSMCL::B0x0,
            true => FTXSMCL::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FTXSMCL::B0x0
    }
    ///Force the clock lane in TX Stop mode
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FTXSMCL::B0x1
    }
}
///Field `FTXSMCL` writer - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
pub type FTXSMCL_W<'a, REG> = crate::BitWriter<'a, REG, FTXSMCL>;
impl<'a, REG> FTXSMCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FTXSMCL::B0x0)
    }
    ///Force the clock lane in TX Stop mode
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FTXSMCL::B0x1)
    }
}
/**Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTXSMDL {
    ///0: No effect
    B0x0 = 0,
    ///1: Force the data lanes in TX Stop mode
    B0x1 = 1,
}
impl From<FTXSMDL> for bool {
    #[inline(always)]
    fn from(variant: FTXSMDL) -> Self {
        variant as u8 != 0
    }
}
///Field `FTXSMDL` reader - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
pub type FTXSMDL_R = crate::BitReader<FTXSMDL>;
impl FTXSMDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FTXSMDL {
        match self.bits {
            false => FTXSMDL::B0x0,
            true => FTXSMDL::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FTXSMDL::B0x0
    }
    ///Force the data lanes in TX Stop mode
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FTXSMDL::B0x1
    }
}
///Field `FTXSMDL` writer - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
pub type FTXSMDL_W<'a, REG> = crate::BitWriter<'a, REG, FTXSMDL>;
impl<'a, REG> FTXSMDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FTXSMDL::B0x0)
    }
    ///Force the data lanes in TX Stop mode
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FTXSMDL::B0x1)
    }
}
/**Contention detection OFF on data lanes When only forward escape mode is used, this signal can be made high to switch off the contention detector and reduce static power consumption.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDOFFDL {
    ///0: Contention detection on data lane ON
    B0x0 = 0,
    ///1: Contention detection on data lane OFF
    B0x1 = 1,
}
impl From<CDOFFDL> for bool {
    #[inline(always)]
    fn from(variant: CDOFFDL) -> Self {
        variant as u8 != 0
    }
}
///Field `CDOFFDL` reader - Contention detection OFF on data lanes When only forward escape mode is used, this signal can be made high to switch off the contention detector and reduce static power consumption.
pub type CDOFFDL_R = crate::BitReader<CDOFFDL>;
impl CDOFFDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CDOFFDL {
        match self.bits {
            false => CDOFFDL::B0x0,
            true => CDOFFDL::B0x1,
        }
    }
    ///Contention detection on data lane ON
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CDOFFDL::B0x0
    }
    ///Contention detection on data lane OFF
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CDOFFDL::B0x1
    }
}
///Field `CDOFFDL` writer - Contention detection OFF on data lanes When only forward escape mode is used, this signal can be made high to switch off the contention detector and reduce static power consumption.
pub type CDOFFDL_W<'a, REG> = crate::BitWriter<'a, REG, CDOFFDL>;
impl<'a, REG> CDOFFDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Contention detection on data lane ON
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CDOFFDL::B0x0)
    }
    ///Contention detection on data lane OFF
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CDOFFDL::B0x1)
    }
}
/**Turn disable data lanes This bit forces the data lane to remain in RX event if it receives a bus-turn-around request from the other side.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDDL {
    ///0: No effect
    B0x0 = 0,
    ///1: Force data lanes in RX mode after a BTA
    B0x1 = 1,
}
impl From<TDDL> for bool {
    #[inline(always)]
    fn from(variant: TDDL) -> Self {
        variant as u8 != 0
    }
}
///Field `TDDL` reader - Turn disable data lanes This bit forces the data lane to remain in RX event if it receives a bus-turn-around request from the other side.
pub type TDDL_R = crate::BitReader<TDDL>;
impl TDDL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDDL {
        match self.bits {
            false => TDDL::B0x0,
            true => TDDL::B0x1,
        }
    }
    ///No effect
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TDDL::B0x0
    }
    ///Force data lanes in RX mode after a BTA
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TDDL::B0x1
    }
}
///Field `TDDL` writer - Turn disable data lanes This bit forces the data lane to remain in RX event if it receives a bus-turn-around request from the other side.
pub type TDDL_W<'a, REG> = crate::BitWriter<'a, REG, TDDL>;
impl<'a, REG> TDDL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TDDL::B0x0)
    }
    ///Force data lanes in RX mode after a BTA
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TDDL::B0x1)
    }
}
/**Pull-down enable This bit enables a pull-down on the lane to prevent from floating states when unused.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDEN {
    ///0: Pull-down on lanes disabled
    B0x0 = 0,
    ///1: Pull-down on lanes enabled
    B0x1 = 1,
}
impl From<PDEN> for bool {
    #[inline(always)]
    fn from(variant: PDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PDEN` reader - Pull-down enable This bit enables a pull-down on the lane to prevent from floating states when unused.
pub type PDEN_R = crate::BitReader<PDEN>;
impl PDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PDEN {
        match self.bits {
            false => PDEN::B0x0,
            true => PDEN::B0x1,
        }
    }
    ///Pull-down on lanes disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PDEN::B0x0
    }
    ///Pull-down on lanes enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PDEN::B0x1
    }
}
///Field `PDEN` writer - Pull-down enable This bit enables a pull-down on the lane to prevent from floating states when unused.
pub type PDEN_W<'a, REG> = crate::BitWriter<'a, REG, PDEN>;
impl<'a, REG> PDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Pull-down on lanes disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PDEN::B0x0)
    }
    ///Pull-down on lanes enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PDEN::B0x1)
    }
}
/**Custom time for t<sub>CLK-PREPARE</sub> enable This bit enables the manual programming of t<sub>CLK-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the TLKCPREP field of the DSI_WPCR2 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCLKPREPEN {
    ///0: Default value is used for t<sub>CLK-PREPARE</sub>
    B0x0 = 0,
    ///1: Programmable value is used for t<sub>CLK-PREPARE</sub>
    B0x1 = 1,
}
impl From<TCLKPREPEN> for bool {
    #[inline(always)]
    fn from(variant: TCLKPREPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TCLKPREPEN` reader - Custom time for t<sub>CLK-PREPARE</sub> enable This bit enables the manual programming of t<sub>CLK-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the TLKCPREP field of the DSI_WPCR2 register.
pub type TCLKPREPEN_R = crate::BitReader<TCLKPREPEN>;
impl TCLKPREPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCLKPREPEN {
        match self.bits {
            false => TCLKPREPEN::B0x0,
            true => TCLKPREPEN::B0x1,
        }
    }
    ///Default value is used for t<sub>CLK-PREPARE</sub>
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCLKPREPEN::B0x0
    }
    ///Programmable value is used for t<sub>CLK-PREPARE</sub>
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCLKPREPEN::B0x1
    }
}
///Field `TCLKPREPEN` writer - Custom time for t<sub>CLK-PREPARE</sub> enable This bit enables the manual programming of t<sub>CLK-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the TLKCPREP field of the DSI_WPCR2 register.
pub type TCLKPREPEN_W<'a, REG> = crate::BitWriter<'a, REG, TCLKPREPEN>;
impl<'a, REG> TCLKPREPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Default value is used for t<sub>CLK-PREPARE</sub>
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCLKPREPEN::B0x0)
    }
    ///Programmable value is used for t<sub>CLK-PREPARE</sub>
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCLKPREPEN::B0x1)
    }
}
/**Custom time for t<sub>CLK-ZERO</sub> enable This bit enables the manual programming of t<sub>CLK-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the TCLKZERO field of the DSI_WPCR2 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCLKZEROEN {
    ///0: Default value is used for t<sub>CLK-ZERO</sub>.
    B0x0 = 0,
    ///1: Programmable value is used for t<sub>CLK-ZERO</sub>.
    B0x1 = 1,
}
impl From<TCLKZEROEN> for bool {
    #[inline(always)]
    fn from(variant: TCLKZEROEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TCLKZEROEN` reader - Custom time for t<sub>CLK-ZERO</sub> enable This bit enables the manual programming of t<sub>CLK-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the TCLKZERO field of the DSI_WPCR2 register.
pub type TCLKZEROEN_R = crate::BitReader<TCLKZEROEN>;
impl TCLKZEROEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCLKZEROEN {
        match self.bits {
            false => TCLKZEROEN::B0x0,
            true => TCLKZEROEN::B0x1,
        }
    }
    ///Default value is used for t<sub>CLK-ZERO</sub>.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCLKZEROEN::B0x0
    }
    ///Programmable value is used for t<sub>CLK-ZERO</sub>.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCLKZEROEN::B0x1
    }
}
///Field `TCLKZEROEN` writer - Custom time for t<sub>CLK-ZERO</sub> enable This bit enables the manual programming of t<sub>CLK-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the TCLKZERO field of the DSI_WPCR2 register.
pub type TCLKZEROEN_W<'a, REG> = crate::BitWriter<'a, REG, TCLKZEROEN>;
impl<'a, REG> TCLKZEROEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Default value is used for t<sub>CLK-ZERO</sub>.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCLKZEROEN::B0x0)
    }
    ///Programmable value is used for t<sub>CLK-ZERO</sub>.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCLKZEROEN::B0x1)
    }
}
/**Custom time for t<sub>HS-PREPARE</sub> enable This bit enables the manual programming of t<sub>HS-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the THSPREP field of the DSI_WPCR2 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THSPREPEN {
    ///0: Default value is used for t<sub>HS-PREPARE</sub>.
    B0x0 = 0,
    ///1: Programmable value is used for t<sub>HS-PREPARE</sub>.
    B0x1 = 1,
}
impl From<THSPREPEN> for bool {
    #[inline(always)]
    fn from(variant: THSPREPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `THSPREPEN` reader - Custom time for t<sub>HS-PREPARE</sub> enable This bit enables the manual programming of t<sub>HS-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the THSPREP field of the DSI_WPCR2 register.
pub type THSPREPEN_R = crate::BitReader<THSPREPEN>;
impl THSPREPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> THSPREPEN {
        match self.bits {
            false => THSPREPEN::B0x0,
            true => THSPREPEN::B0x1,
        }
    }
    ///Default value is used for t<sub>HS-PREPARE</sub>.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == THSPREPEN::B0x0
    }
    ///Programmable value is used for t<sub>HS-PREPARE</sub>.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == THSPREPEN::B0x1
    }
}
///Field `THSPREPEN` writer - Custom time for t<sub>HS-PREPARE</sub> enable This bit enables the manual programming of t<sub>HS-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the THSPREP field of the DSI_WPCR2 register.
pub type THSPREPEN_W<'a, REG> = crate::BitWriter<'a, REG, THSPREPEN>;
impl<'a, REG> THSPREPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Default value is used for t<sub>HS-PREPARE</sub>.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(THSPREPEN::B0x0)
    }
    ///Programmable value is used for t<sub>HS-PREPARE</sub>.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(THSPREPEN::B0x1)
    }
}
/**Custom time for t<sub>HS-TRAIL</sub> enable This bit enables the manual programming of T<sub>HS-TRAIL </sub>duration in the D-PHY. The desired value must be programmed in the THSRAIL field of the DSI_WPCR2 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THSTRAILEN {
    ///0: Default value is used for T<sub>HS-TRAIL</sub>.
    B0x0 = 0,
    ///1: Programmable value is used for T<sub>HS-TRAIL</sub>.
    B0x1 = 1,
}
impl From<THSTRAILEN> for bool {
    #[inline(always)]
    fn from(variant: THSTRAILEN) -> Self {
        variant as u8 != 0
    }
}
///Field `THSTRAILEN` reader - Custom time for t<sub>HS-TRAIL</sub> enable This bit enables the manual programming of T<sub>HS-TRAIL </sub>duration in the D-PHY. The desired value must be programmed in the THSRAIL field of the DSI_WPCR2 register.
pub type THSTRAILEN_R = crate::BitReader<THSTRAILEN>;
impl THSTRAILEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> THSTRAILEN {
        match self.bits {
            false => THSTRAILEN::B0x0,
            true => THSTRAILEN::B0x1,
        }
    }
    ///Default value is used for T<sub>HS-TRAIL</sub>.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == THSTRAILEN::B0x0
    }
    ///Programmable value is used for T<sub>HS-TRAIL</sub>.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == THSTRAILEN::B0x1
    }
}
///Field `THSTRAILEN` writer - Custom time for t<sub>HS-TRAIL</sub> enable This bit enables the manual programming of T<sub>HS-TRAIL </sub>duration in the D-PHY. The desired value must be programmed in the THSRAIL field of the DSI_WPCR2 register.
pub type THSTRAILEN_W<'a, REG> = crate::BitWriter<'a, REG, THSTRAILEN>;
impl<'a, REG> THSTRAILEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Default value is used for T<sub>HS-TRAIL</sub>.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(THSTRAILEN::B0x0)
    }
    ///Programmable value is used for T<sub>HS-TRAIL</sub>.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(THSTRAILEN::B0x1)
    }
}
/**Custom time for t<sub>HS-ZERO</sub> enable This bit enables the manual programming of t<sub>HS-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the THSZERO field of the DSI_WPCR3 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THSZEROEN {
    ///0: Default value is used for t<sub>HS-ZERO</sub>.
    B0x0 = 0,
    ///1: Programmable value is used for t<sub>HS-ZERO</sub>.
    B0x1 = 1,
}
impl From<THSZEROEN> for bool {
    #[inline(always)]
    fn from(variant: THSZEROEN) -> Self {
        variant as u8 != 0
    }
}
///Field `THSZEROEN` reader - Custom time for t<sub>HS-ZERO</sub> enable This bit enables the manual programming of t<sub>HS-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the THSZERO field of the DSI_WPCR3 register.
pub type THSZEROEN_R = crate::BitReader<THSZEROEN>;
impl THSZEROEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> THSZEROEN {
        match self.bits {
            false => THSZEROEN::B0x0,
            true => THSZEROEN::B0x1,
        }
    }
    ///Default value is used for t<sub>HS-ZERO</sub>.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == THSZEROEN::B0x0
    }
    ///Programmable value is used for t<sub>HS-ZERO</sub>.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == THSZEROEN::B0x1
    }
}
///Field `THSZEROEN` writer - Custom time for t<sub>HS-ZERO</sub> enable This bit enables the manual programming of t<sub>HS-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the THSZERO field of the DSI_WPCR3 register.
pub type THSZEROEN_W<'a, REG> = crate::BitWriter<'a, REG, THSZEROEN>;
impl<'a, REG> THSZEROEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Default value is used for t<sub>HS-ZERO</sub>.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(THSZEROEN::B0x0)
    }
    ///Programmable value is used for t<sub>HS-ZERO</sub>.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(THSZEROEN::B0x1)
    }
}
/**Custom time for t<sub>LPX</sub> for data lanes enable This bit enables the manual programming of T<sub>LPX </sub>duration for the data lanes in the D-PHY. The desired value must be programmed in the TLPXD field of the DSI_WPCR3 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TLPXDEN {
    ///0: Default value is used for T<sub>LPX</sub> for the data lanes.
    B0x0 = 0,
    ///1: Programmable value is used for T<sub>LPX</sub> for the data lanes.
    B0x1 = 1,
}
impl From<TLPXDEN> for bool {
    #[inline(always)]
    fn from(variant: TLPXDEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TLPXDEN` reader - Custom time for t<sub>LPX</sub> for data lanes enable This bit enables the manual programming of T<sub>LPX </sub>duration for the data lanes in the D-PHY. The desired value must be programmed in the TLPXD field of the DSI_WPCR3 register.
pub type TLPXDEN_R = crate::BitReader<TLPXDEN>;
impl TLPXDEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TLPXDEN {
        match self.bits {
            false => TLPXDEN::B0x0,
            true => TLPXDEN::B0x1,
        }
    }
    ///Default value is used for T<sub>LPX</sub> for the data lanes.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TLPXDEN::B0x0
    }
    ///Programmable value is used for T<sub>LPX</sub> for the data lanes.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TLPXDEN::B0x1
    }
}
///Field `TLPXDEN` writer - Custom time for t<sub>LPX</sub> for data lanes enable This bit enables the manual programming of T<sub>LPX </sub>duration for the data lanes in the D-PHY. The desired value must be programmed in the TLPXD field of the DSI_WPCR3 register.
pub type TLPXDEN_W<'a, REG> = crate::BitWriter<'a, REG, TLPXDEN>;
impl<'a, REG> TLPXDEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Default value is used for T<sub>LPX</sub> for the data lanes.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TLPXDEN::B0x0)
    }
    ///Programmable value is used for T<sub>LPX</sub> for the data lanes.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TLPXDEN::B0x1)
    }
}
/**Custom time for t<sub>HS-EXIT</sub> enable This bit enables the manual programming of t<sub>HS-EXIT </sub>duration in the D-PHY. The desired value must be programmed in the THSEXIT field of the DSI_WPCR3 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THSEXITEN {
    ///0: Default value is used for t<sub>HS-EXIT</sub>.
    B0x0 = 0,
    ///1: Programmable value is used for t<sub>HS-EXIT</sub>.
    B0x1 = 1,
}
impl From<THSEXITEN> for bool {
    #[inline(always)]
    fn from(variant: THSEXITEN) -> Self {
        variant as u8 != 0
    }
}
///Field `THSEXITEN` reader - Custom time for t<sub>HS-EXIT</sub> enable This bit enables the manual programming of t<sub>HS-EXIT </sub>duration in the D-PHY. The desired value must be programmed in the THSEXIT field of the DSI_WPCR3 register.
pub type THSEXITEN_R = crate::BitReader<THSEXITEN>;
impl THSEXITEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> THSEXITEN {
        match self.bits {
            false => THSEXITEN::B0x0,
            true => THSEXITEN::B0x1,
        }
    }
    ///Default value is used for t<sub>HS-EXIT</sub>.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == THSEXITEN::B0x0
    }
    ///Programmable value is used for t<sub>HS-EXIT</sub>.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == THSEXITEN::B0x1
    }
}
///Field `THSEXITEN` writer - Custom time for t<sub>HS-EXIT</sub> enable This bit enables the manual programming of t<sub>HS-EXIT </sub>duration in the D-PHY. The desired value must be programmed in the THSEXIT field of the DSI_WPCR3 register.
pub type THSEXITEN_W<'a, REG> = crate::BitWriter<'a, REG, THSEXITEN>;
impl<'a, REG> THSEXITEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Default value is used for t<sub>HS-EXIT</sub>.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(THSEXITEN::B0x0)
    }
    ///Programmable value is used for t<sub>HS-EXIT</sub>.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(THSEXITEN::B0x1)
    }
}
/**Custom time for t<sub>LPX</sub> for clock lane enable This bit enables the manual programming of t<sub>LPX</sub> duration for the clock lane in the D-PHY. The desired value must be programmed in the TLPXC field of the DSI_WPCR3 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TLPXCEN {
    ///0: Default value is used for t<sub>LPX</sub> for the clock lane.
    B0x0 = 0,
    ///1: Programmable value is used for t<sub>LPX</sub> for the clock lane.
    B0x1 = 1,
}
impl From<TLPXCEN> for bool {
    #[inline(always)]
    fn from(variant: TLPXCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TLPXCEN` reader - Custom time for t<sub>LPX</sub> for clock lane enable This bit enables the manual programming of t<sub>LPX</sub> duration for the clock lane in the D-PHY. The desired value must be programmed in the TLPXC field of the DSI_WPCR3 register.
pub type TLPXCEN_R = crate::BitReader<TLPXCEN>;
impl TLPXCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TLPXCEN {
        match self.bits {
            false => TLPXCEN::B0x0,
            true => TLPXCEN::B0x1,
        }
    }
    ///Default value is used for t<sub>LPX</sub> for the clock lane.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TLPXCEN::B0x0
    }
    ///Programmable value is used for t<sub>LPX</sub> for the clock lane.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TLPXCEN::B0x1
    }
}
///Field `TLPXCEN` writer - Custom time for t<sub>LPX</sub> for clock lane enable This bit enables the manual programming of t<sub>LPX</sub> duration for the clock lane in the D-PHY. The desired value must be programmed in the TLPXC field of the DSI_WPCR3 register.
pub type TLPXCEN_W<'a, REG> = crate::BitWriter<'a, REG, TLPXCEN>;
impl<'a, REG> TLPXCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Default value is used for t<sub>LPX</sub> for the clock lane.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TLPXCEN::B0x0)
    }
    ///Programmable value is used for t<sub>LPX</sub> for the clock lane.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TLPXCEN::B0x1)
    }
}
/**Custom time for t<sub>CLK-POST</sub> enable This bit enables the manual programming of t<sub>CLK-POST </sub>duration in the D-PHY. The desired value must be programmed in the TCLKPOST field of the DSI_WPCR4 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCLKPOSTEN {
    ///0: Default value is used for t<sub>CLKPOST</sub>.
    B0x0 = 0,
    ///1: Programmable value is used for t<sub>CLKPOST</sub>.
    B0x1 = 1,
}
impl From<TCLKPOSTEN> for bool {
    #[inline(always)]
    fn from(variant: TCLKPOSTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TCLKPOSTEN` reader - Custom time for t<sub>CLK-POST</sub> enable This bit enables the manual programming of t<sub>CLK-POST </sub>duration in the D-PHY. The desired value must be programmed in the TCLKPOST field of the DSI_WPCR4 register.
pub type TCLKPOSTEN_R = crate::BitReader<TCLKPOSTEN>;
impl TCLKPOSTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCLKPOSTEN {
        match self.bits {
            false => TCLKPOSTEN::B0x0,
            true => TCLKPOSTEN::B0x1,
        }
    }
    ///Default value is used for t<sub>CLKPOST</sub>.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCLKPOSTEN::B0x0
    }
    ///Programmable value is used for t<sub>CLKPOST</sub>.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCLKPOSTEN::B0x1
    }
}
///Field `TCLKPOSTEN` writer - Custom time for t<sub>CLK-POST</sub> enable This bit enables the manual programming of t<sub>CLK-POST </sub>duration in the D-PHY. The desired value must be programmed in the TCLKPOST field of the DSI_WPCR4 register.
pub type TCLKPOSTEN_W<'a, REG> = crate::BitWriter<'a, REG, TCLKPOSTEN>;
impl<'a, REG> TCLKPOSTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Default value is used for t<sub>CLKPOST</sub>.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCLKPOSTEN::B0x0)
    }
    ///Programmable value is used for t<sub>CLKPOST</sub>.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCLKPOSTEN::B0x1)
    }
}
impl R {
    ///Bits 0:5 - Unit interval multiplied by 4 This field defines the bit period in high-speed mode in unit of 0.25 ns. As an example, if the unit interval is 3 ns, a value of twelve (0x0C) must be driven to this input. This value is used to generate delays. If the period is not a multiple of 0.25 ns, the value driven must be rounded down. For example, a 600 Mbit/s link uses a unit interval of 1.667 ns, which, multiplied by four gives 6.667 ns. In this case a value of 6 (not 7) must be driven onto the ui_x4 input.
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 6 - Swap clock lane pins This bit swaps the pins on clock lane.
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Swap data lane 0 pins This bit swaps the pins on data lane 0.
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Swap data lane 1 pins This bit swaps the pins on clock lane.
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Invert high-speed data signal on clock lane This bit inverts the high-speed data signal on clock lane.
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Invert the high-speed data signal on data lane 0 This bit inverts the high-speed data signal on clock lane.
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Invert the high-speed data signal on data lane 1 This bit inverts the high-speed data signal on data lane 1.
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Contention detection OFF on data lanes When only forward escape mode is used, this signal can be made high to switch off the contention detector and reduce static power consumption.
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Turn disable data lanes This bit forces the data lane to remain in RX event if it receives a bus-turn-around request from the other side.
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Pull-down enable This bit enables a pull-down on the lane to prevent from floating states when unused.
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Custom time for t<sub>CLK-PREPARE</sub> enable This bit enables the manual programming of t<sub>CLK-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the TLKCPREP field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn tclkprepen(&self) -> TCLKPREPEN_R {
        TCLKPREPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Custom time for t<sub>CLK-ZERO</sub> enable This bit enables the manual programming of t<sub>CLK-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the TCLKZERO field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn tclkzeroen(&self) -> TCLKZEROEN_R {
        TCLKZEROEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Custom time for t<sub>HS-PREPARE</sub> enable This bit enables the manual programming of t<sub>HS-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the THSPREP field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn thsprepen(&self) -> THSPREPEN_R {
        THSPREPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Custom time for t<sub>HS-TRAIL</sub> enable This bit enables the manual programming of T<sub>HS-TRAIL </sub>duration in the D-PHY. The desired value must be programmed in the THSRAIL field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn thstrailen(&self) -> THSTRAILEN_R {
        THSTRAILEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Custom time for t<sub>HS-ZERO</sub> enable This bit enables the manual programming of t<sub>HS-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the THSZERO field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn thszeroen(&self) -> THSZEROEN_R {
        THSZEROEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Custom time for t<sub>LPX</sub> for data lanes enable This bit enables the manual programming of T<sub>LPX </sub>duration for the data lanes in the D-PHY. The desired value must be programmed in the TLPXD field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn tlpxden(&self) -> TLPXDEN_R {
        TLPXDEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Custom time for t<sub>HS-EXIT</sub> enable This bit enables the manual programming of t<sub>HS-EXIT </sub>duration in the D-PHY. The desired value must be programmed in the THSEXIT field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn thsexiten(&self) -> THSEXITEN_R {
        THSEXITEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Custom time for t<sub>LPX</sub> for clock lane enable This bit enables the manual programming of t<sub>LPX</sub> duration for the clock lane in the D-PHY. The desired value must be programmed in the TLPXC field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn tlpxcen(&self) -> TLPXCEN_R {
        TLPXCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Custom time for t<sub>CLK-POST</sub> enable This bit enables the manual programming of t<sub>CLK-POST </sub>duration in the D-PHY. The desired value must be programmed in the TCLKPOST field of the DSI_WPCR4 register.
    #[inline(always)]
    pub fn tclkposten(&self) -> TCLKPOSTEN_R {
        TCLKPOSTEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPCR0")
            .field("uix4", &self.uix4())
            .field("swcl", &self.swcl())
            .field("swdl0", &self.swdl0())
            .field("swdl1", &self.swdl1())
            .field("hsicl", &self.hsicl())
            .field("hsidl0", &self.hsidl0())
            .field("hsidl1", &self.hsidl1())
            .field("ftxsmcl", &self.ftxsmcl())
            .field("ftxsmdl", &self.ftxsmdl())
            .field("cdoffdl", &self.cdoffdl())
            .field("tddl", &self.tddl())
            .field("pden", &self.pden())
            .field("tclkprepen", &self.tclkprepen())
            .field("tclkzeroen", &self.tclkzeroen())
            .field("thsprepen", &self.thsprepen())
            .field("thstrailen", &self.thstrailen())
            .field("thszeroen", &self.thszeroen())
            .field("tlpxden", &self.tlpxden())
            .field("thsexiten", &self.thsexiten())
            .field("tlpxcen", &self.tlpxcen())
            .field("tclkposten", &self.tclkposten())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Unit interval multiplied by 4 This field defines the bit period in high-speed mode in unit of 0.25 ns. As an example, if the unit interval is 3 ns, a value of twelve (0x0C) must be driven to this input. This value is used to generate delays. If the period is not a multiple of 0.25 ns, the value driven must be rounded down. For example, a 600 Mbit/s link uses a unit interval of 1.667 ns, which, multiplied by four gives 6.667 ns. In this case a value of 6 (not 7) must be driven onto the ui_x4 input.
    #[inline(always)]
    pub fn uix4(&mut self) -> UIX4_W<WPCR0rs> {
        UIX4_W::new(self, 0)
    }
    ///Bit 6 - Swap clock lane pins This bit swaps the pins on clock lane.
    #[inline(always)]
    pub fn swcl(&mut self) -> SWCL_W<WPCR0rs> {
        SWCL_W::new(self, 6)
    }
    ///Bit 7 - Swap data lane 0 pins This bit swaps the pins on data lane 0.
    #[inline(always)]
    pub fn swdl0(&mut self) -> SWDL0_W<WPCR0rs> {
        SWDL0_W::new(self, 7)
    }
    ///Bit 8 - Swap data lane 1 pins This bit swaps the pins on clock lane.
    #[inline(always)]
    pub fn swdl1(&mut self) -> SWDL1_W<WPCR0rs> {
        SWDL1_W::new(self, 8)
    }
    ///Bit 9 - Invert high-speed data signal on clock lane This bit inverts the high-speed data signal on clock lane.
    #[inline(always)]
    pub fn hsicl(&mut self) -> HSICL_W<WPCR0rs> {
        HSICL_W::new(self, 9)
    }
    ///Bit 10 - Invert the high-speed data signal on data lane 0 This bit inverts the high-speed data signal on clock lane.
    #[inline(always)]
    pub fn hsidl0(&mut self) -> HSIDL0_W<WPCR0rs> {
        HSIDL0_W::new(self, 10)
    }
    ///Bit 11 - Invert the high-speed data signal on data lane 1 This bit inverts the high-speed data signal on data lane 1.
    #[inline(always)]
    pub fn hsidl1(&mut self) -> HSIDL1_W<WPCR0rs> {
        HSIDL1_W::new(self, 11)
    }
    ///Bit 12 - Force in TX Stop mode the clock lane This bit forces the clock lane in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
    #[inline(always)]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W<WPCR0rs> {
        FTXSMCL_W::new(self, 12)
    }
    ///Bit 13 - Force in TX Stop mode the data lanes This bit forces the data lanes in TX stop mode. It is used to initialize a lane module in transmit mode. It causes the lane module to immediately jump to transmit control mode and to begin transmitting a stop state (LP-11). It can be used to go back in TX mode after a wrong BTA sequence.
    #[inline(always)]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W<WPCR0rs> {
        FTXSMDL_W::new(self, 13)
    }
    ///Bit 14 - Contention detection OFF on data lanes When only forward escape mode is used, this signal can be made high to switch off the contention detector and reduce static power consumption.
    #[inline(always)]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W<WPCR0rs> {
        CDOFFDL_W::new(self, 14)
    }
    ///Bit 16 - Turn disable data lanes This bit forces the data lane to remain in RX event if it receives a bus-turn-around request from the other side.
    #[inline(always)]
    pub fn tddl(&mut self) -> TDDL_W<WPCR0rs> {
        TDDL_W::new(self, 16)
    }
    ///Bit 18 - Pull-down enable This bit enables a pull-down on the lane to prevent from floating states when unused.
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W<WPCR0rs> {
        PDEN_W::new(self, 18)
    }
    ///Bit 19 - Custom time for t<sub>CLK-PREPARE</sub> enable This bit enables the manual programming of t<sub>CLK-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the TLKCPREP field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn tclkprepen(&mut self) -> TCLKPREPEN_W<WPCR0rs> {
        TCLKPREPEN_W::new(self, 19)
    }
    ///Bit 20 - Custom time for t<sub>CLK-ZERO</sub> enable This bit enables the manual programming of t<sub>CLK-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the TCLKZERO field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn tclkzeroen(&mut self) -> TCLKZEROEN_W<WPCR0rs> {
        TCLKZEROEN_W::new(self, 20)
    }
    ///Bit 21 - Custom time for t<sub>HS-PREPARE</sub> enable This bit enables the manual programming of t<sub>HS-PREPARE </sub>duration in the D-PHY. The desired value must be programmed in the THSPREP field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn thsprepen(&mut self) -> THSPREPEN_W<WPCR0rs> {
        THSPREPEN_W::new(self, 21)
    }
    ///Bit 22 - Custom time for t<sub>HS-TRAIL</sub> enable This bit enables the manual programming of T<sub>HS-TRAIL </sub>duration in the D-PHY. The desired value must be programmed in the THSRAIL field of the DSI_WPCR2 register.
    #[inline(always)]
    pub fn thstrailen(&mut self) -> THSTRAILEN_W<WPCR0rs> {
        THSTRAILEN_W::new(self, 22)
    }
    ///Bit 23 - Custom time for t<sub>HS-ZERO</sub> enable This bit enables the manual programming of t<sub>HS-ZERO </sub>duration in the D-PHY. The desired value must be programmed in the THSZERO field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn thszeroen(&mut self) -> THSZEROEN_W<WPCR0rs> {
        THSZEROEN_W::new(self, 23)
    }
    ///Bit 24 - Custom time for t<sub>LPX</sub> for data lanes enable This bit enables the manual programming of T<sub>LPX </sub>duration for the data lanes in the D-PHY. The desired value must be programmed in the TLPXD field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn tlpxden(&mut self) -> TLPXDEN_W<WPCR0rs> {
        TLPXDEN_W::new(self, 24)
    }
    ///Bit 25 - Custom time for t<sub>HS-EXIT</sub> enable This bit enables the manual programming of t<sub>HS-EXIT </sub>duration in the D-PHY. The desired value must be programmed in the THSEXIT field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn thsexiten(&mut self) -> THSEXITEN_W<WPCR0rs> {
        THSEXITEN_W::new(self, 25)
    }
    ///Bit 26 - Custom time for t<sub>LPX</sub> for clock lane enable This bit enables the manual programming of t<sub>LPX</sub> duration for the clock lane in the D-PHY. The desired value must be programmed in the TLPXC field of the DSI_WPCR3 register.
    #[inline(always)]
    pub fn tlpxcen(&mut self) -> TLPXCEN_W<WPCR0rs> {
        TLPXCEN_W::new(self, 26)
    }
    ///Bit 27 - Custom time for t<sub>CLK-POST</sub> enable This bit enables the manual programming of t<sub>CLK-POST </sub>duration in the D-PHY. The desired value must be programmed in the TCLKPOST field of the DSI_WPCR4 register.
    #[inline(always)]
    pub fn tclkposten(&mut self) -> TCLKPOSTEN_W<WPCR0rs> {
        TCLKPOSTEN_W::new(self, 27)
    }
}
/**DSI Wrapper PHY configuration register 0

You can [`read`](crate::Reg::read) this register and get [`wpcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:WPCR0)*/
pub struct WPCR0rs;
impl crate::RegisterSpec for WPCR0rs {
    type Ux = u32;
}
///`read()` method returns [`wpcr0::R`](R) reader structure
impl crate::Readable for WPCR0rs {}
///`write(|w| ..)` method takes [`wpcr0::W`](W) writer structure
impl crate::Writable for WPCR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WPCR0 to value 0
impl crate::Resettable for WPCR0rs {}
