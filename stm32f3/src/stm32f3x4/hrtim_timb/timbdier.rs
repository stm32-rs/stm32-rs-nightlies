#[doc = "Register `TIMBDIER` reader"]
pub type R = crate::R<TIMBDIERrs>;
#[doc = "Register `TIMBDIER` writer"]
pub type W = crate::W<TIMBDIERrs>;
#[doc = "CMP1IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1IE {
    #[doc = "0: Compare interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Compare interrupt enabled"]
    Enabled = 1,
}
impl From<CMP1IE> for bool {
    #[inline(always)]
    fn from(variant: CMP1IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP1IE` reader - CMP1IE"]
pub type CMP1IE_R = crate::BitReader<CMP1IE>;
impl CMP1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMP1IE {
        match self.bits {
            false => CMP1IE::Disabled,
            true => CMP1IE::Enabled,
        }
    }
    #[doc = "Compare interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMP1IE::Disabled
    }
    #[doc = "Compare interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP1IE::Enabled
    }
}
#[doc = "Field `CMP1IE` writer - CMP1IE"]
pub type CMP1IE_W<'a, REG> = crate::BitWriter<'a, REG, CMP1IE>;
impl<'a, REG> CMP1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1IE::Disabled)
    }
    #[doc = "Compare interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1IE::Enabled)
    }
}
#[doc = "Field `CMP2IE` reader - CMP2IE"]
pub use CMP1IE_R as CMP2IE_R;
#[doc = "Field `CMP3IE` reader - CMP3IE"]
pub use CMP1IE_R as CMP3IE_R;
#[doc = "Field `CMP4IE` reader - CMP4IE"]
pub use CMP1IE_R as CMP4IE_R;
#[doc = "Field `CMP2IE` writer - CMP2IE"]
pub use CMP1IE_W as CMP2IE_W;
#[doc = "Field `CMP3IE` writer - CMP3IE"]
pub use CMP1IE_W as CMP3IE_W;
#[doc = "Field `CMP4IE` writer - CMP4IE"]
pub use CMP1IE_W as CMP4IE_W;
#[doc = "REPIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPIE {
    #[doc = "0: Repetition interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Repetition interrupt enabled"]
    Enabled = 1,
}
impl From<REPIE> for bool {
    #[inline(always)]
    fn from(variant: REPIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPIE` reader - REPIE"]
pub type REPIE_R = crate::BitReader<REPIE>;
impl REPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REPIE {
        match self.bits {
            false => REPIE::Disabled,
            true => REPIE::Enabled,
        }
    }
    #[doc = "Repetition interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPIE::Disabled
    }
    #[doc = "Repetition interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPIE::Enabled
    }
}
#[doc = "Field `REPIE` writer - REPIE"]
pub type REPIE_W<'a, REG> = crate::BitWriter<'a, REG, REPIE>;
impl<'a, REG> REPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repetition interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REPIE::Disabled)
    }
    #[doc = "Repetition interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REPIE::Enabled)
    }
}
#[doc = "UPDIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDIE {
    #[doc = "0: Update interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Update interrupt enabled"]
    Enabled = 1,
}
impl From<UPDIE> for bool {
    #[inline(always)]
    fn from(variant: UPDIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDIE` reader - UPDIE"]
pub type UPDIE_R = crate::BitReader<UPDIE>;
impl UPDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UPDIE {
        match self.bits {
            false => UPDIE::Disabled,
            true => UPDIE::Enabled,
        }
    }
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDIE::Disabled
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDIE::Enabled
    }
}
#[doc = "Field `UPDIE` writer - UPDIE"]
pub type UPDIE_W<'a, REG> = crate::BitWriter<'a, REG, UPDIE>;
impl<'a, REG> UPDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDIE::Disabled)
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDIE::Enabled)
    }
}
#[doc = "CPT1IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPT1IE {
    #[doc = "0: Capture interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Capture interrupt enabled"]
    Enabled = 1,
}
impl From<CPT1IE> for bool {
    #[inline(always)]
    fn from(variant: CPT1IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPT1IE` reader - CPT1IE"]
pub type CPT1IE_R = crate::BitReader<CPT1IE>;
impl CPT1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPT1IE {
        match self.bits {
            false => CPT1IE::Disabled,
            true => CPT1IE::Enabled,
        }
    }
    #[doc = "Capture interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPT1IE::Disabled
    }
    #[doc = "Capture interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPT1IE::Enabled
    }
}
#[doc = "Field `CPT1IE` writer - CPT1IE"]
pub type CPT1IE_W<'a, REG> = crate::BitWriter<'a, REG, CPT1IE>;
impl<'a, REG> CPT1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CPT1IE::Disabled)
    }
    #[doc = "Capture interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CPT1IE::Enabled)
    }
}
#[doc = "Field `CPT2IE` reader - CPT2IE"]
pub use CPT1IE_R as CPT2IE_R;
#[doc = "Field `CPT2IE` writer - CPT2IE"]
pub use CPT1IE_W as CPT2IE_W;
#[doc = "SET1xIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETX1IE {
    #[doc = "0: Tx output set interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Tx output set interrupt enabled"]
    Enabled = 1,
}
impl From<SETX1IE> for bool {
    #[inline(always)]
    fn from(variant: SETX1IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETx1IE` reader - SET1xIE"]
pub type SETX1IE_R = crate::BitReader<SETX1IE>;
impl SETX1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SETX1IE {
        match self.bits {
            false => SETX1IE::Disabled,
            true => SETX1IE::Enabled,
        }
    }
    #[doc = "Tx output set interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SETX1IE::Disabled
    }
    #[doc = "Tx output set interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SETX1IE::Enabled
    }
}
#[doc = "Field `SETx1IE` writer - SET1xIE"]
pub type SETX1IE_W<'a, REG> = crate::BitWriter<'a, REG, SETX1IE>;
impl<'a, REG> SETX1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx output set interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SETX1IE::Disabled)
    }
    #[doc = "Tx output set interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SETX1IE::Enabled)
    }
}
#[doc = "RSTx1IE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTX1IE {
    #[doc = "0: Tx output reset interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Tx output reset interrupt enabled"]
    Enabled = 1,
}
impl From<RSTX1IE> for bool {
    #[inline(always)]
    fn from(variant: RSTX1IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTx1IE` reader - RSTx1IE"]
pub type RSTX1IE_R = crate::BitReader<RSTX1IE>;
impl RSTX1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTX1IE {
        match self.bits {
            false => RSTX1IE::Disabled,
            true => RSTX1IE::Enabled,
        }
    }
    #[doc = "Tx output reset interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTX1IE::Disabled
    }
    #[doc = "Tx output reset interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTX1IE::Enabled
    }
}
#[doc = "Field `RSTx1IE` writer - RSTx1IE"]
pub type RSTX1IE_W<'a, REG> = crate::BitWriter<'a, REG, RSTX1IE>;
impl<'a, REG> RSTX1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx output reset interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTX1IE::Disabled)
    }
    #[doc = "Tx output reset interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTX1IE::Enabled)
    }
}
#[doc = "Field `RSTx2IE` reader - RSTx2IE"]
pub use RSTX1IE_R as RSTX2IE_R;
#[doc = "Field `RSTx2IE` writer - RSTx2IE"]
pub use RSTX1IE_W as RSTX2IE_W;
#[doc = "Field `SETx2IE` reader - SETx2IE"]
pub use SETX1IE_R as SETX2IE_R;
#[doc = "Field `SETx2IE` writer - SETx2IE"]
pub use SETX1IE_W as SETX2IE_W;
#[doc = "RSTIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTIE {
    #[doc = "0: Timer x counter/reset roll-over interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Timer x counter/reset roll-over interrupt enabled"]
    Enabled = 1,
}
impl From<RSTIE> for bool {
    #[inline(always)]
    fn from(variant: RSTIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTIE` reader - RSTIE"]
pub type RSTIE_R = crate::BitReader<RSTIE>;
impl RSTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTIE {
        match self.bits {
            false => RSTIE::Disabled,
            true => RSTIE::Enabled,
        }
    }
    #[doc = "Timer x counter/reset roll-over interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTIE::Disabled
    }
    #[doc = "Timer x counter/reset roll-over interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTIE::Enabled
    }
}
#[doc = "Field `RSTIE` writer - RSTIE"]
pub type RSTIE_W<'a, REG> = crate::BitWriter<'a, REG, RSTIE>;
impl<'a, REG> RSTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer x counter/reset roll-over interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTIE::Disabled)
    }
    #[doc = "Timer x counter/reset roll-over interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTIE::Enabled)
    }
}
#[doc = "DLYPRTIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRTIE {
    #[doc = "0: Delayed protection interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Delayed protection interrupt enabled"]
    Enabled = 1,
}
impl From<DLYPRTIE> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLYPRTIE` reader - DLYPRTIE"]
pub type DLYPRTIE_R = crate::BitReader<DLYPRTIE>;
impl DLYPRTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLYPRTIE {
        match self.bits {
            false => DLYPRTIE::Disabled,
            true => DLYPRTIE::Enabled,
        }
    }
    #[doc = "Delayed protection interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTIE::Disabled
    }
    #[doc = "Delayed protection interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTIE::Enabled
    }
}
#[doc = "Field `DLYPRTIE` writer - DLYPRTIE"]
pub type DLYPRTIE_W<'a, REG> = crate::BitWriter<'a, REG, DLYPRTIE>;
impl<'a, REG> DLYPRTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Delayed protection interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRTIE::Disabled)
    }
    #[doc = "Delayed protection interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRTIE::Enabled)
    }
}
#[doc = "CMP1DE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1DE {
    #[doc = "0: Compare DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Compare DMA request enabled"]
    Enabled = 1,
}
impl From<CMP1DE> for bool {
    #[inline(always)]
    fn from(variant: CMP1DE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP1DE` reader - CMP1DE"]
pub type CMP1DE_R = crate::BitReader<CMP1DE>;
impl CMP1DE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMP1DE {
        match self.bits {
            false => CMP1DE::Disabled,
            true => CMP1DE::Enabled,
        }
    }
    #[doc = "Compare DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMP1DE::Disabled
    }
    #[doc = "Compare DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP1DE::Enabled
    }
}
#[doc = "Field `CMP1DE` writer - CMP1DE"]
pub type CMP1DE_W<'a, REG> = crate::BitWriter<'a, REG, CMP1DE>;
impl<'a, REG> CMP1DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1DE::Disabled)
    }
    #[doc = "Compare DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1DE::Enabled)
    }
}
#[doc = "Field `CMP2DE` reader - CMP2DE"]
pub use CMP1DE_R as CMP2DE_R;
#[doc = "Field `CMP3DE` reader - CMP3DE"]
pub use CMP1DE_R as CMP3DE_R;
#[doc = "Field `CMP4DE` reader - CMP4DE"]
pub use CMP1DE_R as CMP4DE_R;
#[doc = "Field `CMP2DE` writer - CMP2DE"]
pub use CMP1DE_W as CMP2DE_W;
#[doc = "Field `CMP3DE` writer - CMP3DE"]
pub use CMP1DE_W as CMP3DE_W;
#[doc = "Field `CMP4DE` writer - CMP4DE"]
pub use CMP1DE_W as CMP4DE_W;
#[doc = "REPDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPDE {
    #[doc = "0: Repetition DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Repetition DMA request enabled"]
    Enabled = 1,
}
impl From<REPDE> for bool {
    #[inline(always)]
    fn from(variant: REPDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPDE` reader - REPDE"]
pub type REPDE_R = crate::BitReader<REPDE>;
impl REPDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REPDE {
        match self.bits {
            false => REPDE::Disabled,
            true => REPDE::Enabled,
        }
    }
    #[doc = "Repetition DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPDE::Disabled
    }
    #[doc = "Repetition DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPDE::Enabled
    }
}
#[doc = "Field `REPDE` writer - REPDE"]
pub type REPDE_W<'a, REG> = crate::BitWriter<'a, REG, REPDE>;
impl<'a, REG> REPDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Repetition DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REPDE::Disabled)
    }
    #[doc = "Repetition DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REPDE::Enabled)
    }
}
#[doc = "UPDDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDDE {
    #[doc = "0: Update DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Update DMA request enabled"]
    Enabled = 1,
}
impl From<UPDDE> for bool {
    #[inline(always)]
    fn from(variant: UPDDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDDE` reader - UPDDE"]
pub type UPDDE_R = crate::BitReader<UPDDE>;
impl UPDDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UPDDE {
        match self.bits {
            false => UPDDE::Disabled,
            true => UPDDE::Enabled,
        }
    }
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDDE::Disabled
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDDE::Enabled
    }
}
#[doc = "Field `UPDDE` writer - UPDDE"]
pub type UPDDE_W<'a, REG> = crate::BitWriter<'a, REG, UPDDE>;
impl<'a, REG> UPDDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDDE::Disabled)
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UPDDE::Enabled)
    }
}
#[doc = "CPT1DE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPT1DE {
    #[doc = "0: Capture DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Capture DMA request enabled"]
    Enabled = 1,
}
impl From<CPT1DE> for bool {
    #[inline(always)]
    fn from(variant: CPT1DE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPT1DE` reader - CPT1DE"]
pub type CPT1DE_R = crate::BitReader<CPT1DE>;
impl CPT1DE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPT1DE {
        match self.bits {
            false => CPT1DE::Disabled,
            true => CPT1DE::Enabled,
        }
    }
    #[doc = "Capture DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPT1DE::Disabled
    }
    #[doc = "Capture DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPT1DE::Enabled
    }
}
#[doc = "Field `CPT1DE` writer - CPT1DE"]
pub type CPT1DE_W<'a, REG> = crate::BitWriter<'a, REG, CPT1DE>;
impl<'a, REG> CPT1DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CPT1DE::Disabled)
    }
    #[doc = "Capture DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CPT1DE::Enabled)
    }
}
#[doc = "Field `CPT2DE` reader - CPT2DE"]
pub use CPT1DE_R as CPT2DE_R;
#[doc = "Field `CPT2DE` writer - CPT2DE"]
pub use CPT1DE_W as CPT2DE_W;
#[doc = "SET1xDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETX1DE {
    #[doc = "0: Tx output set DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Tx output set DMA request enabled"]
    Enabled = 1,
}
impl From<SETX1DE> for bool {
    #[inline(always)]
    fn from(variant: SETX1DE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETx1DE` reader - SET1xDE"]
pub type SETX1DE_R = crate::BitReader<SETX1DE>;
impl SETX1DE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SETX1DE {
        match self.bits {
            false => SETX1DE::Disabled,
            true => SETX1DE::Enabled,
        }
    }
    #[doc = "Tx output set DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SETX1DE::Disabled
    }
    #[doc = "Tx output set DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SETX1DE::Enabled
    }
}
#[doc = "Field `SETx1DE` writer - SET1xDE"]
pub type SETX1DE_W<'a, REG> = crate::BitWriter<'a, REG, SETX1DE>;
impl<'a, REG> SETX1DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx output set DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SETX1DE::Disabled)
    }
    #[doc = "Tx output set DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SETX1DE::Enabled)
    }
}
#[doc = "RSTx1DE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTX1DE {
    #[doc = "0: Tx output reset DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Tx output reset DMA request enabled"]
    Enabled = 1,
}
impl From<RSTX1DE> for bool {
    #[inline(always)]
    fn from(variant: RSTX1DE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTx1DE` reader - RSTx1DE"]
pub type RSTX1DE_R = crate::BitReader<RSTX1DE>;
impl RSTX1DE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTX1DE {
        match self.bits {
            false => RSTX1DE::Disabled,
            true => RSTX1DE::Enabled,
        }
    }
    #[doc = "Tx output reset DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTX1DE::Disabled
    }
    #[doc = "Tx output reset DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTX1DE::Enabled
    }
}
#[doc = "Field `RSTx1DE` writer - RSTx1DE"]
pub type RSTX1DE_W<'a, REG> = crate::BitWriter<'a, REG, RSTX1DE>;
impl<'a, REG> RSTX1DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx output reset DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTX1DE::Disabled)
    }
    #[doc = "Tx output reset DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTX1DE::Enabled)
    }
}
#[doc = "Field `RSTx2DE` reader - RSTx2DE"]
pub use RSTX1DE_R as RSTX2DE_R;
#[doc = "Field `RSTx2DE` writer - RSTx2DE"]
pub use RSTX1DE_W as RSTX2DE_W;
#[doc = "Field `SETx2DE` reader - SETx2DE"]
pub use SETX1DE_R as SETX2DE_R;
#[doc = "Field `SETx2DE` writer - SETx2DE"]
pub use SETX1DE_W as SETX2DE_W;
#[doc = "RSTDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTDE {
    #[doc = "0: Timer x counter reset/roll-over DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Timer x counter reset/roll-over DMA request enabled"]
    Enabled = 1,
}
impl From<RSTDE> for bool {
    #[inline(always)]
    fn from(variant: RSTDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTDE` reader - RSTDE"]
pub type RSTDE_R = crate::BitReader<RSTDE>;
impl RSTDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTDE {
        match self.bits {
            false => RSTDE::Disabled,
            true => RSTDE::Enabled,
        }
    }
    #[doc = "Timer x counter reset/roll-over DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTDE::Disabled
    }
    #[doc = "Timer x counter reset/roll-over DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTDE::Enabled
    }
}
#[doc = "Field `RSTDE` writer - RSTDE"]
pub type RSTDE_W<'a, REG> = crate::BitWriter<'a, REG, RSTDE>;
impl<'a, REG> RSTDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer x counter reset/roll-over DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTDE::Disabled)
    }
    #[doc = "Timer x counter reset/roll-over DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTDE::Enabled)
    }
}
#[doc = "DLYPRTDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRTDE {
    #[doc = "0: Delayed protection DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Delayed protection DMA request enabled"]
    Enabled = 1,
}
impl From<DLYPRTDE> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLYPRTDE` reader - DLYPRTDE"]
pub type DLYPRTDE_R = crate::BitReader<DLYPRTDE>;
impl DLYPRTDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DLYPRTDE {
        match self.bits {
            false => DLYPRTDE::Disabled,
            true => DLYPRTDE::Enabled,
        }
    }
    #[doc = "Delayed protection DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTDE::Disabled
    }
    #[doc = "Delayed protection DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTDE::Enabled
    }
}
#[doc = "Field `DLYPRTDE` writer - DLYPRTDE"]
pub type DLYPRTDE_W<'a, REG> = crate::BitWriter<'a, REG, DLYPRTDE>;
impl<'a, REG> DLYPRTDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Delayed protection DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRTDE::Disabled)
    }
    #[doc = "Delayed protection DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DLYPRTDE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - CMP1IE"]
    #[inline(always)]
    pub fn cmp1ie(&self) -> CMP1IE_R {
        CMP1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP2IE"]
    #[inline(always)]
    pub fn cmp2ie(&self) -> CMP2IE_R {
        CMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMP3IE"]
    #[inline(always)]
    pub fn cmp3ie(&self) -> CMP3IE_R {
        CMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMP4IE"]
    #[inline(always)]
    pub fn cmp4ie(&self) -> CMP4IE_R {
        CMP4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REPIE"]
    #[inline(always)]
    pub fn repie(&self) -> REPIE_R {
        REPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - UPDIE"]
    #[inline(always)]
    pub fn updie(&self) -> UPDIE_R {
        UPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPT1IE"]
    #[inline(always)]
    pub fn cpt1ie(&self) -> CPT1IE_R {
        CPT1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPT2IE"]
    #[inline(always)]
    pub fn cpt2ie(&self) -> CPT2IE_R {
        CPT2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SET1xIE"]
    #[inline(always)]
    pub fn setx1ie(&self) -> SETX1IE_R {
        SETX1IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RSTx1IE"]
    #[inline(always)]
    pub fn rstx1ie(&self) -> RSTX1IE_R {
        RSTX1IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SETx2IE"]
    #[inline(always)]
    pub fn setx2ie(&self) -> SETX2IE_R {
        SETX2IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RSTx2IE"]
    #[inline(always)]
    pub fn rstx2ie(&self) -> RSTX2IE_R {
        RSTX2IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RSTIE"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DLYPRTIE"]
    #[inline(always)]
    pub fn dlyprtie(&self) -> DLYPRTIE_R {
        DLYPRTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - CMP1DE"]
    #[inline(always)]
    pub fn cmp1de(&self) -> CMP1DE_R {
        CMP1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CMP2DE"]
    #[inline(always)]
    pub fn cmp2de(&self) -> CMP2DE_R {
        CMP2DE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CMP3DE"]
    #[inline(always)]
    pub fn cmp3de(&self) -> CMP3DE_R {
        CMP3DE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CMP4DE"]
    #[inline(always)]
    pub fn cmp4de(&self) -> CMP4DE_R {
        CMP4DE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - REPDE"]
    #[inline(always)]
    pub fn repde(&self) -> REPDE_R {
        REPDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - UPDDE"]
    #[inline(always)]
    pub fn updde(&self) -> UPDDE_R {
        UPDDE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPT1DE"]
    #[inline(always)]
    pub fn cpt1de(&self) -> CPT1DE_R {
        CPT1DE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPT2DE"]
    #[inline(always)]
    pub fn cpt2de(&self) -> CPT2DE_R {
        CPT2DE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SET1xDE"]
    #[inline(always)]
    pub fn setx1de(&self) -> SETX1DE_R {
        SETX1DE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RSTx1DE"]
    #[inline(always)]
    pub fn rstx1de(&self) -> RSTX1DE_R {
        RSTX1DE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SETx2DE"]
    #[inline(always)]
    pub fn setx2de(&self) -> SETX2DE_R {
        SETX2DE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RSTx2DE"]
    #[inline(always)]
    pub fn rstx2de(&self) -> RSTX2DE_R {
        RSTX2DE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RSTDE"]
    #[inline(always)]
    pub fn rstde(&self) -> RSTDE_R {
        RSTDE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DLYPRTDE"]
    #[inline(always)]
    pub fn dlyprtde(&self) -> DLYPRTDE_R {
        DLYPRTDE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP1IE"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1ie(&mut self) -> CMP1IE_W<TIMBDIERrs> {
        CMP1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - CMP2IE"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2ie(&mut self) -> CMP2IE_W<TIMBDIERrs> {
        CMP2IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - CMP3IE"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3ie(&mut self) -> CMP3IE_W<TIMBDIERrs> {
        CMP3IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - CMP4IE"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4ie(&mut self) -> CMP4IE_W<TIMBDIERrs> {
        CMP4IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - REPIE"]
    #[inline(always)]
    #[must_use]
    pub fn repie(&mut self) -> REPIE_W<TIMBDIERrs> {
        REPIE_W::new(self, 4)
    }
    #[doc = "Bit 6 - UPDIE"]
    #[inline(always)]
    #[must_use]
    pub fn updie(&mut self) -> UPDIE_W<TIMBDIERrs> {
        UPDIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - CPT1IE"]
    #[inline(always)]
    #[must_use]
    pub fn cpt1ie(&mut self) -> CPT1IE_W<TIMBDIERrs> {
        CPT1IE_W::new(self, 7)
    }
    #[doc = "Bit 8 - CPT2IE"]
    #[inline(always)]
    #[must_use]
    pub fn cpt2ie(&mut self) -> CPT2IE_W<TIMBDIERrs> {
        CPT2IE_W::new(self, 8)
    }
    #[doc = "Bit 9 - SET1xIE"]
    #[inline(always)]
    #[must_use]
    pub fn setx1ie(&mut self) -> SETX1IE_W<TIMBDIERrs> {
        SETX1IE_W::new(self, 9)
    }
    #[doc = "Bit 10 - RSTx1IE"]
    #[inline(always)]
    #[must_use]
    pub fn rstx1ie(&mut self) -> RSTX1IE_W<TIMBDIERrs> {
        RSTX1IE_W::new(self, 10)
    }
    #[doc = "Bit 11 - SETx2IE"]
    #[inline(always)]
    #[must_use]
    pub fn setx2ie(&mut self) -> SETX2IE_W<TIMBDIERrs> {
        SETX2IE_W::new(self, 11)
    }
    #[doc = "Bit 12 - RSTx2IE"]
    #[inline(always)]
    #[must_use]
    pub fn rstx2ie(&mut self) -> RSTX2IE_W<TIMBDIERrs> {
        RSTX2IE_W::new(self, 12)
    }
    #[doc = "Bit 13 - RSTIE"]
    #[inline(always)]
    #[must_use]
    pub fn rstie(&mut self) -> RSTIE_W<TIMBDIERrs> {
        RSTIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - DLYPRTIE"]
    #[inline(always)]
    #[must_use]
    pub fn dlyprtie(&mut self) -> DLYPRTIE_W<TIMBDIERrs> {
        DLYPRTIE_W::new(self, 14)
    }
    #[doc = "Bit 16 - CMP1DE"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1de(&mut self) -> CMP1DE_W<TIMBDIERrs> {
        CMP1DE_W::new(self, 16)
    }
    #[doc = "Bit 17 - CMP2DE"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2de(&mut self) -> CMP2DE_W<TIMBDIERrs> {
        CMP2DE_W::new(self, 17)
    }
    #[doc = "Bit 18 - CMP3DE"]
    #[inline(always)]
    #[must_use]
    pub fn cmp3de(&mut self) -> CMP3DE_W<TIMBDIERrs> {
        CMP3DE_W::new(self, 18)
    }
    #[doc = "Bit 19 - CMP4DE"]
    #[inline(always)]
    #[must_use]
    pub fn cmp4de(&mut self) -> CMP4DE_W<TIMBDIERrs> {
        CMP4DE_W::new(self, 19)
    }
    #[doc = "Bit 20 - REPDE"]
    #[inline(always)]
    #[must_use]
    pub fn repde(&mut self) -> REPDE_W<TIMBDIERrs> {
        REPDE_W::new(self, 20)
    }
    #[doc = "Bit 22 - UPDDE"]
    #[inline(always)]
    #[must_use]
    pub fn updde(&mut self) -> UPDDE_W<TIMBDIERrs> {
        UPDDE_W::new(self, 22)
    }
    #[doc = "Bit 23 - CPT1DE"]
    #[inline(always)]
    #[must_use]
    pub fn cpt1de(&mut self) -> CPT1DE_W<TIMBDIERrs> {
        CPT1DE_W::new(self, 23)
    }
    #[doc = "Bit 24 - CPT2DE"]
    #[inline(always)]
    #[must_use]
    pub fn cpt2de(&mut self) -> CPT2DE_W<TIMBDIERrs> {
        CPT2DE_W::new(self, 24)
    }
    #[doc = "Bit 25 - SET1xDE"]
    #[inline(always)]
    #[must_use]
    pub fn setx1de(&mut self) -> SETX1DE_W<TIMBDIERrs> {
        SETX1DE_W::new(self, 25)
    }
    #[doc = "Bit 26 - RSTx1DE"]
    #[inline(always)]
    #[must_use]
    pub fn rstx1de(&mut self) -> RSTX1DE_W<TIMBDIERrs> {
        RSTX1DE_W::new(self, 26)
    }
    #[doc = "Bit 27 - SETx2DE"]
    #[inline(always)]
    #[must_use]
    pub fn setx2de(&mut self) -> SETX2DE_W<TIMBDIERrs> {
        SETX2DE_W::new(self, 27)
    }
    #[doc = "Bit 28 - RSTx2DE"]
    #[inline(always)]
    #[must_use]
    pub fn rstx2de(&mut self) -> RSTX2DE_W<TIMBDIERrs> {
        RSTX2DE_W::new(self, 28)
    }
    #[doc = "Bit 29 - RSTDE"]
    #[inline(always)]
    #[must_use]
    pub fn rstde(&mut self) -> RSTDE_W<TIMBDIERrs> {
        RSTDE_W::new(self, 29)
    }
    #[doc = "Bit 30 - DLYPRTDE"]
    #[inline(always)]
    #[must_use]
    pub fn dlyprtde(&mut self) -> DLYPRTDE_W<TIMBDIERrs> {
        DLYPRTDE_W::new(self, 30)
    }
}
#[doc = "TIMxDIER5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timbdier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timbdier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMBDIERrs;
impl crate::RegisterSpec for TIMBDIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timbdier::R`](R) reader structure"]
impl crate::Readable for TIMBDIERrs {}
#[doc = "`write(|w| ..)` method takes [`timbdier::W`](W) writer structure"]
impl crate::Writable for TIMBDIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMBDIER to value 0"]
impl crate::Resettable for TIMBDIERrs {
    const RESET_VALUE: u32 = 0;
}
