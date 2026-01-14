///Register `CSR` reader
pub type R = crate::R<CSRrs>;
/**Master ADC ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDY_MST {
    ///0: ADC is not ready to start conversion
    NotReady = 0,
    ///1: ADC is ready to start conversion
    Ready = 1,
}
impl From<ADRDY_MST> for bool {
    #[inline(always)]
    fn from(variant: ADRDY_MST) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY_MST` reader - Master ADC ready
pub type ADRDY_MST_R = crate::BitReader<ADRDY_MST>;
impl ADRDY_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADRDY_MST {
        match self.bits {
            false => ADRDY_MST::NotReady,
            true => ADRDY_MST::Ready,
        }
    }
    ///ADC is not ready to start conversion
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDY_MST::NotReady
    }
    ///ADC is ready to start conversion
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDY_MST::Ready
    }
}
/**End of Sampling phase flag of the master ADC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMP_MST {
    ///0: End of sampling phase no yet reached
    NotEnded = 0,
    ///1: End of sampling phase reached
    Ended = 1,
}
impl From<EOSMP_MST> for bool {
    #[inline(always)]
    fn from(variant: EOSMP_MST) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP_MST` reader - End of Sampling phase flag of the master ADC
pub type EOSMP_MST_R = crate::BitReader<EOSMP_MST>;
impl EOSMP_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSMP_MST {
        match self.bits {
            false => EOSMP_MST::NotEnded,
            true => EOSMP_MST::Ended,
        }
    }
    ///End of sampling phase no yet reached
    #[inline(always)]
    pub fn is_not_ended(&self) -> bool {
        *self == EOSMP_MST::NotEnded
    }
    ///End of sampling phase reached
    #[inline(always)]
    pub fn is_ended(&self) -> bool {
        *self == EOSMP_MST::Ended
    }
}
/**End of regular conversion of the master ADC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOC_MST {
    ///0: Regular conversion is not complete
    NotComplete = 0,
    ///1: Regular conversion complete
    Complete = 1,
}
impl From<EOC_MST> for bool {
    #[inline(always)]
    fn from(variant: EOC_MST) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC_MST` reader - End of regular conversion of the master ADC
pub type EOC_MST_R = crate::BitReader<EOC_MST>;
impl EOC_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOC_MST {
        match self.bits {
            false => EOC_MST::NotComplete,
            true => EOC_MST::Complete,
        }
    }
    ///Regular conversion is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC_MST::NotComplete
    }
    ///Regular conversion complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC_MST::Complete
    }
}
/**End of regular sequence flag of the master ADC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOS_MST {
    ///0: Regular sequence is not complete
    NotComplete = 0,
    ///1: Regular sequence complete
    Complete = 1,
}
impl From<EOS_MST> for bool {
    #[inline(always)]
    fn from(variant: EOS_MST) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS_MST` reader - End of regular sequence flag of the master ADC
pub type EOS_MST_R = crate::BitReader<EOS_MST>;
impl EOS_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOS_MST {
        match self.bits {
            false => EOS_MST::NotComplete,
            true => EOS_MST::Complete,
        }
    }
    ///Regular sequence is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOS_MST::NotComplete
    }
    ///Regular sequence complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOS_MST::Complete
    }
}
/**Overrun flag of the master ADC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_MST {
    ///0: No overrun occurred
    NoOverrun = 0,
    ///1: Overrun occurred
    Overrun = 1,
}
impl From<OVR_MST> for bool {
    #[inline(always)]
    fn from(variant: OVR_MST) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR_MST` reader - Overrun flag of the master ADC
pub type OVR_MST_R = crate::BitReader<OVR_MST>;
impl OVR_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVR_MST {
        match self.bits {
            false => OVR_MST::NoOverrun,
            true => OVR_MST::Overrun,
        }
    }
    ///No overrun occurred
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_MST::NoOverrun
    }
    ///Overrun occurred
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_MST::Overrun
    }
}
/**End of injected conversion flag of the master ADC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOC_MST {
    ///0: Injected conversion is not complete
    NotComplete = 0,
    ///1: Injected conversion complete
    Complete = 1,
}
impl From<JEOC_MST> for bool {
    #[inline(always)]
    fn from(variant: JEOC_MST) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOC_MST` reader - End of injected conversion flag of the master ADC
pub type JEOC_MST_R = crate::BitReader<JEOC_MST>;
impl JEOC_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEOC_MST {
        match self.bits {
            false => JEOC_MST::NotComplete,
            true => JEOC_MST::Complete,
        }
    }
    ///Injected conversion is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC_MST::NotComplete
    }
    ///Injected conversion complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC_MST::Complete
    }
}
/**End of injected sequence flag of the master ADC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOS_MST {
    ///0: Injected sequence is not complete
    NotComplete = 0,
    ///1: Injected sequence complete
    Complete = 1,
}
impl From<JEOS_MST> for bool {
    #[inline(always)]
    fn from(variant: JEOS_MST) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOS_MST` reader - End of injected sequence flag of the master ADC
pub type JEOS_MST_R = crate::BitReader<JEOS_MST>;
impl JEOS_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEOS_MST {
        match self.bits {
            false => JEOS_MST::NotComplete,
            true => JEOS_MST::Complete,
        }
    }
    ///Injected sequence is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOS_MST::NotComplete
    }
    ///Injected sequence complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOS_MST::Complete
    }
}
/**Analog watchdog 1 flag of the master ADC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1_MST {
    ///0: No analog watchdog event occurred
    NoEvent = 0,
    ///1: Analog watchdog event occurred
    Event = 1,
}
impl From<AWD1_MST> for bool {
    #[inline(always)]
    fn from(variant: AWD1_MST) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD1_MST` reader - Analog watchdog 1 flag of the master ADC
pub type AWD1_MST_R = crate::BitReader<AWD1_MST>;
impl AWD1_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1_MST {
        match self.bits {
            false => AWD1_MST::NoEvent,
            true => AWD1_MST::Event,
        }
    }
    ///No analog watchdog event occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1_MST::NoEvent
    }
    ///Analog watchdog event occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1_MST::Event
    }
}
///Field `AWD2_MST` reader - Analog watchdog 2 flag of the master ADC
pub use AWD1_MST_R as AWD2_MST_R;
///Field `AWD3_MST` reader - Analog watchdog 3 flag of the master ADC
pub use AWD1_MST_R as AWD3_MST_R;
/**Injected Context Queue Overflow flag of the master ADC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVF_MST {
    ///0: No injected context queue overflow has occurred
    NoOverflow = 0,
    ///1: Injected context queue overflow has occurred
    Overflow = 1,
}
impl From<JQOVF_MST> for bool {
    #[inline(always)]
    fn from(variant: JQOVF_MST) -> Self {
        variant as u8 != 0
    }
}
///Field `JQOVF_MST` reader - Injected Context Queue Overflow flag of the master ADC
pub type JQOVF_MST_R = crate::BitReader<JQOVF_MST>;
impl JQOVF_MST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JQOVF_MST {
        match self.bits {
            false => JQOVF_MST::NoOverflow,
            true => JQOVF_MST::Overflow,
        }
    }
    ///No injected context queue overflow has occurred
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == JQOVF_MST::NoOverflow
    }
    ///Injected context queue overflow has occurred
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == JQOVF_MST::Overflow
    }
}
///Field `ADRDY_SLV` reader - Slave ADC ready
pub use ADRDY_MST_R as ADRDY_SLV_R;
///Field `AWD1_SLV` reader - Analog watchdog 1 flag of the slave ADC
pub use AWD1_MST_R as AWD1_SLV_R;
///Field `AWD2_SLV` reader - Analog watchdog 2 flag of the slave ADC
pub use AWD1_MST_R as AWD2_SLV_R;
///Field `AWD3_SLV` reader - Analog watchdog 3 flag of the slave ADC
pub use AWD1_MST_R as AWD3_SLV_R;
///Field `EOC_SLV` reader - End of regular conversion of the slave ADC
pub use EOC_MST_R as EOC_SLV_R;
///Field `EOSMP_SLV` reader - End of Sampling phase flag of the slave ADC
pub use EOSMP_MST_R as EOSMP_SLV_R;
///Field `EOS_SLV` reader - End of regular sequence flag of the slave ADC
pub use EOS_MST_R as EOS_SLV_R;
///Field `JEOC_SLV` reader - End of injected conversion flag of the slave ADC
pub use JEOC_MST_R as JEOC_SLV_R;
///Field `JEOS_SLV` reader - End of injected sequence flag of the slave ADC
pub use JEOS_MST_R as JEOS_SLV_R;
///Field `JQOVF_SLV` reader - Injected Context Queue Overflow flag of the slave ADC
pub use JQOVF_MST_R as JQOVF_SLV_R;
///Field `OVR_SLV` reader - Overrun flag of the slave ADC
pub use OVR_MST_R as OVR_SLV_R;
impl R {
    ///Bit 0 - Master ADC ready
    #[inline(always)]
    pub fn adrdy_mst(&self) -> ADRDY_MST_R {
        ADRDY_MST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of Sampling phase flag of the master ADC
    #[inline(always)]
    pub fn eosmp_mst(&self) -> EOSMP_MST_R {
        EOSMP_MST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of regular conversion of the master ADC
    #[inline(always)]
    pub fn eoc_mst(&self) -> EOC_MST_R {
        EOC_MST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of regular sequence flag of the master ADC
    #[inline(always)]
    pub fn eos_mst(&self) -> EOS_MST_R {
        EOS_MST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Overrun flag of the master ADC
    #[inline(always)]
    pub fn ovr_mst(&self) -> OVR_MST_R {
        OVR_MST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - End of injected conversion flag of the master ADC
    #[inline(always)]
    pub fn jeoc_mst(&self) -> JEOC_MST_R {
        JEOC_MST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - End of injected sequence flag of the master ADC
    #[inline(always)]
    pub fn jeos_mst(&self) -> JEOS_MST_R {
        JEOS_MST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog 1 flag of the master ADC
    #[inline(always)]
    pub fn awd1_mst(&self) -> AWD1_MST_R {
        AWD1_MST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 flag of the master ADC
    #[inline(always)]
    pub fn awd2_mst(&self) -> AWD2_MST_R {
        AWD2_MST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 flag of the master ADC
    #[inline(always)]
    pub fn awd3_mst(&self) -> AWD3_MST_R {
        AWD3_MST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Injected Context Queue Overflow flag of the master ADC
    #[inline(always)]
    pub fn jqovf_mst(&self) -> JQOVF_MST_R {
        JQOVF_MST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - Slave ADC ready
    #[inline(always)]
    pub fn adrdy_slv(&self) -> ADRDY_SLV_R {
        ADRDY_SLV_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - End of Sampling phase flag of the slave ADC
    #[inline(always)]
    pub fn eosmp_slv(&self) -> EOSMP_SLV_R {
        EOSMP_SLV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - End of regular conversion of the slave ADC
    #[inline(always)]
    pub fn eoc_slv(&self) -> EOC_SLV_R {
        EOC_SLV_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - End of regular sequence flag of the slave ADC
    #[inline(always)]
    pub fn eos_slv(&self) -> EOS_SLV_R {
        EOS_SLV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Overrun flag of the slave ADC
    #[inline(always)]
    pub fn ovr_slv(&self) -> OVR_SLV_R {
        OVR_SLV_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - End of injected conversion flag of the slave ADC
    #[inline(always)]
    pub fn jeoc_slv(&self) -> JEOC_SLV_R {
        JEOC_SLV_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - End of injected sequence flag of the slave ADC
    #[inline(always)]
    pub fn jeos_slv(&self) -> JEOS_SLV_R {
        JEOS_SLV_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Analog watchdog 1 flag of the slave ADC
    #[inline(always)]
    pub fn awd1_slv(&self) -> AWD1_SLV_R {
        AWD1_SLV_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Analog watchdog 2 flag of the slave ADC
    #[inline(always)]
    pub fn awd2_slv(&self) -> AWD2_SLV_R {
        AWD2_SLV_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Analog watchdog 3 flag of the slave ADC
    #[inline(always)]
    pub fn awd3_slv(&self) -> AWD3_SLV_R {
        AWD3_SLV_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Injected Context Queue Overflow flag of the slave ADC
    #[inline(always)]
    pub fn jqovf_slv(&self) -> JQOVF_SLV_R {
        JQOVF_SLV_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("adrdy_mst", &self.adrdy_mst())
            .field("eosmp_mst", &self.eosmp_mst())
            .field("eoc_mst", &self.eoc_mst())
            .field("eos_mst", &self.eos_mst())
            .field("ovr_mst", &self.ovr_mst())
            .field("jeoc_mst", &self.jeoc_mst())
            .field("jeos_mst", &self.jeos_mst())
            .field("awd1_mst", &self.awd1_mst())
            .field("awd2_mst", &self.awd2_mst())
            .field("awd3_mst", &self.awd3_mst())
            .field("jqovf_mst", &self.jqovf_mst())
            .field("adrdy_slv", &self.adrdy_slv())
            .field("eosmp_slv", &self.eosmp_slv())
            .field("eoc_slv", &self.eoc_slv())
            .field("eos_slv", &self.eos_slv())
            .field("ovr_slv", &self.ovr_slv())
            .field("jeoc_slv", &self.jeoc_slv())
            .field("jeos_slv", &self.jeos_slv())
            .field("awd1_slv", &self.awd1_slv())
            .field("awd2_slv", &self.awd2_slv())
            .field("awd3_slv", &self.awd3_slv())
            .field("jqovf_slv", &self.jqovf_slv())
            .finish()
    }
}
/**ADC Common status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#ADC3_Common:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSRrs {}
