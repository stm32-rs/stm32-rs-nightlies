///Register `GPIOD_AFRL` reader
pub type R = crate::R<GPIOD_AFRLrs>;
///Register `GPIOD_AFRL` writer
pub type W = crate::W<GPIOD_AFRLrs>;
/**Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL0 {
    ///0: AF0
    B0x0 = 0,
    ///1: AF1
    B0x1 = 1,
    ///2: AF2
    B0x2 = 2,
    ///3: AF3
    B0x3 = 3,
    ///4: AF4
    B0x4 = 4,
    ///5: AF5
    B0x5 = 5,
    ///6: AF6
    B0x6 = 6,
    ///7: AF7
    B0x7 = 7,
    ///8: AF8
    B0x8 = 8,
    ///9: AF9
    B0x9 = 9,
    ///10: AF10
    B0xA = 10,
    ///11: AF11
    B0xB = 11,
    ///12: AF12
    B0xC = 12,
    ///13: AF13
    B0xD = 13,
    ///14: AF14
    B0xE = 14,
    ///15: AF15
    B0xF = 15,
}
impl From<AFSEL0> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL0 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL0 {}
///Field `AFSEL0` reader - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL0_R = crate::FieldReader<AFSEL0>;
impl AFSEL0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL0 {
        match self.bits {
            0 => AFSEL0::B0x0,
            1 => AFSEL0::B0x1,
            2 => AFSEL0::B0x2,
            3 => AFSEL0::B0x3,
            4 => AFSEL0::B0x4,
            5 => AFSEL0::B0x5,
            6 => AFSEL0::B0x6,
            7 => AFSEL0::B0x7,
            8 => AFSEL0::B0x8,
            9 => AFSEL0::B0x9,
            10 => AFSEL0::B0xA,
            11 => AFSEL0::B0xB,
            12 => AFSEL0::B0xC,
            13 => AFSEL0::B0xD,
            14 => AFSEL0::B0xE,
            15 => AFSEL0::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL0::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL0::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL0::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL0::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL0::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL0::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL0::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL0::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL0::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL0::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL0::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL0::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL0::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL0::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL0::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL0::B0xF
    }
}
///Field `AFSEL0` writer - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL0, crate::Safe>;
impl<'a, REG> AFSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL0::B0xF)
    }
}
/**Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL1 {
    ///0: AF0
    B0x0 = 0,
    ///1: AF1
    B0x1 = 1,
    ///2: AF2
    B0x2 = 2,
    ///3: AF3
    B0x3 = 3,
    ///4: AF4
    B0x4 = 4,
    ///5: AF5
    B0x5 = 5,
    ///6: AF6
    B0x6 = 6,
    ///7: AF7
    B0x7 = 7,
    ///8: AF8
    B0x8 = 8,
    ///9: AF9
    B0x9 = 9,
    ///10: AF10
    B0xA = 10,
    ///11: AF11
    B0xB = 11,
    ///12: AF12
    B0xC = 12,
    ///13: AF13
    B0xD = 13,
    ///14: AF14
    B0xE = 14,
    ///15: AF15
    B0xF = 15,
}
impl From<AFSEL1> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL1 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL1 {}
///Field `AFSEL1` reader - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL1_R = crate::FieldReader<AFSEL1>;
impl AFSEL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL1 {
        match self.bits {
            0 => AFSEL1::B0x0,
            1 => AFSEL1::B0x1,
            2 => AFSEL1::B0x2,
            3 => AFSEL1::B0x3,
            4 => AFSEL1::B0x4,
            5 => AFSEL1::B0x5,
            6 => AFSEL1::B0x6,
            7 => AFSEL1::B0x7,
            8 => AFSEL1::B0x8,
            9 => AFSEL1::B0x9,
            10 => AFSEL1::B0xA,
            11 => AFSEL1::B0xB,
            12 => AFSEL1::B0xC,
            13 => AFSEL1::B0xD,
            14 => AFSEL1::B0xE,
            15 => AFSEL1::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL1::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL1::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL1::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL1::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL1::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL1::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL1::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL1::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL1::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL1::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL1::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL1::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL1::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL1::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL1::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL1::B0xF
    }
}
///Field `AFSEL1` writer - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL1, crate::Safe>;
impl<'a, REG> AFSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL1::B0xF)
    }
}
/**Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL2 {
    ///0: AF0
    B0x0 = 0,
    ///1: AF1
    B0x1 = 1,
    ///2: AF2
    B0x2 = 2,
    ///3: AF3
    B0x3 = 3,
    ///4: AF4
    B0x4 = 4,
    ///5: AF5
    B0x5 = 5,
    ///6: AF6
    B0x6 = 6,
    ///7: AF7
    B0x7 = 7,
    ///8: AF8
    B0x8 = 8,
    ///9: AF9
    B0x9 = 9,
    ///10: AF10
    B0xA = 10,
    ///11: AF11
    B0xB = 11,
    ///12: AF12
    B0xC = 12,
    ///13: AF13
    B0xD = 13,
    ///14: AF14
    B0xE = 14,
    ///15: AF15
    B0xF = 15,
}
impl From<AFSEL2> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL2 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL2 {}
///Field `AFSEL2` reader - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL2_R = crate::FieldReader<AFSEL2>;
impl AFSEL2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL2 {
        match self.bits {
            0 => AFSEL2::B0x0,
            1 => AFSEL2::B0x1,
            2 => AFSEL2::B0x2,
            3 => AFSEL2::B0x3,
            4 => AFSEL2::B0x4,
            5 => AFSEL2::B0x5,
            6 => AFSEL2::B0x6,
            7 => AFSEL2::B0x7,
            8 => AFSEL2::B0x8,
            9 => AFSEL2::B0x9,
            10 => AFSEL2::B0xA,
            11 => AFSEL2::B0xB,
            12 => AFSEL2::B0xC,
            13 => AFSEL2::B0xD,
            14 => AFSEL2::B0xE,
            15 => AFSEL2::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL2::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL2::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL2::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL2::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL2::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL2::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL2::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL2::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL2::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL2::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL2::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL2::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL2::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL2::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL2::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL2::B0xF
    }
}
///Field `AFSEL2` writer - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL2, crate::Safe>;
impl<'a, REG> AFSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL2::B0xF)
    }
}
/**Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL3 {
    ///0: AF0
    B0x0 = 0,
    ///1: AF1
    B0x1 = 1,
    ///2: AF2
    B0x2 = 2,
    ///3: AF3
    B0x3 = 3,
    ///4: AF4
    B0x4 = 4,
    ///5: AF5
    B0x5 = 5,
    ///6: AF6
    B0x6 = 6,
    ///7: AF7
    B0x7 = 7,
    ///8: AF8
    B0x8 = 8,
    ///9: AF9
    B0x9 = 9,
    ///10: AF10
    B0xA = 10,
    ///11: AF11
    B0xB = 11,
    ///12: AF12
    B0xC = 12,
    ///13: AF13
    B0xD = 13,
    ///14: AF14
    B0xE = 14,
    ///15: AF15
    B0xF = 15,
}
impl From<AFSEL3> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL3 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL3 {}
///Field `AFSEL3` reader - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL3_R = crate::FieldReader<AFSEL3>;
impl AFSEL3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL3 {
        match self.bits {
            0 => AFSEL3::B0x0,
            1 => AFSEL3::B0x1,
            2 => AFSEL3::B0x2,
            3 => AFSEL3::B0x3,
            4 => AFSEL3::B0x4,
            5 => AFSEL3::B0x5,
            6 => AFSEL3::B0x6,
            7 => AFSEL3::B0x7,
            8 => AFSEL3::B0x8,
            9 => AFSEL3::B0x9,
            10 => AFSEL3::B0xA,
            11 => AFSEL3::B0xB,
            12 => AFSEL3::B0xC,
            13 => AFSEL3::B0xD,
            14 => AFSEL3::B0xE,
            15 => AFSEL3::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL3::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL3::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL3::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL3::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL3::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL3::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL3::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL3::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL3::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL3::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL3::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL3::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL3::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL3::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL3::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL3::B0xF
    }
}
///Field `AFSEL3` writer - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL3, crate::Safe>;
impl<'a, REG> AFSEL3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL3::B0xF)
    }
}
/**Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL4 {
    ///0: AF0
    B0x0 = 0,
    ///1: AF1
    B0x1 = 1,
    ///2: AF2
    B0x2 = 2,
    ///3: AF3
    B0x3 = 3,
    ///4: AF4
    B0x4 = 4,
    ///5: AF5
    B0x5 = 5,
    ///6: AF6
    B0x6 = 6,
    ///7: AF7
    B0x7 = 7,
    ///8: AF8
    B0x8 = 8,
    ///9: AF9
    B0x9 = 9,
    ///10: AF10
    B0xA = 10,
    ///11: AF11
    B0xB = 11,
    ///12: AF12
    B0xC = 12,
    ///13: AF13
    B0xD = 13,
    ///14: AF14
    B0xE = 14,
    ///15: AF15
    B0xF = 15,
}
impl From<AFSEL4> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL4 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL4 {}
///Field `AFSEL4` reader - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL4_R = crate::FieldReader<AFSEL4>;
impl AFSEL4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL4 {
        match self.bits {
            0 => AFSEL4::B0x0,
            1 => AFSEL4::B0x1,
            2 => AFSEL4::B0x2,
            3 => AFSEL4::B0x3,
            4 => AFSEL4::B0x4,
            5 => AFSEL4::B0x5,
            6 => AFSEL4::B0x6,
            7 => AFSEL4::B0x7,
            8 => AFSEL4::B0x8,
            9 => AFSEL4::B0x9,
            10 => AFSEL4::B0xA,
            11 => AFSEL4::B0xB,
            12 => AFSEL4::B0xC,
            13 => AFSEL4::B0xD,
            14 => AFSEL4::B0xE,
            15 => AFSEL4::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL4::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL4::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL4::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL4::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL4::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL4::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL4::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL4::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL4::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL4::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL4::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL4::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL4::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL4::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL4::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL4::B0xF
    }
}
///Field `AFSEL4` writer - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL4, crate::Safe>;
impl<'a, REG> AFSEL4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL4::B0xF)
    }
}
/**Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL5 {
    ///0: AF0
    B0x0 = 0,
    ///1: AF1
    B0x1 = 1,
    ///2: AF2
    B0x2 = 2,
    ///3: AF3
    B0x3 = 3,
    ///4: AF4
    B0x4 = 4,
    ///5: AF5
    B0x5 = 5,
    ///6: AF6
    B0x6 = 6,
    ///7: AF7
    B0x7 = 7,
    ///8: AF8
    B0x8 = 8,
    ///9: AF9
    B0x9 = 9,
    ///10: AF10
    B0xA = 10,
    ///11: AF11
    B0xB = 11,
    ///12: AF12
    B0xC = 12,
    ///13: AF13
    B0xD = 13,
    ///14: AF14
    B0xE = 14,
    ///15: AF15
    B0xF = 15,
}
impl From<AFSEL5> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL5 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL5 {}
///Field `AFSEL5` reader - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL5_R = crate::FieldReader<AFSEL5>;
impl AFSEL5_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL5 {
        match self.bits {
            0 => AFSEL5::B0x0,
            1 => AFSEL5::B0x1,
            2 => AFSEL5::B0x2,
            3 => AFSEL5::B0x3,
            4 => AFSEL5::B0x4,
            5 => AFSEL5::B0x5,
            6 => AFSEL5::B0x6,
            7 => AFSEL5::B0x7,
            8 => AFSEL5::B0x8,
            9 => AFSEL5::B0x9,
            10 => AFSEL5::B0xA,
            11 => AFSEL5::B0xB,
            12 => AFSEL5::B0xC,
            13 => AFSEL5::B0xD,
            14 => AFSEL5::B0xE,
            15 => AFSEL5::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL5::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL5::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL5::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL5::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL5::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL5::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL5::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL5::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL5::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL5::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL5::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL5::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL5::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL5::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL5::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL5::B0xF
    }
}
///Field `AFSEL5` writer - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL5, crate::Safe>;
impl<'a, REG> AFSEL5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL5::B0xF)
    }
}
/**Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL6 {
    ///0: AF0
    B0x0 = 0,
    ///1: AF1
    B0x1 = 1,
    ///2: AF2
    B0x2 = 2,
    ///3: AF3
    B0x3 = 3,
    ///4: AF4
    B0x4 = 4,
    ///5: AF5
    B0x5 = 5,
    ///6: AF6
    B0x6 = 6,
    ///7: AF7
    B0x7 = 7,
    ///8: AF8
    B0x8 = 8,
    ///9: AF9
    B0x9 = 9,
    ///10: AF10
    B0xA = 10,
    ///11: AF11
    B0xB = 11,
    ///12: AF12
    B0xC = 12,
    ///13: AF13
    B0xD = 13,
    ///14: AF14
    B0xE = 14,
    ///15: AF15
    B0xF = 15,
}
impl From<AFSEL6> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL6 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL6 {}
///Field `AFSEL6` reader - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL6_R = crate::FieldReader<AFSEL6>;
impl AFSEL6_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL6 {
        match self.bits {
            0 => AFSEL6::B0x0,
            1 => AFSEL6::B0x1,
            2 => AFSEL6::B0x2,
            3 => AFSEL6::B0x3,
            4 => AFSEL6::B0x4,
            5 => AFSEL6::B0x5,
            6 => AFSEL6::B0x6,
            7 => AFSEL6::B0x7,
            8 => AFSEL6::B0x8,
            9 => AFSEL6::B0x9,
            10 => AFSEL6::B0xA,
            11 => AFSEL6::B0xB,
            12 => AFSEL6::B0xC,
            13 => AFSEL6::B0xD,
            14 => AFSEL6::B0xE,
            15 => AFSEL6::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL6::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL6::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL6::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL6::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL6::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL6::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL6::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL6::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL6::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL6::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL6::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL6::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL6::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL6::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL6::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL6::B0xF
    }
}
///Field `AFSEL6` writer - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL6, crate::Safe>;
impl<'a, REG> AFSEL6_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL6::B0xF)
    }
}
/**Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL7 {
    ///0: AF0
    B0x0 = 0,
    ///1: AF1
    B0x1 = 1,
    ///2: AF2
    B0x2 = 2,
    ///3: AF3
    B0x3 = 3,
    ///4: AF4
    B0x4 = 4,
    ///5: AF5
    B0x5 = 5,
    ///6: AF6
    B0x6 = 6,
    ///7: AF7
    B0x7 = 7,
    ///8: AF8
    B0x8 = 8,
    ///9: AF9
    B0x9 = 9,
    ///10: AF10
    B0xA = 10,
    ///11: AF11
    B0xB = 11,
    ///12: AF12
    B0xC = 12,
    ///13: AF13
    B0xD = 13,
    ///14: AF14
    B0xE = 14,
    ///15: AF15
    B0xF = 15,
}
impl From<AFSEL7> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL7 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL7 {}
///Field `AFSEL7` reader - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL7_R = crate::FieldReader<AFSEL7>;
impl AFSEL7_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL7 {
        match self.bits {
            0 => AFSEL7::B0x0,
            1 => AFSEL7::B0x1,
            2 => AFSEL7::B0x2,
            3 => AFSEL7::B0x3,
            4 => AFSEL7::B0x4,
            5 => AFSEL7::B0x5,
            6 => AFSEL7::B0x6,
            7 => AFSEL7::B0x7,
            8 => AFSEL7::B0x8,
            9 => AFSEL7::B0x9,
            10 => AFSEL7::B0xA,
            11 => AFSEL7::B0xB,
            12 => AFSEL7::B0xC,
            13 => AFSEL7::B0xD,
            14 => AFSEL7::B0xE,
            15 => AFSEL7::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL7::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL7::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL7::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL7::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL7::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL7::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL7::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL7::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL7::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL7::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL7::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL7::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL7::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL7::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL7::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL7::B0xF
    }
}
///Field `AFSEL7` writer - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
pub type AFSEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL7, crate::Safe>;
impl<'a, REG> AFSEL7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL7::B0xF)
    }
}
impl R {
    ///Bits 0:3 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel0(&self) -> AFSEL0_R {
        AFSEL0_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel1(&self) -> AFSEL1_R {
        AFSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel2(&self) -> AFSEL2_R {
        AFSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel4(&self) -> AFSEL4_R {
        AFSEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel5(&self) -> AFSEL5_R {
        AFSEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel6(&self) -> AFSEL6_R {
        AFSEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel7(&self) -> AFSEL7_R {
        AFSEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOD_AFRL")
            .field("afsel0", &self.afsel0())
            .field("afsel1", &self.afsel1())
            .field("afsel2", &self.afsel2())
            .field("afsel3", &self.afsel3())
            .field("afsel4", &self.afsel4())
            .field("afsel5", &self.afsel5())
            .field("afsel6", &self.afsel6())
            .field("afsel7", &self.afsel7())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel0(&mut self) -> AFSEL0_W<'_, GPIOD_AFRLrs> {
        AFSEL0_W::new(self, 0)
    }
    ///Bits 4:7 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel1(&mut self) -> AFSEL1_W<'_, GPIOD_AFRLrs> {
        AFSEL1_W::new(self, 4)
    }
    ///Bits 8:11 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel2(&mut self) -> AFSEL2_W<'_, GPIOD_AFRLrs> {
        AFSEL2_W::new(self, 8)
    }
    ///Bits 12:15 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel3(&mut self) -> AFSEL3_W<'_, GPIOD_AFRLrs> {
        AFSEL3_W::new(self, 12)
    }
    ///Bits 16:19 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel4(&mut self) -> AFSEL4_W<'_, GPIOD_AFRLrs> {
        AFSEL4_W::new(self, 16)
    }
    ///Bits 20:23 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel5(&mut self) -> AFSEL5_W<'_, GPIOD_AFRLrs> {
        AFSEL5_W::new(self, 20)
    }
    ///Bits 24:27 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel6(&mut self) -> AFSEL6_W<'_, GPIOD_AFRLrs> {
        AFSEL6_W::new(self, 24)
    }
    ///Bits 28:31 - Alternate function selection for port x pin y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel7(&mut self) -> AFSEL7_W<'_, GPIOD_AFRLrs> {
        AFSEL7_W::new(self, 28)
    }
}
/**GPIO alternate function low register

You can [`read`](crate::Reg::read) this register and get [`gpiod_afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#GPIOD:GPIOD_AFRL)*/
pub struct GPIOD_AFRLrs;
impl crate::RegisterSpec for GPIOD_AFRLrs {
    type Ux = u32;
}
///`read()` method returns [`gpiod_afrl::R`](R) reader structure
impl crate::Readable for GPIOD_AFRLrs {}
///`write(|w| ..)` method takes [`gpiod_afrl::W`](W) writer structure
impl crate::Writable for GPIOD_AFRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOD_AFRL to value 0
impl crate::Resettable for GPIOD_AFRLrs {}
