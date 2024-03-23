#[doc = "Register `COMP2_CSR` reader"]
pub type R = crate::R<COMP2_CSRrs>;
#[doc = "Register `COMP2_CSR` writer"]
pub type W = crate::W<COMP2_CSRrs>;
#[doc = "Comparator 2 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2EN {
    #[doc = "0: Comparator 2 switched OFF"]
    Disabled = 0,
    #[doc = "1: Comparator 2 switched ON"]
    Enabled = 1,
}
impl From<COMP2EN> for bool {
    #[inline(always)]
    fn from(variant: COMP2EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2EN` reader - Comparator 2 enable bit"]
pub type COMP2EN_R = crate::BitReader<COMP2EN>;
impl COMP2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2EN {
        match self.bits {
            false => COMP2EN::Disabled,
            true => COMP2EN::Enabled,
        }
    }
    #[doc = "Comparator 2 switched OFF"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP2EN::Disabled
    }
    #[doc = "Comparator 2 switched ON"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP2EN::Enabled
    }
}
#[doc = "Field `COMP2EN` writer - Comparator 2 enable bit"]
pub type COMP2EN_W<'a, REG> = crate::BitWriter<'a, REG, COMP2EN>;
impl<'a, REG> COMP2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator 2 switched OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2EN::Disabled)
    }
    #[doc = "Comparator 2 switched ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2EN::Enabled)
    }
}
#[doc = "Comparator 2 power mode selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2SPEED {
    #[doc = "0: Slow speed"]
    Slow = 0,
    #[doc = "1: Fast speed"]
    Fast = 1,
}
impl From<COMP2SPEED> for bool {
    #[inline(always)]
    fn from(variant: COMP2SPEED) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2SPEED` reader - Comparator 2 power mode selection bit"]
pub type COMP2SPEED_R = crate::BitReader<COMP2SPEED>;
impl COMP2SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2SPEED {
        match self.bits {
            false => COMP2SPEED::Slow,
            true => COMP2SPEED::Fast,
        }
    }
    #[doc = "Slow speed"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == COMP2SPEED::Slow
    }
    #[doc = "Fast speed"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == COMP2SPEED::Fast
    }
}
#[doc = "Field `COMP2SPEED` writer - Comparator 2 power mode selection bit"]
pub type COMP2SPEED_W<'a, REG> = crate::BitWriter<'a, REG, COMP2SPEED>;
impl<'a, REG> COMP2SPEED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slow speed"]
    #[inline(always)]
    pub fn slow(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2SPEED::Slow)
    }
    #[doc = "Fast speed"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2SPEED::Fast)
    }
}
#[doc = "Comparator 2 Input Minus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2INNSEL {
    #[doc = "0: VREFINT"]
    Vrefint = 0,
    #[doc = "1: PA2"]
    Pa2 = 1,
    #[doc = "2: PA4"]
    Pa4 = 2,
    #[doc = "3: PA5"]
    Pa5 = 3,
    #[doc = "4: 1/4 VREFINT"]
    VrefintDiv4 = 4,
    #[doc = "5: 1/2 VREFINT"]
    VrefintDiv2 = 5,
    #[doc = "6: 3/4 VREFINT"]
    VrefintDiv3_4 = 6,
    #[doc = "7: PB3"]
    Pb3 = 7,
}
impl From<COMP2INNSEL> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INNSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP2INNSEL {
    type Ux = u8;
}
#[doc = "Field `COMP2INNSEL` reader - Comparator 2 Input Minus connection configuration bit"]
pub type COMP2INNSEL_R = crate::FieldReader<COMP2INNSEL>;
impl COMP2INNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2INNSEL {
        match self.bits {
            0 => COMP2INNSEL::Vrefint,
            1 => COMP2INNSEL::Pa2,
            2 => COMP2INNSEL::Pa4,
            3 => COMP2INNSEL::Pa5,
            4 => COMP2INNSEL::VrefintDiv4,
            5 => COMP2INNSEL::VrefintDiv2,
            6 => COMP2INNSEL::VrefintDiv3_4,
            7 => COMP2INNSEL::Pb3,
            _ => unreachable!(),
        }
    }
    #[doc = "VREFINT"]
    #[inline(always)]
    pub fn is_vrefint(&self) -> bool {
        *self == COMP2INNSEL::Vrefint
    }
    #[doc = "PA2"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == COMP2INNSEL::Pa2
    }
    #[doc = "PA4"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == COMP2INNSEL::Pa4
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == COMP2INNSEL::Pa5
    }
    #[doc = "1/4 VREFINT"]
    #[inline(always)]
    pub fn is_vrefint_div4(&self) -> bool {
        *self == COMP2INNSEL::VrefintDiv4
    }
    #[doc = "1/2 VREFINT"]
    #[inline(always)]
    pub fn is_vrefint_div2(&self) -> bool {
        *self == COMP2INNSEL::VrefintDiv2
    }
    #[doc = "3/4 VREFINT"]
    #[inline(always)]
    pub fn is_vrefint_div3_4(&self) -> bool {
        *self == COMP2INNSEL::VrefintDiv3_4
    }
    #[doc = "PB3"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == COMP2INNSEL::Pb3
    }
}
#[doc = "Field `COMP2INNSEL` writer - Comparator 2 Input Minus connection configuration bit"]
pub type COMP2INNSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, COMP2INNSEL>;
impl<'a, REG> COMP2INNSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VREFINT"]
    #[inline(always)]
    pub fn vrefint(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INNSEL::Vrefint)
    }
    #[doc = "PA2"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INNSEL::Pa2)
    }
    #[doc = "PA4"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INNSEL::Pa4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INNSEL::Pa5)
    }
    #[doc = "1/4 VREFINT"]
    #[inline(always)]
    pub fn vrefint_div4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INNSEL::VrefintDiv4)
    }
    #[doc = "1/2 VREFINT"]
    #[inline(always)]
    pub fn vrefint_div2(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INNSEL::VrefintDiv2)
    }
    #[doc = "3/4 VREFINT"]
    #[inline(always)]
    pub fn vrefint_div3_4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INNSEL::VrefintDiv3_4)
    }
    #[doc = "PB3"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INNSEL::Pb3)
    }
}
#[doc = "Comparator 2 Input Plus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2INPSEL {
    #[doc = "0: PA3"]
    Pa3 = 0,
    #[doc = "1: PB4"]
    Pb4 = 1,
    #[doc = "2: PB5"]
    Pb5 = 2,
    #[doc = "3: PB6"]
    Pb6 = 3,
    #[doc = "4: PB7"]
    Pb7 = 4,
    #[doc = "5: PA7"]
    Pa7 = 5,
}
impl From<COMP2INPSEL> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INPSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP2INPSEL {
    type Ux = u8;
}
#[doc = "Field `COMP2INPSEL` reader - Comparator 2 Input Plus connection configuration bit"]
pub type COMP2INPSEL_R = crate::FieldReader<COMP2INPSEL>;
impl COMP2INPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<COMP2INPSEL> {
        match self.bits {
            0 => Some(COMP2INPSEL::Pa3),
            1 => Some(COMP2INPSEL::Pb4),
            2 => Some(COMP2INPSEL::Pb5),
            3 => Some(COMP2INPSEL::Pb6),
            4 => Some(COMP2INPSEL::Pb7),
            5 => Some(COMP2INPSEL::Pa7),
            _ => None,
        }
    }
    #[doc = "PA3"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == COMP2INPSEL::Pa3
    }
    #[doc = "PB4"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == COMP2INPSEL::Pb4
    }
    #[doc = "PB5"]
    #[inline(always)]
    pub fn is_pb5(&self) -> bool {
        *self == COMP2INPSEL::Pb5
    }
    #[doc = "PB6"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == COMP2INPSEL::Pb6
    }
    #[doc = "PB7"]
    #[inline(always)]
    pub fn is_pb7(&self) -> bool {
        *self == COMP2INPSEL::Pb7
    }
    #[doc = "PA7"]
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == COMP2INPSEL::Pa7
    }
}
#[doc = "Field `COMP2INPSEL` writer - Comparator 2 Input Plus connection configuration bit"]
pub type COMP2INPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, COMP2INPSEL>;
impl<'a, REG> COMP2INPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PA3"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INPSEL::Pa3)
    }
    #[doc = "PB4"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INPSEL::Pb4)
    }
    #[doc = "PB5"]
    #[inline(always)]
    pub fn pb5(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INPSEL::Pb5)
    }
    #[doc = "PB6"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INPSEL::Pb6)
    }
    #[doc = "PB7"]
    #[inline(always)]
    pub fn pb7(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INPSEL::Pb7)
    }
    #[doc = "PA7"]
    #[inline(always)]
    pub fn pa7(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2INPSEL::Pa7)
    }
}
#[doc = "Comparator 2 LPTIM input 2 propagation bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2LPTIMIN2 {
    #[doc = "0: Comparator 2 output gated"]
    Gated = 0,
    #[doc = "1: Comparator 2 output sent to LPTIM input 2"]
    NotGated = 1,
}
impl From<COMP2LPTIMIN2> for bool {
    #[inline(always)]
    fn from(variant: COMP2LPTIMIN2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2LPTIMIN2` reader - Comparator 2 LPTIM input 2 propagation bit"]
pub type COMP2LPTIMIN2_R = crate::BitReader<COMP2LPTIMIN2>;
impl COMP2LPTIMIN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2LPTIMIN2 {
        match self.bits {
            false => COMP2LPTIMIN2::Gated,
            true => COMP2LPTIMIN2::NotGated,
        }
    }
    #[doc = "Comparator 2 output gated"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == COMP2LPTIMIN2::Gated
    }
    #[doc = "Comparator 2 output sent to LPTIM input 2"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == COMP2LPTIMIN2::NotGated
    }
}
#[doc = "Field `COMP2LPTIMIN2` writer - Comparator 2 LPTIM input 2 propagation bit"]
pub type COMP2LPTIMIN2_W<'a, REG> = crate::BitWriter<'a, REG, COMP2LPTIMIN2>;
impl<'a, REG> COMP2LPTIMIN2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator 2 output gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2LPTIMIN2::Gated)
    }
    #[doc = "Comparator 2 output sent to LPTIM input 2"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2LPTIMIN2::NotGated)
    }
}
#[doc = "Comparator 2 LPTIM input 1 propagation bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2LPTIMIN1 {
    #[doc = "0: Comparator 2 output gated"]
    Gated = 0,
    #[doc = "1: Comparator 2 output sent to LPTIM input 1"]
    NotGated = 1,
}
impl From<COMP2LPTIMIN1> for bool {
    #[inline(always)]
    fn from(variant: COMP2LPTIMIN1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2LPTIMIN1` reader - Comparator 2 LPTIM input 1 propagation bit"]
pub type COMP2LPTIMIN1_R = crate::BitReader<COMP2LPTIMIN1>;
impl COMP2LPTIMIN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2LPTIMIN1 {
        match self.bits {
            false => COMP2LPTIMIN1::Gated,
            true => COMP2LPTIMIN1::NotGated,
        }
    }
    #[doc = "Comparator 2 output gated"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == COMP2LPTIMIN1::Gated
    }
    #[doc = "Comparator 2 output sent to LPTIM input 1"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == COMP2LPTIMIN1::NotGated
    }
}
#[doc = "Field `COMP2LPTIMIN1` writer - Comparator 2 LPTIM input 1 propagation bit"]
pub type COMP2LPTIMIN1_W<'a, REG> = crate::BitWriter<'a, REG, COMP2LPTIMIN1>;
impl<'a, REG> COMP2LPTIMIN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator 2 output gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2LPTIMIN1::Gated)
    }
    #[doc = "Comparator 2 output sent to LPTIM input 1"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2LPTIMIN1::NotGated)
    }
}
#[doc = "Comparator 2 polarity selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2POLARITY {
    #[doc = "0: Comparator 2 output value not inverted"]
    NotInverted = 0,
    #[doc = "1: Comparator 2 output value inverted"]
    Inverted = 1,
}
impl From<COMP2POLARITY> for bool {
    #[inline(always)]
    fn from(variant: COMP2POLARITY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2POLARITY` reader - Comparator 2 polarity selection bit"]
pub type COMP2POLARITY_R = crate::BitReader<COMP2POLARITY>;
impl COMP2POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2POLARITY {
        match self.bits {
            false => COMP2POLARITY::NotInverted,
            true => COMP2POLARITY::Inverted,
        }
    }
    #[doc = "Comparator 2 output value not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP2POLARITY::NotInverted
    }
    #[doc = "Comparator 2 output value inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP2POLARITY::Inverted
    }
}
#[doc = "Field `COMP2POLARITY` writer - Comparator 2 polarity selection bit"]
pub type COMP2POLARITY_W<'a, REG> = crate::BitWriter<'a, REG, COMP2POLARITY>;
impl<'a, REG> COMP2POLARITY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator 2 output value not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2POLARITY::NotInverted)
    }
    #[doc = "Comparator 2 output value inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP2POLARITY::Inverted)
    }
}
#[doc = "Comparator 2 output status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2VALUER {
    #[doc = "0: Comparator values are not equal"]
    NotEqual = 0,
    #[doc = "1: Comparator values are equal"]
    Equal = 1,
}
impl From<COMP2VALUER> for bool {
    #[inline(always)]
    fn from(variant: COMP2VALUER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2VALUE` reader - Comparator 2 output status bit"]
pub type COMP2VALUE_R = crate::BitReader<COMP2VALUER>;
impl COMP2VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2VALUER {
        match self.bits {
            false => COMP2VALUER::NotEqual,
            true => COMP2VALUER::Equal,
        }
    }
    #[doc = "Comparator values are not equal"]
    #[inline(always)]
    pub fn is_not_equal(&self) -> bool {
        *self == COMP2VALUER::NotEqual
    }
    #[doc = "Comparator values are equal"]
    #[inline(always)]
    pub fn is_equal(&self) -> bool {
        *self == COMP2VALUER::Equal
    }
}
#[doc = "COMP2_CSR register lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2LOCK {
    #[doc = "0: COMP2_CSR\\[31:0\\]
for comparator 2 are read/write"]
    ReadWrite = 0,
    #[doc = "1: COMP2_CSR\\[31:0\\]
for comparator 2 are read-only"]
    ReadOnly = 1,
}
impl From<COMP2LOCK> for bool {
    #[inline(always)]
    fn from(variant: COMP2LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2LOCK` reader - COMP2_CSR register lock bit"]
pub type COMP2LOCK_R = crate::BitReader<COMP2LOCK>;
impl COMP2LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP2LOCK {
        match self.bits {
            false => COMP2LOCK::ReadWrite,
            true => COMP2LOCK::ReadOnly,
        }
    }
    #[doc = "COMP2_CSR\\[31:0\\]
for comparator 2 are read/write"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == COMP2LOCK::ReadWrite
    }
    #[doc = "COMP2_CSR\\[31:0\\]
for comparator 2 are read-only"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == COMP2LOCK::ReadOnly
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator 2 power mode selection bit"]
    #[inline(always)]
    pub fn comp2speed(&self) -> COMP2SPEED_R {
        COMP2SPEED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp2innsel(&self) -> COMP2INNSEL_R {
        COMP2INNSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn comp2inpsel(&self) -> COMP2INPSEL_R {
        COMP2INPSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Comparator 2 LPTIM input 2 propagation bit"]
    #[inline(always)]
    pub fn comp2lptimin2(&self) -> COMP2LPTIMIN2_R {
        COMP2LPTIMIN2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Comparator 2 LPTIM input 1 propagation bit"]
    #[inline(always)]
    pub fn comp2lptimin1(&self) -> COMP2LPTIMIN1_R {
        COMP2LPTIMIN1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn comp2polarity(&self) -> COMP2POLARITY_R {
        COMP2POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn comp2value(&self) -> COMP2VALUE_R {
        COMP2VALUE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock bit"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2en(&mut self) -> COMP2EN_W<COMP2_CSRrs> {
        COMP2EN_W::new(self, 0)
    }
    #[doc = "Bit 3 - Comparator 2 power mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2speed(&mut self) -> COMP2SPEED_W<COMP2_CSRrs> {
        COMP2SPEED_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2innsel(&mut self) -> COMP2INNSEL_W<COMP2_CSRrs> {
        COMP2INNSEL_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2inpsel(&mut self) -> COMP2INPSEL_W<COMP2_CSRrs> {
        COMP2INPSEL_W::new(self, 8)
    }
    #[doc = "Bit 12 - Comparator 2 LPTIM input 2 propagation bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2lptimin2(&mut self) -> COMP2LPTIMIN2_W<COMP2_CSRrs> {
        COMP2LPTIMIN2_W::new(self, 12)
    }
    #[doc = "Bit 13 - Comparator 2 LPTIM input 1 propagation bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2lptimin1(&mut self) -> COMP2LPTIMIN1_W<COMP2_CSRrs> {
        COMP2LPTIMIN1_W::new(self, 13)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp2polarity(&mut self) -> COMP2POLARITY_W<COMP2_CSRrs> {
        COMP2POLARITY_W::new(self, 15)
    }
}
#[doc = "Comparator 2 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP2_CSRrs;
impl crate::RegisterSpec for COMP2_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp2_csr::R`](R) reader structure"]
impl crate::Readable for COMP2_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`comp2_csr::W`](W) writer structure"]
impl crate::Writable for COMP2_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for COMP2_CSRrs {
    const RESET_VALUE: u32 = 0;
}
