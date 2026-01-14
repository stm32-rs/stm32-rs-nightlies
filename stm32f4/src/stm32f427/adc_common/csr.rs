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
///Field `AWD2` reader - Analog watchdog flag of ADC 2
pub use AWD1_R as AWD2_R;
///Field `AWD3` reader - Analog watchdog flag of ADC 3
pub use AWD1_R as AWD3_R;
///Field `EOC2` reader - End of conversion of ADC 2
pub use EOC1_R as EOC2_R;
///Field `EOC3` reader - End of conversion of ADC 3
pub use EOC1_R as EOC3_R;
///Field `JEOC2` reader - Injected channel end of conversion of ADC 2
pub use JEOC1_R as JEOC2_R;
///Field `JEOC3` reader - Injected channel end of conversion of ADC 3
pub use JEOC1_R as JEOC3_R;
///Field `JSTRT2` reader - Injected channel Start flag of ADC 2
pub use JSTRT1_R as JSTRT2_R;
///Field `JSTRT3` reader - Injected channel Start flag of ADC 3
pub use JSTRT1_R as JSTRT3_R;
///Field `OVR2` reader - Overrun flag of ADC 2
pub use OVR1_R as OVR2_R;
///Field `OVR3` reader - Overrun flag of ADC3
pub use OVR1_R as OVR3_R;
///Field `STRT2` reader - Regular channel Start flag of ADC 2
pub use STRT1_R as STRT2_R;
///Field `STRT3` reader - Regular channel Start flag of ADC 3
pub use STRT1_R as STRT3_R;
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
    ///Bit 8 - Analog watchdog flag of ADC 2
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - End of conversion of ADC 2
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R {
        EOC2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Injected channel end of conversion of ADC 2
    #[inline(always)]
    pub fn jeoc2(&self) -> JEOC2_R {
        JEOC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Injected channel Start flag of ADC 2
    #[inline(always)]
    pub fn jstrt2(&self) -> JSTRT2_R {
        JSTRT2_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Regular channel Start flag of ADC 2
    #[inline(always)]
    pub fn strt2(&self) -> STRT2_R {
        STRT2_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Overrun flag of ADC 2
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Analog watchdog flag of ADC 3
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - End of conversion of ADC 3
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R {
        EOC3_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Injected channel end of conversion of ADC 3
    #[inline(always)]
    pub fn jeoc3(&self) -> JEOC3_R {
        JEOC3_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Injected channel Start flag of ADC 3
    #[inline(always)]
    pub fn jstrt3(&self) -> JSTRT3_R {
        JSTRT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Regular channel Start flag of ADC 3
    #[inline(always)]
    pub fn strt3(&self) -> STRT3_R {
        STRT3_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Overrun flag of ADC3
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("ovr1", &self.ovr1())
            .field("ovr3", &self.ovr3())
            .field("strt1", &self.strt1())
            .field("strt3", &self.strt3())
            .field("jstrt1", &self.jstrt1())
            .field("jstrt3", &self.jstrt3())
            .field("jeoc1", &self.jeoc1())
            .field("jeoc3", &self.jeoc3())
            .field("eoc1", &self.eoc1())
            .field("eoc3", &self.eoc3())
            .field("awd1", &self.awd1())
            .field("awd3", &self.awd3())
            .field("ovr2", &self.ovr2())
            .field("strt2", &self.strt2())
            .field("jstrt2", &self.jstrt2())
            .field("jeoc2", &self.jeoc2())
            .field("eoc2", &self.eoc2())
            .field("awd2", &self.awd2())
            .finish()
    }
}
/**ADC Common status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F427.html#ADC_Common:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
