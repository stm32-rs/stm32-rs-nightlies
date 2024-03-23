#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Analog watchdog flag of ADC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1 {
    #[doc = "0: No analog watchdog event occurred"]
    NoEvent = 0,
    #[doc = "1: Analog watchdog event occurred"]
    Event = 1,
}
impl From<AWD1> for bool {
    #[inline(always)]
    fn from(variant: AWD1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1` reader - Analog watchdog flag of ADC1"]
pub type AWD1_R = crate::BitReader<AWD1>;
impl AWD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1 {
        match self.bits {
            false => AWD1::NoEvent,
            true => AWD1::Event,
        }
    }
    #[doc = "No analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1::NoEvent
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1::Event
    }
}
#[doc = "End of conversion of ADC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOC1 {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion complete"]
    Complete = 1,
}
impl From<EOC1> for bool {
    #[inline(always)]
    fn from(variant: EOC1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC1` reader - End of conversion of ADC1"]
pub type EOC1_R = crate::BitReader<EOC1>;
impl EOC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOC1 {
        match self.bits {
            false => EOC1::NotComplete,
            true => EOC1::Complete,
        }
    }
    #[doc = "Conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC1::NotComplete
    }
    #[doc = "Conversion complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC1::Complete
    }
}
#[doc = "Injected channel end of conversion of ADC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOC1 {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion complete"]
    Complete = 1,
}
impl From<JEOC1> for bool {
    #[inline(always)]
    fn from(variant: JEOC1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOC1` reader - Injected channel end of conversion of ADC1"]
pub type JEOC1_R = crate::BitReader<JEOC1>;
impl JEOC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEOC1 {
        match self.bits {
            false => JEOC1::NotComplete,
            true => JEOC1::Complete,
        }
    }
    #[doc = "Conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC1::NotComplete
    }
    #[doc = "Conversion complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC1::Complete
    }
}
#[doc = "Injected channel Start flag of ADC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSTRT1 {
    #[doc = "0: No injected channel conversion started"]
    NotStarted = 0,
    #[doc = "1: Injected channel conversion has started"]
    Started = 1,
}
impl From<JSTRT1> for bool {
    #[inline(always)]
    fn from(variant: JSTRT1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JSTRT1` reader - Injected channel Start flag of ADC1"]
pub type JSTRT1_R = crate::BitReader<JSTRT1>;
impl JSTRT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JSTRT1 {
        match self.bits {
            false => JSTRT1::NotStarted,
            true => JSTRT1::Started,
        }
    }
    #[doc = "No injected channel conversion started"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRT1::NotStarted
    }
    #[doc = "Injected channel conversion has started"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSTRT1::Started
    }
}
#[doc = "Regular channel Start flag of ADC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRT1 {
    #[doc = "0: No regular channel conversion started"]
    NotStarted = 0,
    #[doc = "1: Regular channel conversion has started"]
    Started = 1,
}
impl From<STRT1> for bool {
    #[inline(always)]
    fn from(variant: STRT1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT1` reader - Regular channel Start flag of ADC1"]
pub type STRT1_R = crate::BitReader<STRT1>;
impl STRT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STRT1 {
        match self.bits {
            false => STRT1::NotStarted,
            true => STRT1::Started,
        }
    }
    #[doc = "No regular channel conversion started"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRT1::NotStarted
    }
    #[doc = "Regular channel conversion has started"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRT1::Started
    }
}
#[doc = "Overrun flag of ADC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR1 {
    #[doc = "0: No overrun occurred"]
    NoOverrun = 0,
    #[doc = "1: Overrun occurred"]
    Overrun = 1,
}
impl From<OVR1> for bool {
    #[inline(always)]
    fn from(variant: OVR1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR1` reader - Overrun flag of ADC1"]
pub type OVR1_R = crate::BitReader<OVR1>;
impl OVR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR1 {
        match self.bits {
            false => OVR1::NoOverrun,
            true => OVR1::Overrun,
        }
    }
    #[doc = "No overrun occurred"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR1::NoOverrun
    }
    #[doc = "Overrun occurred"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR1::Overrun
    }
}
#[doc = "Field `AWD2` reader - Analog watchdog flag of ADC2"]
pub use AWD1_R as AWD2_R;
#[doc = "Field `AWD3` reader - Analog watchdog flag of ADC3"]
pub use AWD1_R as AWD3_R;
#[doc = "Field `EOC2` reader - End of conversion of ADC2"]
pub use EOC1_R as EOC2_R;
#[doc = "Field `EOC3` reader - End of conversion of ADC3"]
pub use EOC1_R as EOC3_R;
#[doc = "Field `JEOC2` reader - Injected channel end of conversion of ADC2"]
pub use JEOC1_R as JEOC2_R;
#[doc = "Field `JEOC3` reader - Injected channel end of conversion of ADC3"]
pub use JEOC1_R as JEOC3_R;
#[doc = "Field `JSTRT2` reader - Injected channel Start flag of ADC2"]
pub use JSTRT1_R as JSTRT2_R;
#[doc = "Field `JSTRT3` reader - Injected channel Start flag of ADC3"]
pub use JSTRT1_R as JSTRT3_R;
#[doc = "Field `OVR2` reader - Overrun flag of ADC2"]
pub use OVR1_R as OVR2_R;
#[doc = "Field `OVR3` reader - Overrun flag of ADC3"]
pub use OVR1_R as OVR3_R;
#[doc = "Field `STRT2` reader - Regular channel Start flag of ADC2"]
pub use STRT1_R as STRT2_R;
#[doc = "Field `STRT3` reader - Regular channel Start flag of ADC3"]
pub use STRT1_R as STRT3_R;
impl R {
    #[doc = "Bit 0 - Analog watchdog flag of ADC1"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of conversion of ADC1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion of ADC1"]
    #[inline(always)]
    pub fn jeoc1(&self) -> JEOC1_R {
        JEOC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Injected channel Start flag of ADC1"]
    #[inline(always)]
    pub fn jstrt1(&self) -> JSTRT1_R {
        JSTRT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Regular channel Start flag of ADC1"]
    #[inline(always)]
    pub fn strt1(&self) -> STRT1_R {
        STRT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun flag of ADC1"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog flag of ADC2"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - End of conversion of ADC2"]
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R {
        EOC2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Injected channel end of conversion of ADC2"]
    #[inline(always)]
    pub fn jeoc2(&self) -> JEOC2_R {
        JEOC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Injected channel Start flag of ADC2"]
    #[inline(always)]
    pub fn jstrt2(&self) -> JSTRT2_R {
        JSTRT2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Regular channel Start flag of ADC2"]
    #[inline(always)]
    pub fn strt2(&self) -> STRT2_R {
        STRT2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Overrun flag of ADC2"]
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog watchdog flag of ADC3"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - End of conversion of ADC3"]
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R {
        EOC3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Injected channel end of conversion of ADC3"]
    #[inline(always)]
    pub fn jeoc3(&self) -> JEOC3_R {
        JEOC3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Injected channel Start flag of ADC3"]
    #[inline(always)]
    pub fn jstrt3(&self) -> JSTRT3_R {
        JSTRT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Regular channel Start flag of ADC3"]
    #[inline(always)]
    pub fn strt3(&self) -> STRT3_R {
        STRT3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Overrun flag of ADC3"]
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "ADC common status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0;
}
