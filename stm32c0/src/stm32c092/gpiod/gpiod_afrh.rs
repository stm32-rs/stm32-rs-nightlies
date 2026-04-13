///Register `GPIOD_AFRH` reader
pub type R = crate::R<GPIOD_AFRHrs>;
///Register `GPIOD_AFRH` writer
pub type W = crate::W<GPIOD_AFRHrs>;
/**Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL8 {
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
impl From<AFSEL8> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL8 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL8 {}
///Field `AFSEL8` reader - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL8_R = crate::FieldReader<AFSEL8>;
impl AFSEL8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL8 {
        match self.bits {
            0 => AFSEL8::B0x0,
            1 => AFSEL8::B0x1,
            2 => AFSEL8::B0x2,
            3 => AFSEL8::B0x3,
            4 => AFSEL8::B0x4,
            5 => AFSEL8::B0x5,
            6 => AFSEL8::B0x6,
            7 => AFSEL8::B0x7,
            8 => AFSEL8::B0x8,
            9 => AFSEL8::B0x9,
            10 => AFSEL8::B0xA,
            11 => AFSEL8::B0xB,
            12 => AFSEL8::B0xC,
            13 => AFSEL8::B0xD,
            14 => AFSEL8::B0xE,
            15 => AFSEL8::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL8::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL8::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL8::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL8::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL8::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL8::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL8::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL8::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL8::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL8::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL8::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL8::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL8::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL8::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL8::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL8::B0xF
    }
}
///Field `AFSEL8` writer - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL8_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL8, crate::Safe>;
impl<'a, REG> AFSEL8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL8::B0xF)
    }
}
/**Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL9 {
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
impl From<AFSEL9> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL9 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL9 {}
///Field `AFSEL9` reader - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL9_R = crate::FieldReader<AFSEL9>;
impl AFSEL9_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL9 {
        match self.bits {
            0 => AFSEL9::B0x0,
            1 => AFSEL9::B0x1,
            2 => AFSEL9::B0x2,
            3 => AFSEL9::B0x3,
            4 => AFSEL9::B0x4,
            5 => AFSEL9::B0x5,
            6 => AFSEL9::B0x6,
            7 => AFSEL9::B0x7,
            8 => AFSEL9::B0x8,
            9 => AFSEL9::B0x9,
            10 => AFSEL9::B0xA,
            11 => AFSEL9::B0xB,
            12 => AFSEL9::B0xC,
            13 => AFSEL9::B0xD,
            14 => AFSEL9::B0xE,
            15 => AFSEL9::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL9::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL9::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL9::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL9::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL9::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL9::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL9::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL9::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL9::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL9::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL9::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL9::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL9::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL9::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL9::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL9::B0xF
    }
}
///Field `AFSEL9` writer - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL9_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL9, crate::Safe>;
impl<'a, REG> AFSEL9_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL9::B0xF)
    }
}
/**Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL10 {
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
impl From<AFSEL10> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL10 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL10 {}
///Field `AFSEL10` reader - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL10_R = crate::FieldReader<AFSEL10>;
impl AFSEL10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL10 {
        match self.bits {
            0 => AFSEL10::B0x0,
            1 => AFSEL10::B0x1,
            2 => AFSEL10::B0x2,
            3 => AFSEL10::B0x3,
            4 => AFSEL10::B0x4,
            5 => AFSEL10::B0x5,
            6 => AFSEL10::B0x6,
            7 => AFSEL10::B0x7,
            8 => AFSEL10::B0x8,
            9 => AFSEL10::B0x9,
            10 => AFSEL10::B0xA,
            11 => AFSEL10::B0xB,
            12 => AFSEL10::B0xC,
            13 => AFSEL10::B0xD,
            14 => AFSEL10::B0xE,
            15 => AFSEL10::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL10::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL10::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL10::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL10::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL10::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL10::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL10::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL10::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL10::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL10::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL10::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL10::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL10::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL10::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL10::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL10::B0xF
    }
}
///Field `AFSEL10` writer - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL10_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL10, crate::Safe>;
impl<'a, REG> AFSEL10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL10::B0xF)
    }
}
/**Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL11 {
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
impl From<AFSEL11> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL11 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL11 {}
///Field `AFSEL11` reader - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL11_R = crate::FieldReader<AFSEL11>;
impl AFSEL11_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL11 {
        match self.bits {
            0 => AFSEL11::B0x0,
            1 => AFSEL11::B0x1,
            2 => AFSEL11::B0x2,
            3 => AFSEL11::B0x3,
            4 => AFSEL11::B0x4,
            5 => AFSEL11::B0x5,
            6 => AFSEL11::B0x6,
            7 => AFSEL11::B0x7,
            8 => AFSEL11::B0x8,
            9 => AFSEL11::B0x9,
            10 => AFSEL11::B0xA,
            11 => AFSEL11::B0xB,
            12 => AFSEL11::B0xC,
            13 => AFSEL11::B0xD,
            14 => AFSEL11::B0xE,
            15 => AFSEL11::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL11::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL11::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL11::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL11::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL11::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL11::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL11::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL11::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL11::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL11::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL11::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL11::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL11::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL11::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL11::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL11::B0xF
    }
}
///Field `AFSEL11` writer - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL11_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL11, crate::Safe>;
impl<'a, REG> AFSEL11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL11::B0xF)
    }
}
/**Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL12 {
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
impl From<AFSEL12> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL12 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL12 {}
///Field `AFSEL12` reader - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL12_R = crate::FieldReader<AFSEL12>;
impl AFSEL12_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL12 {
        match self.bits {
            0 => AFSEL12::B0x0,
            1 => AFSEL12::B0x1,
            2 => AFSEL12::B0x2,
            3 => AFSEL12::B0x3,
            4 => AFSEL12::B0x4,
            5 => AFSEL12::B0x5,
            6 => AFSEL12::B0x6,
            7 => AFSEL12::B0x7,
            8 => AFSEL12::B0x8,
            9 => AFSEL12::B0x9,
            10 => AFSEL12::B0xA,
            11 => AFSEL12::B0xB,
            12 => AFSEL12::B0xC,
            13 => AFSEL12::B0xD,
            14 => AFSEL12::B0xE,
            15 => AFSEL12::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL12::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL12::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL12::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL12::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL12::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL12::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL12::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL12::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL12::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL12::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL12::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL12::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL12::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL12::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL12::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL12::B0xF
    }
}
///Field `AFSEL12` writer - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL12_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL12, crate::Safe>;
impl<'a, REG> AFSEL12_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL12::B0xF)
    }
}
/**Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL13 {
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
impl From<AFSEL13> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL13 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL13 {}
///Field `AFSEL13` reader - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL13_R = crate::FieldReader<AFSEL13>;
impl AFSEL13_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL13 {
        match self.bits {
            0 => AFSEL13::B0x0,
            1 => AFSEL13::B0x1,
            2 => AFSEL13::B0x2,
            3 => AFSEL13::B0x3,
            4 => AFSEL13::B0x4,
            5 => AFSEL13::B0x5,
            6 => AFSEL13::B0x6,
            7 => AFSEL13::B0x7,
            8 => AFSEL13::B0x8,
            9 => AFSEL13::B0x9,
            10 => AFSEL13::B0xA,
            11 => AFSEL13::B0xB,
            12 => AFSEL13::B0xC,
            13 => AFSEL13::B0xD,
            14 => AFSEL13::B0xE,
            15 => AFSEL13::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL13::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL13::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL13::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL13::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL13::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL13::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL13::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL13::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL13::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL13::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL13::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL13::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL13::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL13::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL13::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL13::B0xF
    }
}
///Field `AFSEL13` writer - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL13_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL13, crate::Safe>;
impl<'a, REG> AFSEL13_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL13::B0xF)
    }
}
/**Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL14 {
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
impl From<AFSEL14> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL14 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL14 {}
///Field `AFSEL14` reader - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL14_R = crate::FieldReader<AFSEL14>;
impl AFSEL14_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL14 {
        match self.bits {
            0 => AFSEL14::B0x0,
            1 => AFSEL14::B0x1,
            2 => AFSEL14::B0x2,
            3 => AFSEL14::B0x3,
            4 => AFSEL14::B0x4,
            5 => AFSEL14::B0x5,
            6 => AFSEL14::B0x6,
            7 => AFSEL14::B0x7,
            8 => AFSEL14::B0x8,
            9 => AFSEL14::B0x9,
            10 => AFSEL14::B0xA,
            11 => AFSEL14::B0xB,
            12 => AFSEL14::B0xC,
            13 => AFSEL14::B0xD,
            14 => AFSEL14::B0xE,
            15 => AFSEL14::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL14::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL14::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL14::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL14::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL14::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL14::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL14::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL14::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL14::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL14::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL14::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL14::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL14::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL14::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL14::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL14::B0xF
    }
}
///Field `AFSEL14` writer - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL14_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL14, crate::Safe>;
impl<'a, REG> AFSEL14_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL14::B0xF)
    }
}
/**Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AFSEL15 {
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
impl From<AFSEL15> for u8 {
    #[inline(always)]
    fn from(variant: AFSEL15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AFSEL15 {
    type Ux = u8;
}
impl crate::IsEnum for AFSEL15 {}
///Field `AFSEL15` reader - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL15_R = crate::FieldReader<AFSEL15>;
impl AFSEL15_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AFSEL15 {
        match self.bits {
            0 => AFSEL15::B0x0,
            1 => AFSEL15::B0x1,
            2 => AFSEL15::B0x2,
            3 => AFSEL15::B0x3,
            4 => AFSEL15::B0x4,
            5 => AFSEL15::B0x5,
            6 => AFSEL15::B0x6,
            7 => AFSEL15::B0x7,
            8 => AFSEL15::B0x8,
            9 => AFSEL15::B0x9,
            10 => AFSEL15::B0xA,
            11 => AFSEL15::B0xB,
            12 => AFSEL15::B0xC,
            13 => AFSEL15::B0xD,
            14 => AFSEL15::B0xE,
            15 => AFSEL15::B0xF,
            _ => unreachable!(),
        }
    }
    ///AF0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AFSEL15::B0x0
    }
    ///AF1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AFSEL15::B0x1
    }
    ///AF2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AFSEL15::B0x2
    }
    ///AF3
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AFSEL15::B0x3
    }
    ///AF4
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == AFSEL15::B0x4
    }
    ///AF5
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == AFSEL15::B0x5
    }
    ///AF6
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == AFSEL15::B0x6
    }
    ///AF7
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == AFSEL15::B0x7
    }
    ///AF8
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == AFSEL15::B0x8
    }
    ///AF9
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == AFSEL15::B0x9
    }
    ///AF10
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == AFSEL15::B0xA
    }
    ///AF11
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == AFSEL15::B0xB
    }
    ///AF12
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == AFSEL15::B0xC
    }
    ///AF13
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == AFSEL15::B0xD
    }
    ///AF14
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == AFSEL15::B0xE
    }
    ///AF15
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == AFSEL15::B0xF
    }
}
///Field `AFSEL15` writer - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
pub type AFSEL15_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AFSEL15, crate::Safe>;
impl<'a, REG> AFSEL15_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///AF0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0x0)
    }
    ///AF1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0x1)
    }
    ///AF2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0x2)
    }
    ///AF3
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0x3)
    }
    ///AF4
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0x4)
    }
    ///AF5
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0x5)
    }
    ///AF6
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0x6)
    }
    ///AF7
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0x7)
    }
    ///AF8
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0x8)
    }
    ///AF9
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0x9)
    }
    ///AF10
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0xA)
    }
    ///AF11
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0xB)
    }
    ///AF12
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0xC)
    }
    ///AF13
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0xD)
    }
    ///AF14
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0xE)
    }
    ///AF15
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(AFSEL15::B0xF)
    }
}
impl R {
    ///Bits 0:3 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel8(&self) -> AFSEL8_R {
        AFSEL8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel9(&self) -> AFSEL9_R {
        AFSEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel10(&self) -> AFSEL10_R {
        AFSEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel11(&self) -> AFSEL11_R {
        AFSEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel12(&self) -> AFSEL12_R {
        AFSEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel13(&self) -> AFSEL13_R {
        AFSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel14(&self) -> AFSEL14_R {
        AFSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel15(&self) -> AFSEL15_R {
        AFSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIOD_AFRH")
            .field("afsel8", &self.afsel8())
            .field("afsel9", &self.afsel9())
            .field("afsel10", &self.afsel10())
            .field("afsel11", &self.afsel11())
            .field("afsel12", &self.afsel12())
            .field("afsel13", &self.afsel13())
            .field("afsel14", &self.afsel14())
            .field("afsel15", &self.afsel15())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel8(&mut self) -> AFSEL8_W<'_, GPIOD_AFRHrs> {
        AFSEL8_W::new(self, 0)
    }
    ///Bits 4:7 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel9(&mut self) -> AFSEL9_W<'_, GPIOD_AFRHrs> {
        AFSEL9_W::new(self, 4)
    }
    ///Bits 8:11 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel10(&mut self) -> AFSEL10_W<'_, GPIOD_AFRHrs> {
        AFSEL10_W::new(self, 8)
    }
    ///Bits 12:15 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel11(&mut self) -> AFSEL11_W<'_, GPIOD_AFRHrs> {
        AFSEL11_W::new(self, 12)
    }
    ///Bits 16:19 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel12(&mut self) -> AFSEL12_W<'_, GPIOD_AFRHrs> {
        AFSEL12_W::new(self, 16)
    }
    ///Bits 20:23 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel13(&mut self) -> AFSEL13_W<'_, GPIOD_AFRHrs> {
        AFSEL13_W::new(self, 20)
    }
    ///Bits 24:27 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel14(&mut self) -> AFSEL14_W<'_, GPIOD_AFRHrs> {
        AFSEL14_W::new(self, 24)
    }
    ///Bits 28:31 - Alternate function selection for port x, I/O y These bits are written by software to configure alternate function I/Os
    #[inline(always)]
    pub fn afsel15(&mut self) -> AFSEL15_W<'_, GPIOD_AFRHrs> {
        AFSEL15_W::new(self, 28)
    }
}
/**GPIO alternate function high register

You can [`read`](crate::Reg::read) this register and get [`gpiod_afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiod_afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#GPIOD:GPIOD_AFRH)*/
pub struct GPIOD_AFRHrs;
impl crate::RegisterSpec for GPIOD_AFRHrs {
    type Ux = u32;
}
///`read()` method returns [`gpiod_afrh::R`](R) reader structure
impl crate::Readable for GPIOD_AFRHrs {}
///`write(|w| ..)` method takes [`gpiod_afrh::W`](W) writer structure
impl crate::Writable for GPIOD_AFRHrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GPIOD_AFRH to value 0
impl crate::Resettable for GPIOD_AFRHrs {}
