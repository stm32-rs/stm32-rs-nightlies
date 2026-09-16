///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**End of injected conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxJDATAR.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCF {
    ///0: No injected conversion has completed
    Clear = 0,
    ///1: An injected conversion has completed and its data may be read
    Set = 1,
}
impl From<JEOCF> for bool {
    #[inline(always)]
    fn from(variant: JEOCF) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOCF` reader - End of injected conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxJDATAR.
pub type JEOCF_R = crate::BitReader<JEOCF>;
impl JEOCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEOCF {
        match self.bits {
            false => JEOCF::Clear,
            true => JEOCF::Set,
        }
    }
    ///No injected conversion has completed
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == JEOCF::Clear
    }
    ///An injected conversion has completed and its data may be read
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == JEOCF::Set
    }
}
/**End of regular conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxRDATAR.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REOCF {
    ///0: No regular conversion has completed
    Clear = 0,
    ///1: A regular conversion has completed and its data may be read
    Set = 1,
}
impl From<REOCF> for bool {
    #[inline(always)]
    fn from(variant: REOCF) -> Self {
        variant as u8 != 0
    }
}
///Field `REOCF` reader - End of regular conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxRDATAR.
pub type REOCF_R = crate::BitReader<REOCF>;
impl REOCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REOCF {
        match self.bits {
            false => REOCF::Clear,
            true => REOCF::Set,
        }
    }
    ///No regular conversion has completed
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == REOCF::Clear
    }
    ///A regular conversion has completed and its data may be read
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == REOCF::Set
    }
}
/**Injected conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRJOVRF bit in the DFSDM_FLTxICR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVRF {
    ///0: No injected conversion overrun has occurred
    Clear = 0,
    ///1: An injected conversion overrun has occurred, which means that an injected conversion finished while JEOCF was already ‘1’. JDATAR is not affected by overruns
    Set = 1,
}
impl From<JOVRF> for bool {
    #[inline(always)]
    fn from(variant: JOVRF) -> Self {
        variant as u8 != 0
    }
}
///Field `JOVRF` reader - Injected conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRJOVRF bit in the DFSDM_FLTxICR register.
pub type JOVRF_R = crate::BitReader<JOVRF>;
impl JOVRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JOVRF {
        match self.bits {
            false => JOVRF::Clear,
            true => JOVRF::Set,
        }
    }
    ///No injected conversion overrun has occurred
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == JOVRF::Clear
    }
    ///An injected conversion overrun has occurred, which means that an injected conversion finished while JEOCF was already ‘1’. JDATAR is not affected by overruns
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == JOVRF::Set
    }
}
/**Regular conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRROVRF bit in the DFSDM_FLTxICR register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVRF {
    ///0: No regular conversion overrun has occurred
    Clear = 0,
    ///1: A regular conversion overrun has occurred, which means that a regular conversion finished while REOCF was already ‘1’. RDATAR is not affected by overruns
    Set = 1,
}
impl From<ROVRF> for bool {
    #[inline(always)]
    fn from(variant: ROVRF) -> Self {
        variant as u8 != 0
    }
}
///Field `ROVRF` reader - Regular conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRROVRF bit in the DFSDM_FLTxICR register.
pub type ROVRF_R = crate::BitReader<ROVRF>;
impl ROVRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ROVRF {
        match self.bits {
            false => ROVRF::Clear,
            true => ROVRF::Set,
        }
    }
    ///No regular conversion overrun has occurred
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ROVRF::Clear
    }
    ///A regular conversion overrun has occurred, which means that a regular conversion finished while REOCF was already ‘1’. RDATAR is not affected by overruns
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ROVRF::Set
    }
}
/**Analog watchdog This bit is set by hardware. It is cleared by software by clearing all source flag bits AWHTF\[7:0\] and AWLTF\[7:0\] in DFSDM_FLTxAWSR register (by writing '1' into the clear bits in DFSDM_FLTxAWCFR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDF {
    ///0: No Analog watchdog event occurred
    Clear = 0,
    ///1: The analog watchdog block detected voltage which crosses the value programmed in the DFSDM_FLTxAWLTR or DFSDM_FLTxAWHTR registers
    Set = 1,
}
impl From<AWDF> for bool {
    #[inline(always)]
    fn from(variant: AWDF) -> Self {
        variant as u8 != 0
    }
}
///Field `AWDF` reader - Analog watchdog This bit is set by hardware. It is cleared by software by clearing all source flag bits AWHTF\[7:0\] and AWLTF\[7:0\] in DFSDM_FLTxAWSR register (by writing '1' into the clear bits in DFSDM_FLTxAWCFR register).
pub type AWDF_R = crate::BitReader<AWDF>;
impl AWDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWDF {
        match self.bits {
            false => AWDF::Clear,
            true => AWDF::Set,
        }
    }
    ///No Analog watchdog event occurred
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == AWDF::Clear
    }
    ///The analog watchdog block detected voltage which crosses the value programmed in the DFSDM_FLTxAWLTR or DFSDM_FLTxAWHTR registers
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == AWDF::Set
    }
}
/**Injected conversion in progress status A request to start an injected conversion is ignored when JCIP=1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JCIP {
    ///0: No request to convert the injected channel group (neither by software nor by trigger) has been issued
    NotInProgress = 0,
    ///1: The conversion of the injected channel group is in progress or a request for a injected conversion is pending, due either to ‘1’ being written to JSWSTART or to a trigger detection
    InProgress = 1,
}
impl From<JCIP> for bool {
    #[inline(always)]
    fn from(variant: JCIP) -> Self {
        variant as u8 != 0
    }
}
///Field `JCIP` reader - Injected conversion in progress status A request to start an injected conversion is ignored when JCIP=1.
pub type JCIP_R = crate::BitReader<JCIP>;
impl JCIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JCIP {
        match self.bits {
            false => JCIP::NotInProgress,
            true => JCIP::InProgress,
        }
    }
    ///No request to convert the injected channel group (neither by software nor by trigger) has been issued
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == JCIP::NotInProgress
    }
    ///The conversion of the injected channel group is in progress or a request for a injected conversion is pending, due either to ‘1’ being written to JSWSTART or to a trigger detection
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == JCIP::InProgress
    }
}
/**Regular conversion in progress status A request to start a regular conversion is ignored when RCIP=1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCIP {
    ///0: No request to convert the regular channel has been issued
    NotInProgress = 0,
    ///1: The conversion of the regular channel is in progress or a request for a regular conversion is pending
    InProgress = 1,
}
impl From<RCIP> for bool {
    #[inline(always)]
    fn from(variant: RCIP) -> Self {
        variant as u8 != 0
    }
}
///Field `RCIP` reader - Regular conversion in progress status A request to start a regular conversion is ignored when RCIP=1.
pub type RCIP_R = crate::BitReader<RCIP>;
impl RCIP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RCIP {
        match self.bits {
            false => RCIP::NotInProgress,
            true => RCIP::InProgress,
        }
    }
    ///No request to convert the regular channel has been issued
    #[inline(always)]
    pub fn is_not_in_progress(&self) -> bool {
        *self == RCIP::NotInProgress
    }
    ///The conversion of the regular channel is in progress or a request for a regular conversion is pending
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RCIP::InProgress
    }
}
/**Clock absence flag CKABF\[y\]=0: Clock signal on channel y is present. CKABF\[y\]=1: Clock signal on channel y is not present. Given y bit is set by hardware when clock absence is detected on channel y. It is held at CKABF\[y\]=1 state by hardware when CHEN=0 (see DFSDM_CHyCFGR1 register). It is held at CKABF\[y\]=1 state by hardware when the transceiver is not yet synchronized.It can be cleared by software using the corresponding CLRCKABF\[y\] bit in the DFSDM_FLTxICR register. Note: CKABF\[7:0\] is present only in DFSDM_FLT0ISR register (filter x=0)

Value on reset: 255*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKABF {
    ///0: Clock signal on channel y is present.
    Clear = 0,
    ///1: Clock signal on channel y is not present
    Set = 1,
}
impl From<CKABF> for u8 {
    #[inline(always)]
    fn from(variant: CKABF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKABF {
    type Ux = u8;
}
impl crate::IsEnum for CKABF {}
///Field `CKABF` reader - Clock absence flag CKABF\[y\]=0: Clock signal on channel y is present. CKABF\[y\]=1: Clock signal on channel y is not present. Given y bit is set by hardware when clock absence is detected on channel y. It is held at CKABF\[y\]=1 state by hardware when CHEN=0 (see DFSDM_CHyCFGR1 register). It is held at CKABF\[y\]=1 state by hardware when the transceiver is not yet synchronized.It can be cleared by software using the corresponding CLRCKABF\[y\] bit in the DFSDM_FLTxICR register. Note: CKABF\[7:0\] is present only in DFSDM_FLT0ISR register (filter x=0)
pub type CKABF_R = crate::FieldReader<CKABF>;
impl CKABF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKABF> {
        match self.bits {
            0 => Some(CKABF::Clear),
            1 => Some(CKABF::Set),
            _ => None,
        }
    }
    ///Clock signal on channel y is present.
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CKABF::Clear
    }
    ///Clock signal on channel y is not present
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CKABF::Set
    }
}
/**short-circuit detector flag SDCF\[y\]=0: No short-circuit detector event occurred on channel y SDCF\[y\]=1: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers This bit is set by hardware. It can be cleared by software using the corresponding CLRSCDF\[y\] bit in the DFSDM_FLTxICR register. SCDF\[y\] is cleared also by hardware when CHEN\[y\] = 0 (given channel is disabled). Note: SCDF\[7:0\] is present only in DFSDM_FLT0ISR register (filter x=0)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCDF {
    ///0: No short-circuit detector event occurred on channel y
    Clear = 0,
    ///1: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers
    Set = 1,
}
impl From<SCDF> for u8 {
    #[inline(always)]
    fn from(variant: SCDF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCDF {
    type Ux = u8;
}
impl crate::IsEnum for SCDF {}
///Field `SCDF` reader - short-circuit detector flag SDCF\[y\]=0: No short-circuit detector event occurred on channel y SDCF\[y\]=1: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers This bit is set by hardware. It can be cleared by software using the corresponding CLRSCDF\[y\] bit in the DFSDM_FLTxICR register. SCDF\[y\] is cleared also by hardware when CHEN\[y\] = 0 (given channel is disabled). Note: SCDF\[7:0\] is present only in DFSDM_FLT0ISR register (filter x=0)
pub type SCDF_R = crate::FieldReader<SCDF>;
impl SCDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SCDF> {
        match self.bits {
            0 => Some(SCDF::Clear),
            1 => Some(SCDF::Set),
            _ => None,
        }
    }
    ///No short-circuit detector event occurred on channel y
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == SCDF::Clear
    }
    ///The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == SCDF::Set
    }
}
impl R {
    ///Bit 0 - End of injected conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxJDATAR.
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of regular conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxRDATAR.
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRJOVRF bit in the DFSDM_FLTxICR register.
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Regular conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRROVRF bit in the DFSDM_FLTxICR register.
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog watchdog This bit is set by hardware. It is cleared by software by clearing all source flag bits AWHTF\[7:0\] and AWLTF\[7:0\] in DFSDM_FLTxAWSR register (by writing '1' into the clear bits in DFSDM_FLTxAWCFR register).
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 13 - Injected conversion in progress status A request to start an injected conversion is ignored when JCIP=1.
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Regular conversion in progress status A request to start a regular conversion is ignored when RCIP=1.
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:23 - Clock absence flag CKABF\[y\]=0: Clock signal on channel y is present. CKABF\[y\]=1: Clock signal on channel y is not present. Given y bit is set by hardware when clock absence is detected on channel y. It is held at CKABF\[y\]=1 state by hardware when CHEN=0 (see DFSDM_CHyCFGR1 register). It is held at CKABF\[y\]=1 state by hardware when the transceiver is not yet synchronized.It can be cleared by software using the corresponding CLRCKABF\[y\] bit in the DFSDM_FLTxICR register. Note: CKABF\[7:0\] is present only in DFSDM_FLT0ISR register (filter x=0)
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - short-circuit detector flag SDCF\[y\]=0: No short-circuit detector event occurred on channel y SDCF\[y\]=1: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers This bit is set by hardware. It can be cleared by software using the corresponding CLRSCDF\[y\] bit in the DFSDM_FLTxICR register. SCDF\[y\] is cleared also by hardware when CHEN\[y\] = 0 (given channel is disabled). Note: SCDF\[7:0\] is present only in DFSDM_FLT0ISR register (filter x=0)
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("jeocf", &self.jeocf())
            .field("reocf", &self.reocf())
            .field("jovrf", &self.jovrf())
            .field("rovrf", &self.rovrf())
            .field("awdf", &self.awdf())
            .field("jcip", &self.jcip())
            .field("rcip", &self.rcip())
            .field("ckabf", &self.ckabf())
            .field("scdf", &self.scdf())
            .finish()
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0x00ff_0000
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
