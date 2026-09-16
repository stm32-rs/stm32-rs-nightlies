///Register `MACFFR` reader
pub type R = crate::R<MACFFRrs>;
///Register `MACFFR` writer
pub type W = crate::W<MACFFRrs>;
/**Promiscuous mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM {
    ///0: Normal address filtering
    Disabled = 0,
    ///1: Address filters pass all incoming frames regardless of their destination or source address
    Enabled = 1,
}
impl From<PM> for bool {
    #[inline(always)]
    fn from(variant: PM) -> Self {
        variant as u8 != 0
    }
}
///Field `PM` reader - Promiscuous mode
pub type PM_R = crate::BitReader<PM>;
impl PM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PM {
        match self.bits {
            false => PM::Disabled,
            true => PM::Enabled,
        }
    }
    ///Normal address filtering
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PM::Disabled
    }
    ///Address filters pass all incoming frames regardless of their destination or source address
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PM::Enabled
    }
}
///Field `PM` writer - Promiscuous mode
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG, PM>;
impl<'a, REG> PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal address filtering
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PM::Disabled)
    }
    ///Address filters pass all incoming frames regardless of their destination or source address
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PM::Enabled)
    }
}
/**Hash unicast

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HU {
    ///0: MAC performs a perfect destination address filtering for unicast frames
    Perfect = 0,
    ///1: MAC performs destination address filtering of received unicast frames according to the hash table
    Hash = 1,
}
impl From<HU> for bool {
    #[inline(always)]
    fn from(variant: HU) -> Self {
        variant as u8 != 0
    }
}
///Field `HU` reader - Hash unicast
pub type HU_R = crate::BitReader<HU>;
impl HU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HU {
        match self.bits {
            false => HU::Perfect,
            true => HU::Hash,
        }
    }
    ///MAC performs a perfect destination address filtering for unicast frames
    #[inline(always)]
    pub fn is_perfect(&self) -> bool {
        *self == HU::Perfect
    }
    ///MAC performs destination address filtering of received unicast frames according to the hash table
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        *self == HU::Hash
    }
}
///Field `HU` writer - Hash unicast
pub type HU_W<'a, REG> = crate::BitWriter<'a, REG, HU>;
impl<'a, REG> HU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MAC performs a perfect destination address filtering for unicast frames
    #[inline(always)]
    pub fn perfect(self) -> &'a mut crate::W<REG> {
        self.variant(HU::Perfect)
    }
    ///MAC performs destination address filtering of received unicast frames according to the hash table
    #[inline(always)]
    pub fn hash(self) -> &'a mut crate::W<REG> {
        self.variant(HU::Hash)
    }
}
/**Hash multicast

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HM {
    ///0: MAC performs a perfect destination address filtering for multicast frames
    Perfect = 0,
    ///1: MAC performs destination address filtering of received multicast frames according to the hash table
    Hash = 1,
}
impl From<HM> for bool {
    #[inline(always)]
    fn from(variant: HM) -> Self {
        variant as u8 != 0
    }
}
///Field `HM` reader - Hash multicast
pub type HM_R = crate::BitReader<HM>;
impl HM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HM {
        match self.bits {
            false => HM::Perfect,
            true => HM::Hash,
        }
    }
    ///MAC performs a perfect destination address filtering for multicast frames
    #[inline(always)]
    pub fn is_perfect(&self) -> bool {
        *self == HM::Perfect
    }
    ///MAC performs destination address filtering of received multicast frames according to the hash table
    #[inline(always)]
    pub fn is_hash(&self) -> bool {
        *self == HM::Hash
    }
}
///Field `HM` writer - Hash multicast
pub type HM_W<'a, REG> = crate::BitWriter<'a, REG, HM>;
impl<'a, REG> HM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MAC performs a perfect destination address filtering for multicast frames
    #[inline(always)]
    pub fn perfect(self) -> &'a mut crate::W<REG> {
        self.variant(HM::Perfect)
    }
    ///MAC performs destination address filtering of received multicast frames according to the hash table
    #[inline(always)]
    pub fn hash(self) -> &'a mut crate::W<REG> {
        self.variant(HM::Hash)
    }
}
/**Destination address unique filtering

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAIF {
    ///0: Normal filtering of frames
    Normal = 0,
    ///1: Address check block operates in inverse filtering mode for the DA address comparison
    Invert = 1,
}
impl From<DAIF> for bool {
    #[inline(always)]
    fn from(variant: DAIF) -> Self {
        variant as u8 != 0
    }
}
///Field `DAIF` reader - Destination address unique filtering
pub type DAIF_R = crate::BitReader<DAIF>;
impl DAIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DAIF {
        match self.bits {
            false => DAIF::Normal,
            true => DAIF::Invert,
        }
    }
    ///Normal filtering of frames
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == DAIF::Normal
    }
    ///Address check block operates in inverse filtering mode for the DA address comparison
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == DAIF::Invert
    }
}
///Field `DAIF` writer - Destination address unique filtering
pub type DAIF_W<'a, REG> = crate::BitWriter<'a, REG, DAIF>;
impl<'a, REG> DAIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal filtering of frames
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(DAIF::Normal)
    }
    ///Address check block operates in inverse filtering mode for the DA address comparison
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(DAIF::Invert)
    }
}
/**Pass all multicast

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PAM {
    ///0: Filtering of multicast frames depends on HM
    Disabled = 0,
    ///1: All received frames with a multicast destination address are passed
    Enabled = 1,
}
impl From<PAM> for bool {
    #[inline(always)]
    fn from(variant: PAM) -> Self {
        variant as u8 != 0
    }
}
///Field `PAM` reader - Pass all multicast
pub type PAM_R = crate::BitReader<PAM>;
impl PAM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PAM {
        match self.bits {
            false => PAM::Disabled,
            true => PAM::Enabled,
        }
    }
    ///Filtering of multicast frames depends on HM
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PAM::Disabled
    }
    ///All received frames with a multicast destination address are passed
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PAM::Enabled
    }
}
///Field `PAM` writer - Pass all multicast
pub type PAM_W<'a, REG> = crate::BitWriter<'a, REG, PAM>;
impl<'a, REG> PAM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Filtering of multicast frames depends on HM
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PAM::Disabled)
    }
    ///All received frames with a multicast destination address are passed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PAM::Enabled)
    }
}
/**Broadcast frames disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFD {
    ///0: Address filters pass all received broadcast frames
    Enabled = 0,
    ///1: Address filters filter all incoming broadcast frames
    Disabled = 1,
}
impl From<BFD> for bool {
    #[inline(always)]
    fn from(variant: BFD) -> Self {
        variant as u8 != 0
    }
}
///Field `BFD` reader - Broadcast frames disable
pub type BFD_R = crate::BitReader<BFD>;
impl BFD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BFD {
        match self.bits {
            false => BFD::Enabled,
            true => BFD::Disabled,
        }
    }
    ///Address filters pass all received broadcast frames
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BFD::Enabled
    }
    ///Address filters filter all incoming broadcast frames
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BFD::Disabled
    }
}
///Field `BFD` writer - Broadcast frames disable
pub type BFD_W<'a, REG> = crate::BitWriter<'a, REG, BFD>;
impl<'a, REG> BFD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Address filters pass all received broadcast frames
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BFD::Enabled)
    }
    ///Address filters filter all incoming broadcast frames
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BFD::Disabled)
    }
}
/**Pass control frames

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCF {
    ///0: MAC prevents all control frames from reaching the application
    PreventAll = 0,
    ///1: MAC forwards all control frames to application except Pause
    ForwardAllExceptPause = 1,
    ///2: MAC forwards all control frames to application even if they fail the address filter
    ForwardAll = 2,
    ///3: MAC forwards control frames that pass the address filter
    ForwardAllFiltered = 3,
}
impl From<PCF> for u8 {
    #[inline(always)]
    fn from(variant: PCF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PCF {
    type Ux = u8;
}
impl crate::IsEnum for PCF {}
///Field `PCF` reader - Pass control frames
pub type PCF_R = crate::FieldReader<PCF>;
impl PCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCF {
        match self.bits {
            0 => PCF::PreventAll,
            1 => PCF::ForwardAllExceptPause,
            2 => PCF::ForwardAll,
            3 => PCF::ForwardAllFiltered,
            _ => unreachable!(),
        }
    }
    ///MAC prevents all control frames from reaching the application
    #[inline(always)]
    pub fn is_prevent_all(&self) -> bool {
        *self == PCF::PreventAll
    }
    ///MAC forwards all control frames to application except Pause
    #[inline(always)]
    pub fn is_forward_all_except_pause(&self) -> bool {
        *self == PCF::ForwardAllExceptPause
    }
    ///MAC forwards all control frames to application even if they fail the address filter
    #[inline(always)]
    pub fn is_forward_all(&self) -> bool {
        *self == PCF::ForwardAll
    }
    ///MAC forwards control frames that pass the address filter
    #[inline(always)]
    pub fn is_forward_all_filtered(&self) -> bool {
        *self == PCF::ForwardAllFiltered
    }
}
///Field `PCF` writer - Pass control frames
pub type PCF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PCF, crate::Safe>;
impl<'a, REG> PCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///MAC prevents all control frames from reaching the application
    #[inline(always)]
    pub fn prevent_all(self) -> &'a mut crate::W<REG> {
        self.variant(PCF::PreventAll)
    }
    ///MAC forwards all control frames to application except Pause
    #[inline(always)]
    pub fn forward_all_except_pause(self) -> &'a mut crate::W<REG> {
        self.variant(PCF::ForwardAllExceptPause)
    }
    ///MAC forwards all control frames to application even if they fail the address filter
    #[inline(always)]
    pub fn forward_all(self) -> &'a mut crate::W<REG> {
        self.variant(PCF::ForwardAll)
    }
    ///MAC forwards control frames that pass the address filter
    #[inline(always)]
    pub fn forward_all_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(PCF::ForwardAllFiltered)
    }
}
/**Source address inverse filtering

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIF {
    ///0: Source address filter operates normally
    Normal = 0,
    ///1: Source address filter operation inverted
    Invert = 1,
}
impl From<SAIF> for bool {
    #[inline(always)]
    fn from(variant: SAIF) -> Self {
        variant as u8 != 0
    }
}
///Field `SAIF` reader - Source address inverse filtering
pub type SAIF_R = crate::BitReader<SAIF>;
impl SAIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAIF {
        match self.bits {
            false => SAIF::Normal,
            true => SAIF::Invert,
        }
    }
    ///Source address filter operates normally
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SAIF::Normal
    }
    ///Source address filter operation inverted
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == SAIF::Invert
    }
}
///Field `SAIF` writer - Source address inverse filtering
pub type SAIF_W<'a, REG> = crate::BitWriter<'a, REG, SAIF>;
impl<'a, REG> SAIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Source address filter operates normally
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SAIF::Normal)
    }
    ///Source address filter operation inverted
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(SAIF::Invert)
    }
}
/**Source address filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAF {
    ///0: Source address ignored
    Disabled = 0,
    ///1: MAC drops frames that fail the source address filter
    Enabled = 1,
}
impl From<SAF> for bool {
    #[inline(always)]
    fn from(variant: SAF) -> Self {
        variant as u8 != 0
    }
}
///Field `SAF` reader - Source address filter
pub type SAF_R = crate::BitReader<SAF>;
impl SAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SAF {
        match self.bits {
            false => SAF::Disabled,
            true => SAF::Enabled,
        }
    }
    ///Source address ignored
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAF::Disabled
    }
    ///MAC drops frames that fail the source address filter
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAF::Enabled
    }
}
///Field `SAF` writer - Source address filter
pub type SAF_W<'a, REG> = crate::BitWriter<'a, REG, SAF>;
impl<'a, REG> SAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Source address ignored
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAF::Disabled)
    }
    ///MAC drops frames that fail the source address filter
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SAF::Enabled)
    }
}
/**Hash or perfect filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPF {
    ///0: If HM or HU is set, only frames that match the Hash filter are passed
    HashOnly = 0,
    ///1: If HM or HU is set, frames that match either the perfect filter or the hash filter are passed
    HashOrPerfect = 1,
}
impl From<HPF> for bool {
    #[inline(always)]
    fn from(variant: HPF) -> Self {
        variant as u8 != 0
    }
}
///Field `HPF` reader - Hash or perfect filter
pub type HPF_R = crate::BitReader<HPF>;
impl HPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HPF {
        match self.bits {
            false => HPF::HashOnly,
            true => HPF::HashOrPerfect,
        }
    }
    ///If HM or HU is set, only frames that match the Hash filter are passed
    #[inline(always)]
    pub fn is_hash_only(&self) -> bool {
        *self == HPF::HashOnly
    }
    ///If HM or HU is set, frames that match either the perfect filter or the hash filter are passed
    #[inline(always)]
    pub fn is_hash_or_perfect(&self) -> bool {
        *self == HPF::HashOrPerfect
    }
}
///Field `HPF` writer - Hash or perfect filter
pub type HPF_W<'a, REG> = crate::BitWriter<'a, REG, HPF>;
impl<'a, REG> HPF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///If HM or HU is set, only frames that match the Hash filter are passed
    #[inline(always)]
    pub fn hash_only(self) -> &'a mut crate::W<REG> {
        self.variant(HPF::HashOnly)
    }
    ///If HM or HU is set, frames that match either the perfect filter or the hash filter are passed
    #[inline(always)]
    pub fn hash_or_perfect(self) -> &'a mut crate::W<REG> {
        self.variant(HPF::HashOrPerfect)
    }
}
/**Receive all

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RA {
    ///0: MAC receiver passes on to the application only those frames that have passed the SA/DA address file
    Disabled = 0,
    ///1: MAC receiver passes oll received frames on to the application
    Enabled = 1,
}
impl From<RA> for bool {
    #[inline(always)]
    fn from(variant: RA) -> Self {
        variant as u8 != 0
    }
}
///Field `RA` reader - Receive all
pub type RA_R = crate::BitReader<RA>;
impl RA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RA {
        match self.bits {
            false => RA::Disabled,
            true => RA::Enabled,
        }
    }
    ///MAC receiver passes on to the application only those frames that have passed the SA/DA address file
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RA::Disabled
    }
    ///MAC receiver passes oll received frames on to the application
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RA::Enabled
    }
}
///Field `RA` writer - Receive all
pub type RA_W<'a, REG> = crate::BitWriter<'a, REG, RA>;
impl<'a, REG> RA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///MAC receiver passes on to the application only those frames that have passed the SA/DA address file
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RA::Disabled)
    }
    ///MAC receiver passes oll received frames on to the application
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RA::Enabled)
    }
}
impl R {
    ///Bit 0 - Promiscuous mode
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Hash unicast
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Hash multicast
    #[inline(always)]
    pub fn hm(&self) -> HM_R {
        HM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Destination address unique filtering
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pass all multicast
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Broadcast frames disable
    #[inline(always)]
    pub fn bfd(&self) -> BFD_R {
        BFD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Pass control frames
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - Source address inverse filtering
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Source address filter
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Hash or perfect filter
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 31 - Receive all
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACFFR")
            .field("pm", &self.pm())
            .field("hu", &self.hu())
            .field("hm", &self.hm())
            .field("daif", &self.daif())
            .field("pam", &self.pam())
            .field("bfd", &self.bfd())
            .field("pcf", &self.pcf())
            .field("saif", &self.saif())
            .field("saf", &self.saf())
            .field("hpf", &self.hpf())
            .field("ra", &self.ra())
            .finish()
    }
}
impl W {
    ///Bit 0 - Promiscuous mode
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<'_, MACFFRrs> {
        PM_W::new(self, 0)
    }
    ///Bit 1 - Hash unicast
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W<'_, MACFFRrs> {
        HU_W::new(self, 1)
    }
    ///Bit 2 - Hash multicast
    #[inline(always)]
    pub fn hm(&mut self) -> HM_W<'_, MACFFRrs> {
        HM_W::new(self, 2)
    }
    ///Bit 3 - Destination address unique filtering
    #[inline(always)]
    pub fn daif(&mut self) -> DAIF_W<'_, MACFFRrs> {
        DAIF_W::new(self, 3)
    }
    ///Bit 4 - Pass all multicast
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W<'_, MACFFRrs> {
        PAM_W::new(self, 4)
    }
    ///Bit 5 - Broadcast frames disable
    #[inline(always)]
    pub fn bfd(&mut self) -> BFD_W<'_, MACFFRrs> {
        BFD_W::new(self, 5)
    }
    ///Bits 6:7 - Pass control frames
    #[inline(always)]
    pub fn pcf(&mut self) -> PCF_W<'_, MACFFRrs> {
        PCF_W::new(self, 6)
    }
    ///Bit 8 - Source address inverse filtering
    #[inline(always)]
    pub fn saif(&mut self) -> SAIF_W<'_, MACFFRrs> {
        SAIF_W::new(self, 8)
    }
    ///Bit 9 - Source address filter
    #[inline(always)]
    pub fn saf(&mut self) -> SAF_W<'_, MACFFRrs> {
        SAF_W::new(self, 9)
    }
    ///Bit 10 - Hash or perfect filter
    #[inline(always)]
    pub fn hpf(&mut self) -> HPF_W<'_, MACFFRrs> {
        HPF_W::new(self, 10)
    }
    ///Bit 31 - Receive all
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W<'_, MACFFRrs> {
        RA_W::new(self, 31)
    }
}
/**Ethernet MAC frame filter register (ETH_MACCFFR)

You can [`read`](crate::Reg::read) this register and get [`macffr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macffr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F107.html#Ethernet_MAC:MACFFR)*/
pub struct MACFFRrs;
impl crate::RegisterSpec for MACFFRrs {
    type Ux = u32;
}
///`read()` method returns [`macffr::R`](R) reader structure
impl crate::Readable for MACFFRrs {}
///`write(|w| ..)` method takes [`macffr::W`](W) writer structure
impl crate::Writable for MACFFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACFFR to value 0
impl crate::Resettable for MACFFRrs {}
