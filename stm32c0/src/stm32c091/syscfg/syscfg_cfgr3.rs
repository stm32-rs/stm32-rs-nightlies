///Register `SYSCFG_CFGR3` reader
pub type R = crate::R<SYSCFG_CFGR3rs>;
///Register `SYSCFG_CFGR3` writer
pub type W = crate::W<SYSCFG_CFGR3rs>;
/**Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINMUX0 {
    ///0: PB7
    B0x0Stm32c011x_GpioAssignedToSo8Pin1 = 0,
}
impl From<PINMUX0> for u8 {
    #[inline(always)]
    fn from(variant: PINMUX0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PINMUX0 {
    type Ux = u8;
}
impl crate::IsEnum for PINMUX0 {}
///Field `PINMUX0` reader - Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved
pub type PINMUX0_R = crate::FieldReader<PINMUX0>;
impl PINMUX0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PINMUX0> {
        match self.bits {
            0 => Some(PINMUX0::B0x0Stm32c011x_GpioAssignedToSo8Pin1),
            _ => None,
        }
    }
    ///PB7
    #[inline(always)]
    pub fn is_b_0x0_stm32c011x___gpio_assigned_to_so8_pin_1(&self) -> bool {
        *self == PINMUX0::B0x0Stm32c011x_GpioAssignedToSo8Pin1
    }
}
///Field `PINMUX0` writer - Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved
pub type PINMUX0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PINMUX0>;
impl<'a, REG> PINMUX0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PB7
    #[inline(always)]
    pub fn b_0x0_stm32c011x___gpio_assigned_to_so8_pin_1(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX0::B0x0Stm32c011x_GpioAssignedToSo8Pin1)
    }
}
/**Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINMUX1 {
    ///0: PF2
    B0x0Stm32c011x_GpioAssignedToSo8Pin4 = 0,
    ///1: PA0
    B0x1Stm32c011x_GpioAssignedToSo8Pin4 = 1,
    ///2: PA1
    B0x2Stm32c011x_GpioAssignedToSo8Pin4 = 2,
    ///3: PA2
    B0x3Stm32c011x_GpioAssignedToSo8Pin4 = 3,
}
impl From<PINMUX1> for u8 {
    #[inline(always)]
    fn from(variant: PINMUX1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PINMUX1 {
    type Ux = u8;
}
impl crate::IsEnum for PINMUX1 {}
///Field `PINMUX1` reader - Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
pub type PINMUX1_R = crate::FieldReader<PINMUX1>;
impl PINMUX1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PINMUX1 {
        match self.bits {
            0 => PINMUX1::B0x0Stm32c011x_GpioAssignedToSo8Pin4,
            1 => PINMUX1::B0x1Stm32c011x_GpioAssignedToSo8Pin4,
            2 => PINMUX1::B0x2Stm32c011x_GpioAssignedToSo8Pin4,
            3 => PINMUX1::B0x3Stm32c011x_GpioAssignedToSo8Pin4,
            _ => unreachable!(),
        }
    }
    ///PF2
    #[inline(always)]
    pub fn is_b_0x0_stm32c011x___gpio_assigned_to_so8_pin_4(&self) -> bool {
        *self == PINMUX1::B0x0Stm32c011x_GpioAssignedToSo8Pin4
    }
    ///PA0
    #[inline(always)]
    pub fn is_b_0x1_stm32c011x___gpio_assigned_to_so8_pin_4(&self) -> bool {
        *self == PINMUX1::B0x1Stm32c011x_GpioAssignedToSo8Pin4
    }
    ///PA1
    #[inline(always)]
    pub fn is_b_0x2_stm32c011x___gpio_assigned_to_so8_pin_4(&self) -> bool {
        *self == PINMUX1::B0x2Stm32c011x_GpioAssignedToSo8Pin4
    }
    ///PA2
    #[inline(always)]
    pub fn is_b_0x3_stm32c011x___gpio_assigned_to_so8_pin_4(&self) -> bool {
        *self == PINMUX1::B0x3Stm32c011x_GpioAssignedToSo8Pin4
    }
}
///Field `PINMUX1` writer - Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
pub type PINMUX1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PINMUX1, crate::Safe>;
impl<'a, REG> PINMUX1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PF2
    #[inline(always)]
    pub fn b_0x0_stm32c011x___gpio_assigned_to_so8_pin_4(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX1::B0x0Stm32c011x_GpioAssignedToSo8Pin4)
    }
    ///PA0
    #[inline(always)]
    pub fn b_0x1_stm32c011x___gpio_assigned_to_so8_pin_4(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX1::B0x1Stm32c011x_GpioAssignedToSo8Pin4)
    }
    ///PA1
    #[inline(always)]
    pub fn b_0x2_stm32c011x___gpio_assigned_to_so8_pin_4(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX1::B0x2Stm32c011x_GpioAssignedToSo8Pin4)
    }
    ///PA2
    #[inline(always)]
    pub fn b_0x3_stm32c011x___gpio_assigned_to_so8_pin_4(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX1::B0x3Stm32c011x_GpioAssignedToSo8Pin4)
    }
}
/**Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Note: The PA11_RMP bit of the SYSCFG_CFGR1 takes priority over the selection through this bitfield. Refer to the description of the SYSCFG_CFGR1 register for more details.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINMUX2 {
    ///0: PA8
    B0x0Stm32c011x_GpioAssignedToSo8Pin5 = 0,
    ///1: PA11
    B0x1Stm32c011x_GpioAssignedToSo8Pin5 = 1,
}
impl From<PINMUX2> for u8 {
    #[inline(always)]
    fn from(variant: PINMUX2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PINMUX2 {
    type Ux = u8;
}
impl crate::IsEnum for PINMUX2 {}
///Field `PINMUX2` reader - Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Note: The PA11_RMP bit of the SYSCFG_CFGR1 takes priority over the selection through this bitfield. Refer to the description of the SYSCFG_CFGR1 register for more details.
pub type PINMUX2_R = crate::FieldReader<PINMUX2>;
impl PINMUX2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PINMUX2> {
        match self.bits {
            0 => Some(PINMUX2::B0x0Stm32c011x_GpioAssignedToSo8Pin5),
            1 => Some(PINMUX2::B0x1Stm32c011x_GpioAssignedToSo8Pin5),
            _ => None,
        }
    }
    ///PA8
    #[inline(always)]
    pub fn is_b_0x0_stm32c011x___gpio_assigned_to_so8_pin_5(&self) -> bool {
        *self == PINMUX2::B0x0Stm32c011x_GpioAssignedToSo8Pin5
    }
    ///PA11
    #[inline(always)]
    pub fn is_b_0x1_stm32c011x___gpio_assigned_to_so8_pin_5(&self) -> bool {
        *self == PINMUX2::B0x1Stm32c011x_GpioAssignedToSo8Pin5
    }
}
///Field `PINMUX2` writer - Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Note: The PA11_RMP bit of the SYSCFG_CFGR1 takes priority over the selection through this bitfield. Refer to the description of the SYSCFG_CFGR1 register for more details.
pub type PINMUX2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PINMUX2>;
impl<'a, REG> PINMUX2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA8
    #[inline(always)]
    pub fn b_0x0_stm32c011x___gpio_assigned_to_so8_pin_5(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX2::B0x0Stm32c011x_GpioAssignedToSo8Pin5)
    }
    ///PA11
    #[inline(always)]
    pub fn b_0x1_stm32c011x___gpio_assigned_to_so8_pin_5(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX2::B0x1Stm32c011x_GpioAssignedToSo8Pin5)
    }
}
/**Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINMUX3 {
    ///0: PA14
    B0x0Stm32c011x_GpioAssignedToSo8Pin8 = 0,
    ///1: PB6
    B0x1Stm32c011x_GpioAssignedToSo8Pin8 = 1,
    ///2: PC15
    B0x2Stm32c011x_GpioAssignedToSo8Pin8 = 2,
}
impl From<PINMUX3> for u8 {
    #[inline(always)]
    fn from(variant: PINMUX3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PINMUX3 {
    type Ux = u8;
}
impl crate::IsEnum for PINMUX3 {}
///Field `PINMUX3` reader - Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
pub type PINMUX3_R = crate::FieldReader<PINMUX3>;
impl PINMUX3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PINMUX3> {
        match self.bits {
            0 => Some(PINMUX3::B0x0Stm32c011x_GpioAssignedToSo8Pin8),
            1 => Some(PINMUX3::B0x1Stm32c011x_GpioAssignedToSo8Pin8),
            2 => Some(PINMUX3::B0x2Stm32c011x_GpioAssignedToSo8Pin8),
            _ => None,
        }
    }
    ///PA14
    #[inline(always)]
    pub fn is_b_0x0_stm32c011x___gpio_assigned_to_so8_pin_8(&self) -> bool {
        *self == PINMUX3::B0x0Stm32c011x_GpioAssignedToSo8Pin8
    }
    ///PB6
    #[inline(always)]
    pub fn is_b_0x1_stm32c011x___gpio_assigned_to_so8_pin_8(&self) -> bool {
        *self == PINMUX3::B0x1Stm32c011x_GpioAssignedToSo8Pin8
    }
    ///PC15
    #[inline(always)]
    pub fn is_b_0x2_stm32c011x___gpio_assigned_to_so8_pin_8(&self) -> bool {
        *self == PINMUX3::B0x2Stm32c011x_GpioAssignedToSo8Pin8
    }
}
///Field `PINMUX3` writer - Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
pub type PINMUX3_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PINMUX3>;
impl<'a, REG> PINMUX3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA14
    #[inline(always)]
    pub fn b_0x0_stm32c011x___gpio_assigned_to_so8_pin_8(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX3::B0x0Stm32c011x_GpioAssignedToSo8Pin8)
    }
    ///PB6
    #[inline(always)]
    pub fn b_0x1_stm32c011x___gpio_assigned_to_so8_pin_8(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX3::B0x1Stm32c011x_GpioAssignedToSo8Pin8)
    }
    ///PC15
    #[inline(always)]
    pub fn b_0x2_stm32c011x___gpio_assigned_to_so8_pin_8(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX3::B0x2Stm32c011x_GpioAssignedToSo8Pin8)
    }
}
/**Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Note: The PA12_RMP bit of the SYSCFG_CFGR1 takes priority over the selection through this bitfield. Refer to the description of the SYSCFG_CFGR1 register for more details.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINMUX4 {
    ///0: PA7
    B0x0Stm32c011x_GpioAssignedToWlcsp12PinE2 = 0,
    ///1: PA12
    B0x1Stm32c011x_GpioAssignedToWlcsp12PinE2 = 1,
}
impl From<PINMUX4> for u8 {
    #[inline(always)]
    fn from(variant: PINMUX4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PINMUX4 {
    type Ux = u8;
}
impl crate::IsEnum for PINMUX4 {}
///Field `PINMUX4` reader - Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Note: The PA12_RMP bit of the SYSCFG_CFGR1 takes priority over the selection through this bitfield. Refer to the description of the SYSCFG_CFGR1 register for more details.
pub type PINMUX4_R = crate::FieldReader<PINMUX4>;
impl PINMUX4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PINMUX4> {
        match self.bits {
            0 => Some(PINMUX4::B0x0Stm32c011x_GpioAssignedToWlcsp12PinE2),
            1 => Some(PINMUX4::B0x1Stm32c011x_GpioAssignedToWlcsp12PinE2),
            _ => None,
        }
    }
    ///PA7
    #[inline(always)]
    pub fn is_b_0x0_stm32c011x___gpio_assigned_to_wlcsp12_pin_e2(&self) -> bool {
        *self == PINMUX4::B0x0Stm32c011x_GpioAssignedToWlcsp12PinE2
    }
    ///PA12
    #[inline(always)]
    pub fn is_b_0x1_stm32c011x___gpio_assigned_to_wlcsp12_pin_e2(&self) -> bool {
        *self == PINMUX4::B0x1Stm32c011x_GpioAssignedToWlcsp12PinE2
    }
}
///Field `PINMUX4` writer - Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Note: The PA12_RMP bit of the SYSCFG_CFGR1 takes priority over the selection through this bitfield. Refer to the description of the SYSCFG_CFGR1 register for more details.
pub type PINMUX4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PINMUX4>;
impl<'a, REG> PINMUX4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA7
    #[inline(always)]
    pub fn b_0x0_stm32c011x___gpio_assigned_to_wlcsp12_pin_e2(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX4::B0x0Stm32c011x_GpioAssignedToWlcsp12PinE2)
    }
    ///PA12
    #[inline(always)]
    pub fn b_0x1_stm32c011x___gpio_assigned_to_wlcsp12_pin_e2(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX4::B0x1Stm32c011x_GpioAssignedToWlcsp12PinE2)
    }
}
/**Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINMUX5 {
    ///0: PA3
    B0x0Stm32c011x_GpioAssignedToWlcsp12PinF1 = 0,
    ///1: PA4
    B0x1Stm32c011x_GpioAssignedToWlcsp12PinF1 = 1,
    ///2: PA5
    B0x2Stm32c011x_GpioAssignedToWlcsp12PinF1 = 2,
    ///3: PA6
    B0x3Stm32c011x_GpioAssignedToWlcsp12PinF1 = 3,
}
impl From<PINMUX5> for u8 {
    #[inline(always)]
    fn from(variant: PINMUX5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PINMUX5 {
    type Ux = u8;
}
impl crate::IsEnum for PINMUX5 {}
///Field `PINMUX5` reader - Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin.
pub type PINMUX5_R = crate::FieldReader<PINMUX5>;
impl PINMUX5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PINMUX5 {
        match self.bits {
            0 => PINMUX5::B0x0Stm32c011x_GpioAssignedToWlcsp12PinF1,
            1 => PINMUX5::B0x1Stm32c011x_GpioAssignedToWlcsp12PinF1,
            2 => PINMUX5::B0x2Stm32c011x_GpioAssignedToWlcsp12PinF1,
            3 => PINMUX5::B0x3Stm32c011x_GpioAssignedToWlcsp12PinF1,
            _ => unreachable!(),
        }
    }
    ///PA3
    #[inline(always)]
    pub fn is_b_0x0_stm32c011x___gpio_assigned_to_wlcsp12_pin_f1(&self) -> bool {
        *self == PINMUX5::B0x0Stm32c011x_GpioAssignedToWlcsp12PinF1
    }
    ///PA4
    #[inline(always)]
    pub fn is_b_0x1_stm32c011x___gpio_assigned_to_wlcsp12_pin_f1(&self) -> bool {
        *self == PINMUX5::B0x1Stm32c011x_GpioAssignedToWlcsp12PinF1
    }
    ///PA5
    #[inline(always)]
    pub fn is_b_0x2_stm32c011x___gpio_assigned_to_wlcsp12_pin_f1(&self) -> bool {
        *self == PINMUX5::B0x2Stm32c011x_GpioAssignedToWlcsp12PinF1
    }
    ///PA6
    #[inline(always)]
    pub fn is_b_0x3_stm32c011x___gpio_assigned_to_wlcsp12_pin_f1(&self) -> bool {
        *self == PINMUX5::B0x3Stm32c011x_GpioAssignedToWlcsp12PinF1
    }
}
///Field `PINMUX5` writer - Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin.
pub type PINMUX5_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PINMUX5, crate::Safe>;
impl<'a, REG> PINMUX5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///PA3
    #[inline(always)]
    pub fn b_0x0_stm32c011x___gpio_assigned_to_wlcsp12_pin_f1(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX5::B0x0Stm32c011x_GpioAssignedToWlcsp12PinF1)
    }
    ///PA4
    #[inline(always)]
    pub fn b_0x1_stm32c011x___gpio_assigned_to_wlcsp12_pin_f1(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX5::B0x1Stm32c011x_GpioAssignedToWlcsp12PinF1)
    }
    ///PA5
    #[inline(always)]
    pub fn b_0x2_stm32c011x___gpio_assigned_to_wlcsp12_pin_f1(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX5::B0x2Stm32c011x_GpioAssignedToWlcsp12PinF1)
    }
    ///PA6
    #[inline(always)]
    pub fn b_0x3_stm32c011x___gpio_assigned_to_wlcsp12_pin_f1(self) -> &'a mut crate::W<REG> {
        self.variant(PINMUX5::B0x3Stm32c011x_GpioAssignedToWlcsp12PinF1)
    }
}
impl R {
    ///Bits 0:1 - Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved
    #[inline(always)]
    pub fn pinmux0(&self) -> PINMUX0_R {
        PINMUX0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
    #[inline(always)]
    pub fn pinmux1(&self) -> PINMUX1_R {
        PINMUX1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Note: The PA11_RMP bit of the SYSCFG_CFGR1 takes priority over the selection through this bitfield. Refer to the description of the SYSCFG_CFGR1 register for more details.
    #[inline(always)]
    pub fn pinmux2(&self) -> PINMUX2_R {
        PINMUX2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
    #[inline(always)]
    pub fn pinmux3(&self) -> PINMUX3_R {
        PINMUX3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Note: The PA12_RMP bit of the SYSCFG_CFGR1 takes priority over the selection through this bitfield. Refer to the description of the SYSCFG_CFGR1 register for more details.
    #[inline(always)]
    pub fn pinmux4(&self) -> PINMUX4_R {
        PINMUX4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin.
    #[inline(always)]
    pub fn pinmux5(&self) -> PINMUX5_R {
        PINMUX5_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_CFGR3")
            .field("pinmux0", &self.pinmux0())
            .field("pinmux1", &self.pinmux1())
            .field("pinmux2", &self.pinmux2())
            .field("pinmux3", &self.pinmux3())
            .field("pinmux4", &self.pinmux4())
            .field("pinmux5", &self.pinmux5())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Pin GPIO multiplexer 0 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved 1x: Reserved
    #[inline(always)]
    pub fn pinmux0(&mut self) -> PINMUX0_W<'_, SYSCFG_CFGR3rs> {
        PINMUX0_W::new(self, 0)
    }
    ///Bits 2:3 - Pin GPIO multiplexer 1 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
    #[inline(always)]
    pub fn pinmux1(&mut self) -> PINMUX1_W<'_, SYSCFG_CFGR3rs> {
        PINMUX1_W::new(self, 2)
    }
    ///Bits 4:5 - Pin GPIO multiplexer 2 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Note: The PA11_RMP bit of the SYSCFG_CFGR1 takes priority over the selection through this bitfield. Refer to the description of the SYSCFG_CFGR1 register for more details.
    #[inline(always)]
    pub fn pinmux2(&mut self) -> PINMUX2_W<'_, SYSCFG_CFGR3rs> {
        PINMUX2_W::new(self, 4)
    }
    ///Bits 6:7 - Pin GPIO multiplexer 3 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved
    #[inline(always)]
    pub fn pinmux3(&mut self) -> PINMUX3_W<'_, SYSCFG_CFGR3rs> {
        PINMUX3_W::new(self, 6)
    }
    ///Bits 8:9 - Pin GPIO multiplexer 4 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin. 1x: Reserved Note: The PA12_RMP bit of the SYSCFG_CFGR1 takes priority over the selection through this bitfield. Refer to the description of the SYSCFG_CFGR1 register for more details.
    #[inline(always)]
    pub fn pinmux4(&mut self) -> PINMUX4_W<'_, SYSCFG_CFGR3rs> {
        PINMUX4_W::new(self, 8)
    }
    ///Bits 10:11 - Pin GPIO multiplexer 5 This bit is set by software and cleared by system reset. It assigns a GPIO to a pin.
    #[inline(always)]
    pub fn pinmux5(&mut self) -> PINMUX5_W<'_, SYSCFG_CFGR3rs> {
        PINMUX5_W::new(self, 10)
    }
}
/**SYSCFG configuration register 3

You can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#SYSCFG:SYSCFG_CFGR3)*/
pub struct SYSCFG_CFGR3rs;
impl crate::RegisterSpec for SYSCFG_CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_cfgr3::R`](R) reader structure
impl crate::Readable for SYSCFG_CFGR3rs {}
///`write(|w| ..)` method takes [`syscfg_cfgr3::W`](W) writer structure
impl crate::Writable for SYSCFG_CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSCFG_CFGR3 to value 0
impl crate::Resettable for SYSCFG_CFGR3rs {}
