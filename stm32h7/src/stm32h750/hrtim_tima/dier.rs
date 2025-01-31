///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
/**CMP%sIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1IE {
    ///0: Compare interrupt disabled
    Disabled = 0,
    ///1: Compare interrupt enabled
    Enabled = 1,
}
impl From<CMP1IE> for bool {
    #[inline(always)]
    fn from(variant: CMP1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPIE(1-4)` reader - CMP%sIE
pub type CMPIE_R = crate::BitReader<CMP1IE>;
impl CMPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMP1IE {
        match self.bits {
            false => CMP1IE::Disabled,
            true => CMP1IE::Enabled,
        }
    }
    ///Compare interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMP1IE::Disabled
    }
    ///Compare interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP1IE::Enabled
    }
}
///Field `CMPIE(1-4)` writer - CMP%sIE
pub type CMPIE_W<'a, REG> = crate::BitWriter<'a, REG, CMP1IE>;
impl<'a, REG> CMPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Compare interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1IE::Disabled)
    }
    ///Compare interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1IE::Enabled)
    }
}
/**REPIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPIE {
    ///0: Repetition interrupt disabled
    Disabled = 0,
    ///1: Repetition interrupt enabled
    Enabled = 1,
}
impl From<REPIE> for bool {
    #[inline(always)]
    fn from(variant: REPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `REPIE` reader - REPIE
pub type REPIE_R = crate::BitReader<REPIE>;
impl REPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REPIE {
        match self.bits {
            false => REPIE::Disabled,
            true => REPIE::Enabled,
        }
    }
    ///Repetition interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPIE::Disabled
    }
    ///Repetition interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPIE::Enabled
    }
}
///Field `REPIE` writer - REPIE
pub type REPIE_W<'a, REG> = crate::BitWriter<'a, REG, REPIE>;
impl<'a, REG> REPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Repetition interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REPIE::Disabled)
    }
    ///Repetition interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REPIE::Enabled)
    }
}
/**UPDIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDIE {
    ///0: Update interrupt disabled
    Disabled = 0,
    ///1: Update interrupt enabled
    Enabled = 1,
}
impl From<UPDIE> for bool {
    #[inline(always)]
    fn from(variant: UPDIE) -> Self {
        variant as u8 != 0
    }
}
///Field `UPDIE` reader - UPDIE
pub type UPDIE_R = crate::BitReader<UPDIE>;
impl UPDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UPDIE {
        match self.bits {
            false => UPDIE::Disabled,
            true => UPDIE::Enabled,
        }
    }
    ///Update interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDIE::Disabled
    }
    ///Update interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDIE::Enabled
    }
}
///Field `UPDIE` writer - UPDIE
pub type UPDIE_W<'a, REG> = crate::BitWriter<'a, REG, UPDIE>;
impl<'a, REG> UPDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDIE::Disabled)
    }
    ///Update interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDIE::Enabled)
    }
}
/**CPT%sIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPT1IE {
    ///0: Capture interrupt disabled
    Disabled = 0,
    ///1: Capture interrupt enabled
    Enabled = 1,
}
impl From<CPT1IE> for bool {
    #[inline(always)]
    fn from(variant: CPT1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `CPTIE(1-2)` reader - CPT%sIE
pub type CPTIE_R = crate::BitReader<CPT1IE>;
impl CPTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPT1IE {
        match self.bits {
            false => CPT1IE::Disabled,
            true => CPT1IE::Enabled,
        }
    }
    ///Capture interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPT1IE::Disabled
    }
    ///Capture interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPT1IE::Enabled
    }
}
///Field `CPTIE(1-2)` writer - CPT%sIE
pub type CPTIE_W<'a, REG> = crate::BitWriter<'a, REG, CPT1IE>;
impl<'a, REG> CPTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Capture interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CPT1IE::Disabled)
    }
    ///Capture interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CPT1IE::Enabled)
    }
}
/**Output %s set interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET1IE {
    ///0: Tx output set interrupt disabled
    Disabled = 0,
    ///1: Tx output set interrupt enabled
    Enabled = 1,
}
impl From<SET1IE> for bool {
    #[inline(always)]
    fn from(variant: SET1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `SETIE(1-2)` reader - Output %s set interrupt enable
pub type SETIE_R = crate::BitReader<SET1IE>;
impl SETIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SET1IE {
        match self.bits {
            false => SET1IE::Disabled,
            true => SET1IE::Enabled,
        }
    }
    ///Tx output set interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SET1IE::Disabled
    }
    ///Tx output set interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SET1IE::Enabled
    }
}
///Field `SETIE(1-2)` writer - Output %s set interrupt enable
pub type SETIE_W<'a, REG> = crate::BitWriter<'a, REG, SET1IE>;
impl<'a, REG> SETIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tx output set interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SET1IE::Disabled)
    }
    ///Tx output set interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SET1IE::Enabled)
    }
}
/**RSTx1IE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST1IE {
    ///0: Tx output reset interrupt disabled
    Disabled = 0,
    ///1: Tx output reset interrupt enabled
    Enabled = 1,
}
impl From<RST1IE> for bool {
    #[inline(always)]
    fn from(variant: RST1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `RST1IE` reader - RSTx1IE
pub type RST1IE_R = crate::BitReader<RST1IE>;
impl RST1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RST1IE {
        match self.bits {
            false => RST1IE::Disabled,
            true => RST1IE::Enabled,
        }
    }
    ///Tx output reset interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RST1IE::Disabled
    }
    ///Tx output reset interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RST1IE::Enabled
    }
}
///Field `RST1IE` writer - RSTx1IE
pub type RST1IE_W<'a, REG> = crate::BitWriter<'a, REG, RST1IE>;
impl<'a, REG> RST1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tx output reset interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RST1IE::Disabled)
    }
    ///Tx output reset interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RST1IE::Enabled)
    }
}
///Field `RST2IE` reader - RSTx2IE
pub use RST1IE_R as RST2IE_R;
///Field `RST2IE` writer - RSTx2IE
pub use RST1IE_W as RST2IE_W;
/**RSTIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTIE {
    ///0: Timer x counter/reset roll-over interrupt disabled
    Disabled = 0,
    ///1: Timer x counter/reset roll-over interrupt enabled
    Enabled = 1,
}
impl From<RSTIE> for bool {
    #[inline(always)]
    fn from(variant: RSTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RSTIE` reader - RSTIE
pub type RSTIE_R = crate::BitReader<RSTIE>;
impl RSTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSTIE {
        match self.bits {
            false => RSTIE::Disabled,
            true => RSTIE::Enabled,
        }
    }
    ///Timer x counter/reset roll-over interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTIE::Disabled
    }
    ///Timer x counter/reset roll-over interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTIE::Enabled
    }
}
///Field `RSTIE` writer - RSTIE
pub type RSTIE_W<'a, REG> = crate::BitWriter<'a, REG, RSTIE>;
impl<'a, REG> RSTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer x counter/reset roll-over interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTIE::Disabled)
    }
    ///Timer x counter/reset roll-over interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTIE::Enabled)
    }
}
/**DLYPRTIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRTIE {
    ///0: Delayed protection interrupt disabled
    Disabled = 0,
    ///1: Delayed protection interrupt enabled
    Enabled = 1,
}
impl From<DLYPRTIE> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYPRTIE` reader - DLYPRTIE
pub type DLYPRTIE_R = crate::BitReader<DLYPRTIE>;
impl DLYPRTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYPRTIE {
        match self.bits {
            false => DLYPRTIE::Disabled,
            true => DLYPRTIE::Enabled,
        }
    }
    ///Delayed protection interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTIE::Disabled
    }
    ///Delayed protection interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTIE::Enabled
    }
}
///Field `DLYPRTIE` writer - DLYPRTIE
pub type DLYPRTIE_W<'a, REG> = crate::BitWriter<'a, REG, DLYPRTIE>;
impl<'a, REG> DLYPRTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Delayed protection interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRTIE::Disabled)
    }
    ///Delayed protection interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRTIE::Enabled)
    }
}
/**CMP%sDE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1DE {
    ///0: Compare DMA request disabled
    Disabled = 0,
    ///1: Compare DMA request enabled
    Enabled = 1,
}
impl From<CMP1DE> for bool {
    #[inline(always)]
    fn from(variant: CMP1DE) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPDE(1-4)` reader - CMP%sDE
pub type CMPDE_R = crate::BitReader<CMP1DE>;
impl CMPDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMP1DE {
        match self.bits {
            false => CMP1DE::Disabled,
            true => CMP1DE::Enabled,
        }
    }
    ///Compare DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMP1DE::Disabled
    }
    ///Compare DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP1DE::Enabled
    }
}
///Field `CMPDE(1-4)` writer - CMP%sDE
pub type CMPDE_W<'a, REG> = crate::BitWriter<'a, REG, CMP1DE>;
impl<'a, REG> CMPDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Compare DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1DE::Disabled)
    }
    ///Compare DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1DE::Enabled)
    }
}
/**REPDE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPDE {
    ///0: Repetition DMA request disabled
    Disabled = 0,
    ///1: Repetition DMA request enabled
    Enabled = 1,
}
impl From<REPDE> for bool {
    #[inline(always)]
    fn from(variant: REPDE) -> Self {
        variant as u8 != 0
    }
}
///Field `REPDE` reader - REPDE
pub type REPDE_R = crate::BitReader<REPDE>;
impl REPDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REPDE {
        match self.bits {
            false => REPDE::Disabled,
            true => REPDE::Enabled,
        }
    }
    ///Repetition DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPDE::Disabled
    }
    ///Repetition DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPDE::Enabled
    }
}
///Field `REPDE` writer - REPDE
pub type REPDE_W<'a, REG> = crate::BitWriter<'a, REG, REPDE>;
impl<'a, REG> REPDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Repetition DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REPDE::Disabled)
    }
    ///Repetition DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REPDE::Enabled)
    }
}
/**UPDDE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDDE {
    ///0: Update DMA request disabled
    Disabled = 0,
    ///1: Update DMA request enabled
    Enabled = 1,
}
impl From<UPDDE> for bool {
    #[inline(always)]
    fn from(variant: UPDDE) -> Self {
        variant as u8 != 0
    }
}
///Field `UPDDE` reader - UPDDE
pub type UPDDE_R = crate::BitReader<UPDDE>;
impl UPDDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UPDDE {
        match self.bits {
            false => UPDDE::Disabled,
            true => UPDDE::Enabled,
        }
    }
    ///Update DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDDE::Disabled
    }
    ///Update DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDDE::Enabled
    }
}
///Field `UPDDE` writer - UPDDE
pub type UPDDE_W<'a, REG> = crate::BitWriter<'a, REG, UPDDE>;
impl<'a, REG> UPDDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDDE::Disabled)
    }
    ///Update DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDDE::Enabled)
    }
}
/**CPT%sDE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPT1DE {
    ///0: Capture DMA request disabled
    Disabled = 0,
    ///1: Capture DMA request enabled
    Enabled = 1,
}
impl From<CPT1DE> for bool {
    #[inline(always)]
    fn from(variant: CPT1DE) -> Self {
        variant as u8 != 0
    }
}
///Field `CPTDE(1-2)` reader - CPT%sDE
pub type CPTDE_R = crate::BitReader<CPT1DE>;
impl CPTDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPT1DE {
        match self.bits {
            false => CPT1DE::Disabled,
            true => CPT1DE::Enabled,
        }
    }
    ///Capture DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPT1DE::Disabled
    }
    ///Capture DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPT1DE::Enabled
    }
}
///Field `CPTDE(1-2)` writer - CPT%sDE
pub type CPTDE_W<'a, REG> = crate::BitWriter<'a, REG, CPT1DE>;
impl<'a, REG> CPTDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Capture DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CPT1DE::Disabled)
    }
    ///Capture DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CPT1DE::Enabled)
    }
}
/**Output %s set DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET1DE {
    ///0: Tx output set DMA request disabled
    Disabled = 0,
    ///1: Tx output set DMA request enabled
    Enabled = 1,
}
impl From<SET1DE> for bool {
    #[inline(always)]
    fn from(variant: SET1DE) -> Self {
        variant as u8 != 0
    }
}
///Field `SETDE(1-2)` reader - Output %s set DMA request enable
pub type SETDE_R = crate::BitReader<SET1DE>;
impl SETDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SET1DE {
        match self.bits {
            false => SET1DE::Disabled,
            true => SET1DE::Enabled,
        }
    }
    ///Tx output set DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SET1DE::Disabled
    }
    ///Tx output set DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SET1DE::Enabled
    }
}
///Field `SETDE(1-2)` writer - Output %s set DMA request enable
pub type SETDE_W<'a, REG> = crate::BitWriter<'a, REG, SET1DE>;
impl<'a, REG> SETDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tx output set DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SET1DE::Disabled)
    }
    ///Tx output set DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SET1DE::Enabled)
    }
}
/**RSTx1DE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST1DE {
    ///0: Tx output reset DMA request disabled
    Disabled = 0,
    ///1: Tx output reset DMA request enabled
    Enabled = 1,
}
impl From<RST1DE> for bool {
    #[inline(always)]
    fn from(variant: RST1DE) -> Self {
        variant as u8 != 0
    }
}
///Field `RST1DE` reader - RSTx1DE
pub type RST1DE_R = crate::BitReader<RST1DE>;
impl RST1DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RST1DE {
        match self.bits {
            false => RST1DE::Disabled,
            true => RST1DE::Enabled,
        }
    }
    ///Tx output reset DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RST1DE::Disabled
    }
    ///Tx output reset DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RST1DE::Enabled
    }
}
///Field `RST1DE` writer - RSTx1DE
pub type RST1DE_W<'a, REG> = crate::BitWriter<'a, REG, RST1DE>;
impl<'a, REG> RST1DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tx output reset DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RST1DE::Disabled)
    }
    ///Tx output reset DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RST1DE::Enabled)
    }
}
///Field `RST2DE` reader - RSTx2DE
pub use RST1DE_R as RST2DE_R;
///Field `RST2DE` writer - RSTx2DE
pub use RST1DE_W as RST2DE_W;
/**RSTDE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTDE {
    ///0: Timer x counter reset/roll-over DMA request disabled
    Disabled = 0,
    ///1: Timer x counter reset/roll-over DMA request enabled
    Enabled = 1,
}
impl From<RSTDE> for bool {
    #[inline(always)]
    fn from(variant: RSTDE) -> Self {
        variant as u8 != 0
    }
}
///Field `RSTDE` reader - RSTDE
pub type RSTDE_R = crate::BitReader<RSTDE>;
impl RSTDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RSTDE {
        match self.bits {
            false => RSTDE::Disabled,
            true => RSTDE::Enabled,
        }
    }
    ///Timer x counter reset/roll-over DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTDE::Disabled
    }
    ///Timer x counter reset/roll-over DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTDE::Enabled
    }
}
///Field `RSTDE` writer - RSTDE
pub type RSTDE_W<'a, REG> = crate::BitWriter<'a, REG, RSTDE>;
impl<'a, REG> RSTDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer x counter reset/roll-over DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTDE::Disabled)
    }
    ///Timer x counter reset/roll-over DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTDE::Enabled)
    }
}
/**DLYPRTDE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRTDE {
    ///0: Delayed protection DMA request disabled
    Disabled = 0,
    ///1: Delayed protection DMA request enabled
    Enabled = 1,
}
impl From<DLYPRTDE> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTDE) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYPRTDE` reader - DLYPRTDE
pub type DLYPRTDE_R = crate::BitReader<DLYPRTDE>;
impl DLYPRTDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYPRTDE {
        match self.bits {
            false => DLYPRTDE::Disabled,
            true => DLYPRTDE::Enabled,
        }
    }
    ///Delayed protection DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTDE::Disabled
    }
    ///Delayed protection DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTDE::Enabled
    }
}
///Field `DLYPRTDE` writer - DLYPRTDE
pub type DLYPRTDE_W<'a, REG> = crate::BitWriter<'a, REG, DLYPRTDE>;
impl<'a, REG> DLYPRTDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Delayed protection DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRTDE::Disabled)
    }
    ///Delayed protection DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRTDE::Enabled)
    }
}
impl R {
    ///CMP(1-4)IE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1IE` field.</div>
    #[inline(always)]
    pub fn cmpie(&self, n: u8) -> CMPIE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPIE_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///CMP(1-4)IE
    #[inline(always)]
    pub fn cmpie_iter(&self) -> impl Iterator<Item = CMPIE_R> + '_ {
        (0..4).map(move |n| CMPIE_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - CMP1IE
    #[inline(always)]
    pub fn cmp1ie(&self) -> CMPIE_R {
        CMPIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CMP2IE
    #[inline(always)]
    pub fn cmp2ie(&self) -> CMPIE_R {
        CMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CMP3IE
    #[inline(always)]
    pub fn cmp3ie(&self) -> CMPIE_R {
        CMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CMP4IE
    #[inline(always)]
    pub fn cmp4ie(&self) -> CMPIE_R {
        CMPIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - REPIE
    #[inline(always)]
    pub fn repie(&self) -> REPIE_R {
        REPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - UPDIE
    #[inline(always)]
    pub fn updie(&self) -> UPDIE_R {
        UPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///CPT(1-2)IE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CPT1IE` field.</div>
    #[inline(always)]
    pub fn cptie(&self, n: u8) -> CPTIE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CPTIE_R::new(((self.bits >> (n + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///CPT(1-2)IE
    #[inline(always)]
    pub fn cptie_iter(&self) -> impl Iterator<Item = CPTIE_R> + '_ {
        (0..2).map(move |n| CPTIE_R::new(((self.bits >> (n + 7)) & 1) != 0))
    }
    ///Bit 7 - CPT1IE
    #[inline(always)]
    pub fn cpt1ie(&self) -> CPTIE_R {
        CPTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPT2IE
    #[inline(always)]
    pub fn cpt2ie(&self) -> CPTIE_R {
        CPTIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Output (1-2) set interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SET1IE` field.</div>
    #[inline(always)]
    pub fn setie(&self, n: u8) -> SETIE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SETIE_R::new(((self.bits >> (n * 2 + 9)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output (1-2) set interrupt enable
    #[inline(always)]
    pub fn setie_iter(&self) -> impl Iterator<Item = SETIE_R> + '_ {
        (0..2).map(move |n| SETIE_R::new(((self.bits >> (n * 2 + 9)) & 1) != 0))
    }
    ///Bit 9 - Output 1 set interrupt enable
    #[inline(always)]
    pub fn set1ie(&self) -> SETIE_R {
        SETIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Output 2 set interrupt enable
    #[inline(always)]
    pub fn set2ie(&self) -> SETIE_R {
        SETIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 10 - RSTx1IE
    #[inline(always)]
    pub fn rst1ie(&self) -> RST1IE_R {
        RST1IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - RSTx2IE
    #[inline(always)]
    pub fn rst2ie(&self) -> RST2IE_R {
        RST2IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RSTIE
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DLYPRTIE
    #[inline(always)]
    pub fn dlyprtie(&self) -> DLYPRTIE_R {
        DLYPRTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///CMP(1-4)DE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1DE` field.</div>
    #[inline(always)]
    pub fn cmpde(&self, n: u8) -> CMPDE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPDE_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///CMP(1-4)DE
    #[inline(always)]
    pub fn cmpde_iter(&self) -> impl Iterator<Item = CMPDE_R> + '_ {
        (0..4).map(move |n| CMPDE_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - CMP1DE
    #[inline(always)]
    pub fn cmp1de(&self) -> CMPDE_R {
        CMPDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CMP2DE
    #[inline(always)]
    pub fn cmp2de(&self) -> CMPDE_R {
        CMPDE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CMP3DE
    #[inline(always)]
    pub fn cmp3de(&self) -> CMPDE_R {
        CMPDE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CMP4DE
    #[inline(always)]
    pub fn cmp4de(&self) -> CMPDE_R {
        CMPDE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - REPDE
    #[inline(always)]
    pub fn repde(&self) -> REPDE_R {
        REPDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - UPDDE
    #[inline(always)]
    pub fn updde(&self) -> UPDDE_R {
        UPDDE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///CPT(1-2)DE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CPT1DE` field.</div>
    #[inline(always)]
    pub fn cptde(&self, n: u8) -> CPTDE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CPTDE_R::new(((self.bits >> (n + 23)) & 1) != 0)
    }
    ///Iterator for array of:
    ///CPT(1-2)DE
    #[inline(always)]
    pub fn cptde_iter(&self) -> impl Iterator<Item = CPTDE_R> + '_ {
        (0..2).map(move |n| CPTDE_R::new(((self.bits >> (n + 23)) & 1) != 0))
    }
    ///Bit 23 - CPT1DE
    #[inline(always)]
    pub fn cpt1de(&self) -> CPTDE_R {
        CPTDE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPT2DE
    #[inline(always)]
    pub fn cpt2de(&self) -> CPTDE_R {
        CPTDE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Output (1-2) set DMA request enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SET1DE` field.</div>
    #[inline(always)]
    pub fn setde(&self, n: u8) -> SETDE_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SETDE_R::new(((self.bits >> (n * 2 + 25)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output (1-2) set DMA request enable
    #[inline(always)]
    pub fn setde_iter(&self) -> impl Iterator<Item = SETDE_R> + '_ {
        (0..2).map(move |n| SETDE_R::new(((self.bits >> (n * 2 + 25)) & 1) != 0))
    }
    ///Bit 25 - Output 1 set DMA request enable
    #[inline(always)]
    pub fn set1de(&self) -> SETDE_R {
        SETDE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 27 - Output 2 set DMA request enable
    #[inline(always)]
    pub fn set2de(&self) -> SETDE_R {
        SETDE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 26 - RSTx1DE
    #[inline(always)]
    pub fn rst1de(&self) -> RST1DE_R {
        RST1DE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - RSTx2DE
    #[inline(always)]
    pub fn rst2de(&self) -> RST2DE_R {
        RST2DE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - RSTDE
    #[inline(always)]
    pub fn rstde(&self) -> RSTDE_R {
        RSTDE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DLYPRTDE
    #[inline(always)]
    pub fn dlyprtde(&self) -> DLYPRTDE_R {
        DLYPRTDE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
            .field("dlyprtde", &self.dlyprtde())
            .field("rstde", &self.rstde())
            .field("rst1de", &self.rst1de())
            .field("rst2de", &self.rst2de())
            .field("set1de", &self.set1de())
            .field("set2de", &self.set2de())
            .field("cpt1de", &self.cpt1de())
            .field("cpt2de", &self.cpt2de())
            .field("updde", &self.updde())
            .field("repde", &self.repde())
            .field("cmp1de", &self.cmp1de())
            .field("cmp2de", &self.cmp2de())
            .field("cmp3de", &self.cmp3de())
            .field("cmp4de", &self.cmp4de())
            .field("dlyprtie", &self.dlyprtie())
            .field("rstie", &self.rstie())
            .field("rst1ie", &self.rst1ie())
            .field("rst2ie", &self.rst2ie())
            .field("set1ie", &self.set1ie())
            .field("set2ie", &self.set2ie())
            .field("cpt1ie", &self.cpt1ie())
            .field("cpt2ie", &self.cpt2ie())
            .field("updie", &self.updie())
            .field("repie", &self.repie())
            .field("cmp1ie", &self.cmp1ie())
            .field("cmp2ie", &self.cmp2ie())
            .field("cmp3ie", &self.cmp3ie())
            .field("cmp4ie", &self.cmp4ie())
            .finish()
    }
}
impl W {
    ///CMP(1-4)IE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1IE` field.</div>
    #[inline(always)]
    pub fn cmpie(&mut self, n: u8) -> CMPIE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPIE_W::new(self, n)
    }
    ///Bit 0 - CMP1IE
    #[inline(always)]
    pub fn cmp1ie(&mut self) -> CMPIE_W<DIERrs> {
        CMPIE_W::new(self, 0)
    }
    ///Bit 1 - CMP2IE
    #[inline(always)]
    pub fn cmp2ie(&mut self) -> CMPIE_W<DIERrs> {
        CMPIE_W::new(self, 1)
    }
    ///Bit 2 - CMP3IE
    #[inline(always)]
    pub fn cmp3ie(&mut self) -> CMPIE_W<DIERrs> {
        CMPIE_W::new(self, 2)
    }
    ///Bit 3 - CMP4IE
    #[inline(always)]
    pub fn cmp4ie(&mut self) -> CMPIE_W<DIERrs> {
        CMPIE_W::new(self, 3)
    }
    ///Bit 4 - REPIE
    #[inline(always)]
    pub fn repie(&mut self) -> REPIE_W<DIERrs> {
        REPIE_W::new(self, 4)
    }
    ///Bit 6 - UPDIE
    #[inline(always)]
    pub fn updie(&mut self) -> UPDIE_W<DIERrs> {
        UPDIE_W::new(self, 6)
    }
    ///CPT(1-2)IE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CPT1IE` field.</div>
    #[inline(always)]
    pub fn cptie(&mut self, n: u8) -> CPTIE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CPTIE_W::new(self, n + 7)
    }
    ///Bit 7 - CPT1IE
    #[inline(always)]
    pub fn cpt1ie(&mut self) -> CPTIE_W<DIERrs> {
        CPTIE_W::new(self, 7)
    }
    ///Bit 8 - CPT2IE
    #[inline(always)]
    pub fn cpt2ie(&mut self) -> CPTIE_W<DIERrs> {
        CPTIE_W::new(self, 8)
    }
    ///Output (1-2) set interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SET1IE` field.</div>
    #[inline(always)]
    pub fn setie(&mut self, n: u8) -> SETIE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SETIE_W::new(self, n * 2 + 9)
    }
    ///Bit 9 - Output 1 set interrupt enable
    #[inline(always)]
    pub fn set1ie(&mut self) -> SETIE_W<DIERrs> {
        SETIE_W::new(self, 9)
    }
    ///Bit 11 - Output 2 set interrupt enable
    #[inline(always)]
    pub fn set2ie(&mut self) -> SETIE_W<DIERrs> {
        SETIE_W::new(self, 11)
    }
    ///Bit 10 - RSTx1IE
    #[inline(always)]
    pub fn rst1ie(&mut self) -> RST1IE_W<DIERrs> {
        RST1IE_W::new(self, 10)
    }
    ///Bit 12 - RSTx2IE
    #[inline(always)]
    pub fn rst2ie(&mut self) -> RST2IE_W<DIERrs> {
        RST2IE_W::new(self, 12)
    }
    ///Bit 13 - RSTIE
    #[inline(always)]
    pub fn rstie(&mut self) -> RSTIE_W<DIERrs> {
        RSTIE_W::new(self, 13)
    }
    ///Bit 14 - DLYPRTIE
    #[inline(always)]
    pub fn dlyprtie(&mut self) -> DLYPRTIE_W<DIERrs> {
        DLYPRTIE_W::new(self, 14)
    }
    ///CMP(1-4)DE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1DE` field.</div>
    #[inline(always)]
    pub fn cmpde(&mut self, n: u8) -> CMPDE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPDE_W::new(self, n + 16)
    }
    ///Bit 16 - CMP1DE
    #[inline(always)]
    pub fn cmp1de(&mut self) -> CMPDE_W<DIERrs> {
        CMPDE_W::new(self, 16)
    }
    ///Bit 17 - CMP2DE
    #[inline(always)]
    pub fn cmp2de(&mut self) -> CMPDE_W<DIERrs> {
        CMPDE_W::new(self, 17)
    }
    ///Bit 18 - CMP3DE
    #[inline(always)]
    pub fn cmp3de(&mut self) -> CMPDE_W<DIERrs> {
        CMPDE_W::new(self, 18)
    }
    ///Bit 19 - CMP4DE
    #[inline(always)]
    pub fn cmp4de(&mut self) -> CMPDE_W<DIERrs> {
        CMPDE_W::new(self, 19)
    }
    ///Bit 20 - REPDE
    #[inline(always)]
    pub fn repde(&mut self) -> REPDE_W<DIERrs> {
        REPDE_W::new(self, 20)
    }
    ///Bit 22 - UPDDE
    #[inline(always)]
    pub fn updde(&mut self) -> UPDDE_W<DIERrs> {
        UPDDE_W::new(self, 22)
    }
    ///CPT(1-2)DE
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CPT1DE` field.</div>
    #[inline(always)]
    pub fn cptde(&mut self, n: u8) -> CPTDE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CPTDE_W::new(self, n + 23)
    }
    ///Bit 23 - CPT1DE
    #[inline(always)]
    pub fn cpt1de(&mut self) -> CPTDE_W<DIERrs> {
        CPTDE_W::new(self, 23)
    }
    ///Bit 24 - CPT2DE
    #[inline(always)]
    pub fn cpt2de(&mut self) -> CPTDE_W<DIERrs> {
        CPTDE_W::new(self, 24)
    }
    ///Output (1-2) set DMA request enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SET1DE` field.</div>
    #[inline(always)]
    pub fn setde(&mut self, n: u8) -> SETDE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SETDE_W::new(self, n * 2 + 25)
    }
    ///Bit 25 - Output 1 set DMA request enable
    #[inline(always)]
    pub fn set1de(&mut self) -> SETDE_W<DIERrs> {
        SETDE_W::new(self, 25)
    }
    ///Bit 27 - Output 2 set DMA request enable
    #[inline(always)]
    pub fn set2de(&mut self) -> SETDE_W<DIERrs> {
        SETDE_W::new(self, 27)
    }
    ///Bit 26 - RSTx1DE
    #[inline(always)]
    pub fn rst1de(&mut self) -> RST1DE_W<DIERrs> {
        RST1DE_W::new(self, 26)
    }
    ///Bit 28 - RSTx2DE
    #[inline(always)]
    pub fn rst2de(&mut self) -> RST2DE_W<DIERrs> {
        RST2DE_W::new(self, 28)
    }
    ///Bit 29 - RSTDE
    #[inline(always)]
    pub fn rstde(&mut self) -> RSTDE_W<DIERrs> {
        RSTDE_W::new(self, 29)
    }
    ///Bit 30 - DLYPRTDE
    #[inline(always)]
    pub fn dlyprtde(&mut self) -> DLYPRTDE_W<DIERrs> {
        DLYPRTDE_W::new(self, 30)
    }
}
/**TIMxDIER5

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#HRTIM_TIMA:DIER)*/
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {
    const RESET_VALUE: u32 = 0;
}
