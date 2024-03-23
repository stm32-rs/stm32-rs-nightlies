#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "CKSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKSEL {
    #[doc = "0: LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    Internal = 0,
    #[doc = "1: LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    External = 1,
}
impl From<CKSEL> for bool {
    #[inline(always)]
    fn from(variant: CKSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKSEL` reader - CKSEL"]
pub type CKSEL_R = crate::BitReader<CKSEL>;
impl CKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CKSEL {
        match self.bits {
            false => CKSEL::Internal,
            true => CKSEL::External,
        }
    }
    #[doc = "LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == CKSEL::Internal
    }
    #[doc = "LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == CKSEL::External
    }
}
#[doc = "Field `CKSEL` writer - CKSEL"]
pub type CKSEL_W<'a, REG> = crate::BitWriter<'a, REG, CKSEL>;
impl<'a, REG> CKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL::Internal)
    }
    #[doc = "LPTIM is clocked by an external clock source through the LPTIM external Input1"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(CKSEL::External)
    }
}
#[doc = "CKPOL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKPOL {
    #[doc = "0: The rising edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 1 is active."]
    RisingEdge = 0,
    #[doc = "1: The falling edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 2 is active."]
    FallingEdge = 1,
    #[doc = "2: Both edges are active edge. If LPTIM is in encoder mode: Encoder sub-mode 3 is active."]
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
#[doc = "Field `CKPOL` reader - CKPOL"]
pub type CKPOL_R = crate::FieldReader<CKPOL>;
impl CKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CKPOL> {
        match self.bits {
            0 => Some(CKPOL::RisingEdge),
            1 => Some(CKPOL::FallingEdge),
            2 => Some(CKPOL::BothEdges),
            _ => None,
        }
    }
    #[doc = "The rising edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 1 is active."]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CKPOL::RisingEdge
    }
    #[doc = "The falling edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 2 is active."]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CKPOL::FallingEdge
    }
    #[doc = "Both edges are active edge. If LPTIM is in encoder mode: Encoder sub-mode 3 is active."]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == CKPOL::BothEdges
    }
}
#[doc = "Field `CKPOL` writer - CKPOL"]
pub type CKPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CKPOL>;
impl<'a, REG> CKPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The rising edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 1 is active."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::RisingEdge)
    }
    #[doc = "The falling edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 2 is active."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::FallingEdge)
    }
    #[doc = "Both edges are active edge. If LPTIM is in encoder mode: Encoder sub-mode 3 is active."]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(CKPOL::BothEdges)
    }
}
#[doc = "CKFLT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKFLT {
    #[doc = "0: Any external clock signal level change is considered as a valid transition"]
    Immediate = 0,
    #[doc = "1: External clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition"]
    Clocks2 = 1,
    #[doc = "2: External clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition"]
    Clocks4 = 2,
    #[doc = "3: External clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition"]
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
#[doc = "Field `CKFLT` reader - CKFLT"]
pub type CKFLT_R = crate::FieldReader<CKFLT>;
impl CKFLT_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Any external clock signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == CKFLT::Immediate
    }
    #[doc = "External clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition"]
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == CKFLT::Clocks2
    }
    #[doc = "External clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition"]
    #[inline(always)]
    pub fn is_clocks4(&self) -> bool {
        *self == CKFLT::Clocks4
    }
    #[doc = "External clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition"]
    #[inline(always)]
    pub fn is_clocks8(&self) -> bool {
        *self == CKFLT::Clocks8
    }
}
#[doc = "Field `CKFLT` writer - CKFLT"]
pub type CKFLT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CKFLT>;
impl<'a, REG> CKFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any external clock signal level change is considered as a valid transition"]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(CKFLT::Immediate)
    }
    #[doc = "External clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition"]
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut crate::W<REG> {
        self.variant(CKFLT::Clocks2)
    }
    #[doc = "External clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition"]
    #[inline(always)]
    pub fn clocks4(self) -> &'a mut crate::W<REG> {
        self.variant(CKFLT::Clocks4)
    }
    #[doc = "External clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition"]
    #[inline(always)]
    pub fn clocks8(self) -> &'a mut crate::W<REG> {
        self.variant(CKFLT::Clocks8)
    }
}
#[doc = "TRGFLT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGFLT {
    #[doc = "0: Any trigger active level change is considered as a valid trigger"]
    Immediate = 0,
    #[doc = "1: Trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger"]
    Clocks2 = 1,
    #[doc = "2: Trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger"]
    Clocks4 = 2,
    #[doc = "3: Trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger"]
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
#[doc = "Field `TRGFLT` reader - TRGFLT"]
pub type TRGFLT_R = crate::FieldReader<TRGFLT>;
impl TRGFLT_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Any trigger active level change is considered as a valid trigger"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == TRGFLT::Immediate
    }
    #[doc = "Trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger"]
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == TRGFLT::Clocks2
    }
    #[doc = "Trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger"]
    #[inline(always)]
    pub fn is_clocks4(&self) -> bool {
        *self == TRGFLT::Clocks4
    }
    #[doc = "Trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger"]
    #[inline(always)]
    pub fn is_clocks8(&self) -> bool {
        *self == TRGFLT::Clocks8
    }
}
#[doc = "Field `TRGFLT` writer - TRGFLT"]
pub type TRGFLT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TRGFLT>;
impl<'a, REG> TRGFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Any trigger active level change is considered as a valid trigger"]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(TRGFLT::Immediate)
    }
    #[doc = "Trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger"]
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut crate::W<REG> {
        self.variant(TRGFLT::Clocks2)
    }
    #[doc = "Trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger"]
    #[inline(always)]
    pub fn clocks4(self) -> &'a mut crate::W<REG> {
        self.variant(TRGFLT::Clocks4)
    }
    #[doc = "Trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger"]
    #[inline(always)]
    pub fn clocks8(self) -> &'a mut crate::W<REG> {
        self.variant(TRGFLT::Clocks8)
    }
}
#[doc = "PRESC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    #[doc = "0: /1"]
    Div1 = 0,
    #[doc = "1: /2"]
    Div2 = 1,
    #[doc = "2: /4"]
    Div4 = 2,
    #[doc = "3: /8"]
    Div8 = 3,
    #[doc = "4: /16"]
    Div16 = 4,
    #[doc = "5: /32"]
    Div32 = 5,
    #[doc = "6: /64"]
    Div64 = 6,
    #[doc = "7: /128"]
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
#[doc = "Field `PRESC` reader - PRESC"]
pub type PRESC_R = crate::FieldReader<PRESC>;
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `PRESC` writer - PRESC"]
pub type PRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, PRESC>;
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
#[doc = "TRIGSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSEL {
    #[doc = "0: lptim_ext_trig0"]
    Trig0 = 0,
    #[doc = "1: lptim_ext_trig1"]
    Trig1 = 1,
    #[doc = "2: lptim_ext_trig2"]
    Trig2 = 2,
    #[doc = "3: lptim_ext_trig3"]
    Trig3 = 3,
    #[doc = "4: lptim_ext_trig4"]
    Trig4 = 4,
    #[doc = "5: lptim_ext_trig5"]
    Trig5 = 5,
    #[doc = "6: lptim_ext_trig6"]
    Trig6 = 6,
    #[doc = "7: lptim_ext_trig7"]
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
#[doc = "Field `TRIGSEL` reader - TRIGSEL"]
pub type TRIGSEL_R = crate::FieldReader<TRIGSEL>;
impl TRIGSEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "lptim_ext_trig0"]
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        *self == TRIGSEL::Trig0
    }
    #[doc = "lptim_ext_trig1"]
    #[inline(always)]
    pub fn is_trig1(&self) -> bool {
        *self == TRIGSEL::Trig1
    }
    #[doc = "lptim_ext_trig2"]
    #[inline(always)]
    pub fn is_trig2(&self) -> bool {
        *self == TRIGSEL::Trig2
    }
    #[doc = "lptim_ext_trig3"]
    #[inline(always)]
    pub fn is_trig3(&self) -> bool {
        *self == TRIGSEL::Trig3
    }
    #[doc = "lptim_ext_trig4"]
    #[inline(always)]
    pub fn is_trig4(&self) -> bool {
        *self == TRIGSEL::Trig4
    }
    #[doc = "lptim_ext_trig5"]
    #[inline(always)]
    pub fn is_trig5(&self) -> bool {
        *self == TRIGSEL::Trig5
    }
    #[doc = "lptim_ext_trig6"]
    #[inline(always)]
    pub fn is_trig6(&self) -> bool {
        *self == TRIGSEL::Trig6
    }
    #[doc = "lptim_ext_trig7"]
    #[inline(always)]
    pub fn is_trig7(&self) -> bool {
        *self == TRIGSEL::Trig7
    }
}
#[doc = "Field `TRIGSEL` writer - TRIGSEL"]
pub type TRIGSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, TRIGSEL>;
impl<'a, REG> TRIGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "lptim_ext_trig0"]
    #[inline(always)]
    pub fn trig0(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig0)
    }
    #[doc = "lptim_ext_trig1"]
    #[inline(always)]
    pub fn trig1(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig1)
    }
    #[doc = "lptim_ext_trig2"]
    #[inline(always)]
    pub fn trig2(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig2)
    }
    #[doc = "lptim_ext_trig3"]
    #[inline(always)]
    pub fn trig3(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig3)
    }
    #[doc = "lptim_ext_trig4"]
    #[inline(always)]
    pub fn trig4(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig4)
    }
    #[doc = "lptim_ext_trig5"]
    #[inline(always)]
    pub fn trig5(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig5)
    }
    #[doc = "lptim_ext_trig6"]
    #[inline(always)]
    pub fn trig6(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig6)
    }
    #[doc = "lptim_ext_trig7"]
    #[inline(always)]
    pub fn trig7(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGSEL::Trig7)
    }
}
#[doc = "TRIGEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGEN {
    #[doc = "0: Software trigger (counting start is initiated by software)"]
    Sw = 0,
    #[doc = "1: Rising edge is the active edge"]
    RisingEdge = 1,
    #[doc = "2: Falling edge is the active edge"]
    FallingEdge = 2,
    #[doc = "3: Both edges are active edges"]
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
#[doc = "Field `TRIGEN` reader - TRIGEN"]
pub type TRIGEN_R = crate::FieldReader<TRIGEN>;
impl TRIGEN_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Software trigger (counting start is initiated by software)"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == TRIGEN::Sw
    }
    #[doc = "Rising edge is the active edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TRIGEN::RisingEdge
    }
    #[doc = "Falling edge is the active edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TRIGEN::FallingEdge
    }
    #[doc = "Both edges are active edges"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == TRIGEN::BothEdges
    }
}
#[doc = "Field `TRIGEN` writer - TRIGEN"]
pub type TRIGEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TRIGEN>;
impl<'a, REG> TRIGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software trigger (counting start is initiated by software)"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN::Sw)
    }
    #[doc = "Rising edge is the active edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN::RisingEdge)
    }
    #[doc = "Falling edge is the active edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN::FallingEdge)
    }
    #[doc = "Both edges are active edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(TRIGEN::BothEdges)
    }
}
#[doc = "TIMOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUT {
    #[doc = "0: A trigger event arriving when the timer is already started will be ignored"]
    Disabled = 0,
    #[doc = "1: A trigger event arriving when the timer is already started will reset and restart the counter"]
    Enabled = 1,
}
impl From<TIMOUT> for bool {
    #[inline(always)]
    fn from(variant: TIMOUT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMOUT` reader - TIMOUT"]
pub type TIMOUT_R = crate::BitReader<TIMOUT>;
impl TIMOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIMOUT {
        match self.bits {
            false => TIMOUT::Disabled,
            true => TIMOUT::Enabled,
        }
    }
    #[doc = "A trigger event arriving when the timer is already started will be ignored"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMOUT::Disabled
    }
    #[doc = "A trigger event arriving when the timer is already started will reset and restart the counter"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMOUT::Enabled
    }
}
#[doc = "Field `TIMOUT` writer - TIMOUT"]
pub type TIMOUT_W<'a, REG> = crate::BitWriter<'a, REG, TIMOUT>;
impl<'a, REG> TIMOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A trigger event arriving when the timer is already started will be ignored"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUT::Disabled)
    }
    #[doc = "A trigger event arriving when the timer is already started will reset and restart the counter"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUT::Enabled)
    }
}
#[doc = "WAVE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAVE {
    #[doc = "0: Deactivate Set-once mode, PWM / One Pulse waveform (depending on OPMODE bit)"]
    Inactive = 0,
    #[doc = "1: Activate the Set-once mode"]
    Active = 1,
}
impl From<WAVE> for bool {
    #[inline(always)]
    fn from(variant: WAVE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAVE` reader - WAVE"]
pub type WAVE_R = crate::BitReader<WAVE>;
impl WAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAVE {
        match self.bits {
            false => WAVE::Inactive,
            true => WAVE::Active,
        }
    }
    #[doc = "Deactivate Set-once mode, PWM / One Pulse waveform (depending on OPMODE bit)"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == WAVE::Inactive
    }
    #[doc = "Activate the Set-once mode"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == WAVE::Active
    }
}
#[doc = "Field `WAVE` writer - WAVE"]
pub type WAVE_W<'a, REG> = crate::BitWriter<'a, REG, WAVE>;
impl<'a, REG> WAVE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deactivate Set-once mode, PWM / One Pulse waveform (depending on OPMODE bit)"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE::Inactive)
    }
    #[doc = "Activate the Set-once mode"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(WAVE::Active)
    }
}
#[doc = "WAVPOL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAVPOL {
    #[doc = "0: The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers"]
    Positive = 0,
    #[doc = "1: The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers"]
    Negative = 1,
}
impl From<WAVPOL> for bool {
    #[inline(always)]
    fn from(variant: WAVPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAVPOL` reader - WAVPOL"]
pub type WAVPOL_R = crate::BitReader<WAVPOL>;
impl WAVPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAVPOL {
        match self.bits {
            false => WAVPOL::Positive,
            true => WAVPOL::Negative,
        }
    }
    #[doc = "The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == WAVPOL::Positive
    }
    #[doc = "The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == WAVPOL::Negative
    }
}
#[doc = "Field `WAVPOL` writer - WAVPOL"]
pub type WAVPOL_W<'a, REG> = crate::BitWriter<'a, REG, WAVPOL>;
impl<'a, REG> WAVPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(WAVPOL::Positive)
    }
    #[doc = "The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(WAVPOL::Negative)
    }
}
#[doc = "PRELOAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRELOAD {
    #[doc = "0: Registers are updated after each APB bus write access"]
    Immediate = 0,
    #[doc = "1: Registers are updated at the end of the current LPTIM period"]
    EndOfPeriod = 1,
}
impl From<PRELOAD> for bool {
    #[inline(always)]
    fn from(variant: PRELOAD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRELOAD` reader - PRELOAD"]
pub type PRELOAD_R = crate::BitReader<PRELOAD>;
impl PRELOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRELOAD {
        match self.bits {
            false => PRELOAD::Immediate,
            true => PRELOAD::EndOfPeriod,
        }
    }
    #[doc = "Registers are updated after each APB bus write access"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == PRELOAD::Immediate
    }
    #[doc = "Registers are updated at the end of the current LPTIM period"]
    #[inline(always)]
    pub fn is_end_of_period(&self) -> bool {
        *self == PRELOAD::EndOfPeriod
    }
}
#[doc = "Field `PRELOAD` writer - PRELOAD"]
pub type PRELOAD_W<'a, REG> = crate::BitWriter<'a, REG, PRELOAD>;
impl<'a, REG> PRELOAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Registers are updated after each APB bus write access"]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(PRELOAD::Immediate)
    }
    #[doc = "Registers are updated at the end of the current LPTIM period"]
    #[inline(always)]
    pub fn end_of_period(self) -> &'a mut crate::W<REG> {
        self.variant(PRELOAD::EndOfPeriod)
    }
}
#[doc = "COUNTMODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COUNTMODE {
    #[doc = "0: The counter is incremented following each internal clock pulse"]
    Internal = 0,
    #[doc = "1: The counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    External = 1,
}
impl From<COUNTMODE> for bool {
    #[inline(always)]
    fn from(variant: COUNTMODE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COUNTMODE` reader - COUNTMODE"]
pub type COUNTMODE_R = crate::BitReader<COUNTMODE>;
impl COUNTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COUNTMODE {
        match self.bits {
            false => COUNTMODE::Internal,
            true => COUNTMODE::External,
        }
    }
    #[doc = "The counter is incremented following each internal clock pulse"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == COUNTMODE::Internal
    }
    #[doc = "The counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == COUNTMODE::External
    }
}
#[doc = "Field `COUNTMODE` writer - COUNTMODE"]
pub type COUNTMODE_W<'a, REG> = crate::BitWriter<'a, REG, COUNTMODE>;
impl<'a, REG> COUNTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The counter is incremented following each internal clock pulse"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(COUNTMODE::Internal)
    }
    #[doc = "The counter is incremented following each valid clock pulse on the LPTIM external Input1"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(COUNTMODE::External)
    }
}
#[doc = "ENC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENC {
    #[doc = "0: Encoder mode disabled"]
    Disabled = 0,
    #[doc = "1: Encoder mode enabled"]
    Enabled = 1,
}
impl From<ENC> for bool {
    #[inline(always)]
    fn from(variant: ENC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENC` reader - ENC"]
pub type ENC_R = crate::BitReader<ENC>;
impl ENC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENC {
        match self.bits {
            false => ENC::Disabled,
            true => ENC::Enabled,
        }
    }
    #[doc = "Encoder mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENC::Disabled
    }
    #[doc = "Encoder mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENC::Enabled
    }
}
#[doc = "Field `ENC` writer - ENC"]
pub type ENC_W<'a, REG> = crate::BitWriter<'a, REG, ENC>;
impl<'a, REG> ENC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Encoder mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENC::Disabled)
    }
    #[doc = "Encoder mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENC::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - CKSEL"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - CKPOL"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - CKFLT"]
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TRGFLT"]
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 9:11 - PRESC"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 13:15 - TRIGSEL"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 17:18 - TRIGEN"]
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - TIMOUT"]
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WAVE"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WAVPOL"]
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PRELOAD"]
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - COUNTMODE"]
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ENC"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<CFGRrs> {
        CKSEL_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - CKPOL"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<CFGRrs> {
        CKPOL_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - CKFLT"]
    #[inline(always)]
    #[must_use]
    pub fn ckflt(&mut self) -> CKFLT_W<CFGRrs> {
        CKFLT_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - TRGFLT"]
    #[inline(always)]
    #[must_use]
    pub fn trgflt(&mut self) -> TRGFLT_W<CFGRrs> {
        TRGFLT_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - PRESC"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<CFGRrs> {
        PRESC_W::new(self, 9)
    }
    #[doc = "Bits 13:15 - TRIGSEL"]
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<CFGRrs> {
        TRIGSEL_W::new(self, 13)
    }
    #[doc = "Bits 17:18 - TRIGEN"]
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<CFGRrs> {
        TRIGEN_W::new(self, 17)
    }
    #[doc = "Bit 19 - TIMOUT"]
    #[inline(always)]
    #[must_use]
    pub fn timout(&mut self) -> TIMOUT_W<CFGRrs> {
        TIMOUT_W::new(self, 19)
    }
    #[doc = "Bit 20 - WAVE"]
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WAVE_W<CFGRrs> {
        WAVE_W::new(self, 20)
    }
    #[doc = "Bit 21 - WAVPOL"]
    #[inline(always)]
    #[must_use]
    pub fn wavpol(&mut self) -> WAVPOL_W<CFGRrs> {
        WAVPOL_W::new(self, 21)
    }
    #[doc = "Bit 22 - PRELOAD"]
    #[inline(always)]
    #[must_use]
    pub fn preload(&mut self) -> PRELOAD_W<CFGRrs> {
        PRELOAD_W::new(self, 22)
    }
    #[doc = "Bit 23 - COUNTMODE"]
    #[inline(always)]
    #[must_use]
    pub fn countmode(&mut self) -> COUNTMODE_W<CFGRrs> {
        COUNTMODE_W::new(self, 23)
    }
    #[doc = "Bit 24 - ENC"]
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<CFGRrs> {
        ENC_W::new(self, 24)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
