#[doc = "Register `COMP2_CSR` reader"]
pub type R = crate::R<COMP2_CSRrs>;
#[doc = "Register `COMP2_CSR` writer"]
pub type W = crate::W<COMP2_CSRrs>;
#[doc = "Comparator 2 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    #[doc = "0: Comparator x switched OFF"]
    Disabled = 0,
    #[doc = "1: Comparator x switched ON"]
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Comparator 2 enable bit"]
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    #[doc = "Comparator x switched OFF"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    #[doc = "Comparator x switched ON"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
#[doc = "Field `EN` writer - Comparator 2 enable bit"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator x switched OFF"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    #[doc = "Comparator x switched ON"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
#[doc = "Power Mode of the comparator 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRMODE {
    #[doc = "0: High speed"]
    High = 0,
    #[doc = "1: Medium speed"]
    Medium = 1,
    #[doc = "3: Ultra low power"]
    Low = 3,
}
impl From<PWRMODE> for u8 {
    #[inline(always)]
    fn from(variant: PWRMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWRMODE {
    type Ux = u8;
}
#[doc = "Field `PWRMODE` reader - Power Mode of the comparator 2"]
pub type PWRMODE_R = crate::FieldReader<PWRMODE>;
impl PWRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWRMODE> {
        match self.bits {
            0 => Some(PWRMODE::High),
            1 => Some(PWRMODE::Medium),
            3 => Some(PWRMODE::Low),
            _ => None,
        }
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PWRMODE::High
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == PWRMODE::Medium
    }
    #[doc = "Ultra low power"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PWRMODE::Low
    }
}
#[doc = "Field `PWRMODE` writer - Power Mode of the comparator 2"]
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PWRMODE>;
impl<'a, REG> PWRMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE::High)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE::Medium)
    }
    #[doc = "Ultra low power"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(PWRMODE::Low)
    }
}
#[doc = "Field `INMSEL` reader - Comparator 2 Input Minus connection configuration bit"]
pub type INMSEL_R = crate::FieldReader;
#[doc = "Field `INMSEL` writer - Comparator 2 Input Minus connection configuration bit"]
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Comparator 2 Input Plus connection configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPSEL {
    #[doc = "0: PB4"]
    Pb4 = 0,
    #[doc = "1: PB6"]
    Pb6 = 1,
}
impl From<INPSEL> for bool {
    #[inline(always)]
    fn from(variant: INPSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPSEL` reader - Comparator 2 Input Plus connection configuration bit"]
pub type INPSEL_R = crate::BitReader<INPSEL>;
impl INPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INPSEL {
        match self.bits {
            false => INPSEL::Pb4,
            true => INPSEL::Pb6,
        }
    }
    #[doc = "PB4"]
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == INPSEL::Pb4
    }
    #[doc = "PB6"]
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == INPSEL::Pb6
    }
}
#[doc = "Field `INPSEL` writer - Comparator 2 Input Plus connection configuration bit"]
pub type INPSEL_W<'a, REG> = crate::BitWriter<'a, REG, INPSEL>;
impl<'a, REG> INPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PB4"]
    #[inline(always)]
    pub fn pb4(self) -> &'a mut crate::W<REG> {
        self.variant(INPSEL::Pb4)
    }
    #[doc = "PB6"]
    #[inline(always)]
    pub fn pb6(self) -> &'a mut crate::W<REG> {
        self.variant(INPSEL::Pb6)
    }
}
#[doc = "Windows mode selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WINMODE {
    #[doc = "0: Input plus of Comparator 2 is not connected to Comparator 1"]
    Disabled = 0,
    #[doc = "1: Input plus of Comparator 2 is connected with input plus of Comparator 1"]
    Enabled = 1,
}
impl From<WINMODE> for bool {
    #[inline(always)]
    fn from(variant: WINMODE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WINMODE` reader - Windows mode selection bit"]
pub type WINMODE_R = crate::BitReader<WINMODE>;
impl WINMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WINMODE {
        match self.bits {
            false => WINMODE::Disabled,
            true => WINMODE::Enabled,
        }
    }
    #[doc = "Input plus of Comparator 2 is not connected to Comparator 1"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WINMODE::Disabled
    }
    #[doc = "Input plus of Comparator 2 is connected with input plus of Comparator 1"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WINMODE::Enabled
    }
}
#[doc = "Field `WINMODE` writer - Windows mode selection bit"]
pub type WINMODE_W<'a, REG> = crate::BitWriter<'a, REG, WINMODE>;
impl<'a, REG> WINMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input plus of Comparator 2 is not connected to Comparator 1"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODE::Disabled)
    }
    #[doc = "Input plus of Comparator 2 is connected with input plus of Comparator 1"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WINMODE::Enabled)
    }
}
#[doc = "Comparator 2 polarity selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLARITY {
    #[doc = "0: Comparator x output value not inverted"]
    Normal = 0,
    #[doc = "1: Comparator x output value inverted"]
    Inverted = 1,
}
impl From<POLARITY> for bool {
    #[inline(always)]
    fn from(variant: POLARITY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLARITY` reader - Comparator 2 polarity selection bit"]
pub type POLARITY_R = crate::BitReader<POLARITY>;
impl POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POLARITY {
        match self.bits {
            false => POLARITY::Normal,
            true => POLARITY::Inverted,
        }
    }
    #[doc = "Comparator x output value not inverted"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == POLARITY::Normal
    }
    #[doc = "Comparator x output value inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == POLARITY::Inverted
    }
}
#[doc = "Field `POLARITY` writer - Comparator 2 polarity selection bit"]
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG, POLARITY>;
impl<'a, REG> POLARITY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator x output value not inverted"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY::Normal)
    }
    #[doc = "Comparator x output value inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY::Inverted)
    }
}
#[doc = "Comparator 2 hysteresis selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYST {
    #[doc = "0: No hysteresis"]
    None = 0,
    #[doc = "1: Low hysteresis"]
    Low = 1,
    #[doc = "2: Medium hysteresis"]
    Medium = 2,
    #[doc = "3: High hysteresis"]
    High = 3,
}
impl From<HYST> for u8 {
    #[inline(always)]
    fn from(variant: HYST) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HYST {
    type Ux = u8;
}
#[doc = "Field `HYST` reader - Comparator 2 hysteresis selection bits"]
pub type HYST_R = crate::FieldReader<HYST>;
impl HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HYST {
        match self.bits {
            0 => HYST::None,
            1 => HYST::Low,
            2 => HYST::Medium,
            3 => HYST::High,
            _ => unreachable!(),
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == HYST::None
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == HYST::Low
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == HYST::Medium
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == HYST::High
    }
}
#[doc = "Field `HYST` writer - Comparator 2 hysteresis selection bits"]
pub type HYST_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, HYST>;
impl<'a, REG> HYST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::None)
    }
    #[doc = "Low hysteresis"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Low)
    }
    #[doc = "Medium hysteresis"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Medium)
    }
    #[doc = "High hysteresis"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::High)
    }
}
#[doc = "Comparator 2 blanking source selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLANKING {
    #[doc = "0: No blanking"]
    Disabled = 0,
    #[doc = "4: TIM15 OC1 selected as blanking source"]
    Tim15oc1 = 4,
}
impl From<BLANKING> for u8 {
    #[inline(always)]
    fn from(variant: BLANKING) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BLANKING {
    type Ux = u8;
}
#[doc = "Field `BLANKING` reader - Comparator 2 blanking source selection bits"]
pub type BLANKING_R = crate::FieldReader<BLANKING>;
impl BLANKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BLANKING> {
        match self.bits {
            0 => Some(BLANKING::Disabled),
            4 => Some(BLANKING::Tim15oc1),
            _ => None,
        }
    }
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BLANKING::Disabled
    }
    #[doc = "TIM15 OC1 selected as blanking source"]
    #[inline(always)]
    pub fn is_tim15oc1(&self) -> bool {
        *self == BLANKING::Tim15oc1
    }
}
#[doc = "Field `BLANKING` writer - Comparator 2 blanking source selection bits"]
pub type BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BLANKING>;
impl<'a, REG> BLANKING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING::Disabled)
    }
    #[doc = "TIM15 OC1 selected as blanking source"]
    #[inline(always)]
    pub fn tim15oc1(self) -> &'a mut crate::W<REG> {
        self.variant(BLANKING::Tim15oc1)
    }
}
#[doc = "Scaler bridge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRGEN {
    #[doc = "0: Scaler resistor bridge disabled (if BRGEN bit of COMP2_CSR register is also reset)"]
    Disabled = 0,
    #[doc = "1: Scaler resistor bridge enabled"]
    Enabled = 1,
}
impl From<BRGEN> for bool {
    #[inline(always)]
    fn from(variant: BRGEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRGEN` reader - Scaler bridge enable"]
pub type BRGEN_R = crate::BitReader<BRGEN>;
impl BRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BRGEN {
        match self.bits {
            false => BRGEN::Disabled,
            true => BRGEN::Enabled,
        }
    }
    #[doc = "Scaler resistor bridge disabled (if BRGEN bit of COMP2_CSR register is also reset)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRGEN::Disabled
    }
    #[doc = "Scaler resistor bridge enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRGEN::Enabled
    }
}
#[doc = "Field `BRGEN` writer - Scaler bridge enable"]
pub type BRGEN_W<'a, REG> = crate::BitWriter<'a, REG, BRGEN>;
impl<'a, REG> BRGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Scaler resistor bridge disabled (if BRGEN bit of COMP2_CSR register is also reset)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRGEN::Disabled)
    }
    #[doc = "Scaler resistor bridge enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRGEN::Enabled)
    }
}
#[doc = "Voltage scaler enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCALEN {
    #[doc = "0: Bandgap scaler disabled (if SCALEN bit of COMP2_CSR register is also reset)"]
    Disabled = 0,
    #[doc = "1: Bandgap scaler enabled"]
    Enabled = 1,
}
impl From<SCALEN> for bool {
    #[inline(always)]
    fn from(variant: SCALEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCALEN` reader - Voltage scaler enable bit"]
pub type SCALEN_R = crate::BitReader<SCALEN>;
impl SCALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCALEN {
        match self.bits {
            false => SCALEN::Disabled,
            true => SCALEN::Enabled,
        }
    }
    #[doc = "Bandgap scaler disabled (if SCALEN bit of COMP2_CSR register is also reset)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCALEN::Disabled
    }
    #[doc = "Bandgap scaler enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCALEN::Enabled
    }
}
#[doc = "Field `SCALEN` writer - Voltage scaler enable bit"]
pub type SCALEN_W<'a, REG> = crate::BitWriter<'a, REG, SCALEN>;
impl<'a, REG> SCALEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bandgap scaler disabled (if SCALEN bit of COMP2_CSR register is also reset)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCALEN::Disabled)
    }
    #[doc = "Bandgap scaler enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCALEN::Enabled)
    }
}
#[doc = "Field `VALUE` reader - Comparator 2 output status bit"]
pub type VALUE_R = crate::BitReader;
#[doc = "COMP2_CSR register lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK {
    #[doc = "0: COMPx_CSR\\[31:0\\]
for comparator x are read/write"]
    Unlocked = 0,
    #[doc = "1: COMPx_CSR\\[31:0\\]
for comparator x are read-only"]
    Locked = 1,
}
impl From<LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` writer - COMP2_CSR register lock bit"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMPx_CSR\\[31:0\\]
for comparator x are read/write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Unlocked)
    }
    #[doc = "COMPx_CSR\\[31:0\\]
for comparator x are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Locked)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 2 output status bit"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<COMP2_CSRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<COMP2_CSRrs> {
        PWRMODE_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<COMP2_CSRrs> {
        INMSEL_W::new(self, 4)
    }
    #[doc = "Bit 7 - Comparator 2 Input Plus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<COMP2_CSRrs> {
        INPSEL_W::new(self, 7)
    }
    #[doc = "Bit 9 - Windows mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WINMODE_W<COMP2_CSRrs> {
        WINMODE_W::new(self, 9)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<COMP2_CSRrs> {
        POLARITY_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<COMP2_CSRrs> {
        HYST_W::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn blanking(&mut self) -> BLANKING_W<COMP2_CSRrs> {
        BLANKING_W::new(self, 18)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    #[must_use]
    pub fn brgen(&mut self) -> BRGEN_W<COMP2_CSRrs> {
        BRGEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn scalen(&mut self) -> SCALEN_W<COMP2_CSRrs> {
        SCALEN_W::new(self, 23)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<COMP2_CSRrs> {
        LOCK_W::new(self, 31)
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
