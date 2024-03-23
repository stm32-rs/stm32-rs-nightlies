#[doc = "Register `C2CR` reader"]
pub type R = crate::R<C2CRrs>;
#[doc = "Register `C2CR` writer"]
pub type W = crate::W<C2CRrs>;
#[doc = "Programming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PG {
    #[doc = "0: Flash programming disabled"]
    Disabled = 0,
    #[doc = "1: Flash programming enabled"]
    Enabled = 1,
}
impl From<PG> for bool {
    #[inline(always)]
    fn from(variant: PG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PG` reader - Programming"]
pub type PG_R = crate::BitReader<PG>;
impl PG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PG {
        match self.bits {
            false => PG::Disabled,
            true => PG::Enabled,
        }
    }
    #[doc = "Flash programming disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PG::Disabled
    }
    #[doc = "Flash programming enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PG::Enabled
    }
}
#[doc = "Field `PG` writer - Programming"]
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG, PG>;
impl<'a, REG> PG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash programming disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PG::Disabled)
    }
    #[doc = "Flash programming enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PG::Enabled)
    }
}
#[doc = "Page erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER {
    #[doc = "0: Page erase disabled"]
    Disabled = 0,
    #[doc = "1: Page erase enabled"]
    Enabled = 1,
}
impl From<PER> for bool {
    #[inline(always)]
    fn from(variant: PER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - Page erase"]
pub type PER_R = crate::BitReader<PER>;
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PER {
        match self.bits {
            false => PER::Disabled,
            true => PER::Enabled,
        }
    }
    #[doc = "Page erase disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PER::Disabled
    }
    #[doc = "Page erase enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PER::Enabled
    }
}
#[doc = "Field `PER` writer - Page erase"]
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG, PER>;
impl<'a, REG> PER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Page erase disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PER::Disabled)
    }
    #[doc = "Page erase enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PER::Enabled)
    }
}
#[doc = "Mass erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER {
    #[doc = "0: No mass erase"]
    NoErase = 0,
    #[doc = "1: Trigger mass erase"]
    MassErase = 1,
}
impl From<MER> for bool {
    #[inline(always)]
    fn from(variant: MER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MER` reader - Mass erase"]
pub type MER_R = crate::BitReader<MER>;
impl MER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MER {
        match self.bits {
            false => MER::NoErase,
            true => MER::MassErase,
        }
    }
    #[doc = "No mass erase"]
    #[inline(always)]
    pub fn is_no_erase(&self) -> bool {
        *self == MER::NoErase
    }
    #[doc = "Trigger mass erase"]
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER::MassErase
    }
}
#[doc = "Field `MER` writer - Mass erase"]
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG, MER>;
impl<'a, REG> MER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No mass erase"]
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER::NoErase)
    }
    #[doc = "Trigger mass erase"]
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER::MassErase)
    }
}
#[doc = "Field `PNB` reader - Page number selection"]
pub type PNB_R = crate::FieldReader;
#[doc = "Field `PNB` writer - Page number selection"]
pub type PNB_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTR {
    #[doc = "0: Options modification completed or idle"]
    Done = 0,
}
impl From<STRTR> for bool {
    #[inline(always)]
    fn from(variant: STRTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT` reader - Start"]
pub type STRT_R = crate::BitReader<STRTR>;
impl STRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STRTR> {
        match self.bits {
            false => Some(STRTR::Done),
            _ => None,
        }
    }
    #[doc = "Options modification completed or idle"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == STRTR::Done
    }
}
#[doc = "Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTW {
    #[doc = "1: Trigger options programming operation"]
    Start = 1,
}
impl From<STRTW> for bool {
    #[inline(always)]
    fn from(variant: STRTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRT` writer - Start"]
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG, STRTW>;
impl<'a, REG> STRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger options programming operation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STRTW::Start)
    }
}
#[doc = "Fast programming\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSTPG {
    #[doc = "0: Fast programming disabled"]
    Disabled = 0,
    #[doc = "1: Fast programming enabled"]
    Enabled = 1,
}
impl From<FSTPG> for bool {
    #[inline(always)]
    fn from(variant: FSTPG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSTPG` reader - Fast programming"]
pub type FSTPG_R = crate::BitReader<FSTPG>;
impl FSTPG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FSTPG {
        match self.bits {
            false => FSTPG::Disabled,
            true => FSTPG::Enabled,
        }
    }
    #[doc = "Fast programming disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FSTPG::Disabled
    }
    #[doc = "Fast programming enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FSTPG::Enabled
    }
}
#[doc = "Field `FSTPG` writer - Fast programming"]
pub type FSTPG_W<'a, REG> = crate::BitWriter<'a, REG, FSTPG>;
impl<'a, REG> FSTPG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast programming disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSTPG::Disabled)
    }
    #[doc = "Fast programming enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSTPG::Enabled)
    }
}
#[doc = "End of operation interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPIE {
    #[doc = "0: End of program interrupt disable"]
    Disabled = 0,
    #[doc = "1: End of program interrupt enable"]
    Enabled = 1,
}
impl From<EOPIE> for bool {
    #[inline(always)]
    fn from(variant: EOPIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOPIE` reader - End of operation interrupt enable"]
pub type EOPIE_R = crate::BitReader<EOPIE>;
impl EOPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOPIE {
        match self.bits {
            false => EOPIE::Disabled,
            true => EOPIE::Enabled,
        }
    }
    #[doc = "End of program interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOPIE::Disabled
    }
    #[doc = "End of program interrupt enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOPIE::Enabled
    }
}
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOPIE>;
impl<'a, REG> EOPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "End of program interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE::Disabled)
    }
    #[doc = "End of program interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE::Enabled)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    #[doc = "0: OPERR Error interrupt disable"]
    Disabled = 0,
    #[doc = "1: OPERR Error interrupt enable"]
    Enabled = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Disabled,
            true => ERRIE::Enabled,
        }
    }
    #[doc = "OPERR Error interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE::Disabled
    }
    #[doc = "OPERR Error interrupt enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE::Enabled
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OPERR Error interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Disabled)
    }
    #[doc = "OPERR Error interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Enabled)
    }
}
#[doc = "RDERRIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRIE {
    #[doc = "0: PCROP read error interrupt disable"]
    Disabled = 0,
    #[doc = "1: PCROP read error interrupt enable"]
    Enabled = 1,
}
impl From<RDERRIE> for bool {
    #[inline(always)]
    fn from(variant: RDERRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDERRIE` reader - RDERRIE"]
pub type RDERRIE_R = crate::BitReader<RDERRIE>;
impl RDERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDERRIE {
        match self.bits {
            false => RDERRIE::Disabled,
            true => RDERRIE::Enabled,
        }
    }
    #[doc = "PCROP read error interrupt disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDERRIE::Disabled
    }
    #[doc = "PCROP read error interrupt enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDERRIE::Enabled
    }
}
#[doc = "Field `RDERRIE` writer - RDERRIE"]
pub type RDERRIE_W<'a, REG> = crate::BitWriter<'a, REG, RDERRIE>;
impl<'a, REG> RDERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PCROP read error interrupt disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRIE::Disabled)
    }
    #[doc = "PCROP read error interrupt enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass erase"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:9 - Page number selection"]
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    pub fn fstpg(&self) -> FSTPG_R {
        FSTPG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RDERRIE"]
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<C2CRrs> {
        PG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Page erase"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<C2CRrs> {
        PER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mass erase"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<C2CRrs> {
        MER_W::new(self, 2)
    }
    #[doc = "Bits 3:9 - Page number selection"]
    #[inline(always)]
    #[must_use]
    pub fn pnb(&mut self) -> PNB_W<C2CRrs> {
        PNB_W::new(self, 3)
    }
    #[doc = "Bit 16 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<C2CRrs> {
        STRT_W::new(self, 16)
    }
    #[doc = "Bit 18 - Fast programming"]
    #[inline(always)]
    #[must_use]
    pub fn fstpg(&mut self) -> FSTPG_W<C2CRrs> {
        FSTPG_W::new(self, 18)
    }
    #[doc = "Bit 24 - End of operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<C2CRrs> {
        EOPIE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<C2CRrs> {
        ERRIE_W::new(self, 25)
    }
    #[doc = "Bit 26 - RDERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rderrie(&mut self) -> RDERRIE_W<C2CRrs> {
        RDERRIE_W::new(self, 26)
    }
}
#[doc = "Flash CPU2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2CRrs;
impl crate::RegisterSpec for C2CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2cr::R`](R) reader structure"]
impl crate::Readable for C2CRrs {}
#[doc = "`write(|w| ..)` method takes [`c2cr::W`](W) writer structure"]
impl crate::Writable for C2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2CR to value 0xc000_0000"]
impl crate::Resettable for C2CRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
