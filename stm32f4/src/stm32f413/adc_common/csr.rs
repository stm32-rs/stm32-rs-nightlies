///Register `CSR` reader
pub type R = crate::R<CSRrs>;
/**Analog watchdog flag of ADC 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1 {
    ///0: No analog watchdog event occurred
    NoEvent = 0,
    ///1: Analog watchdog event occurred
    Event = 1,
}
impl From<AWD1> for bool {
    #[inline(always)]
    fn from(variant: AWD1) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1` reader - Analog watchdog flag of ADC 1
pub type AWD1_R = crate::BitReader<AWD1>;
impl AWD1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1 {
        match self.bits {
            false => AWD1::NoEvent,
            true => AWD1::Event,
        }
    }
    ///No analog watchdog event occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1::NoEvent
    }
    ///Analog watchdog event occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1::Event
    }
}
/**End of conversion of ADC 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOC1 {
    ///0: Conversion is not complete
    NotComplete = 0,
    ///1: Conversion complete
    Complete = 1,
}
impl From<EOC1> for bool {
    #[inline(always)]
    fn from(variant: EOC1) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC1` reader - End of conversion of ADC 1
pub type EOC1_R = crate::BitReader<EOC1>;
impl EOC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOC1 {
        match self.bits {
            false => EOC1::NotComplete,
            true => EOC1::Complete,
        }
    }
    ///Conversion is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC1::NotComplete
    }
    ///Conversion complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC1::Complete
    }
}
/**Injected channel end of conversion of ADC 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOC1 {
    ///0: Conversion is not complete
    NotComplete = 0,
    ///1: Conversion complete
    Complete = 1,
}
impl From<JEOC1> for bool {
    #[inline(always)]
    fn from(variant: JEOC1) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOC1` reader - Injected channel end of conversion of ADC 1
pub type JEOC1_R = crate::BitReader<JEOC1>;
impl JEOC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEOC1 {
        match self.bits {
            false => JEOC1::NotComplete,
            true => JEOC1::Complete,
        }
    }
    ///Conversion is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC1::NotComplete
    }
    ///Conversion complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC1::Complete
    }
}
/**Injected channel Start flag of ADC 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSTRT1 {
    ///0: No injected channel conversion started
    NotStarted = 0,
    ///1: Injected channel conversion has started
    Started = 1,
}
impl From<JSTRT1> for bool {
    #[inline(always)]
    fn from(variant: JSTRT1) -> Self {
        variant as u8 != 0
    }
}
///Field `JSTRT1` reader - Injected channel Start flag of ADC 1
pub type JSTRT1_R = crate::BitReader<JSTRT1>;
impl JSTRT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JSTRT1 {
        match self.bits {
            false => JSTRT1::NotStarted,
            true => JSTRT1::Started,
        }
    }
    ///No injected channel conversion started
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRT1::NotStarted
    }
    ///Injected channel conversion has started
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSTRT1::Started
    }
}
/**Regular channel Start flag of ADC 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRT1 {
    ///0: No regular channel conversion started
    NotStarted = 0,
    ///1: Regular channel conversion has started
    Started = 1,
}
impl From<STRT1> for bool {
    #[inline(always)]
    fn from(variant: STRT1) -> Self {
        variant as u8 != 0
    }
}
///Field `STRT1` reader - Regular channel Start flag of ADC 1
pub type STRT1_R = crate::BitReader<STRT1>;
impl STRT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STRT1 {
        match self.bits {
            false => STRT1::NotStarted,
            true => STRT1::Started,
        }
    }
    ///No regular channel conversion started
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRT1::NotStarted
    }
    ///Regular channel conversion has started
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRT1::Started
    }
}
/**Overrun flag of ADC 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR1 {
    ///0: No overrun occurred
    NoOverrun = 0,
    ///1: Overrun occurred
    Overrun = 1,
}
impl From<OVR1> for bool {
    #[inline(always)]
    fn from(variant: OVR1) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR1` reader - Overrun flag of ADC 1
pub type OVR1_R = crate::BitReader<OVR1>;
impl OVR1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVR1 {
        match self.bits {
            false => OVR1::NoOverrun,
            true => OVR1::Overrun,
        }
    }
    ///No overrun occurred
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR1::NoOverrun
    }
    ///Overrun occurred
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR1::Overrun
    }
}
impl R {
    ///Bit 0 - Analog watchdog flag of ADC 1
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of conversion of ADC 1
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected channel end of conversion of ADC 1
    #[inline(always)]
    pub fn jeoc1(&self) -> JEOC1_R {
        JEOC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Injected channel Start flag of ADC 1
    #[inline(always)]
    pub fn jstrt1(&self) -> JSTRT1_R {
        JSTRT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Regular channel Start flag of ADC 1
    #[inline(always)]
    pub fn strt1(&self) -> STRT1_R {
        STRT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Overrun flag of ADC 1
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("ovr1", &self.ovr1())
            .field("strt1", &self.strt1())
            .field("jstrt1", &self.jstrt1())
            .field("jeoc1", &self.jeoc1())
            .field("eoc1", &self.eoc1())
            .field("awd1", &self.awd1())
            .finish()
    }
}
/**ADC Common status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#ADC_Common:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
