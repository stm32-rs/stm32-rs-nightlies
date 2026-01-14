///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
/**Clock selector The CKSEL bit selects which clock source the LPTIM will use:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKSEL {
    ///0: LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)
    Internal = 0,
    ///1: LPTIM is clocked by an external clock source through the LPTIM external Input1
    External = 1,
}
impl From<CKSEL> for bool {
    #[inline(always)]
    fn from(variant: CKSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `CKSEL` reader - Clock selector The CKSEL bit selects which clock source the LPTIM will use:
pub type CKSEL_R = crate::BitReader<CKSEL>;
impl CKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKSEL {
        match self.bits {
            false => CKSEL::Internal,
            true => CKSEL::External,
        }
    }
    ///LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == CKSEL::Internal
    }
    ///LPTIM is clocked by an external clock source through the LPTIM external Input1
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == CKSEL::External
    }
}
///Field `CKSEL` writer - Clock selector The CKSEL bit selects which clock source the LPTIM will use:
pub type CKSEL_W<'a, REG> = crate::BitWriter<'a, REG, CKSEL>;
impl<'a, REG> CKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL::Internal)
    }
    ///LPTIM is clocked by an external clock source through the LPTIM external Input1
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL::External)
    }
}
/**Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKPOL {
    ///0: The rising edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 1 is active.
    RisingEdge = 0,
    ///1: The falling edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 2 is active.
    FallingEdge = 1,
    ///2: Both edges are active edge. If LPTIM is in encoder mode: Encoder sub-mode 3 is active.
    BothEdges = 2,
}
impl From<CKPOL> for u8 {
    #[inline(always)]
    fn from(variant: CKPOL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKPOL {
    type Ux = u8;
}
impl crate::IsEnum for CKPOL {}
///Field `CKPOL` reader - Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes.
pub type CKPOL_R = crate::FieldReader<CKPOL>;
impl CKPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKPOL> {
        match self.bits {
            0 => Some(CKPOL::RisingEdge),
            1 => Some(CKPOL::FallingEdge),
            2 => Some(CKPOL::BothEdges),
            _ => None,
        }
    }
    ///The rising edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 1 is active.
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CKPOL::RisingEdge
    }
    ///The falling edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 2 is active.
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CKPOL::FallingEdge
    }
    ///Both edges are active edge. If LPTIM is in encoder mode: Encoder sub-mode 3 is active.
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == CKPOL::BothEdges
    }
}
///Field `CKPOL` writer - Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes.
pub type CKPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKPOL>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The rising edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 1 is active.
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::RisingEdge)
    }
    ///The falling edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 2 is active.
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::FallingEdge)
    }
    ///Both edges are active edge. If LPTIM is in encoder mode: Encoder sub-mode 3 is active.
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::BothEdges)
    }
}
/**Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKFLT {
    ///0: Any external clock signal level change is considered as a valid transition
    Immediate = 0,
    ///1: External clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition
    Clocks2 = 1,
    ///2: External clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition
    Clocks4 = 2,
    ///3: External clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition
    Clocks8 = 3,
}
impl From<CKFLT> for u8 {
    #[inline(always)]
    fn from(variant: CKFLT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKFLT {
    type Ux = u8;
}
impl crate::IsEnum for CKFLT {}
///Field `CKFLT` reader - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature
pub type CKFLT_R = crate::FieldReader<CKFLT>;
impl CKFLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CKFLT {
        match self.bits {
            0 => CKFLT::Immediate,
            1 => CKFLT::Clocks2,
            2 => CKFLT::Clocks4,
            3 => CKFLT::Clocks8,
            _ => unreachable!(),
        }
    }
    ///Any external clock signal level change is considered as a valid transition
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == CKFLT::Immediate
    }
    ///External clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == CKFLT::Clocks2
    }
    ///External clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition
    #[inline(always)]
    pub fn is_clocks4(&self) -> bool {
        *self == CKFLT::Clocks4
    }
    ///External clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition
    #[inline(always)]
    pub fn is_clocks8(&self) -> bool {
        *self == CKFLT::Clocks8
    }
}
///Field `CKFLT` writer - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature
pub type CKFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKFLT, crate::Safe>;
impl<'a, REG> CKFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Any external clock signal level change is considered as a valid transition
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(CKFLT::Immediate)
    }
    ///External clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut crate::W<REG> {
        self.variant(CKFLT::Clocks2)
    }
    ///External clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition
    #[inline(always)]
    pub fn clocks4(self) -> &'a mut crate::W<REG> {
        self.variant(CKFLT::Clocks4)
    }
    ///External clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition
    #[inline(always)]
    pub fn clocks8(self) -> &'a mut crate::W<REG> {
        self.variant(CKFLT::Clocks8)
    }
}
/**Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGFLT {
    ///0: Any trigger active level change is considered as a valid trigger
    Immediate = 0,
    ///1: Trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger
    Clocks2 = 1,
    ///2: Trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger
    Clocks4 = 2,
    ///3: Trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger
    Clocks8 = 3,
}
impl From<TRGFLT> for u8 {
    #[inline(always)]
    fn from(variant: TRGFLT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRGFLT {
    type Ux = u8;
}
impl crate::IsEnum for TRGFLT {}
///Field `TRGFLT` reader - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature
pub type TRGFLT_R = crate::FieldReader<TRGFLT>;
impl TRGFLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRGFLT {
        match self.bits {
            0 => TRGFLT::Immediate,
            1 => TRGFLT::Clocks2,
            2 => TRGFLT::Clocks4,
            3 => TRGFLT::Clocks8,
            _ => unreachable!(),
        }
    }
    ///Any trigger active level change is considered as a valid trigger
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == TRGFLT::Immediate
    }
    ///Trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == TRGFLT::Clocks2
    }
    ///Trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger
    #[inline(always)]
    pub fn is_clocks4(&self) -> bool {
        *self == TRGFLT::Clocks4
    }
    ///Trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger
    #[inline(always)]
    pub fn is_clocks8(&self) -> bool {
        *self == TRGFLT::Clocks8
    }
}
///Field `TRGFLT` writer - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature
pub type TRGFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRGFLT, crate::Safe>;
impl<'a, REG> TRGFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Any trigger active level change is considered as a valid trigger
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(TRGFLT::Immediate)
    }
    ///Trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut crate::W<REG> {
        self.variant(TRGFLT::Clocks2)
    }
    ///Trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger
    #[inline(always)]
    pub fn clocks4(self) -> &'a mut crate::W<REG> {
        self.variant(TRGFLT::Clocks4)
    }
    ///Trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger
    #[inline(always)]
    pub fn clocks8(self) -> &'a mut crate::W<REG> {
        self.variant(TRGFLT::Clocks8)
    }
}
/**Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    ///0: /1
    Div1 = 0,
    ///1: /2
    Div2 = 1,
    ///2: /4
    Div4 = 2,
    ///3: /8
    Div8 = 3,
    ///4: /16
    Div16 = 4,
    ///5: /32
    Div32 = 5,
    ///6: /64
    Div64 = 6,
    ///7: /128
    Div128 = 7,
}
impl From<PRESC> for u8 {
    #[inline(always)]
    fn from(variant: PRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC {
    type Ux = u8;
}
impl crate::IsEnum for PRESC {}
///Field `PRESC` reader - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:
pub type PRESC_R = crate::FieldReader<PRESC>;
impl PRESC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRESC {
        match self.bits {
            0 => PRESC::Div1,
            1 => PRESC::Div2,
            2 => PRESC::Div4,
            3 => PRESC::Div8,
            4 => PRESC::Div16,
            5 => PRESC::Div32,
            6 => PRESC::Div64,
            7 => PRESC::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC::Div1
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC::Div2
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC::Div4
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC::Div8
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC::Div16
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC::Div32
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC::Div64
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC::Div128
    }
}
///Field `PRESC` writer - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PRESC, crate::Safe>;
impl<'a, REG> PRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div1)
    }
    #[doc = "/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div2)
    }
    #[doc = "/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div4)
    }
    #[doc = "/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div8)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div16)
    }
    #[doc = "/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div32)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div64)
    }
    #[doc = "/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div128)
    }
}
/**Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSEL {
    ///0: lptim_ext_trig0
    Trig0 = 0,
    ///1: lptim_ext_trig1
    Trig1 = 1,
    ///2: lptim_ext_trig2
    Trig2 = 2,
    ///3: lptim_ext_trig3
    Trig3 = 3,
    ///4: lptim_ext_trig4
    Trig4 = 4,
    ///5: lptim_ext_trig5
    Trig5 = 5,
    ///6: lptim_ext_trig6
    Trig6 = 6,
    ///7: lptim_ext_trig7
    Trig7 = 7,
}
impl From<TRIGSEL> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGSEL {
    type Ux = u8;
}
impl crate::IsEnum for TRIGSEL {}
///Field `TRIGSEL` reader - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details.
pub type TRIGSEL_R = crate::FieldReader<TRIGSEL>;
impl TRIGSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRIGSEL {
        match self.bits {
            0 => TRIGSEL::Trig0,
            1 => TRIGSEL::Trig1,
            2 => TRIGSEL::Trig2,
            3 => TRIGSEL::Trig3,
            4 => TRIGSEL::Trig4,
            5 => TRIGSEL::Trig5,
            6 => TRIGSEL::Trig6,
            7 => TRIGSEL::Trig7,
            _ => unreachable!(),
        }
    }
    ///lptim_ext_trig0
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        *self == TRIGSEL::Trig0
    }
    ///lptim_ext_trig1
    #[inline(always)]
    pub fn is_trig1(&self) -> bool {
        *self == TRIGSEL::Trig1
    }
    ///lptim_ext_trig2
    #[inline(always)]
    pub fn is_trig2(&self) -> bool {
        *self == TRIGSEL::Trig2
    }
    ///lptim_ext_trig3
    #[inline(always)]
    pub fn is_trig3(&self) -> bool {
        *self == TRIGSEL::Trig3
    }
    ///lptim_ext_trig4
    #[inline(always)]
    pub fn is_trig4(&self) -> bool {
        *self == TRIGSEL::Trig4
    }
    ///lptim_ext_trig5
    #[inline(always)]
    pub fn is_trig5(&self) -> bool {
        *self == TRIGSEL::Trig5
    }
    ///lptim_ext_trig6
    #[inline(always)]
    pub fn is_trig6(&self) -> bool {
        *self == TRIGSEL::Trig6
    }
    ///lptim_ext_trig7
    #[inline(always)]
    pub fn is_trig7(&self) -> bool {
        *self == TRIGSEL::Trig7
    }
}
///Field `TRIGSEL` writer - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details.
pub type TRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TRIGSEL, crate::Safe>;
impl<'a, REG> TRIGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///lptim_ext_trig0
    #[inline(always)]
    pub fn trig0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig0)
    }
    ///lptim_ext_trig1
    #[inline(always)]
    pub fn trig1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig1)
    }
    ///lptim_ext_trig2
    #[inline(always)]
    pub fn trig2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig2)
    }
    ///lptim_ext_trig3
    #[inline(always)]
    pub fn trig3(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig3)
    }
    ///lptim_ext_trig4
    #[inline(always)]
    pub fn trig4(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig4)
    }
    ///lptim_ext_trig5
    #[inline(always)]
    pub fn trig5(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig5)
    }
    ///lptim_ext_trig6
    #[inline(always)]
    pub fn trig6(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig6)
    }
    ///lptim_ext_trig7
    #[inline(always)]
    pub fn trig7(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig7)
    }
}
/**Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGEN {
    ///0: Software trigger (counting start is initiated by software)
    Sw = 0,
    ///1: Rising edge is the active edge
    RisingEdge = 1,
    ///2: Falling edge is the active edge
    FallingEdge = 2,
    ///3: Both edges are active edges
    BothEdges = 3,
}
impl From<TRIGEN> for u8 {
    #[inline(always)]
    fn from(variant: TRIGEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRIGEN {
    type Ux = u8;
}
impl crate::IsEnum for TRIGEN {}
///Field `TRIGEN` reader - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:
pub type TRIGEN_R = crate::FieldReader<TRIGEN>;
impl TRIGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRIGEN {
        match self.bits {
            0 => TRIGEN::Sw,
            1 => TRIGEN::RisingEdge,
            2 => TRIGEN::FallingEdge,
            3 => TRIGEN::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Software trigger (counting start is initiated by software)
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == TRIGEN::Sw
    }
    ///Rising edge is the active edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TRIGEN::RisingEdge
    }
    ///Falling edge is the active edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TRIGEN::FallingEdge
    }
    ///Both edges are active edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == TRIGEN::BothEdges
    }
}
///Field `TRIGEN` writer - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:
pub type TRIGEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TRIGEN, crate::Safe>;
impl<'a, REG> TRIGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Software trigger (counting start is initiated by software)
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN::Sw)
    }
    ///Rising edge is the active edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN::RisingEdge)
    }
    ///Falling edge is the active edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN::FallingEdge)
    }
    ///Both edges are active edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN::BothEdges)
    }
}
/**Timeout enable The TIMOUT bit controls the Timeout feature

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUT {
    ///0: A trigger event arriving when the timer is already started will be ignored
    Disabled = 0,
    ///1: A trigger event arriving when the timer is already started will reset and restart the counter
    Enabled = 1,
}
impl From<TIMOUT> for bool {
    #[inline(always)]
    fn from(variant: TIMOUT) -> Self {
        variant as u8 != 0
    }
}
///Field `TIMOUT` reader - Timeout enable The TIMOUT bit controls the Timeout feature
pub type TIMOUT_R = crate::BitReader<TIMOUT>;
impl TIMOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIMOUT {
        match self.bits {
            false => TIMOUT::Disabled,
            true => TIMOUT::Enabled,
        }
    }
    ///A trigger event arriving when the timer is already started will be ignored
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMOUT::Disabled
    }
    ///A trigger event arriving when the timer is already started will reset and restart the counter
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMOUT::Enabled
    }
}
///Field `TIMOUT` writer - Timeout enable The TIMOUT bit controls the Timeout feature
pub type TIMOUT_W<'a, REG> = crate::BitWriter<'a, REG, TIMOUT>;
impl<'a, REG> TIMOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///A trigger event arriving when the timer is already started will be ignored
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUT::Disabled)
    }
    ///A trigger event arriving when the timer is already started will reset and restart the counter
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUT::Enabled)
    }
}
/**Waveform shape The WAVE bit controls the output shape

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAVE {
    ///0: Deactivate Set-once mode, PWM / One Pulse waveform (depending on OPMODE bit)
    Inactive = 0,
    ///1: Activate the Set-once mode
    Active = 1,
}
impl From<WAVE> for bool {
    #[inline(always)]
    fn from(variant: WAVE) -> Self {
        variant as u8 != 0
    }
}
///Field `WAVE` reader - Waveform shape The WAVE bit controls the output shape
pub type WAVE_R = crate::BitReader<WAVE>;
impl WAVE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAVE {
        match self.bits {
            false => WAVE::Inactive,
            true => WAVE::Active,
        }
    }
    ///Deactivate Set-once mode, PWM / One Pulse waveform (depending on OPMODE bit)
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == WAVE::Inactive
    }
    ///Activate the Set-once mode
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == WAVE::Active
    }
}
///Field `WAVE` writer - Waveform shape The WAVE bit controls the output shape
pub type WAVE_W<'a, REG> = crate::BitWriter<'a, REG, WAVE>;
impl<'a, REG> WAVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Deactivate Set-once mode, PWM / One Pulse waveform (depending on OPMODE bit)
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE::Inactive)
    }
    ///Activate the Set-once mode
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE::Active)
    }
}
/**Waveform shape polarity The WAVEPOL bit controls the output polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAVPOL {
    ///0: The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers
    Positive = 0,
    ///1: The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers
    Negative = 1,
}
impl From<WAVPOL> for bool {
    #[inline(always)]
    fn from(variant: WAVPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `WAVPOL` reader - Waveform shape polarity The WAVEPOL bit controls the output polarity
pub type WAVPOL_R = crate::BitReader<WAVPOL>;
impl WAVPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAVPOL {
        match self.bits {
            false => WAVPOL::Positive,
            true => WAVPOL::Negative,
        }
    }
    ///The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == WAVPOL::Positive
    }
    ///The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == WAVPOL::Negative
    }
}
///Field `WAVPOL` writer - Waveform shape polarity The WAVEPOL bit controls the output polarity
pub type WAVPOL_W<'a, REG> = crate::BitWriter<'a, REG, WAVPOL>;
impl<'a, REG> WAVPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(WAVPOL::Positive)
    }
    ///The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(WAVPOL::Negative)
    }
}
/**Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRELOAD {
    ///0: Registers are updated after each APB bus write access
    Immediate = 0,
    ///1: Registers are updated at the end of the current LPTIM period
    EndOfPeriod = 1,
}
impl From<PRELOAD> for bool {
    #[inline(always)]
    fn from(variant: PRELOAD) -> Self {
        variant as u8 != 0
    }
}
///Field `PRELOAD` reader - Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality
pub type PRELOAD_R = crate::BitReader<PRELOAD>;
impl PRELOAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRELOAD {
        match self.bits {
            false => PRELOAD::Immediate,
            true => PRELOAD::EndOfPeriod,
        }
    }
    ///Registers are updated after each APB bus write access
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == PRELOAD::Immediate
    }
    ///Registers are updated at the end of the current LPTIM period
    #[inline(always)]
    pub fn is_end_of_period(&self) -> bool {
        *self == PRELOAD::EndOfPeriod
    }
}
///Field `PRELOAD` writer - Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality
pub type PRELOAD_W<'a, REG> = crate::BitWriter<'a, REG, PRELOAD>;
impl<'a, REG> PRELOAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Registers are updated after each APB bus write access
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(PRELOAD::Immediate)
    }
    ///Registers are updated at the end of the current LPTIM period
    #[inline(always)]
    pub fn end_of_period(self) -> &'a mut crate::W<REG> {
        self.variant(PRELOAD::EndOfPeriod)
    }
}
/**counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COUNTMODE {
    ///0: The counter is incremented following each internal clock pulse
    Internal = 0,
    ///1: The counter is incremented following each valid clock pulse on the LPTIM external Input1
    External = 1,
}
impl From<COUNTMODE> for bool {
    #[inline(always)]
    fn from(variant: COUNTMODE) -> Self {
        variant as u8 != 0
    }
}
///Field `COUNTMODE` reader - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:
pub type COUNTMODE_R = crate::BitReader<COUNTMODE>;
impl COUNTMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COUNTMODE {
        match self.bits {
            false => COUNTMODE::Internal,
            true => COUNTMODE::External,
        }
    }
    ///The counter is incremented following each internal clock pulse
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == COUNTMODE::Internal
    }
    ///The counter is incremented following each valid clock pulse on the LPTIM external Input1
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == COUNTMODE::External
    }
}
///Field `COUNTMODE` writer - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:
pub type COUNTMODE_W<'a, REG> = crate::BitWriter<'a, REG, COUNTMODE>;
impl<'a, REG> COUNTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The counter is incremented following each internal clock pulse
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(COUNTMODE::Internal)
    }
    ///The counter is incremented following each valid clock pulse on the LPTIM external Input1
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(COUNTMODE::External)
    }
}
/**Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENC {
    ///0: Encoder mode disabled
    Disabled = 0,
    ///1: Encoder mode enabled
    Enabled = 1,
}
impl From<ENC> for bool {
    #[inline(always)]
    fn from(variant: ENC) -> Self {
        variant as u8 != 0
    }
}
///Field `ENC` reader - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
pub type ENC_R = crate::BitReader<ENC>;
impl ENC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENC {
        match self.bits {
            false => ENC::Disabled,
            true => ENC::Enabled,
        }
    }
    ///Encoder mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENC::Disabled
    }
    ///Encoder mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENC::Enabled
    }
}
///Field `ENC` writer - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
pub type ENC_W<'a, REG> = crate::BitWriter<'a, REG, ENC>;
impl<'a, REG> ENC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Encoder mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENC::Disabled)
    }
    ///Encoder mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENC::Enabled)
    }
}
impl R {
    ///Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM will use:
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes.
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details.
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Waveform shape The WAVE bit controls the output shape
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Waveform shape polarity The WAVEPOL bit controls the output polarity
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("cksel", &self.cksel())
            .field("ckpol", &self.ckpol())
            .field("ckflt", &self.ckflt())
            .field("trgflt", &self.trgflt())
            .field("presc", &self.presc())
            .field("trigsel", &self.trigsel())
            .field("trigen", &self.trigen())
            .field("timout", &self.timout())
            .field("wave", &self.wave())
            .field("wavpol", &self.wavpol())
            .field("preload", &self.preload())
            .field("countmode", &self.countmode())
            .field("enc", &self.enc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clock selector The CKSEL bit selects which clock source the LPTIM will use:
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W<'_, CFGRrs> {
        CKSEL_W::new(self, 0)
    }
    ///Bits 1:2 - Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes.
    #[inline(always)]
    pub fn ckpol(&mut self) -> CKPOL_W<'_, CFGRrs> {
        CKPOL_W::new(self, 1)
    }
    ///Bits 3:4 - Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature
    #[inline(always)]
    pub fn ckflt(&mut self) -> CKFLT_W<'_, CFGRrs> {
        CKFLT_W::new(self, 3)
    }
    ///Bits 6:7 - Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature
    #[inline(always)]
    pub fn trgflt(&mut self) -> TRGFLT_W<'_, CFGRrs> {
        TRGFLT_W::new(self, 6)
    }
    ///Bits 9:11 - Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W<'_, CFGRrs> {
        PRESC_W::new(self, 9)
    }
    ///Bits 13:15 - Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details.
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W<'_, CFGRrs> {
        TRIGSEL_W::new(self, 13)
    }
    ///Bits 17:18 - Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:
    #[inline(always)]
    pub fn trigen(&mut self) -> TRIGEN_W<'_, CFGRrs> {
        TRIGEN_W::new(self, 17)
    }
    ///Bit 19 - Timeout enable The TIMOUT bit controls the Timeout feature
    #[inline(always)]
    pub fn timout(&mut self) -> TIMOUT_W<'_, CFGRrs> {
        TIMOUT_W::new(self, 19)
    }
    ///Bit 20 - Waveform shape The WAVE bit controls the output shape
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W<'_, CFGRrs> {
        WAVE_W::new(self, 20)
    }
    ///Bit 21 - Waveform shape polarity The WAVEPOL bit controls the output polarity
    #[inline(always)]
    pub fn wavpol(&mut self) -> WAVPOL_W<'_, CFGRrs> {
        WAVPOL_W::new(self, 21)
    }
    ///Bit 22 - Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality
    #[inline(always)]
    pub fn preload(&mut self) -> PRELOAD_W<'_, CFGRrs> {
        PRELOAD_W::new(self, 22)
    }
    ///Bit 23 - counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:
    #[inline(always)]
    pub fn countmode(&mut self) -> COUNTMODE_W<'_, CFGRrs> {
        COUNTMODE_W::new(self, 23)
    }
    ///Bit 24 - Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W<'_, CFGRrs> {
        ENC_W::new(self, 24)
    }
}
/**Configuration Register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#LPTIM1:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
