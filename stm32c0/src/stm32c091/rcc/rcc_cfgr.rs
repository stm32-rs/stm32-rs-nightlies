///Register `RCC_CFGR` reader
pub type R = crate::R<RCC_CFGRrs>;
///Register `RCC_CFGR` writer
pub type W = crate::W<RCC_CFGRrs>;
/**System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW {
    ///0: HSISYS
    B0x0 = 0,
    ///1: HSE
    B0x1 = 1,
    ///3: LSI
    B0x3 = 3,
    ///4: LSE
    B0x4 = 4,
}
impl From<SW> for u8 {
    #[inline(always)]
    fn from(variant: SW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SW {
    type Ux = u8;
}
impl crate::IsEnum for SW {}
///Field `SW` reader - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
pub type SW_R = crate::FieldReader<SW>;
impl SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SW> {
        match self.bits {
            0 => Some(SW::B0x0),
            1 => Some(SW::B0x1),
            3 => Some(SW::B0x3),
            4 => Some(SW::B0x4),
            _ => None,
        }
    }
    ///HSISYS
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SW::B0x0
    }
    ///HSE
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SW::B0x1
    }
    ///LSI
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SW::B0x3
    }
    ///LSE
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SW::B0x4
    }
}
///Field `SW` writer - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SW>;
impl<'a, REG> SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///HSISYS
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SW::B0x0)
    }
    ///HSE
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SW::B0x1)
    }
    ///LSI
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(SW::B0x3)
    }
    ///LSE
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(SW::B0x4)
    }
}
/**System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWS {
    ///0: HSISYS
    B0x0 = 0,
    ///1: HSE
    B0x1 = 1,
    ///3: LSI
    B0x3 = 3,
    ///4: LSE
    B0x4 = 4,
}
impl From<SWS> for u8 {
    #[inline(always)]
    fn from(variant: SWS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWS {
    type Ux = u8;
}
impl crate::IsEnum for SWS {}
///Field `SWS` reader - System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved
pub type SWS_R = crate::FieldReader<SWS>;
impl SWS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWS> {
        match self.bits {
            0 => Some(SWS::B0x0),
            1 => Some(SWS::B0x1),
            3 => Some(SWS::B0x3),
            4 => Some(SWS::B0x4),
            _ => None,
        }
    }
    ///HSISYS
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWS::B0x0
    }
    ///HSE
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWS::B0x1
    }
    ///LSI
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SWS::B0x3
    }
    ///LSE
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SWS::B0x4
    }
}
/**AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE {
    ///8: 2
    B0x8 = 8,
    ///9: 4
    B0x9 = 9,
    ///10: 8
    B0xA = 10,
    ///11: 16
    B0xB = 11,
    ///12: 64
    B0xC = 12,
    ///13: 128
    B0xD = 13,
    ///14: 256
    B0xE = 14,
    ///15: 512
    B0xF = 15,
}
impl From<HPRE> for u8 {
    #[inline(always)]
    fn from(variant: HPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPRE {
    type Ux = u8;
}
impl crate::IsEnum for HPRE {}
///Field `HPRE` reader - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1
pub type HPRE_R = crate::FieldReader<HPRE>;
impl HPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<HPRE> {
        match self.bits {
            8 => Some(HPRE::B0x8),
            9 => Some(HPRE::B0x9),
            10 => Some(HPRE::B0xA),
            11 => Some(HPRE::B0xB),
            12 => Some(HPRE::B0xC),
            13 => Some(HPRE::B0xD),
            14 => Some(HPRE::B0xE),
            15 => Some(HPRE::B0xF),
            _ => None,
        }
    }
    ///2
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == HPRE::B0x8
    }
    ///4
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == HPRE::B0x9
    }
    ///8
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == HPRE::B0xA
    }
    ///16
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == HPRE::B0xB
    }
    ///64
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == HPRE::B0xC
    }
    ///128
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == HPRE::B0xD
    }
    ///256
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == HPRE::B0xE
    }
    ///512
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == HPRE::B0xF
    }
}
///Field `HPRE` writer - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HPRE>;
impl<'a, REG> HPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::B0x8)
    }
    ///4
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::B0x9)
    }
    ///8
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::B0xA)
    }
    ///16
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::B0xB)
    }
    ///64
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::B0xC)
    }
    ///128
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::B0xD)
    }
    ///256
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::B0xE)
    }
    ///512
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(HPRE::B0xF)
    }
}
/**APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE {
    ///4: 2
    B0x4 = 4,
    ///5: 4
    B0x5 = 5,
    ///6: 8
    B0x6 = 6,
    ///7: 16
    B0x7 = 7,
}
impl From<PPRE> for u8 {
    #[inline(always)]
    fn from(variant: PPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE {
    type Ux = u8;
}
impl crate::IsEnum for PPRE {}
///Field `PPRE` reader - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
pub type PPRE_R = crate::FieldReader<PPRE>;
impl PPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PPRE> {
        match self.bits {
            4 => Some(PPRE::B0x4),
            5 => Some(PPRE::B0x5),
            6 => Some(PPRE::B0x6),
            7 => Some(PPRE::B0x7),
            _ => None,
        }
    }
    ///2
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PPRE::B0x4
    }
    ///4
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PPRE::B0x5
    }
    ///8
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PPRE::B0x6
    }
    ///16
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PPRE::B0x7
    }
}
///Field `PPRE` writer - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
pub type PPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PPRE>;
impl<'a, REG> PPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///2
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE::B0x4)
    }
    ///4
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE::B0x5)
    }
    ///8
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE::B0x6)
    }
    ///16
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PPRE::B0x7)
    }
}
/**Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Other: reserved, must not be used Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching. On STM32C011xx and STM32C031xx, MCOSEL\[3\] is reserved.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2SEL {
    ///0: no clock
    B0x0 = 0,
    ///1: SYSCLK
    B0x1 = 1,
    ///3: HSI48
    B0x3 = 3,
    ///4: HSE
    B0x4 = 4,
    ///6: LSI
    B0x6 = 6,
    ///7: LSE
    B0x7 = 7,
    ///8: HSIUSB48
    B0x8 = 8,
}
impl From<MCO2SEL> for u8 {
    #[inline(always)]
    fn from(variant: MCO2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO2SEL {
    type Ux = u8;
}
impl crate::IsEnum for MCO2SEL {}
///Field `MCO2SEL` reader - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Other: reserved, must not be used Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching. On STM32C011xx and STM32C031xx, MCOSEL\[3\] is reserved.
pub type MCO2SEL_R = crate::FieldReader<MCO2SEL>;
impl MCO2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO2SEL> {
        match self.bits {
            0 => Some(MCO2SEL::B0x0),
            1 => Some(MCO2SEL::B0x1),
            3 => Some(MCO2SEL::B0x3),
            4 => Some(MCO2SEL::B0x4),
            6 => Some(MCO2SEL::B0x6),
            7 => Some(MCO2SEL::B0x7),
            8 => Some(MCO2SEL::B0x8),
            _ => None,
        }
    }
    ///no clock
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCO2SEL::B0x0
    }
    ///SYSCLK
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCO2SEL::B0x1
    }
    ///HSI48
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MCO2SEL::B0x3
    }
    ///HSE
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MCO2SEL::B0x4
    }
    ///LSI
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == MCO2SEL::B0x6
    }
    ///LSE
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == MCO2SEL::B0x7
    }
    ///HSIUSB48
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == MCO2SEL::B0x8
    }
}
///Field `MCO2SEL` writer - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Other: reserved, must not be used Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching. On STM32C011xx and STM32C031xx, MCOSEL\[3\] is reserved.
pub type MCO2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCO2SEL>;
impl<'a, REG> MCO2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///no clock
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::B0x0)
    }
    ///SYSCLK
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::B0x1)
    }
    ///HSI48
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::B0x3)
    }
    ///HSE
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::B0x4)
    }
    ///LSI
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::B0x6)
    }
    ///LSE
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::B0x7)
    }
    ///HSIUSB48
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2SEL::B0x8)
    }
}
/**Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Other: Reserved It is highly recommended to set this field before the MCO2 output is enabled. Note: Values above 0111 are only significant for STM32C071xx. On STM32C011xx and STM32C031xx devices, MCOPRE\[3\] is reserved.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2PRE {
    ///0: 1
    B0x0 = 0,
    ///1: 2
    B0x1 = 1,
    ///2: 4
    B0x2 = 2,
    ///7: 128
    B0x7 = 7,
    ///8: 256
    B0x8 = 8,
    ///9: 512
    B0x9 = 9,
    ///10: 1024
    B0xA = 10,
}
impl From<MCO2PRE> for u8 {
    #[inline(always)]
    fn from(variant: MCO2PRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO2PRE {
    type Ux = u8;
}
impl crate::IsEnum for MCO2PRE {}
///Field `MCO2PRE` reader - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Other: Reserved It is highly recommended to set this field before the MCO2 output is enabled. Note: Values above 0111 are only significant for STM32C071xx. On STM32C011xx and STM32C031xx devices, MCOPRE\[3\] is reserved.
pub type MCO2PRE_R = crate::FieldReader<MCO2PRE>;
impl MCO2PRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCO2PRE> {
        match self.bits {
            0 => Some(MCO2PRE::B0x0),
            1 => Some(MCO2PRE::B0x1),
            2 => Some(MCO2PRE::B0x2),
            7 => Some(MCO2PRE::B0x7),
            8 => Some(MCO2PRE::B0x8),
            9 => Some(MCO2PRE::B0x9),
            10 => Some(MCO2PRE::B0xA),
            _ => None,
        }
    }
    ///1
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCO2PRE::B0x0
    }
    ///2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCO2PRE::B0x1
    }
    ///4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MCO2PRE::B0x2
    }
    ///128
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == MCO2PRE::B0x7
    }
    ///256
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == MCO2PRE::B0x8
    }
    ///512
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == MCO2PRE::B0x9
    }
    ///1024
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == MCO2PRE::B0xA
    }
}
///Field `MCO2PRE` writer - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Other: Reserved It is highly recommended to set this field before the MCO2 output is enabled. Note: Values above 0111 are only significant for STM32C071xx. On STM32C011xx and STM32C031xx devices, MCOPRE\[3\] is reserved.
pub type MCO2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCO2PRE>;
impl<'a, REG> MCO2PRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::B0x0)
    }
    ///2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::B0x1)
    }
    ///4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::B0x2)
    }
    ///128
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::B0x7)
    }
    ///256
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::B0x8)
    }
    ///512
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::B0x9)
    }
    ///1024
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(MCO2PRE::B0xA)
    }
}
/**Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Other: reserved, must not be used Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. On STM32C011xx and STM32C031xx, MCOSEL\[3\] is reserved.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOSEL {
    ///0: no clock
    B0x0 = 0,
    ///1: SYSCLK
    B0x1 = 1,
    ///3: HSI48
    B0x3 = 3,
    ///4: HSE
    B0x4 = 4,
    ///6: LSI
    B0x6 = 6,
    ///7: LSE
    B0x7 = 7,
    ///8: HSIUSB48
    B0x8 = 8,
}
impl From<MCOSEL> for u8 {
    #[inline(always)]
    fn from(variant: MCOSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCOSEL {
    type Ux = u8;
}
impl crate::IsEnum for MCOSEL {}
///Field `MCOSEL` reader - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Other: reserved, must not be used Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. On STM32C011xx and STM32C031xx, MCOSEL\[3\] is reserved.
pub type MCOSEL_R = crate::FieldReader<MCOSEL>;
impl MCOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCOSEL> {
        match self.bits {
            0 => Some(MCOSEL::B0x0),
            1 => Some(MCOSEL::B0x1),
            3 => Some(MCOSEL::B0x3),
            4 => Some(MCOSEL::B0x4),
            6 => Some(MCOSEL::B0x6),
            7 => Some(MCOSEL::B0x7),
            8 => Some(MCOSEL::B0x8),
            _ => None,
        }
    }
    ///no clock
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCOSEL::B0x0
    }
    ///SYSCLK
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCOSEL::B0x1
    }
    ///HSI48
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MCOSEL::B0x3
    }
    ///HSE
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MCOSEL::B0x4
    }
    ///LSI
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == MCOSEL::B0x6
    }
    ///LSE
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == MCOSEL::B0x7
    }
    ///HSIUSB48
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == MCOSEL::B0x8
    }
}
///Field `MCOSEL` writer - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Other: reserved, must not be used Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. On STM32C011xx and STM32C031xx, MCOSEL\[3\] is reserved.
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCOSEL>;
impl<'a, REG> MCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///no clock
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::B0x0)
    }
    ///SYSCLK
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::B0x1)
    }
    ///HSI48
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::B0x3)
    }
    ///HSE
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::B0x4)
    }
    ///LSI
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::B0x6)
    }
    ///LSE
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::B0x7)
    }
    ///HSIUSB48
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(MCOSEL::B0x8)
    }
}
/**Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Other: Reserved It is highly recommended to set this field before the MCO output is enabled. Note: Values above 0111 are only significant for STM32C071xx. On STM32C011xx and STM32C031xx devices, MCOPRE\[3\] is reserved.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOPRE {
    ///0: 1
    B0x0 = 0,
    ///1: 2
    B0x1 = 1,
    ///2: 4
    B0x2 = 2,
    ///7: 128
    B0x7 = 7,
    ///8: 256
    B0x8 = 8,
    ///9: 512
    B0x9 = 9,
    ///10: 1024
    B0xA = 10,
}
impl From<MCOPRE> for u8 {
    #[inline(always)]
    fn from(variant: MCOPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCOPRE {
    type Ux = u8;
}
impl crate::IsEnum for MCOPRE {}
///Field `MCOPRE` reader - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Other: Reserved It is highly recommended to set this field before the MCO output is enabled. Note: Values above 0111 are only significant for STM32C071xx. On STM32C011xx and STM32C031xx devices, MCOPRE\[3\] is reserved.
pub type MCOPRE_R = crate::FieldReader<MCOPRE>;
impl MCOPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MCOPRE> {
        match self.bits {
            0 => Some(MCOPRE::B0x0),
            1 => Some(MCOPRE::B0x1),
            2 => Some(MCOPRE::B0x2),
            7 => Some(MCOPRE::B0x7),
            8 => Some(MCOPRE::B0x8),
            9 => Some(MCOPRE::B0x9),
            10 => Some(MCOPRE::B0xA),
            _ => None,
        }
    }
    ///1
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCOPRE::B0x0
    }
    ///2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCOPRE::B0x1
    }
    ///4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MCOPRE::B0x2
    }
    ///128
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == MCOPRE::B0x7
    }
    ///256
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == MCOPRE::B0x8
    }
    ///512
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == MCOPRE::B0x9
    }
    ///1024
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == MCOPRE::B0xA
    }
}
///Field `MCOPRE` writer - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Other: Reserved It is highly recommended to set this field before the MCO output is enabled. Note: Values above 0111 are only significant for STM32C071xx. On STM32C011xx and STM32C031xx devices, MCOPRE\[3\] is reserved.
pub type MCOPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MCOPRE>;
impl<'a, REG> MCOPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::B0x0)
    }
    ///2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::B0x1)
    }
    ///4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::B0x2)
    }
    ///128
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::B0x7)
    }
    ///256
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::B0x8)
    }
    ///512
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::B0x9)
    }
    ///1024
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(MCOPRE::B0xA)
    }
}
impl R {
    ///Bits 0:2 - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - System clock switch status This bitfield is controlled by hardware to indicate the clock source used as system clock: Others: Reserved
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 8:11 - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
    #[inline(always)]
    pub fn ppre(&self) -> PPRE_R {
        PPRE_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:19 - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Other: reserved, must not be used Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching. On STM32C011xx and STM32C031xx, MCOSEL\[3\] is reserved.
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Other: Reserved It is highly recommended to set this field before the MCO2 output is enabled. Note: Values above 0111 are only significant for STM32C071xx. On STM32C011xx and STM32C031xx devices, MCOPRE\[3\] is reserved.
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Other: reserved, must not be used Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. On STM32C011xx and STM32C031xx, MCOSEL\[3\] is reserved.
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Other: Reserved It is highly recommended to set this field before the MCO output is enabled. Note: Values above 0111 are only significant for STM32C071xx. On STM32C011xx and STM32C031xx devices, MCOPRE\[3\] is reserved.
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CFGR")
            .field("sw", &self.sw())
            .field("sws", &self.sws())
            .field("hpre", &self.hpre())
            .field("ppre", &self.ppre())
            .field("mco2sel", &self.mco2sel())
            .field("mco2pre", &self.mco2pre())
            .field("mcosel", &self.mcosel())
            .field("mcopre", &self.mcopre())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - System clock switch This bitfield is controlled by software and hardware. The bitfield selects the clock for SYSCLK as follows: Others: Reserved The setting is forced by hardware to 000 (HSISYS selected) when the MCU exits Stop, or Standby, or Shutdown mode, or when the setting is 001 (HSE selected) and HSE oscillator failure is detected.
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<'_, RCC_CFGRrs> {
        SW_W::new(self, 0)
    }
    ///Bits 8:11 - AHB prescaler This bitfield is controlled by software. To produce HCLK clock, it sets the division factor of SYSCLK clock as follows: 0xxx: 1
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<'_, RCC_CFGRrs> {
        HPRE_W::new(self, 8)
    }
    ///Bits 12:14 - APB prescaler This bitfield is controlled by software. To produce PCLK clock, it sets the division factor of HCLK clock as follows: 0xx: 1
    #[inline(always)]
    pub fn ppre(&mut self) -> PPRE_W<'_, RCC_CFGRrs> {
        PPRE_W::new(self, 12)
    }
    ///Bits 16:19 - Microcontroller clock output 2 clock selector This bitfield is controlled by software. It sets the clock selector for MCO2 output as follows: Other: reserved, must not be used Note: This clock output may have some truncated cycles at startup or during MCO2 clock source switching. On STM32C011xx and STM32C031xx, MCOSEL\[3\] is reserved.
    #[inline(always)]
    pub fn mco2sel(&mut self) -> MCO2SEL_W<'_, RCC_CFGRrs> {
        MCO2SEL_W::new(self, 16)
    }
    ///Bits 20:23 - Microcontroller clock output 2 prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO2 output as follows: ... Other: Reserved It is highly recommended to set this field before the MCO2 output is enabled. Note: Values above 0111 are only significant for STM32C071xx. On STM32C011xx and STM32C031xx devices, MCOPRE\[3\] is reserved.
    #[inline(always)]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<'_, RCC_CFGRrs> {
        MCO2PRE_W::new(self, 20)
    }
    ///Bits 24:27 - Microcontroller clock output clock selector This bitfield is controlled by software. It sets the clock selector for MCO output as follows: Other: reserved, must not be used Note: This clock output may have some truncated cycles at startup or during MCO clock source switching. On STM32C011xx and STM32C031xx, MCOSEL\[3\] is reserved.
    #[inline(always)]
    pub fn mcosel(&mut self) -> MCOSEL_W<'_, RCC_CFGRrs> {
        MCOSEL_W::new(self, 24)
    }
    ///Bits 28:31 - Microcontroller clock output prescaler This bitfield is controlled by software. It sets the division factor of the clock sent to the MCO output as follows: ... Other: Reserved It is highly recommended to set this field before the MCO output is enabled. Note: Values above 0111 are only significant for STM32C071xx. On STM32C011xx and STM32C031xx devices, MCOPRE\[3\] is reserved.
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W<'_, RCC_CFGRrs> {
        MCOPRE_W::new(self, 28)
    }
}
/**RCC clock configuration register

You can [`read`](crate::Reg::read) this register and get [`rcc_cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#RCC:RCC_CFGR)*/
pub struct RCC_CFGRrs;
impl crate::RegisterSpec for RCC_CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_cfgr::R`](R) reader structure
impl crate::Readable for RCC_CFGRrs {}
///`write(|w| ..)` method takes [`rcc_cfgr::W`](W) writer structure
impl crate::Writable for RCC_CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCC_CFGR to value 0
impl crate::Resettable for RCC_CFGRrs {}
