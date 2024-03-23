#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPC {
    #[doc = "0: CCxE, CCxNE and OCxM bits are not preloaded"]
    NotPreloaded = 0,
    #[doc = "1: CCxE, CCxNE and OCxM bits are preloaded"]
    Preloaded = 1,
}
impl From<CCPC> for bool {
    #[inline(always)]
    fn from(variant: CCPC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPC` reader - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CCPC_R = crate::BitReader<CCPC>;
impl CCPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCPC {
        match self.bits {
            false => CCPC::NotPreloaded,
            true => CCPC::Preloaded,
        }
    }
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline(always)]
    pub fn is_not_preloaded(&self) -> bool {
        *self == CCPC::NotPreloaded
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded"]
    #[inline(always)]
    pub fn is_preloaded(&self) -> bool {
        *self == CCPC::Preloaded
    }
}
#[doc = "Field `CCPC` writer - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
pub type CCPC_W<'a, REG> = crate::BitWriter<'a, REG, CCPC>;
impl<'a, REG> CCPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline(always)]
    pub fn not_preloaded(self) -> &'a mut crate::W<REG> {
        self.variant(CCPC::NotPreloaded)
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded"]
    #[inline(always)]
    pub fn preloaded(self) -> &'a mut crate::W<REG> {
        self.variant(CCPC::Preloaded)
    }
}
#[doc = "Capture/compare control update selection Note: This bit acts only on channels that have a complementary output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUS {
    #[doc = "0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    Bit = 0,
    #[doc = "1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    BitOrEdge = 1,
}
impl From<CCUS> for bool {
    #[inline(always)]
    fn from(variant: CCUS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUS` reader - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CCUS_R = crate::BitReader<CCUS>;
impl CCUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCUS {
        match self.bits {
            false => CCUS::Bit,
            true => CCUS::BitOrEdge,
        }
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    #[inline(always)]
    pub fn is_bit(&self) -> bool {
        *self == CCUS::Bit
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn is_bit_or_edge(&self) -> bool {
        *self == CCUS::BitOrEdge
    }
}
#[doc = "Field `CCUS` writer - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG, CCUS>;
impl<'a, REG> CCUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only"]
    #[inline(always)]
    pub fn bit_(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS::Bit)
    }
    #[doc = "When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn bit_or_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS::BitOrEdge)
    }
}
#[doc = "Capture/compare DMA selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCDS {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    OnCompare = 0,
    #[doc = "1: CCx DMA request sent when update event occurs"]
    OnUpdate = 1,
}
impl From<CCDS> for bool {
    #[inline(always)]
    fn from(variant: CCDS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CCDS_R = crate::BitReader<CCDS>;
impl CCDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCDS {
        match self.bits {
            false => CCDS::OnCompare,
            true => CCDS::OnUpdate,
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == CCDS::OnCompare
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == CCDS::OnUpdate
    }
}
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CCDS_W<'a, REG> = crate::BitWriter<'a, REG, CCDS>;
impl<'a, REG> CCDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut crate::W<REG> {
        self.variant(CCDS::OnCompare)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn on_update(self) -> &'a mut crate::W<REG> {
        self.variant(CCDS::OnUpdate)
    }
}
#[doc = "Field `MMS_L` reader - MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_L_R = crate::FieldReader;
#[doc = "Field `MMS_L` writer - MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS_L_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "tim_ti1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1S {
    #[doc = "0: The TIMx_CH1 pin is connected to TI1 input"]
    Normal = 0,
    #[doc = "1: The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    Xor = 1,
}
impl From<TI1S> for bool {
    #[inline(always)]
    fn from(variant: TI1S) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI1S` reader - tim_ti1 selection"]
pub type TI1S_R = crate::BitReader<TI1S>;
impl TI1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TI1S {
        match self.bits {
            false => TI1S::Normal,
            true => TI1S::Xor,
        }
    }
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TI1S::Normal
    }
    #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == TI1S::Xor
    }
}
#[doc = "Field `TI1S` writer - tim_ti1 selection"]
pub type TI1S_W<'a, REG> = crate::BitWriter<'a, REG, TI1S>;
impl<'a, REG> TI1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S::Normal)
    }
    #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    #[inline(always)]
    pub fn xor(self) -> &'a mut crate::W<REG> {
        self.variant(TI1S::Xor)
    }
}
#[doc = "Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1 {
    #[doc = "0: OCx=0 (after a dead-time if OCx(N) is implemented) when MOE=0"]
    Reset = 0,
    #[doc = "1: OCx=1 (after a dead-time if OCx(N) is implemented) when MOE=0"]
    Set = 1,
}
impl From<OIS1> for bool {
    #[inline(always)]
    fn from(variant: OIS1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1` reader - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1_R = crate::BitReader<OIS1>;
impl OIS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OIS1 {
        match self.bits {
            false => OIS1::Reset,
            true => OIS1::Set,
        }
    }
    #[doc = "OCx=0 (after a dead-time if OCx(N) is implemented) when MOE=0"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OIS1::Reset
    }
    #[doc = "OCx=1 (after a dead-time if OCx(N) is implemented) when MOE=0"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == OIS1::Set
    }
}
#[doc = "Field `OIS1` writer - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG, OIS1>;
impl<'a, REG> OIS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OCx=0 (after a dead-time if OCx(N) is implemented) when MOE=0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1::Reset)
    }
    #[doc = "OCx=1 (after a dead-time if OCx(N) is implemented) when MOE=0"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1::Set)
    }
}
#[doc = "Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1N {
    #[doc = "0: OCxN=0 after a dead-time when MOE=0"]
    Reset = 0,
    #[doc = "1: OCxN=1 after a dead-time when MOE=0"]
    Set = 1,
}
impl From<OIS1N> for bool {
    #[inline(always)]
    fn from(variant: OIS1N) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1N` reader - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1N_R = crate::BitReader<OIS1N>;
impl OIS1N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OIS1N {
        match self.bits {
            false => OIS1N::Reset,
            true => OIS1N::Set,
        }
    }
    #[doc = "OCxN=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OIS1N::Reset
    }
    #[doc = "OCxN=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == OIS1N::Set
    }
}
#[doc = "Field `OIS1N` writer - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG, OIS1N>;
impl<'a, REG> OIS1N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OCxN=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1N::Reset)
    }
    #[doc = "OCxN=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1N::Set)
    }
}
#[doc = "Field `OIS2N` reader - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit"]
pub use OIS1N_R as OIS2N_R;
#[doc = "Field `OIS3N` reader - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit"]
pub use OIS1N_R as OIS3N_R;
#[doc = "Field `OIS4N` reader - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit"]
pub use OIS1N_R as OIS4N_R;
#[doc = "Field `OIS2N` writer - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit"]
pub use OIS1N_W as OIS2N_W;
#[doc = "Field `OIS3N` writer - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit"]
pub use OIS1N_W as OIS3N_W;
#[doc = "Field `OIS4N` writer - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit"]
pub use OIS1N_W as OIS4N_W;
#[doc = "Field `OIS2` reader - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit"]
pub use OIS1_R as OIS2_R;
#[doc = "Field `OIS3` reader - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit"]
pub use OIS1_R as OIS3_R;
#[doc = "Field `OIS4` reader - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit"]
pub use OIS1_R as OIS4_R;
#[doc = "Field `OIS5` reader - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit"]
pub use OIS1_R as OIS5_R;
#[doc = "Field `OIS6` reader - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit"]
pub use OIS1_R as OIS6_R;
#[doc = "Field `OIS2` writer - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit"]
pub use OIS1_W as OIS2_W;
#[doc = "Field `OIS3` writer - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit"]
pub use OIS1_W as OIS3_W;
#[doc = "Field `OIS4` writer - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit"]
pub use OIS1_W as OIS4_W;
#[doc = "Field `OIS5` writer - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit"]
pub use OIS1_W as OIS5_W;
#[doc = "Field `OIS6` writer - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit"]
pub use OIS1_W as OIS6_W;
#[doc = "Field `MMS2` reader - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS2_R = crate::FieldReader;
#[doc = "Field `MMS2` writer - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
pub type MMS2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MMS_H` reader - MMS\\[3\\]"]
pub type MMS_H_R = crate::BitReader;
#[doc = "Field `MMS_H` writer - MMS\\[3\\]"]
pub type MMS_H_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms_l(&self) -> MMS_L_R {
        MMS_L_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - tim_ti1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit"]
    #[inline(always)]
    pub fn ois4n(&self) -> OIS4N_R {
        OIS4N_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit"]
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - MMS\\[3\\]"]
    #[inline(always)]
    pub fn mms_h(&self) -> MMS_H_R {
        MMS_H_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<CR2rs> {
        CCPC_W::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection Note: This bit acts only on channels that have a complementary output."]
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<CR2rs> {
        CCUS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<CR2rs> {
        CCDS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - MMS\\[2:0\\]: Master mode selection These bits select the information to be sent in master mode to slave timers for synchronization (tim_trgo). The combination is as follows: Other codes reserved Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms_l(&mut self) -> MMS_L_W<CR2rs> {
        MMS_L_W::new(self, 4)
    }
    #[doc = "Bit 7 - tim_ti1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<CR2rs> {
        TI1S_W::new(self, 7)
    }
    #[doc = "Bit 8 - Output idle state 1 (tim_oc1 output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<CR2rs> {
        OIS1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Output idle state 1 (tim_oc1n output) Note: This bit can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<CR2rs> {
        OIS1N_W::new(self, 9)
    }
    #[doc = "Bit 10 - Output idle state 2 (tim_oc2 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> OIS2_W<CR2rs> {
        OIS2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Output idle state 2 (tim_oc2n output) Refer to OIS1N bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois2n(&mut self) -> OIS2N_W<CR2rs> {
        OIS2N_W::new(self, 11)
    }
    #[doc = "Bit 12 - Output idle state 3 (tim_oc3n output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois3(&mut self) -> OIS3_W<CR2rs> {
        OIS3_W::new(self, 12)
    }
    #[doc = "Bit 13 - Output idle state 3 (tim_oc3n output) Refer to OIS1N bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois3n(&mut self) -> OIS3N_W<CR2rs> {
        OIS3N_W::new(self, 13)
    }
    #[doc = "Bit 14 - Output idle state 4 (tim_oc4 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois4(&mut self) -> OIS4_W<CR2rs> {
        OIS4_W::new(self, 14)
    }
    #[doc = "Bit 15 - Output idle state 4 (tim_oc4n output) Refer to OIS1N bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois4n(&mut self) -> OIS4N_W<CR2rs> {
        OIS4N_W::new(self, 15)
    }
    #[doc = "Bit 16 - Output idle state 5 (tim_oc5 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois5(&mut self) -> OIS5_W<CR2rs> {
        OIS5_W::new(self, 16)
    }
    #[doc = "Bit 18 - Output idle state 6 (tim_oc6 output) Refer to OIS1 bit"]
    #[inline(always)]
    #[must_use]
    pub fn ois6(&mut self) -> OIS6_W<CR2rs> {
        OIS6_W::new(self, 18)
    }
    #[doc = "Bits 20:23 - Master mode selection 2 These bits allow the information to be sent to ADC for synchronization (tim_trgo2) to be selected. The combination is as follows: Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn mms2(&mut self) -> MMS2_W<CR2rs> {
        MMS2_W::new(self, 20)
    }
    #[doc = "Bit 25 - MMS\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn mms_h(&mut self) -> MMS_H_W<CR2rs> {
        MMS_H_W::new(self, 25)
    }
}
#[doc = "TIM1 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
