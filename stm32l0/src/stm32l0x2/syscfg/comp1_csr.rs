#[doc = "Register `COMP1_CSR` reader"]
pub type R = crate::R<COMP1_CSRrs>;
#[doc = "Register `COMP1_CSR` writer"]
pub type W = crate::W<COMP1_CSRrs>;
#[doc = "Comparator 1 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1EN {
    #[doc = "0: Comparator 1 switched OFF"]
    Disabled = 0,
    #[doc = "1: Comparator 1 switched ON"]
    Enabled = 1,
}
impl From<COMP1EN> for bool {
    #[inline(always)]
    fn from(variant: COMP1EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1EN` reader - Comparator 1 enable bit"]
pub type COMP1EN_R = crate::BitReader<COMP1EN>;
impl COMP1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1EN {
        match self.bits {
            false => COMP1EN::Disabled,
            true => COMP1EN::Enabled,
        }
    }
    #[doc = "Comparator 1 switched OFF"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP1EN::Disabled
    }
    #[doc = "Comparator 1 switched ON"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP1EN::Enabled
    }
}
#[doc = "Field `COMP1EN` writer - Comparator 1 enable bit"]
pub type COMP1EN_W<'a, REG> = crate::BitWriter<'a, REG, COMP1EN>;
impl<'a, REG> COMP1EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator 1 switched OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1EN::Disabled)
    }
    #[doc = "Comparator 1 switched ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1EN::Enabled)
    }
}
#[doc = "Comparator 1 Input Minus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP1INNSEL {
    #[doc = "0: VREFINT"]
    Vrefint = 0,
    #[doc = "1: PA0"]
    Pa0 = 1,
    #[doc = "2: PA4"]
    Pa4 = 2,
    #[doc = "3: PA5"]
    Pa5 = 3,
}
impl From<COMP1INNSEL> for u8 {
    #[inline(always)]
    fn from(variant: COMP1INNSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP1INNSEL {
    type Ux = u8;
}
#[doc = "Field `COMP1INNSEL` reader - Comparator 1 Input Minus connection configuration bit"]
pub type COMP1INNSEL_R = crate::FieldReader<COMP1INNSEL>;
impl COMP1INNSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1INNSEL {
        match self.bits {
            0 => COMP1INNSEL::Vrefint,
            1 => COMP1INNSEL::Pa0,
            2 => COMP1INNSEL::Pa4,
            3 => COMP1INNSEL::Pa5,
            _ => unreachable!(),
        }
    }
    #[doc = "VREFINT"]
    #[inline(always)]
    pub fn is_vrefint(&self) -> bool {
        *self == COMP1INNSEL::Vrefint
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == COMP1INNSEL::Pa0
    }
    #[doc = "PA4"]
    #[inline(always)]
    pub fn is_pa4(&self) -> bool {
        *self == COMP1INNSEL::Pa4
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == COMP1INNSEL::Pa5
    }
}
#[doc = "Field `COMP1INNSEL` writer - Comparator 1 Input Minus connection configuration bit"]
pub type COMP1INNSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COMP1INNSEL>;
impl<'a, REG> COMP1INNSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VREFINT"]
    #[inline(always)]
    pub fn vrefint(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INNSEL::Vrefint)
    }
    #[doc = "PA0"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INNSEL::Pa0)
    }
    #[doc = "PA4"]
    #[inline(always)]
    pub fn pa4(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INNSEL::Pa4)
    }
    #[doc = "PA5"]
    #[inline(always)]
    pub fn pa5(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1INNSEL::Pa5)
    }
}
#[doc = "Comparator 1 window mode selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1WM {
    #[doc = "0: Plus input of comparator 1 connected to PA1"]
    Pa1 = 0,
    #[doc = "1: Plus input of comparator 1 shorted with Plus input of comparator 2 (see COMP1_CSR)"]
    Comp2plus = 1,
}
impl From<COMP1WM> for bool {
    #[inline(always)]
    fn from(variant: COMP1WM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1WM` reader - Comparator 1 window mode selection bit"]
pub type COMP1WM_R = crate::BitReader<COMP1WM>;
impl COMP1WM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1WM {
        match self.bits {
            false => COMP1WM::Pa1,
            true => COMP1WM::Comp2plus,
        }
    }
    #[doc = "Plus input of comparator 1 connected to PA1"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == COMP1WM::Pa1
    }
    #[doc = "Plus input of comparator 1 shorted with Plus input of comparator 2 (see COMP1_CSR)"]
    #[inline(always)]
    pub fn is_comp2plus(&self) -> bool {
        *self == COMP1WM::Comp2plus
    }
}
#[doc = "Field `COMP1WM` writer - Comparator 1 window mode selection bit"]
pub type COMP1WM_W<'a, REG> = crate::BitWriter<'a, REG, COMP1WM>;
impl<'a, REG> COMP1WM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Plus input of comparator 1 connected to PA1"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1WM::Pa1)
    }
    #[doc = "Plus input of comparator 1 shorted with Plus input of comparator 2 (see COMP1_CSR)"]
    #[inline(always)]
    pub fn comp2plus(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1WM::Comp2plus)
    }
}
#[doc = "Comparator 1 LPTIM input propagation bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1LPTIMIN1 {
    #[doc = "0: Comparator 1 output gated"]
    Gated = 0,
    #[doc = "1: Comparator 1 output sent to LPTIM input 1"]
    NotGated = 1,
}
impl From<COMP1LPTIMIN1> for bool {
    #[inline(always)]
    fn from(variant: COMP1LPTIMIN1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1LPTIMIN1` reader - Comparator 1 LPTIM input propagation bit"]
pub type COMP1LPTIMIN1_R = crate::BitReader<COMP1LPTIMIN1>;
impl COMP1LPTIMIN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1LPTIMIN1 {
        match self.bits {
            false => COMP1LPTIMIN1::Gated,
            true => COMP1LPTIMIN1::NotGated,
        }
    }
    #[doc = "Comparator 1 output gated"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == COMP1LPTIMIN1::Gated
    }
    #[doc = "Comparator 1 output sent to LPTIM input 1"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == COMP1LPTIMIN1::NotGated
    }
}
#[doc = "Field `COMP1LPTIMIN1` writer - Comparator 1 LPTIM input propagation bit"]
pub type COMP1LPTIMIN1_W<'a, REG> = crate::BitWriter<'a, REG, COMP1LPTIMIN1>;
impl<'a, REG> COMP1LPTIMIN1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator 1 output gated"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1LPTIMIN1::Gated)
    }
    #[doc = "Comparator 1 output sent to LPTIM input 1"]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1LPTIMIN1::NotGated)
    }
}
#[doc = "Comparator 1 polarity selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1POLARITY {
    #[doc = "0: Comparator 1 output value not inverted"]
    NotInverted = 0,
    #[doc = "1: Comparator 1 output value inverted"]
    Inverted = 1,
}
impl From<COMP1POLARITY> for bool {
    #[inline(always)]
    fn from(variant: COMP1POLARITY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1POLARITY` reader - Comparator 1 polarity selection bit"]
pub type COMP1POLARITY_R = crate::BitReader<COMP1POLARITY>;
impl COMP1POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1POLARITY {
        match self.bits {
            false => COMP1POLARITY::NotInverted,
            true => COMP1POLARITY::Inverted,
        }
    }
    #[doc = "Comparator 1 output value not inverted"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP1POLARITY::NotInverted
    }
    #[doc = "Comparator 1 output value inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP1POLARITY::Inverted
    }
}
#[doc = "Field `COMP1POLARITY` writer - Comparator 1 polarity selection bit"]
pub type COMP1POLARITY_W<'a, REG> = crate::BitWriter<'a, REG, COMP1POLARITY>;
impl<'a, REG> COMP1POLARITY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator 1 output value not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1POLARITY::NotInverted)
    }
    #[doc = "Comparator 1 output value inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(COMP1POLARITY::Inverted)
    }
}
#[doc = "Comparator 1 output status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1VALUER {
    #[doc = "0: Comparator values are not equal"]
    NotEqual = 0,
    #[doc = "1: Comparator values are equal"]
    Equal = 1,
}
impl From<COMP1VALUER> for bool {
    #[inline(always)]
    fn from(variant: COMP1VALUER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1VALUE` reader - Comparator 1 output status bit"]
pub type COMP1VALUE_R = crate::BitReader<COMP1VALUER>;
impl COMP1VALUE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1VALUER {
        match self.bits {
            false => COMP1VALUER::NotEqual,
            true => COMP1VALUER::Equal,
        }
    }
    #[doc = "Comparator values are not equal"]
    #[inline(always)]
    pub fn is_not_equal(&self) -> bool {
        *self == COMP1VALUER::NotEqual
    }
    #[doc = "Comparator values are equal"]
    #[inline(always)]
    pub fn is_equal(&self) -> bool {
        *self == COMP1VALUER::Equal
    }
}
#[doc = "COMP1_CSR register lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP1LOCK {
    #[doc = "0: COMP1_CSR\\[31:0\\]
for comparator 1 are read/write"]
    ReadWrite = 0,
    #[doc = "1: COMP1_CSR\\[31:0\\]
for comparator 1 are read-only"]
    ReadOnly = 1,
}
impl From<COMP1LOCK> for bool {
    #[inline(always)]
    fn from(variant: COMP1LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1LOCK` reader - COMP1_CSR register lock bit"]
pub type COMP1LOCK_R = crate::BitReader<COMP1LOCK>;
impl COMP1LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMP1LOCK {
        match self.bits {
            false => COMP1LOCK::ReadWrite,
            true => COMP1LOCK::ReadOnly,
        }
    }
    #[doc = "COMP1_CSR\\[31:0\\]
for comparator 1 are read/write"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == COMP1LOCK::ReadWrite
    }
    #[doc = "COMP1_CSR\\[31:0\\]
for comparator 1 are read-only"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == COMP1LOCK::ReadOnly
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1innsel(&self) -> COMP1INNSEL_R {
        COMP1INNSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    pub fn comp1wm(&self) -> COMP1WM_R {
        COMP1WM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparator 1 LPTIM input propagation bit"]
    #[inline(always)]
    pub fn comp1lptimin1(&self) -> COMP1LPTIMIN1_R {
        COMP1LPTIMIN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1polarity(&self) -> COMP1POLARITY_R {
        COMP1POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn comp1value(&self) -> COMP1VALUE_R {
        COMP1VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1en(&mut self) -> COMP1EN_W<COMP1_CSRrs> {
        COMP1EN_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1innsel(&mut self) -> COMP1INNSEL_W<COMP1_CSRrs> {
        COMP1INNSEL_W::new(self, 4)
    }
    #[doc = "Bit 8 - Comparator 1 window mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1wm(&mut self) -> COMP1WM_W<COMP1_CSRrs> {
        COMP1WM_W::new(self, 8)
    }
    #[doc = "Bit 12 - Comparator 1 LPTIM input propagation bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1lptimin1(&mut self) -> COMP1LPTIMIN1_W<COMP1_CSRrs> {
        COMP1LPTIMIN1_W::new(self, 12)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1polarity(&mut self) -> COMP1POLARITY_W<COMP1_CSRrs> {
        COMP1POLARITY_W::new(self, 15)
    }
}
#[doc = "Comparator 1 control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1_csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMP1_CSRrs;
impl crate::RegisterSpec for COMP1_CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1_csr::R`](R) reader structure"]
impl crate::Readable for COMP1_CSRrs {}
#[doc = "`write(|w| ..)` method takes [`comp1_csr::W`](W) writer structure"]
impl crate::Writable for COMP1_CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for COMP1_CSRrs {
    const RESET_VALUE: u32 = 0;
}
