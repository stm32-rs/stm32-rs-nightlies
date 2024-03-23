#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Capture/compare preloaded control\n\nValue on reset: 0"]
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
#[doc = "Field `CCPC` reader - Capture/compare preloaded control"]
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
#[doc = "Field `CCPC` writer - Capture/compare preloaded control"]
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
#[doc = "Capture/compare control update selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUS {
    #[doc = "0: Capture/compare are updated only by setting the COMG bit"]
    Default = 0,
    #[doc = "1: Capture/compare are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    WithRisingEdge = 1,
}
impl From<CCUS> for bool {
    #[inline(always)]
    fn from(variant: CCUS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUS` reader - Capture/compare control update selection"]
pub type CCUS_R = crate::BitReader<CCUS>;
impl CCUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCUS {
        match self.bits {
            false => CCUS::Default,
            true => CCUS::WithRisingEdge,
        }
    }
    #[doc = "Capture/compare are updated only by setting the COMG bit"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == CCUS::Default
    }
    #[doc = "Capture/compare are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn is_with_rising_edge(&self) -> bool {
        *self == CCUS::WithRisingEdge
    }
}
#[doc = "Field `CCUS` writer - Capture/compare control update selection"]
pub type CCUS_W<'a, REG> = crate::BitWriter<'a, REG, CCUS>;
impl<'a, REG> CCUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare are updated only by setting the COMG bit"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS::Default)
    }
    #[doc = "Capture/compare are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    #[inline(always)]
    pub fn with_rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CCUS::WithRisingEdge)
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
#[doc = "Output Idle state 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1 {
    #[doc = "0: OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    Reset = 0,
    #[doc = "1: OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    Set = 1,
}
impl From<OIS1> for bool {
    #[inline(always)]
    fn from(variant: OIS1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1` reader - Output Idle state 1"]
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
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OIS1::Reset
    }
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == OIS1::Set
    }
}
#[doc = "Field `OIS1` writer - Output Idle state 1"]
pub type OIS1_W<'a, REG> = crate::BitWriter<'a, REG, OIS1>;
impl<'a, REG> OIS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1::Reset)
    }
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1::Set)
    }
}
#[doc = "Output Idle state 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1N {
    #[doc = "0: OC1N=0 after a dead-time when MOE=0"]
    Reset = 0,
    #[doc = "1: OC1N=1 after a dead-time when MOE=0"]
    Set = 1,
}
impl From<OIS1N> for bool {
    #[inline(always)]
    fn from(variant: OIS1N) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OIS1N` reader - Output Idle state 1"]
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
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OIS1N::Reset
    }
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == OIS1N::Set
    }
}
#[doc = "Field `OIS1N` writer - Output Idle state 1"]
pub type OIS1N_W<'a, REG> = crate::BitWriter<'a, REG, OIS1N>;
impl<'a, REG> OIS1N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1N::Reset)
    }
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(OIS1N::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<CR2rs> {
        CCPC_W::new(self, 0)
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
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
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<CR2rs> {
        OIS1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<CR2rs> {
        OIS1N_W::new(self, 9)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
