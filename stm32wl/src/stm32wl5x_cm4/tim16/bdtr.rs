#[doc = "Register `BDTR` reader"]
pub type R = crate::R<BDTRrs>;
#[doc = "Register `BDTR` writer"]
pub type W = crate::W<BDTRrs>;
#[doc = "Field `DTG` reader - Dead-time generator setup"]
pub type DTG_R = crate::FieldReader;
#[doc = "Field `DTG` writer - Dead-time generator setup"]
pub type DTG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Lock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK {
    #[doc = "0: No write protection"]
    Off = 0,
    #[doc = "1: Level 1 write protection"]
    Level1 = 1,
    #[doc = "2: Level 2 write protection"]
    Level2 = 2,
    #[doc = "3: Level 3 write protection"]
    Level3 = 3,
}
impl From<LOCK> for u8 {
    #[inline(always)]
    fn from(variant: LOCK) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCK {
    type Ux = u8;
}
#[doc = "Field `LOCK` reader - Lock configuration"]
pub type LOCK_R = crate::FieldReader<LOCK>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCK {
        match self.bits {
            0 => LOCK::Off,
            1 => LOCK::Level1,
            2 => LOCK::Level2,
            3 => LOCK::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "No write protection"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LOCK::Off
    }
    #[doc = "Level 1 write protection"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == LOCK::Level1
    }
    #[doc = "Level 2 write protection"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == LOCK::Level2
    }
    #[doc = "Level 3 write protection"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == LOCK::Level3
    }
}
#[doc = "Field `LOCK` writer - Lock configuration"]
pub type LOCK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LOCK>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No write protection"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Off)
    }
    #[doc = "Level 1 write protection"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level1)
    }
    #[doc = "Level 2 write protection"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level2)
    }
    #[doc = "Level 3 write protection"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut crate::W<REG> {
        self.variant(LOCK::Level3)
    }
}
#[doc = "Off-state selection for Idle mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSI {
    #[doc = "0: OC/OCN outputs are disabled when inactive"]
    Disabled = 0,
    #[doc = "1: OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime"]
    Enabled = 1,
}
impl From<OSSI> for bool {
    #[inline(always)]
    fn from(variant: OSSI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSSI` reader - Off-state selection for Idle mode"]
pub type OSSI_R = crate::BitReader<OSSI>;
impl OSSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSSI {
        match self.bits {
            false => OSSI::Disabled,
            true => OSSI::Enabled,
        }
    }
    #[doc = "OC/OCN outputs are disabled when inactive"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSI::Disabled
    }
    #[doc = "OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSSI::Enabled
    }
}
#[doc = "Field `OSSI` writer - Off-state selection for Idle mode"]
pub type OSSI_W<'a, REG> = crate::BitWriter<'a, REG, OSSI>;
impl<'a, REG> OSSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC/OCN outputs are disabled when inactive"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::Disabled)
    }
    #[doc = "OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSI::Enabled)
    }
}
#[doc = "Off-state selection for Run mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSR {
    #[doc = "0: OC/OCN outputs are disabled when inactive"]
    Disabled = 0,
    #[doc = "1: OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1"]
    Enabled = 1,
}
impl From<OSSR> for bool {
    #[inline(always)]
    fn from(variant: OSSR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSSR` reader - Off-state selection for Run mode"]
pub type OSSR_R = crate::BitReader<OSSR>;
impl OSSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSSR {
        match self.bits {
            false => OSSR::Disabled,
            true => OSSR::Enabled,
        }
    }
    #[doc = "OC/OCN outputs are disabled when inactive"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSR::Disabled
    }
    #[doc = "OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSSR::Enabled
    }
}
#[doc = "Field `OSSR` writer - Off-state selection for Run mode"]
pub type OSSR_W<'a, REG> = crate::BitWriter<'a, REG, OSSR>;
impl<'a, REG> OSSR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC/OCN outputs are disabled when inactive"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::Disabled)
    }
    #[doc = "OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSSR::Enabled)
    }
}
#[doc = "Break enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKE {
    #[doc = "0: Break inputs (BRK and CCS clock failure event) disabled"]
    Disabled = 0,
    #[doc = "1: Break inputs (BRK and CCS clock failure event) enabled"]
    Enabled = 1,
}
impl From<BKE> for bool {
    #[inline(always)]
    fn from(variant: BKE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKE` reader - Break enable"]
pub type BKE_R = crate::BitReader<BKE>;
impl BKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKE {
        match self.bits {
            false => BKE::Disabled,
            true => BKE::Enabled,
        }
    }
    #[doc = "Break inputs (BRK and CCS clock failure event) disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKE::Disabled
    }
    #[doc = "Break inputs (BRK and CCS clock failure event) enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKE::Enabled
    }
}
#[doc = "Field `BKE` writer - Break enable"]
pub type BKE_W<'a, REG> = crate::BitWriter<'a, REG, BKE>;
impl<'a, REG> BKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break inputs (BRK and CCS clock failure event) disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::Disabled)
    }
    #[doc = "Break inputs (BRK and CCS clock failure event) enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BKE::Enabled)
    }
}
#[doc = "Break polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP {
    #[doc = "0: Break input BRK is active low"]
    ActiveLow = 0,
    #[doc = "1: Break input BRK is active high"]
    ActiveHigh = 1,
}
impl From<BKP> for bool {
    #[inline(always)]
    fn from(variant: BKP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKP` reader - Break polarity"]
pub type BKP_R = crate::BitReader<BKP>;
impl BKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKP {
        match self.bits {
            false => BKP::ActiveLow,
            true => BKP::ActiveHigh,
        }
    }
    #[doc = "Break input BRK is active low"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == BKP::ActiveLow
    }
    #[doc = "Break input BRK is active high"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == BKP::ActiveHigh
    }
}
#[doc = "Field `BKP` writer - Break polarity"]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG, BKP>;
impl<'a, REG> BKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::ActiveLow)
    }
    #[doc = "Break input BRK is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::ActiveHigh)
    }
}
#[doc = "Automatic output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOE {
    #[doc = "0: MOE can be set only by software"]
    Disabled = 0,
    #[doc = "1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    Enabled = 1,
}
impl From<AOE> for bool {
    #[inline(always)]
    fn from(variant: AOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AOE` reader - Automatic output enable"]
pub type AOE_R = crate::BitReader<AOE>;
impl AOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AOE {
        match self.bits {
            false => AOE::Disabled,
            true => AOE::Enabled,
        }
    }
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AOE::Disabled
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AOE::Enabled
    }
}
#[doc = "Field `AOE` writer - Automatic output enable"]
pub type AOE_W<'a, REG> = crate::BitWriter<'a, REG, AOE>;
impl<'a, REG> AOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MOE can be set only by software"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::Disabled)
    }
    #[doc = "MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AOE::Enabled)
    }
}
#[doc = "Main output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOE {
    #[doc = "0: OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit"]
    Disabled = 0,
    #[doc = "1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)"]
    Enabled = 1,
}
impl From<MOE> for bool {
    #[inline(always)]
    fn from(variant: MOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOE` reader - Main output enable"]
pub type MOE_R = crate::BitReader<MOE>;
impl MOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MOE {
        match self.bits {
            false => MOE::Disabled,
            true => MOE::Enabled,
        }
    }
    #[doc = "OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MOE::Disabled
    }
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MOE::Enabled
    }
}
#[doc = "Field `MOE` writer - Main output enable"]
pub type MOE_W<'a, REG> = crate::BitWriter<'a, REG, MOE>;
impl<'a, REG> MOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::Disabled)
    }
    #[doc = "OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MOE::Enabled)
    }
}
#[doc = "Break Disarm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKDSRM {
    #[doc = "0: Break input BRK is armed"]
    Armed = 0,
    #[doc = "1: Break input BRK is disarmed"]
    Disarmed = 1,
}
impl From<BKDSRM> for bool {
    #[inline(always)]
    fn from(variant: BKDSRM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKDSRM` reader - Break Disarm"]
pub type BKDSRM_R = crate::BitReader<BKDSRM>;
impl BKDSRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKDSRM {
        match self.bits {
            false => BKDSRM::Armed,
            true => BKDSRM::Disarmed,
        }
    }
    #[doc = "Break input BRK is armed"]
    #[inline(always)]
    pub fn is_armed(&self) -> bool {
        *self == BKDSRM::Armed
    }
    #[doc = "Break input BRK is disarmed"]
    #[inline(always)]
    pub fn is_disarmed(&self) -> bool {
        *self == BKDSRM::Disarmed
    }
}
#[doc = "Field `BKDSRM` writer - Break Disarm"]
pub type BKDSRM_W<'a, REG> = crate::BitWriter<'a, REG, BKDSRM>;
impl<'a, REG> BKDSRM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK is armed"]
    #[inline(always)]
    pub fn armed(self) -> &'a mut crate::W<REG> {
        self.variant(BKDSRM::Armed)
    }
    #[doc = "Break input BRK is disarmed"]
    #[inline(always)]
    pub fn disarmed(self) -> &'a mut crate::W<REG> {
        self.variant(BKDSRM::Disarmed)
    }
}
#[doc = "Break Bidirectional\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKBID {
    #[doc = "0: Break input BRK in input mode"]
    Input = 0,
    #[doc = "1: Break input BRK in bidirectional mode"]
    Bidirectional = 1,
}
impl From<BKBID> for bool {
    #[inline(always)]
    fn from(variant: BKBID) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKBID` reader - Break Bidirectional"]
pub type BKBID_R = crate::BitReader<BKBID>;
impl BKBID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKBID {
        match self.bits {
            false => BKBID::Input,
            true => BKBID::Bidirectional,
        }
    }
    #[doc = "Break input BRK in input mode"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == BKBID::Input
    }
    #[doc = "Break input BRK in bidirectional mode"]
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BKBID::Bidirectional
    }
}
#[doc = "Field `BKBID` writer - Break Bidirectional"]
pub type BKBID_W<'a, REG> = crate::BitWriter<'a, REG, BKBID>;
impl<'a, REG> BKBID_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break input BRK in input mode"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(BKBID::Input)
    }
    #[doc = "Break input BRK in bidirectional mode"]
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut crate::W<REG> {
        self.variant(BKBID::Bidirectional)
    }
}
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 26 - Break Disarm"]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Break Bidirectional"]
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup"]
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<BDTRrs> {
        DTG_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Lock configuration"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<BDTRrs> {
        LOCK_W::new(self, 8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode"]
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<BDTRrs> {
        OSSI_W::new(self, 10)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode"]
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<BDTRrs> {
        OSSR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Break enable"]
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<BDTRrs> {
        BKE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Break polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<BDTRrs> {
        BKP_W::new(self, 13)
    }
    #[doc = "Bit 14 - Automatic output enable"]
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<BDTRrs> {
        AOE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Main output enable"]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<BDTRrs> {
        MOE_W::new(self, 15)
    }
    #[doc = "Bit 26 - Break Disarm"]
    #[inline(always)]
    #[must_use]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<BDTRrs> {
        BKDSRM_W::new(self, 26)
    }
    #[doc = "Bit 28 - Break Bidirectional"]
    #[inline(always)]
    #[must_use]
    pub fn bkbid(&mut self) -> BKBID_W<BDTRrs> {
        BKBID_W::new(self, 28)
    }
}
#[doc = "TIM16/TIM17 break and dead-time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDTRrs;
impl crate::RegisterSpec for BDTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdtr::R`](R) reader structure"]
impl crate::Readable for BDTRrs {}
#[doc = "`write(|w| ..)` method takes [`bdtr::W`](W) writer structure"]
impl crate::Writable for BDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDTR to value 0"]
impl crate::Resettable for BDTRrs {
    const RESET_VALUE: u32 = 0;
}
