///Register `SYSCFG_CFGR1` reader
pub type R = crate::R<SYSCFG_CFGR1rs>;
///Register `SYSCFG_CFGR1` writer
pub type W = crate::W<SYSCFG_CFGR1rs>;
/**Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to Section 3: Boot configuration for more details. x0: Main Flash memory

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MODE {
    ///1: System Flash memory
    B0x1 = 1,
    ///3: Embedded SRAM
    B0x3 = 3,
}
impl From<MEM_MODE> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MEM_MODE {
    type Ux = u8;
}
impl crate::IsEnum for MEM_MODE {}
///Field `MEM_MODE` reader - Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to Section 3: Boot configuration for more details. x0: Main Flash memory
pub type MEM_MODE_R = crate::FieldReader<MEM_MODE>;
impl MEM_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MEM_MODE> {
        match self.bits {
            1 => Some(MEM_MODE::B0x1),
            3 => Some(MEM_MODE::B0x3),
            _ => None,
        }
    }
    ///System Flash memory
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MEM_MODE::B0x1
    }
    ///Embedded SRAM
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MEM_MODE::B0x3
    }
}
///Field `MEM_MODE` writer - Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to Section 3: Boot configuration for more details. x0: Main Flash memory
pub type MEM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MEM_MODE>;
impl<'a, REG> MEM_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///System Flash memory
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::B0x1)
    }
    ///Embedded SRAM
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MODE::B0x3)
    }
}
/**PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port. Note: If the PINMUX2\[1:0\] bitfield of the SYSCFG_CFGR3 register is at 00, PA11_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PA11_RMP {
    ///0: No remap (PA11)
    B0x0 = 0,
    ///1: Remap (PA9)
    B0x1 = 1,
}
impl From<PA11_RMP> for bool {
    #[inline(always)]
    fn from(variant: PA11_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `PA11_RMP` reader - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port. Note: If the PINMUX2\[1:0\] bitfield of the SYSCFG_CFGR3 register is at 00, PA11_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.
pub type PA11_RMP_R = crate::BitReader<PA11_RMP>;
impl PA11_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PA11_RMP {
        match self.bits {
            false => PA11_RMP::B0x0,
            true => PA11_RMP::B0x1,
        }
    }
    ///No remap (PA11)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PA11_RMP::B0x0
    }
    ///Remap (PA9)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PA11_RMP::B0x1
    }
}
///Field `PA11_RMP` writer - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port. Note: If the PINMUX2\[1:0\] bitfield of the SYSCFG_CFGR3 register is at 00, PA11_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.
pub type PA11_RMP_W<'a, REG> = crate::BitWriter<'a, REG, PA11_RMP>;
impl<'a, REG> PA11_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No remap (PA11)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PA11_RMP::B0x0)
    }
    ///Remap (PA9)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PA11_RMP::B0x1)
    }
}
/**PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port. Note: If the PINMUX4\[1:0\] bitfield of the SYSCFG_CFGR3 register is at 00, PA12_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PA12_RMP {
    ///0: No remap (PA12)
    B0x0 = 0,
    ///1: Remap (PA10)
    B0x1 = 1,
}
impl From<PA12_RMP> for bool {
    #[inline(always)]
    fn from(variant: PA12_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `PA12_RMP` reader - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port. Note: If the PINMUX4\[1:0\] bitfield of the SYSCFG_CFGR3 register is at 00, PA12_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.
pub type PA12_RMP_R = crate::BitReader<PA12_RMP>;
impl PA12_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PA12_RMP {
        match self.bits {
            false => PA12_RMP::B0x0,
            true => PA12_RMP::B0x1,
        }
    }
    ///No remap (PA12)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PA12_RMP::B0x0
    }
    ///Remap (PA10)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PA12_RMP::B0x1
    }
}
///Field `PA12_RMP` writer - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port. Note: If the PINMUX4\[1:0\] bitfield of the SYSCFG_CFGR3 register is at 00, PA12_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.
pub type PA12_RMP_W<'a, REG> = crate::BitWriter<'a, REG, PA12_RMP>;
impl<'a, REG> PA12_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No remap (PA12)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PA12_RMP::B0x0)
    }
    ///Remap (PA10)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PA12_RMP::B0x1)
    }
}
/**IR output polarity selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IR_POL {
    ///0: Output of IRTIM (IR_OUT) is not inverted
    B0x0 = 0,
    ///1: Output of IRTIM (IR_OUT) is inverted
    B0x1 = 1,
}
impl From<IR_POL> for bool {
    #[inline(always)]
    fn from(variant: IR_POL) -> Self {
        variant as u8 != 0
    }
}
///Field `IR_POL` reader - IR output polarity selection
pub type IR_POL_R = crate::BitReader<IR_POL>;
impl IR_POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IR_POL {
        match self.bits {
            false => IR_POL::B0x0,
            true => IR_POL::B0x1,
        }
    }
    ///Output of IRTIM (IR_OUT) is not inverted
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IR_POL::B0x0
    }
    ///Output of IRTIM (IR_OUT) is inverted
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IR_POL::B0x1
    }
}
///Field `IR_POL` writer - IR output polarity selection
pub type IR_POL_W<'a, REG> = crate::BitWriter<'a, REG, IR_POL>;
impl<'a, REG> IR_POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output of IRTIM (IR_OUT) is not inverted
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IR_POL::B0x0)
    }
    ///Output of IRTIM (IR_OUT) is inverted
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IR_POL::B0x1)
    }
}
/**IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IR_MOD {
    ///0: TIM16
    B0x0 = 0,
    ///1: USART1
    B0x1 = 1,
    ///2: USART2
    B0x2 = 2,
}
impl From<IR_MOD> for u8 {
    #[inline(always)]
    fn from(variant: IR_MOD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IR_MOD {
    type Ux = u8;
}
impl crate::IsEnum for IR_MOD {}
///Field `IR_MOD` reader - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:
pub type IR_MOD_R = crate::FieldReader<IR_MOD>;
impl IR_MOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<IR_MOD> {
        match self.bits {
            0 => Some(IR_MOD::B0x0),
            1 => Some(IR_MOD::B0x1),
            2 => Some(IR_MOD::B0x2),
            _ => None,
        }
    }
    ///TIM16
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IR_MOD::B0x0
    }
    ///USART1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IR_MOD::B0x1
    }
    ///USART2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == IR_MOD::B0x2
    }
}
///Field `IR_MOD` writer - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:
pub type IR_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IR_MOD>;
impl<'a, REG> IR_MOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM16
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IR_MOD::B0x0)
    }
    ///USART1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IR_MOD::B0x1)
    }
    ///USART2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(IR_MOD::B0x2)
    }
}
/**Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB6_FMP {
    ///0: Disable disabled if not enabled through I2Cx_FMP
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C_PB6_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB6_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB6_FMP` reader - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port.
pub type I2C_PB6_FMP_R = crate::BitReader<I2C_PB6_FMP>;
impl I2C_PB6_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB6_FMP {
        match self.bits {
            false => I2C_PB6_FMP::B0x0,
            true => I2C_PB6_FMP::B0x1,
        }
    }
    ///Disable disabled if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C_PB6_FMP::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C_PB6_FMP::B0x1
    }
}
///Field `I2C_PB6_FMP` writer - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port.
pub type I2C_PB6_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB6_FMP>;
impl<'a, REG> I2C_PB6_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable disabled if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB6_FMP::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB6_FMP::B0x1)
    }
}
/**Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB7_FMP {
    ///0: Disable disabled if not enabled through I2Cx_FMP
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C_PB7_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB7_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB7_FMP` reader - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port.
pub type I2C_PB7_FMP_R = crate::BitReader<I2C_PB7_FMP>;
impl I2C_PB7_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB7_FMP {
        match self.bits {
            false => I2C_PB7_FMP::B0x0,
            true => I2C_PB7_FMP::B0x1,
        }
    }
    ///Disable disabled if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C_PB7_FMP::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C_PB7_FMP::B0x1
    }
}
///Field `I2C_PB7_FMP` writer - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port.
pub type I2C_PB7_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB7_FMP>;
impl<'a, REG> I2C_PB7_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable disabled if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB7_FMP::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB7_FMP::B0x1)
    }
}
/**Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. Note: Not available on STM32C011xx.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB8_FMP {
    ///0: Disable disabled if not enabled through I2Cx_FMP
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C_PB8_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB8_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB8_FMP` reader - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. Note: Not available on STM32C011xx.
pub type I2C_PB8_FMP_R = crate::BitReader<I2C_PB8_FMP>;
impl I2C_PB8_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB8_FMP {
        match self.bits {
            false => I2C_PB8_FMP::B0x0,
            true => I2C_PB8_FMP::B0x1,
        }
    }
    ///Disable disabled if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C_PB8_FMP::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C_PB8_FMP::B0x1
    }
}
///Field `I2C_PB8_FMP` writer - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. Note: Not available on STM32C011xx.
pub type I2C_PB8_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB8_FMP>;
impl<'a, REG> I2C_PB8_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable disabled if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB8_FMP::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB8_FMP::B0x1)
    }
}
/**Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. Note: Not available on STM32C011xx.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PB9_FMP {
    ///0: Disable disabled if not enabled through I2Cx_FMP
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C_PB9_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PB9_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PB9_FMP` reader - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. Note: Not available on STM32C011xx.
pub type I2C_PB9_FMP_R = crate::BitReader<I2C_PB9_FMP>;
impl I2C_PB9_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PB9_FMP {
        match self.bits {
            false => I2C_PB9_FMP::B0x0,
            true => I2C_PB9_FMP::B0x1,
        }
    }
    ///Disable disabled if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C_PB9_FMP::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C_PB9_FMP::B0x1
    }
}
///Field `I2C_PB9_FMP` writer - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. Note: Not available on STM32C011xx.
pub type I2C_PB9_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PB9_FMP>;
impl<'a, REG> I2C_PB9_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable disabled if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB9_FMP::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PB9_FMP::B0x1)
    }
}
/**Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_FMP {
    ///0: Disable disabled if not enabled through I2C_y_FMP
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C1_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C1_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C1_FMP` reader - Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers.
pub type I2C1_FMP_R = crate::BitReader<I2C1_FMP>;
impl I2C1_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C1_FMP {
        match self.bits {
            false => I2C1_FMP::B0x0,
            true => I2C1_FMP::B0x1,
        }
    }
    ///Disable disabled if not enabled through I2C_y_FMP
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C1_FMP::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C1_FMP::B0x1
    }
}
///Field `I2C1_FMP` writer - Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers.
pub type I2C1_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C1_FMP>;
impl<'a, REG> I2C1_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable disabled if not enabled through I2C_y_FMP
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_FMP::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_FMP::B0x1)
    }
}
/**Fast Mode Plus (FM+) enable for I2C2 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C2 through GPIOx_AFR registers. Note: Only applicable to STM32C071xx. Reserved on the other products.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C2_FMP {
    ///0: Disable disabled if not enabled through I2C_y_FMP
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C2_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C2_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C2_FMP` reader - Fast Mode Plus (FM+) enable for I2C2 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C2 through GPIOx_AFR registers. Note: Only applicable to STM32C071xx. Reserved on the other products.
pub type I2C2_FMP_R = crate::BitReader<I2C2_FMP>;
impl I2C2_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C2_FMP {
        match self.bits {
            false => I2C2_FMP::B0x0,
            true => I2C2_FMP::B0x1,
        }
    }
    ///Disable disabled if not enabled through I2C_y_FMP
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C2_FMP::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C2_FMP::B0x1
    }
}
///Field `I2C2_FMP` writer - Fast Mode Plus (FM+) enable for I2C2 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C2 through GPIOx_AFR registers. Note: Only applicable to STM32C071xx. Reserved on the other products.
pub type I2C2_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C2_FMP>;
impl<'a, REG> I2C2_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable disabled if not enabled through I2C_y_FMP
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2_FMP::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C2_FMP::B0x1)
    }
}
/**Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PA9_FMP {
    ///0: Disable disabled if not enabled through I2Cx_FMP
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C_PA9_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PA9_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PA9_FMP` reader - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port.
pub type I2C_PA9_FMP_R = crate::BitReader<I2C_PA9_FMP>;
impl I2C_PA9_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PA9_FMP {
        match self.bits {
            false => I2C_PA9_FMP::B0x0,
            true => I2C_PA9_FMP::B0x1,
        }
    }
    ///Disable disabled if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C_PA9_FMP::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C_PA9_FMP::B0x1
    }
}
///Field `I2C_PA9_FMP` writer - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port.
pub type I2C_PA9_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PA9_FMP>;
impl<'a, REG> I2C_PA9_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable disabled if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PA9_FMP::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PA9_FMP::B0x1)
    }
}
/**Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PA10_FMP {
    ///0: Disable disabled if not enabled through I2Cx_FMP
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C_PA10_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PA10_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PA10_FMP` reader - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port.
pub type I2C_PA10_FMP_R = crate::BitReader<I2C_PA10_FMP>;
impl I2C_PA10_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PA10_FMP {
        match self.bits {
            false => I2C_PA10_FMP::B0x0,
            true => I2C_PA10_FMP::B0x1,
        }
    }
    ///Disable disabled if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C_PA10_FMP::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C_PA10_FMP::B0x1
    }
}
///Field `I2C_PA10_FMP` writer - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port.
pub type I2C_PA10_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PA10_FMP>;
impl<'a, REG> I2C_PA10_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable disabled if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PA10_FMP::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PA10_FMP::B0x1)
    }
}
/**Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PC14 I/O port. Note: Not available on STM32C011xx.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C_PC14_FMP {
    ///0: Disable if not enabled through I2Cx_FMP
    B0x0 = 0,
    ///1: Enable
    B0x1 = 1,
}
impl From<I2C_PC14_FMP> for bool {
    #[inline(always)]
    fn from(variant: I2C_PC14_FMP) -> Self {
        variant as u8 != 0
    }
}
///Field `I2C_PC14_FMP` reader - Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PC14 I/O port. Note: Not available on STM32C011xx.
pub type I2C_PC14_FMP_R = crate::BitReader<I2C_PC14_FMP>;
impl I2C_PC14_FMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> I2C_PC14_FMP {
        match self.bits {
            false => I2C_PC14_FMP::B0x0,
            true => I2C_PC14_FMP::B0x1,
        }
    }
    ///Disable if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C_PC14_FMP::B0x0
    }
    ///Enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C_PC14_FMP::B0x1
    }
}
///Field `I2C_PC14_FMP` writer - Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PC14 I/O port. Note: Not available on STM32C011xx.
pub type I2C_PC14_FMP_W<'a, REG> = crate::BitWriter<'a, REG, I2C_PC14_FMP>;
impl<'a, REG> I2C_PC14_FMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disable if not enabled through I2Cx_FMP
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PC14_FMP::B0x0)
    }
    ///Enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_PC14_FMP::B0x1)
    }
}
impl R {
    ///Bits 0:1 - Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to Section 3: Boot configuration for more details. x0: Main Flash memory
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port. Note: If the PINMUX2\[1:0\] bitfield of the SYSCFG_CFGR3 register is at 00, PA11_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.
    #[inline(always)]
    pub fn pa11_rmp(&self) -> PA11_RMP_R {
        PA11_RMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port. Note: If the PINMUX4\[1:0\] bitfield of the SYSCFG_CFGR3 register is at 00, PA12_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.
    #[inline(always)]
    pub fn pa12_rmp(&self) -> PA12_RMP_R {
        PA12_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IR output polarity selection
    #[inline(always)]
    pub fn ir_pol(&self) -> IR_POL_R {
        IR_POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:
    #[inline(always)]
    pub fn ir_mod(&self) -> IR_MOD_R {
        IR_MOD_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 16 - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port.
    #[inline(always)]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMP_R {
        I2C_PB6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port.
    #[inline(always)]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMP_R {
        I2C_PB7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. Note: Not available on STM32C011xx.
    #[inline(always)]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMP_R {
        I2C_PB8_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. Note: Not available on STM32C011xx.
    #[inline(always)]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMP_R {
        I2C_PB9_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers.
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Fast Mode Plus (FM+) enable for I2C2 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C2 through GPIOx_AFR registers. Note: Only applicable to STM32C071xx. Reserved on the other products.
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port.
    #[inline(always)]
    pub fn i2c_pa9_fmp(&self) -> I2C_PA9_FMP_R {
        I2C_PA9_FMP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port.
    #[inline(always)]
    pub fn i2c_pa10_fmp(&self) -> I2C_PA10_FMP_R {
        I2C_PA10_FMP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PC14 I/O port. Note: Not available on STM32C011xx.
    #[inline(always)]
    pub fn i2c_pc14_fmp(&self) -> I2C_PC14_FMP_R {
        I2C_PC14_FMP_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG_CFGR1")
            .field("mem_mode", &self.mem_mode())
            .field("pa11_rmp", &self.pa11_rmp())
            .field("pa12_rmp", &self.pa12_rmp())
            .field("ir_pol", &self.ir_pol())
            .field("ir_mod", &self.ir_mod())
            .field("i2c_pb6_fmp", &self.i2c_pb6_fmp())
            .field("i2c_pb7_fmp", &self.i2c_pb7_fmp())
            .field("i2c_pb8_fmp", &self.i2c_pb8_fmp())
            .field("i2c_pb9_fmp", &self.i2c_pb9_fmp())
            .field("i2c1_fmp", &self.i2c1_fmp())
            .field("i2c2_fmp", &self.i2c2_fmp())
            .field("i2c_pa9_fmp", &self.i2c_pa9_fmp())
            .field("i2c_pa10_fmp", &self.i2c_pa10_fmp())
            .field("i2c_pc14_fmp", &self.i2c_pc14_fmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Memory mapping selection bits This bitfield controlled by software selects the memory internally mapped at the address 0x0000 0000. Its reset value is determined by the boot mode configuration. Refer to Section 3: Boot configuration for more details. x0: Main Flash memory
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<'_, SYSCFG_CFGR1rs> {
        MEM_MODE_W::new(self, 0)
    }
    ///Bit 3 - PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port. Note: If the PINMUX2\[1:0\] bitfield of the SYSCFG_CFGR3 register is at 00, PA11_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.
    #[inline(always)]
    pub fn pa11_rmp(&mut self) -> PA11_RMP_W<'_, SYSCFG_CFGR1rs> {
        PA11_RMP_W::new(self, 3)
    }
    ///Bit 4 - PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port. Note: If the PINMUX4\[1:0\] bitfield of the SYSCFG_CFGR3 register is at 00, PA12_RMP must be kept at 0 to prevent conflict due to two GPIO outputs with different output levels connected to the same pin.
    #[inline(always)]
    pub fn pa12_rmp(&mut self) -> PA12_RMP_W<'_, SYSCFG_CFGR1rs> {
        PA12_RMP_W::new(self, 4)
    }
    ///Bit 5 - IR output polarity selection
    #[inline(always)]
    pub fn ir_pol(&mut self) -> IR_POL_W<'_, SYSCFG_CFGR1rs> {
        IR_POL_W::new(self, 5)
    }
    ///Bits 6:7 - IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:
    #[inline(always)]
    pub fn ir_mod(&mut self) -> IR_MOD_W<'_, SYSCFG_CFGR1rs> {
        IR_MOD_W::new(self, 6)
    }
    ///Bit 16 - Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port.
    #[inline(always)]
    pub fn i2c_pb6_fmp(&mut self) -> I2C_PB6_FMP_W<'_, SYSCFG_CFGR1rs> {
        I2C_PB6_FMP_W::new(self, 16)
    }
    ///Bit 17 - Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port.
    #[inline(always)]
    pub fn i2c_pb7_fmp(&mut self) -> I2C_PB7_FMP_W<'_, SYSCFG_CFGR1rs> {
        I2C_PB7_FMP_W::new(self, 17)
    }
    ///Bit 18 - Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. Note: Not available on STM32C011xx.
    #[inline(always)]
    pub fn i2c_pb8_fmp(&mut self) -> I2C_PB8_FMP_W<'_, SYSCFG_CFGR1rs> {
        I2C_PB8_FMP_W::new(self, 18)
    }
    ///Bit 19 - Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. Note: Not available on STM32C011xx.
    #[inline(always)]
    pub fn i2c_pb9_fmp(&mut self) -> I2C_PB9_FMP_W<'_, SYSCFG_CFGR1rs> {
        I2C_PB9_FMP_W::new(self, 19)
    }
    ///Bit 20 - Fast Mode Plus (FM+) enable for I2C1 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers.
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W<'_, SYSCFG_CFGR1rs> {
        I2C1_FMP_W::new(self, 20)
    }
    ///Bit 21 - Fast Mode Plus (FM+) enable for I2C2 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C2 through GPIOx_AFR registers. Note: Only applicable to STM32C071xx. Reserved on the other products.
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W<'_, SYSCFG_CFGR1rs> {
        I2C2_FMP_W::new(self, 21)
    }
    ///Bit 22 - Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port.
    #[inline(always)]
    pub fn i2c_pa9_fmp(&mut self) -> I2C_PA9_FMP_W<'_, SYSCFG_CFGR1rs> {
        I2C_PA9_FMP_W::new(self, 22)
    }
    ///Bit 23 - Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port.
    #[inline(always)]
    pub fn i2c_pa10_fmp(&mut self) -> I2C_PA10_FMP_W<'_, SYSCFG_CFGR1rs> {
        I2C_PA10_FMP_W::new(self, 23)
    }
    ///Bit 24 - Fast Mode Plus (FM+) enable for PC14 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PC14 I/O port. Note: Not available on STM32C011xx.
    #[inline(always)]
    pub fn i2c_pc14_fmp(&mut self) -> I2C_PC14_FMP_W<'_, SYSCFG_CFGR1rs> {
        I2C_PC14_FMP_W::new(self, 24)
    }
}
/**SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`syscfg_cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SYSCFG:SYSCFG_CFGR1)*/
pub struct SYSCFG_CFGR1rs;
impl crate::RegisterSpec for SYSCFG_CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`syscfg_cfgr1::R`](R) reader structure
impl crate::Readable for SYSCFG_CFGR1rs {}
///`write(|w| ..)` method takes [`syscfg_cfgr1::W`](W) writer structure
impl crate::Writable for SYSCFG_CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYSCFG_CFGR1 to value 0
impl crate::Resettable for SYSCFG_CFGR1rs {}
