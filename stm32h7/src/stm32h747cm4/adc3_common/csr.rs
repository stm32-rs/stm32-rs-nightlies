#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Master ADC ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDY_MST {
    #[doc = "0: ADC is not ready to start conversion"]
    NotReady = 0,
    #[doc = "1: ADC is ready to start conversion"]
    Ready = 1,
}
impl From<ADRDY_MST> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_MST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDY_MST` reader - Master ADC ready"]
pub type ADRDY_MST_R = crate::BitReader<ADRDY_MST>;
impl ADRDY_MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADRDY_MST {
        match self.bits {
            false => ADRDY_MST::NotReady,
            true => ADRDY_MST::Ready,
        }
    }
    #[doc = "ADC is not ready to start conversion"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDY_MST::NotReady
    }
    #[doc = "ADC is ready to start conversion"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDY_MST::Ready
    }
}
#[doc = "End of Sampling phase flag of the master ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMP_MST {
    #[doc = "0: End of sampling phase no yet reached"]
    NotEnded = 0,
    #[doc = "1: End of sampling phase reached"]
    Ended = 1,
}
impl From<EOSMP_MST> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_MST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMP_MST` reader - End of Sampling phase flag of the master ADC"]
pub type EOSMP_MST_R = crate::BitReader<EOSMP_MST>;
impl EOSMP_MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOSMP_MST {
        match self.bits {
            false => EOSMP_MST::NotEnded,
            true => EOSMP_MST::Ended,
        }
    }
    #[doc = "End of sampling phase no yet reached"]
    #[inline(always)]
    pub fn is_not_ended(&self) -> bool {
        *self == EOSMP_MST::NotEnded
    }
    #[doc = "End of sampling phase reached"]
    #[inline(always)]
    pub fn is_ended(&self) -> bool {
        *self == EOSMP_MST::Ended
    }
}
#[doc = "End of regular conversion of the master ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOC_MST {
    #[doc = "0: Regular conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Regular conversion complete"]
    Complete = 1,
}
impl From<EOC_MST> for bool {
    #[inline(always)]
    fn from(variant: EOC_MST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC_MST` reader - End of regular conversion of the master ADC"]
pub type EOC_MST_R = crate::BitReader<EOC_MST>;
impl EOC_MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOC_MST {
        match self.bits {
            false => EOC_MST::NotComplete,
            true => EOC_MST::Complete,
        }
    }
    #[doc = "Regular conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC_MST::NotComplete
    }
    #[doc = "Regular conversion complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC_MST::Complete
    }
}
#[doc = "End of regular sequence flag of the master ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOS_MST {
    #[doc = "0: Regular sequence is not complete"]
    NotComplete = 0,
    #[doc = "1: Regular sequence complete"]
    Complete = 1,
}
impl From<EOS_MST> for bool {
    #[inline(always)]
    fn from(variant: EOS_MST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOS_MST` reader - End of regular sequence flag of the master ADC"]
pub type EOS_MST_R = crate::BitReader<EOS_MST>;
impl EOS_MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOS_MST {
        match self.bits {
            false => EOS_MST::NotComplete,
            true => EOS_MST::Complete,
        }
    }
    #[doc = "Regular sequence is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOS_MST::NotComplete
    }
    #[doc = "Regular sequence complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOS_MST::Complete
    }
}
#[doc = "Overrun flag of the master ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_MST {
    #[doc = "0: No overrun occurred"]
    NoOverrun = 0,
    #[doc = "1: Overrun occurred"]
    Overrun = 1,
}
impl From<OVR_MST> for bool {
    #[inline(always)]
    fn from(variant: OVR_MST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR_MST` reader - Overrun flag of the master ADC"]
pub type OVR_MST_R = crate::BitReader<OVR_MST>;
impl OVR_MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVR_MST {
        match self.bits {
            false => OVR_MST::NoOverrun,
            true => OVR_MST::Overrun,
        }
    }
    #[doc = "No overrun occurred"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_MST::NoOverrun
    }
    #[doc = "Overrun occurred"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_MST::Overrun
    }
}
#[doc = "End of injected conversion flag of the master ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOC_MST {
    #[doc = "0: Injected conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Injected conversion complete"]
    Complete = 1,
}
impl From<JEOC_MST> for bool {
    #[inline(always)]
    fn from(variant: JEOC_MST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOC_MST` reader - End of injected conversion flag of the master ADC"]
pub type JEOC_MST_R = crate::BitReader<JEOC_MST>;
impl JEOC_MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEOC_MST {
        match self.bits {
            false => JEOC_MST::NotComplete,
            true => JEOC_MST::Complete,
        }
    }
    #[doc = "Injected conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC_MST::NotComplete
    }
    #[doc = "Injected conversion complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC_MST::Complete
    }
}
#[doc = "End of injected sequence flag of the master ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOS_MST {
    #[doc = "0: Injected sequence is not complete"]
    NotComplete = 0,
    #[doc = "1: Injected sequence complete"]
    Complete = 1,
}
impl From<JEOS_MST> for bool {
    #[inline(always)]
    fn from(variant: JEOS_MST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOS_MST` reader - End of injected sequence flag of the master ADC"]
pub type JEOS_MST_R = crate::BitReader<JEOS_MST>;
impl JEOS_MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEOS_MST {
        match self.bits {
            false => JEOS_MST::NotComplete,
            true => JEOS_MST::Complete,
        }
    }
    #[doc = "Injected sequence is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOS_MST::NotComplete
    }
    #[doc = "Injected sequence complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOS_MST::Complete
    }
}
#[doc = "Analog watchdog 1 flag of the master ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1_MST {
    #[doc = "0: No analog watchdog event occurred"]
    NoEvent = 0,
    #[doc = "1: Analog watchdog event occurred"]
    Event = 1,
}
impl From<AWD1_MST> for bool {
    #[inline(always)]
    fn from(variant: AWD1_MST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1_MST` reader - Analog watchdog 1 flag of the master ADC"]
pub type AWD1_MST_R = crate::BitReader<AWD1_MST>;
impl AWD1_MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AWD1_MST {
        match self.bits {
            false => AWD1_MST::NoEvent,
            true => AWD1_MST::Event,
        }
    }
    #[doc = "No analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1_MST::NoEvent
    }
    #[doc = "Analog watchdog event occurred"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1_MST::Event
    }
}
#[doc = "Field `AWD2_MST` reader - Analog watchdog 2 flag of the master ADC"]
pub use AWD1_MST_R as AWD2_MST_R;
#[doc = "Field `AWD3_MST` reader - Analog watchdog 3 flag of the master ADC"]
pub use AWD1_MST_R as AWD3_MST_R;
#[doc = "Injected Context Queue Overflow flag of the master ADC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVF_MST {
    #[doc = "0: No injected context queue overflow has occurred"]
    NoOverflow = 0,
    #[doc = "1: Injected context queue overflow has occurred"]
    Overflow = 1,
}
impl From<JQOVF_MST> for bool {
    #[inline(always)]
    fn from(variant: JQOVF_MST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQOVF_MST` reader - Injected Context Queue Overflow flag of the master ADC"]
pub type JQOVF_MST_R = crate::BitReader<JQOVF_MST>;
impl JQOVF_MST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JQOVF_MST {
        match self.bits {
            false => JQOVF_MST::NoOverflow,
            true => JQOVF_MST::Overflow,
        }
    }
    #[doc = "No injected context queue overflow has occurred"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == JQOVF_MST::NoOverflow
    }
    #[doc = "Injected context queue overflow has occurred"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == JQOVF_MST::Overflow
    }
}
#[doc = "Field `ADRDY_SLV` reader - Slave ADC ready"]
pub use ADRDY_MST_R as ADRDY_SLV_R;
#[doc = "Field `AWD1_SLV` reader - Analog watchdog 1 flag of the slave ADC"]
pub use AWD1_MST_R as AWD1_SLV_R;
#[doc = "Field `AWD2_SLV` reader - Analog watchdog 2 flag of the slave ADC"]
pub use AWD1_MST_R as AWD2_SLV_R;
#[doc = "Field `AWD3_SLV` reader - Analog watchdog 3 flag of the slave ADC"]
pub use AWD1_MST_R as AWD3_SLV_R;
#[doc = "Field `EOC_SLV` reader - End of regular conversion of the slave ADC"]
pub use EOC_MST_R as EOC_SLV_R;
#[doc = "Field `EOSMP_SLV` reader - End of Sampling phase flag of the slave ADC"]
pub use EOSMP_MST_R as EOSMP_SLV_R;
#[doc = "Field `EOS_SLV` reader - End of regular sequence flag of the slave ADC"]
pub use EOS_MST_R as EOS_SLV_R;
#[doc = "Field `JEOC_SLV` reader - End of injected conversion flag of the slave ADC"]
pub use JEOC_MST_R as JEOC_SLV_R;
#[doc = "Field `JEOS_SLV` reader - End of injected sequence flag of the slave ADC"]
pub use JEOS_MST_R as JEOS_SLV_R;
#[doc = "Field `JQOVF_SLV` reader - Injected Context Queue Overflow flag of the slave ADC"]
pub use JQOVF_MST_R as JQOVF_SLV_R;
#[doc = "Field `OVR_SLV` reader - Overrun flag of the slave ADC"]
pub use OVR_MST_R as OVR_SLV_R;
impl R {
    #[doc = "Bit 0 - Master ADC ready"]
    #[inline(always)]
    pub fn adrdy_mst(&self) -> ADRDY_MST_R {
        ADRDY_MST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Sampling phase flag of the master ADC"]
    #[inline(always)]
    pub fn eosmp_mst(&self) -> EOSMP_MST_R {
        EOSMP_MST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of regular conversion of the master ADC"]
    #[inline(always)]
    pub fn eoc_mst(&self) -> EOC_MST_R {
        EOC_MST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of regular sequence flag of the master ADC"]
    #[inline(always)]
    pub fn eos_mst(&self) -> EOS_MST_R {
        EOS_MST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overrun flag of the master ADC"]
    #[inline(always)]
    pub fn ovr_mst(&self) -> OVR_MST_R {
        OVR_MST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of injected conversion flag of the master ADC"]
    #[inline(always)]
    pub fn jeoc_mst(&self) -> JEOC_MST_R {
        JEOC_MST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of injected sequence flag of the master ADC"]
    #[inline(always)]
    pub fn jeos_mst(&self) -> JEOS_MST_R {
        JEOS_MST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag of the master ADC"]
    #[inline(always)]
    pub fn awd1_mst(&self) -> AWD1_MST_R {
        AWD1_MST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag of the master ADC"]
    #[inline(always)]
    pub fn awd2_mst(&self) -> AWD2_MST_R {
        AWD2_MST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag of the master ADC"]
    #[inline(always)]
    pub fn awd3_mst(&self) -> AWD3_MST_R {
        AWD3_MST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Injected Context Queue Overflow flag of the master ADC"]
    #[inline(always)]
    pub fn jqovf_mst(&self) -> JQOVF_MST_R {
        JQOVF_MST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave ADC ready"]
    #[inline(always)]
    pub fn adrdy_slv(&self) -> ADRDY_SLV_R {
        ADRDY_SLV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - End of Sampling phase flag of the slave ADC"]
    #[inline(always)]
    pub fn eosmp_slv(&self) -> EOSMP_SLV_R {
        EOSMP_SLV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - End of regular conversion of the slave ADC"]
    #[inline(always)]
    pub fn eoc_slv(&self) -> EOC_SLV_R {
        EOC_SLV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - End of regular sequence flag of the slave ADC"]
    #[inline(always)]
    pub fn eos_slv(&self) -> EOS_SLV_R {
        EOS_SLV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Overrun flag of the slave ADC"]
    #[inline(always)]
    pub fn ovr_slv(&self) -> OVR_SLV_R {
        OVR_SLV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - End of injected conversion flag of the slave ADC"]
    #[inline(always)]
    pub fn jeoc_slv(&self) -> JEOC_SLV_R {
        JEOC_SLV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - End of injected sequence flag of the slave ADC"]
    #[inline(always)]
    pub fn jeos_slv(&self) -> JEOS_SLV_R {
        JEOS_SLV_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Analog watchdog 1 flag of the slave ADC"]
    #[inline(always)]
    pub fn awd1_slv(&self) -> AWD1_SLV_R {
        AWD1_SLV_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Analog watchdog 2 flag of the slave ADC"]
    #[inline(always)]
    pub fn awd2_slv(&self) -> AWD2_SLV_R {
        AWD2_SLV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Analog watchdog 3 flag of the slave ADC"]
    #[inline(always)]
    pub fn awd3_slv(&self) -> AWD3_SLV_R {
        AWD3_SLV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Injected Context Queue Overflow flag of the slave ADC"]
    #[inline(always)]
    pub fn jqovf_slv(&self) -> JQOVF_SLV_R {
        JQOVF_SLV_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "ADC Common status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
