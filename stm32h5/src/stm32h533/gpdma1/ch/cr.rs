///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: Channel disabled
    Disabled = 0,
    ///1: Channel enabled
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - enable
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    ///Channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    ///Channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
///Field `EN` writer - enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    ///Channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
/**reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETW {
    ///1: Reset channel
    Reset = 1,
}
impl From<RESETW> for bool {
    #[inline(always)]
    fn from(variant: RESETW) -> Self {
        variant as u8 != 0
    }
}
///Field `RESET` writer - reset
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG, RESETW>;
impl<'a, REG> RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Reset channel
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RESETW::Reset)
    }
}
/**suspend

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP {
    ///0: Channel operation not suspended
    NotSuspended = 0,
    ///1: Channel operation suspended
    Suspended = 1,
}
impl From<SUSP> for bool {
    #[inline(always)]
    fn from(variant: SUSP) -> Self {
        variant as u8 != 0
    }
}
///Field `SUSP` reader - suspend
pub type SUSP_R = crate::BitReader<SUSP>;
impl SUSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SUSP {
        match self.bits {
            false => SUSP::NotSuspended,
            true => SUSP::Suspended,
        }
    }
    ///Channel operation not suspended
    #[inline(always)]
    pub fn is_not_suspended(&self) -> bool {
        *self == SUSP::NotSuspended
    }
    ///Channel operation suspended
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == SUSP::Suspended
    }
}
///Field `SUSP` writer - suspend
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG, SUSP>;
impl<'a, REG> SUSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel operation not suspended
    #[inline(always)]
    pub fn not_suspended(self) -> &'a mut crate::W<REG> {
        self.variant(SUSP::NotSuspended)
    }
    ///Channel operation suspended
    #[inline(always)]
    pub fn suspended(self) -> &'a mut crate::W<REG> {
        self.variant(SUSP::Suspended)
    }
}
/**transfer complete interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIE` reader - transfer complete interrupt enable
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::Disabled,
            true => TCIE::Enabled,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
///Field `TCIE` writer - transfer complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
///Field `HTIE` reader - half transfer complete interrupt enable
pub use TCIE_R as HTIE_R;
///Field `DTEIE` reader - data transfer error interrupt enable
pub use TCIE_R as DTEIE_R;
///Field `ULEIE` reader - update link transfer error interrupt enable
pub use TCIE_R as ULEIE_R;
///Field `USEIE` reader - user setting error interrupt enable
pub use TCIE_R as USEIE_R;
///Field `SUSPIE` reader - completed suspension interrupt enable
pub use TCIE_R as SUSPIE_R;
///Field `TOIE` reader - trigger overrun interrupt enable
pub use TCIE_R as TOIE_R;
///Field `HTIE` writer - half transfer complete interrupt enable
pub use TCIE_W as HTIE_W;
///Field `DTEIE` writer - data transfer error interrupt enable
pub use TCIE_W as DTEIE_W;
///Field `ULEIE` writer - update link transfer error interrupt enable
pub use TCIE_W as ULEIE_W;
///Field `USEIE` writer - user setting error interrupt enable
pub use TCIE_W as USEIE_W;
///Field `SUSPIE` writer - completed suspension interrupt enable
pub use TCIE_W as SUSPIE_W;
///Field `TOIE` writer - trigger overrun interrupt enable
pub use TCIE_W as TOIE_W;
/**Link step mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSM {
    ///0: Channel executed for full linked list
    FullLinkedList = 0,
    ///1: Channel executed once for current linked list
    Once = 1,
}
impl From<LSM> for bool {
    #[inline(always)]
    fn from(variant: LSM) -> Self {
        variant as u8 != 0
    }
}
///Field `LSM` reader - Link step mode
pub type LSM_R = crate::BitReader<LSM>;
impl LSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSM {
        match self.bits {
            false => LSM::FullLinkedList,
            true => LSM::Once,
        }
    }
    ///Channel executed for full linked list
    #[inline(always)]
    pub fn is_full_linked_list(&self) -> bool {
        *self == LSM::FullLinkedList
    }
    ///Channel executed once for current linked list
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == LSM::Once
    }
}
///Field `LSM` writer - Link step mode
pub type LSM_W<'a, REG> = crate::BitWriter<'a, REG, LSM>;
impl<'a, REG> LSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel executed for full linked list
    #[inline(always)]
    pub fn full_linked_list(self) -> &'a mut crate::W<REG> {
        self.variant(LSM::FullLinkedList)
    }
    ///Channel executed once for current linked list
    #[inline(always)]
    pub fn once(self) -> &'a mut crate::W<REG> {
        self.variant(LSM::Once)
    }
}
/**linked-list allocated port

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LAP {
    ///0: Port 0 (AHB) allocated
    Port0 = 0,
    ///1: Port 1 (AHB) allocated
    Port1 = 1,
}
impl From<LAP> for bool {
    #[inline(always)]
    fn from(variant: LAP) -> Self {
        variant as u8 != 0
    }
}
///Field `LAP` reader - linked-list allocated port
pub type LAP_R = crate::BitReader<LAP>;
impl LAP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LAP {
        match self.bits {
            false => LAP::Port0,
            true => LAP::Port1,
        }
    }
    ///Port 0 (AHB) allocated
    #[inline(always)]
    pub fn is_port0(&self) -> bool {
        *self == LAP::Port0
    }
    ///Port 1 (AHB) allocated
    #[inline(always)]
    pub fn is_port1(&self) -> bool {
        *self == LAP::Port1
    }
}
///Field `LAP` writer - linked-list allocated port
pub type LAP_W<'a, REG> = crate::BitWriter<'a, REG, LAP>;
impl<'a, REG> LAP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Port 0 (AHB) allocated
    #[inline(always)]
    pub fn port0(self) -> &'a mut crate::W<REG> {
        self.variant(LAP::Port0)
    }
    ///Port 1 (AHB) allocated
    #[inline(always)]
    pub fn port1(self) -> &'a mut crate::W<REG> {
        self.variant(LAP::Port1)
    }
}
/**priority level of the channel x GPDMA transfer versus others

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRIO {
    ///0: Low priority, low weight
    LowPrioLowWeight = 0,
    ///1: Low priority, mid weight
    LowPrioMidWeight = 1,
    ///2: Low priority, high weight
    LowPrioHighWeight = 2,
    ///3: High priority
    HighPrio = 3,
}
impl From<PRIO> for u8 {
    #[inline(always)]
    fn from(variant: PRIO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRIO {
    type Ux = u8;
}
impl crate::IsEnum for PRIO {}
///Field `PRIO` reader - priority level of the channel x GPDMA transfer versus others
pub type PRIO_R = crate::FieldReader<PRIO>;
impl PRIO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRIO {
        match self.bits {
            0 => PRIO::LowPrioLowWeight,
            1 => PRIO::LowPrioMidWeight,
            2 => PRIO::LowPrioHighWeight,
            3 => PRIO::HighPrio,
            _ => unreachable!(),
        }
    }
    ///Low priority, low weight
    #[inline(always)]
    pub fn is_low_prio_low_weight(&self) -> bool {
        *self == PRIO::LowPrioLowWeight
    }
    ///Low priority, mid weight
    #[inline(always)]
    pub fn is_low_prio_mid_weight(&self) -> bool {
        *self == PRIO::LowPrioMidWeight
    }
    ///Low priority, high weight
    #[inline(always)]
    pub fn is_low_prio_high_weight(&self) -> bool {
        *self == PRIO::LowPrioHighWeight
    }
    ///High priority
    #[inline(always)]
    pub fn is_high_prio(&self) -> bool {
        *self == PRIO::HighPrio
    }
}
///Field `PRIO` writer - priority level of the channel x GPDMA transfer versus others
pub type PRIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PRIO, crate::Safe>;
impl<'a, REG> PRIO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Low priority, low weight
    #[inline(always)]
    pub fn low_prio_low_weight(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO::LowPrioLowWeight)
    }
    ///Low priority, mid weight
    #[inline(always)]
    pub fn low_prio_mid_weight(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO::LowPrioMidWeight)
    }
    ///Low priority, high weight
    #[inline(always)]
    pub fn low_prio_high_weight(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO::LowPrioHighWeight)
    }
    ///High priority
    #[inline(always)]
    pub fn high_prio(self) -> &'a mut crate::W<REG> {
        self.variant(PRIO::HighPrio)
    }
}
impl R {
    ///Bit 0 - enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - suspend
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - half transfer complete interrupt enable
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - data transfer error interrupt enable
    #[inline(always)]
    pub fn dteie(&self) -> DTEIE_R {
        DTEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - update link transfer error interrupt enable
    #[inline(always)]
    pub fn uleie(&self) -> ULEIE_R {
        ULEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - user setting error interrupt enable
    #[inline(always)]
    pub fn useie(&self) -> USEIE_R {
        USEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - completed suspension interrupt enable
    #[inline(always)]
    pub fn suspie(&self) -> SUSPIE_R {
        SUSPIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - trigger overrun interrupt enable
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Link step mode
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - linked-list allocated port
    #[inline(always)]
    pub fn lap(&self) -> LAP_R {
        LAP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 22:23 - priority level of the channel x GPDMA transfer versus others
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("susp", &self.susp())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("dteie", &self.dteie())
            .field("uleie", &self.uleie())
            .field("useie", &self.useie())
            .field("suspie", &self.suspie())
            .field("toie", &self.toie())
            .field("lsm", &self.lsm())
            .field("lap", &self.lap())
            .field("prio", &self.prio())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - reset
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<'_, CRrs> {
        RESET_W::new(self, 1)
    }
    ///Bit 2 - suspend
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<'_, CRrs> {
        SUSP_W::new(self, 2)
    }
    ///Bit 8 - transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CRrs> {
        TCIE_W::new(self, 8)
    }
    ///Bit 9 - half transfer complete interrupt enable
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<'_, CRrs> {
        HTIE_W::new(self, 9)
    }
    ///Bit 10 - data transfer error interrupt enable
    #[inline(always)]
    pub fn dteie(&mut self) -> DTEIE_W<'_, CRrs> {
        DTEIE_W::new(self, 10)
    }
    ///Bit 11 - update link transfer error interrupt enable
    #[inline(always)]
    pub fn uleie(&mut self) -> ULEIE_W<'_, CRrs> {
        ULEIE_W::new(self, 11)
    }
    ///Bit 12 - user setting error interrupt enable
    #[inline(always)]
    pub fn useie(&mut self) -> USEIE_W<'_, CRrs> {
        USEIE_W::new(self, 12)
    }
    ///Bit 13 - completed suspension interrupt enable
    #[inline(always)]
    pub fn suspie(&mut self) -> SUSPIE_W<'_, CRrs> {
        SUSPIE_W::new(self, 13)
    }
    ///Bit 14 - trigger overrun interrupt enable
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W<'_, CRrs> {
        TOIE_W::new(self, 14)
    }
    ///Bit 16 - Link step mode
    #[inline(always)]
    pub fn lsm(&mut self) -> LSM_W<'_, CRrs> {
        LSM_W::new(self, 16)
    }
    ///Bit 17 - linked-list allocated port
    #[inline(always)]
    pub fn lap(&mut self) -> LAP_W<'_, CRrs> {
        LAP_W::new(self, 17)
    }
    ///Bits 22:23 - priority level of the channel x GPDMA transfer versus others
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W<'_, CRrs> {
        PRIO_W::new(self, 22)
    }
}
/**GPDMA channel 0 control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
