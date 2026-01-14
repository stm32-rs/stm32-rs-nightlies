///Register `COMP1_CSR` reader
pub type R = crate::R<COMP1_CSRrs>;
///Register `COMP1_CSR` writer
pub type W = crate::W<COMP1_CSRrs>;
/**Comparator 1 enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: Comparator X disabled
    Disabled = 0,
    ///1: Comparator X enabled
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Comparator 1 enable bit
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    ///Comparator X disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    ///Comparator X enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
///Field `EN` writer - Comparator 1 enable bit
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator X disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    ///Comparator X enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
///Field `PWRMODE` reader - Power Mode of the comparator 1
pub type PWRMODE_R = crate::FieldReader;
///Field `PWRMODE` writer - Power Mode of the comparator 1
pub type PWRMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `INMSEL` reader - Comparator 1 Input Minus connection configuration bit
pub type INMSEL_R = crate::FieldReader;
///Field `INMSEL` writer - Comparator 1 Input Minus connection configuration bit
pub type INMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `INPSEL` reader - Comparator1 input plus selection bit
pub type INPSEL_R = crate::BitReader;
///Field `INPSEL` writer - Comparator1 input plus selection bit
pub type INPSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Comparator 1 polarity selection bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLARITY {
    ///0: Comparator X output value not inverted
    Normal = 0,
    ///1: Comparator X output value inverted
    Inverted = 1,
}
impl From<POLARITY> for bool {
    #[inline(always)]
    fn from(variant: POLARITY) -> Self {
        variant as u8 != 0
    }
}
///Field `POLARITY` reader - Comparator 1 polarity selection bit
pub type POLARITY_R = crate::BitReader<POLARITY>;
impl POLARITY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> POLARITY {
        match self.bits {
            false => POLARITY::Normal,
            true => POLARITY::Inverted,
        }
    }
    ///Comparator X output value not inverted
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == POLARITY::Normal
    }
    ///Comparator X output value inverted
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == POLARITY::Inverted
    }
}
///Field `POLARITY` writer - Comparator 1 polarity selection bit
pub type POLARITY_W<'a, REG> = crate::BitWriter<'a, REG, POLARITY>;
impl<'a, REG> POLARITY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator X output value not inverted
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY::Normal)
    }
    ///Comparator X output value inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(POLARITY::Inverted)
    }
}
/**Comparator 1 hysteresis selection bits

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYST {
    ///0: No hysteresis
    None = 0,
    ///1: Low hysteresis
    Low = 1,
    ///2: Medium hysteresis
    Medium = 2,
    ///3: High hysteresis
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
impl crate::IsEnum for HYST {}
///Field `HYST` reader - Comparator 1 hysteresis selection bits
pub type HYST_R = crate::FieldReader<HYST>;
impl HYST_R {
    ///Get enumerated values variant
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
    ///No hysteresis
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == HYST::None
    }
    ///Low hysteresis
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == HYST::Low
    }
    ///Medium hysteresis
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == HYST::Medium
    }
    ///High hysteresis
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == HYST::High
    }
}
///Field `HYST` writer - Comparator 1 hysteresis selection bits
pub type HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HYST, crate::Safe>;
impl<'a, REG> HYST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No hysteresis
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::None)
    }
    ///Low hysteresis
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Low)
    }
    ///Medium hysteresis
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Medium)
    }
    ///High hysteresis
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::High)
    }
}
///Field `BLANKING` reader - Comparator 1 blanking source selection bits
pub type BLANKING_R = crate::FieldReader;
///Field `BLANKING` writer - Comparator 1 blanking source selection bits
pub type BLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Scaler bridge enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRGEN {
    ///0: Scaler resistor bridge disabled
    Disabled = 0,
    ///1: Scaler resistor bridge enabled
    Enabled = 1,
}
impl From<BRGEN> for bool {
    #[inline(always)]
    fn from(variant: BRGEN) -> Self {
        variant as u8 != 0
    }
}
///Field `BRGEN` reader - Scaler bridge enable
pub type BRGEN_R = crate::BitReader<BRGEN>;
impl BRGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BRGEN {
        match self.bits {
            false => BRGEN::Disabled,
            true => BRGEN::Enabled,
        }
    }
    ///Scaler resistor bridge disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRGEN::Disabled
    }
    ///Scaler resistor bridge enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRGEN::Enabled
    }
}
///Field `BRGEN` writer - Scaler bridge enable
pub type BRGEN_W<'a, REG> = crate::BitWriter<'a, REG, BRGEN>;
impl<'a, REG> BRGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Scaler resistor bridge disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRGEN::Disabled)
    }
    ///Scaler resistor bridge enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BRGEN::Enabled)
    }
}
/**Voltage scaler enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCALEN {
    ///0: Voltage scaler disabled
    Disabled = 0,
    ///1: Voltage scaler enabled
    Enabled = 1,
}
impl From<SCALEN> for bool {
    #[inline(always)]
    fn from(variant: SCALEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SCALEN` reader - Voltage scaler enable bit
pub type SCALEN_R = crate::BitReader<SCALEN>;
impl SCALEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCALEN {
        match self.bits {
            false => SCALEN::Disabled,
            true => SCALEN::Enabled,
        }
    }
    ///Voltage scaler disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCALEN::Disabled
    }
    ///Voltage scaler enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCALEN::Enabled
    }
}
///Field `SCALEN` writer - Voltage scaler enable bit
pub type SCALEN_W<'a, REG> = crate::BitWriter<'a, REG, SCALEN>;
impl<'a, REG> SCALEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Voltage scaler disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCALEN::Disabled)
    }
    ///Voltage scaler enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCALEN::Enabled)
    }
}
/**Comparator 1 output status bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VALUE {
    ///0: Comparator output is low
    Low = 0,
    ///1: Comparator output is high
    High = 1,
}
impl From<VALUE> for bool {
    #[inline(always)]
    fn from(variant: VALUE) -> Self {
        variant as u8 != 0
    }
}
///Field `VALUE` reader - Comparator 1 output status bit
pub type VALUE_R = crate::BitReader<VALUE>;
impl VALUE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> VALUE {
        match self.bits {
            false => VALUE::Low,
            true => VALUE::High,
        }
    }
    ///Comparator output is low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == VALUE::Low
    }
    ///Comparator output is high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == VALUE::High
    }
}
/**COMP1_CSR register lock bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK {
    ///0: Comparator CSR bits are read-write
    Unlocked = 0,
    ///1: Comparator CSR bits are read-only
    Locked = 1,
}
impl From<LOCK> for bool {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` writer - COMP1_CSR register lock bit
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCK>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Comparator CSR bits are read-write
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Unlocked)
    }
    ///Comparator CSR bits are read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Locked)
    }
}
impl R {
    ///Bit 0 - Comparator 1 enable bit
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Power Mode of the comparator 1
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Comparator 1 Input Minus connection configuration bit
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Comparator1 input plus selection bit
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - Comparator 1 polarity selection bit
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 1 hysteresis selection bits
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator 1 blanking source selection bits
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 22 - Scaler bridge enable
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Voltage scaler enable bit
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 30 - Comparator 1 output status bit
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP1_CSR")
            .field("en", &self.en())
            .field("pwrmode", &self.pwrmode())
            .field("inmsel", &self.inmsel())
            .field("inpsel", &self.inpsel())
            .field("polarity", &self.polarity())
            .field("hyst", &self.hyst())
            .field("blanking", &self.blanking())
            .field("brgen", &self.brgen())
            .field("scalen", &self.scalen())
            .field("value", &self.value())
            .finish()
    }
}
impl W {
    ///Bit 0 - Comparator 1 enable bit
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, COMP1_CSRrs> {
        EN_W::new(self, 0)
    }
    ///Bits 2:3 - Power Mode of the comparator 1
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PWRMODE_W<'_, COMP1_CSRrs> {
        PWRMODE_W::new(self, 2)
    }
    ///Bits 4:6 - Comparator 1 Input Minus connection configuration bit
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W<'_, COMP1_CSRrs> {
        INMSEL_W::new(self, 4)
    }
    ///Bit 7 - Comparator1 input plus selection bit
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W<'_, COMP1_CSRrs> {
        INPSEL_W::new(self, 7)
    }
    ///Bit 15 - Comparator 1 polarity selection bit
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<'_, COMP1_CSRrs> {
        POLARITY_W::new(self, 15)
    }
    ///Bits 16:17 - Comparator 1 hysteresis selection bits
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<'_, COMP1_CSRrs> {
        HYST_W::new(self, 16)
    }
    ///Bits 18:20 - Comparator 1 blanking source selection bits
    #[inline(always)]
    pub fn blanking(&mut self) -> BLANKING_W<'_, COMP1_CSRrs> {
        BLANKING_W::new(self, 18)
    }
    ///Bit 22 - Scaler bridge enable
    #[inline(always)]
    pub fn brgen(&mut self) -> BRGEN_W<'_, COMP1_CSRrs> {
        BRGEN_W::new(self, 22)
    }
    ///Bit 23 - Voltage scaler enable bit
    #[inline(always)]
    pub fn scalen(&mut self) -> SCALEN_W<'_, COMP1_CSRrs> {
        SCALEN_W::new(self, 23)
    }
    ///Bit 31 - COMP1_CSR register lock bit
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, COMP1_CSRrs> {
        LOCK_W::new(self, 31)
    }
}
/**Comparator 1 control and status register

You can [`read`](crate::Reg::read) this register and get [`comp1_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#COMP:COMP1_CSR)*/
pub struct COMP1_CSRrs;
impl crate::RegisterSpec for COMP1_CSRrs {
    type Ux = u32;
}
///`read()` method returns [`comp1_csr::R`](R) reader structure
impl crate::Readable for COMP1_CSRrs {}
///`write(|w| ..)` method takes [`comp1_csr::W`](W) writer structure
impl crate::Writable for COMP1_CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COMP1_CSR to value 0
impl crate::Resettable for COMP1_CSRrs {}
