///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**Capture/compare preloaded control

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPC {
    ///0: CCxE, CCxNE and OCxM bits are not preloaded
    NotPreloaded = 0,
    ///1: CCxE, CCxNE and OCxM bits are preloaded
    Preloaded = 1,
}
impl From<CCPC> for bool {
    #[inline(always)]
    fn from(variant: CCPC) -> Self {
        variant as u8 != 0
    }
}
///Field `CCPC` reader - Capture/compare preloaded control
pub type CCPC_R = crate::BitReader<CCPC>;
impl CCPC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCPC {
        match self.bits {
            false => CCPC::NotPreloaded,
            true => CCPC::Preloaded,
        }
    }
    ///CCxE, CCxNE and OCxM bits are not preloaded
    #[inline(always)]
    pub fn is_not_preloaded(&self) -> bool {
        *self == CCPC::NotPreloaded
    }
    ///CCxE, CCxNE and OCxM bits are preloaded
    #[inline(always)]
    pub fn is_preloaded(&self) -> bool {
        *self == CCPC::Preloaded
    }
}
///Field `CCPC` writer - Capture/compare preloaded control
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG, CCPC>;
impl<'a, REG> CCPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CCxE, CCxNE and OCxM bits are not preloaded
    #[inline(always)]
    pub fn not_preloaded(self) -> &'a mut crate::W<REG> {
        self.variant(CCPC::NotPreloaded)
    }
    ///CCxE, CCxNE and OCxM bits are preloaded
    #[inline(always)]
    pub fn preloaded(self) -> &'a mut crate::W<REG> {
        self.variant(CCPC::Preloaded)
    }
}
/**Capture/compare control update selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUS {
    ///0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only
    Sw = 0,
    ///1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI
    SwOrEdge = 1,
}
impl From<CCUS> for bool {
    #[inline(always)]
    fn from(variant: CCUS) -> Self {
        variant as u8 != 0
    }
}
///Field `CCUS` reader - Capture/compare control update selection
pub type CCUS_R = crate::BitReader<CCUS>;
impl CCUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCUS {
        match self.bits {
            false => CCUS::Sw,
            true => CCUS::SwOrEdge,
        }
    }
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == CCUS::Sw
    }
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI
    #[inline(always)]
    pub fn is_sw_or_edge(&self) -> bool {
        *self == CCUS::SwOrEdge
    }
}
///Field `CCUS` writer - Capture/compare control update selection
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG, CCUS>;
impl<'a, REG> CCUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS::Sw)
    }
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI
    #[inline(always)]
    pub fn sw_or_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS::SwOrEdge)
    }
}
/**Capture/compare DMA selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCDS {
    ///0: CCx DMA request sent when CCx event occurs
    OnCompare = 0,
    ///1: CCx DMA request sent when update event occurs
    OnUpdate = 1,
}
impl From<CCDS> for bool {
    #[inline(always)]
    fn from(variant: CCDS) -> Self {
        variant as u8 != 0
    }
}
///Field `CCDS` reader - Capture/compare DMA selection
pub type CCDS_R = crate::BitReader<CCDS>;
impl CCDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CCDS {
        match self.bits {
            false => CCDS::OnCompare,
            true => CCDS::OnUpdate,
        }
    }
    ///CCx DMA request sent when CCx event occurs
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == CCDS::OnCompare
    }
    ///CCx DMA request sent when update event occurs
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == CCDS::OnUpdate
    }
}
///Field `CCDS` writer - Capture/compare DMA selection
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG, CCDS>;
impl<'a, REG> CCDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CCx DMA request sent when CCx event occurs
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut crate::W<REG> {
        self.variant(CCDS::OnCompare)
    }
    ///CCx DMA request sent when update event occurs
    #[inline(always)]
    pub fn on_update(self) -> &'a mut crate::W<REG> {
        self.variant(CCDS::OnUpdate)
    }
}
/**Master mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS {
    ///0: The UG bit from the TIMx_EGR register is used as trigger output
    Reset = 0,
    ///1: The counter enable signal, CNT_EN, is used as trigger output
    Enable = 1,
    ///2: The update event is selected as trigger output
    Update = 2,
    ///3: The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred
    ComparePulse = 3,
    ///4: OC1REF signal is used as trigger output
    CompareOc1 = 4,
    ///5: OC2REF signal is used as trigger output
    CompareOc2 = 5,
    ///6: OC3REF signal is used as trigger output
    CompareOc3 = 6,
    ///7: OC4REF signal is used as trigger output
    CompareOc4 = 7,
}
impl From<MMS> for u8 {
    #[inline(always)]
    fn from(variant: MMS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMS {
    type Ux = u8;
}
impl crate::IsEnum for MMS {}
///Field `MMS` reader - Master mode selection
pub type MMS_R = crate::FieldReader<MMS>;
impl MMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MMS {
        match self.bits {
            0 => MMS::Reset,
            1 => MMS::Enable,
            2 => MMS::Update,
            3 => MMS::ComparePulse,
            4 => MMS::CompareOc1,
            5 => MMS::CompareOc2,
            6 => MMS::CompareOc3,
            7 => MMS::CompareOc4,
            _ => unreachable!(),
        }
    }
    ///The UG bit from the TIMx_EGR register is used as trigger output
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS::Reset
    }
    ///The counter enable signal, CNT_EN, is used as trigger output
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS::Enable
    }
    ///The update event is selected as trigger output
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS::Update
    }
    ///The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMS::ComparePulse
    }
    ///OC1REF signal is used as trigger output
    #[inline(always)]
    pub fn is_compare_oc1(&self) -> bool {
        *self == MMS::CompareOc1
    }
    ///OC2REF signal is used as trigger output
    #[inline(always)]
    pub fn is_compare_oc2(&self) -> bool {
        *self == MMS::CompareOc2
    }
    ///OC3REF signal is used as trigger output
    #[inline(always)]
    pub fn is_compare_oc3(&self) -> bool {
        *self == MMS::CompareOc3
    }
    ///OC4REF signal is used as trigger output
    #[inline(always)]
    pub fn is_compare_oc4(&self) -> bool {
        *self == MMS::CompareOc4
    }
}
///Field `MMS` writer - Master mode selection
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MMS, crate::Safe>;
impl<'a, REG> MMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The UG bit from the TIMx_EGR register is used as trigger output
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Reset)
    }
    ///The counter enable signal, CNT_EN, is used as trigger output
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Enable)
    }
    ///The update event is selected as trigger output
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Update)
    }
    ///The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::ComparePulse)
    }
    ///OC1REF signal is used as trigger output
    #[inline(always)]
    pub fn compare_oc1(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::CompareOc1)
    }
    ///OC2REF signal is used as trigger output
    #[inline(always)]
    pub fn compare_oc2(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::CompareOc2)
    }
    ///OC3REF signal is used as trigger output
    #[inline(always)]
    pub fn compare_oc3(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::CompareOc3)
    }
    ///OC4REF signal is used as trigger output
    #[inline(always)]
    pub fn compare_oc4(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::CompareOc4)
    }
}
/**TI1 selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1S {
    ///0: The TIMx_CH1 pin is connected to TI1 input
    Normal = 0,
    ///1: The TIMx_CH1, CH2, CH3 pins are connected to TI1 input
    Xor = 1,
}
impl From<TI1S> for bool {
    #[inline(always)]
    fn from(variant: TI1S) -> Self {
        variant as u8 != 0
    }
}
///Field `TI1S` reader - TI1 selection
pub type TI1S_R = crate::BitReader<TI1S>;
impl TI1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TI1S {
        match self.bits {
            false => TI1S::Normal,
            true => TI1S::Xor,
        }
    }
    ///The TIMx_CH1 pin is connected to TI1 input
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TI1S::Normal
    }
    ///The TIMx_CH1, CH2, CH3 pins are connected to TI1 input
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == TI1S::Xor
    }
}
///Field `TI1S` writer - TI1 selection
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG, TI1S>;
impl<'a, REG> TI1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The TIMx_CH1 pin is connected to TI1 input
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S::Normal)
    }
    ///The TIMx_CH1, CH2, CH3 pins are connected to TI1 input
    #[inline(always)]
    pub fn xor(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S::Xor)
    }
}
/**Output Idle state (OC%s output)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1 {
    ///0: OCx=0 (after a dead-time if OCx(N) is implemented) when MOE=0
    Reset = 0,
    ///1: OCx=1 (after a dead-time if OCx(N) is implemented) when MOE=0
    Set = 1,
}
impl From<OIS1> for bool {
    #[inline(always)]
    fn from(variant: OIS1) -> Self {
        variant as u8 != 0
    }
}
///Field `OIS(1-6)` reader - Output Idle state (OC%s output)
pub type OIS_R = crate::BitReader<OIS1>;
impl OIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OIS1 {
        match self.bits {
            false => OIS1::Reset,
            true => OIS1::Set,
        }
    }
    ///OCx=0 (after a dead-time if OCx(N) is implemented) when MOE=0
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OIS1::Reset
    }
    ///OCx=1 (after a dead-time if OCx(N) is implemented) when MOE=0
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == OIS1::Set
    }
}
///Field `OIS(1-6)` writer - Output Idle state (OC%s output)
pub type OIS_W<'a, REG> = crate::BitWriter<'a, REG, OIS1>;
impl<'a, REG> OIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OCx=0 (after a dead-time if OCx(N) is implemented) when MOE=0
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1::Reset)
    }
    ///OCx=1 (after a dead-time if OCx(N) is implemented) when MOE=0
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1::Set)
    }
}
/**Output Idle state (OC%sN output)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1N {
    ///0: OCxN=0 after a dead-time when MOE=0
    Reset = 0,
    ///1: OCxN=1 after a dead-time when MOE=0
    Set = 1,
}
impl From<OIS1N> for bool {
    #[inline(always)]
    fn from(variant: OIS1N) -> Self {
        variant as u8 != 0
    }
}
///Field `OISN(1-3)` reader - Output Idle state (OC%sN output)
pub type OISN_R = crate::BitReader<OIS1N>;
impl OISN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OIS1N {
        match self.bits {
            false => OIS1N::Reset,
            true => OIS1N::Set,
        }
    }
    ///OCxN=0 after a dead-time when MOE=0
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OIS1N::Reset
    }
    ///OCxN=1 after a dead-time when MOE=0
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == OIS1N::Set
    }
}
///Field `OISN(1-3)` writer - Output Idle state (OC%sN output)
pub type OISN_W<'a, REG> = crate::BitWriter<'a, REG, OIS1N>;
impl<'a, REG> OISN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OCxN=0 after a dead-time when MOE=0
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1N::Reset)
    }
    ///OCxN=1 after a dead-time when MOE=0
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1N::Set)
    }
}
///Field `MMS2` reader - Master mode selection 2
pub type MMS2_R = crate::FieldReader;
///Field `MMS2` writer - Master mode selection 2
pub type MMS2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Output Idle state (OC(1-6) output)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OIS1` field.</div>
    #[inline(always)]
    pub fn ois(&self, n: u8) -> OIS_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        OIS_R::new(((self.bits >> (n * 2 + 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output Idle state (OC(1-6) output)
    #[inline(always)]
    pub fn ois_iter(&self) -> impl Iterator<Item = OIS_R> + '_ {
        (0..6).map(move |n| OIS_R::new(((self.bits >> (n * 2 + 8)) & 1) != 0))
    }
    ///Bit 8 - Output Idle state (OC1 output)
    #[inline(always)]
    pub fn ois1(&self) -> OIS_R {
        OIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Output Idle state (OC2 output)
    #[inline(always)]
    pub fn ois2(&self) -> OIS_R {
        OIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Output Idle state (OC3 output)
    #[inline(always)]
    pub fn ois3(&self) -> OIS_R {
        OIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Output Idle state (OC4 output)
    #[inline(always)]
    pub fn ois4(&self) -> OIS_R {
        OIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Output Idle state (OC5 output)
    #[inline(always)]
    pub fn ois5(&self) -> OIS_R {
        OIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Output Idle state (OC6 output)
    #[inline(always)]
    pub fn ois6(&self) -> OIS_R {
        OIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Output Idle state (OC(1-3)N output)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OIS1N` field.</div>
    #[inline(always)]
    pub fn oisn(&self, n: u8) -> OISN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OISN_R::new(((self.bits >> (n * 2 + 9)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output Idle state (OC(1-3)N output)
    #[inline(always)]
    pub fn oisn_iter(&self) -> impl Iterator<Item = OISN_R> + '_ {
        (0..3).map(move |n| OISN_R::new(((self.bits >> (n * 2 + 9)) & 1) != 0))
    }
    ///Bit 9 - Output Idle state (OC1N output)
    #[inline(always)]
    pub fn ois1n(&self) -> OISN_R {
        OISN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Output Idle state (OC2N output)
    #[inline(always)]
    pub fn ois2n(&self) -> OISN_R {
        OISN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - Output Idle state (OC3N output)
    #[inline(always)]
    pub fn ois3n(&self) -> OISN_R {
        OISN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 20:23 - Master mode selection 2
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("mms2", &self.mms2())
            .field("ois1", &self.ois1())
            .field("ois2", &self.ois2())
            .field("ois3", &self.ois3())
            .field("ois4", &self.ois4())
            .field("ois5", &self.ois5())
            .field("ois6", &self.ois6())
            .field("ois1n", &self.ois1n())
            .field("ois2n", &self.ois2n())
            .field("ois3n", &self.ois3n())
            .field("ti1s", &self.ti1s())
            .field("mms", &self.mms())
            .field("ccds", &self.ccds())
            .field("ccus", &self.ccus())
            .field("ccpc", &self.ccpc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&mut self) -> CCPC_W<CR2rs> {
        CCPC_W::new(self, 0)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&mut self) -> CCUS_W<CR2rs> {
        CCUS_W::new(self, 2)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&mut self) -> CCDS_W<CR2rs> {
        CCDS_W::new(self, 3)
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<CR2rs> {
        MMS_W::new(self, 4)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&mut self) -> TI1S_W<CR2rs> {
        TI1S_W::new(self, 7)
    }
    ///Output Idle state (OC(1-6) output)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OIS1` field.</div>
    #[inline(always)]
    pub fn ois(&mut self, n: u8) -> OIS_W<CR2rs> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        OIS_W::new(self, n * 2 + 8)
    }
    ///Bit 8 - Output Idle state (OC1 output)
    #[inline(always)]
    pub fn ois1(&mut self) -> OIS_W<CR2rs> {
        OIS_W::new(self, 8)
    }
    ///Bit 10 - Output Idle state (OC2 output)
    #[inline(always)]
    pub fn ois2(&mut self) -> OIS_W<CR2rs> {
        OIS_W::new(self, 10)
    }
    ///Bit 12 - Output Idle state (OC3 output)
    #[inline(always)]
    pub fn ois3(&mut self) -> OIS_W<CR2rs> {
        OIS_W::new(self, 12)
    }
    ///Bit 14 - Output Idle state (OC4 output)
    #[inline(always)]
    pub fn ois4(&mut self) -> OIS_W<CR2rs> {
        OIS_W::new(self, 14)
    }
    ///Bit 16 - Output Idle state (OC5 output)
    #[inline(always)]
    pub fn ois5(&mut self) -> OIS_W<CR2rs> {
        OIS_W::new(self, 16)
    }
    ///Bit 18 - Output Idle state (OC6 output)
    #[inline(always)]
    pub fn ois6(&mut self) -> OIS_W<CR2rs> {
        OIS_W::new(self, 18)
    }
    ///Output Idle state (OC(1-3)N output)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OIS1N` field.</div>
    #[inline(always)]
    pub fn oisn(&mut self, n: u8) -> OISN_W<CR2rs> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OISN_W::new(self, n * 2 + 9)
    }
    ///Bit 9 - Output Idle state (OC1N output)
    #[inline(always)]
    pub fn ois1n(&mut self) -> OISN_W<CR2rs> {
        OISN_W::new(self, 9)
    }
    ///Bit 11 - Output Idle state (OC2N output)
    #[inline(always)]
    pub fn ois2n(&mut self) -> OISN_W<CR2rs> {
        OISN_W::new(self, 11)
    }
    ///Bit 13 - Output Idle state (OC3N output)
    #[inline(always)]
    pub fn ois3n(&mut self) -> OISN_W<CR2rs> {
        OISN_W::new(self, 13)
    }
    ///Bits 20:23 - Master mode selection 2
    #[inline(always)]
    pub fn mms2(&mut self) -> MMS2_W<CR2rs> {
        MMS2_W::new(self, 20)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x3.html#TIM1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
