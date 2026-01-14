///Register `C2CR` reader
pub type R = crate::R<C2CRrs>;
///Register `C2CR` writer
pub type W = crate::W<C2CRrs>;
/**Programming

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PG {
    ///0: Flash programming disabled
    Disabled = 0,
    ///1: Flash programming enabled
    Enabled = 1,
}
impl From<PG> for bool {
    #[inline(always)]
    fn from(variant: PG) -> Self {
        variant as u8 != 0
    }
}
///Field `PG` reader - Programming
pub type PG_R = crate::BitReader<PG>;
impl PG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PG {
        match self.bits {
            false => PG::Disabled,
            true => PG::Enabled,
        }
    }
    ///Flash programming disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PG::Disabled
    }
    ///Flash programming enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PG::Enabled
    }
}
///Field `PG` writer - Programming
pub type PG_W<'a, REG> = crate::BitWriter<'a, REG, PG>;
impl<'a, REG> PG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Flash programming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PG::Disabled)
    }
    ///Flash programming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PG::Enabled)
    }
}
/**Page erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER {
    ///0: Page erase disabled
    Disabled = 0,
    ///1: Page erase enabled
    Enabled = 1,
}
impl From<PER> for bool {
    #[inline(always)]
    fn from(variant: PER) -> Self {
        variant as u8 != 0
    }
}
///Field `PER` reader - Page erase
pub type PER_R = crate::BitReader<PER>;
impl PER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PER {
        match self.bits {
            false => PER::Disabled,
            true => PER::Enabled,
        }
    }
    ///Page erase disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PER::Disabled
    }
    ///Page erase enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PER::Enabled
    }
}
///Field `PER` writer - Page erase
pub type PER_W<'a, REG> = crate::BitWriter<'a, REG, PER>;
impl<'a, REG> PER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Page erase disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PER::Disabled)
    }
    ///Page erase enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PER::Enabled)
    }
}
/**Mass erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MER {
    ///0: No mass erase
    NoErase = 0,
    ///1: Trigger mass erase
    MassErase = 1,
}
impl From<MER> for bool {
    #[inline(always)]
    fn from(variant: MER) -> Self {
        variant as u8 != 0
    }
}
///Field `MER` reader - Mass erase
pub type MER_R = crate::BitReader<MER>;
impl MER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MER {
        match self.bits {
            false => MER::NoErase,
            true => MER::MassErase,
        }
    }
    ///No mass erase
    #[inline(always)]
    pub fn is_no_erase(&self) -> bool {
        *self == MER::NoErase
    }
    ///Trigger mass erase
    #[inline(always)]
    pub fn is_mass_erase(&self) -> bool {
        *self == MER::MassErase
    }
}
///Field `MER` writer - Mass erase
pub type MER_W<'a, REG> = crate::BitWriter<'a, REG, MER>;
impl<'a, REG> MER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No mass erase
    #[inline(always)]
    pub fn no_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER::NoErase)
    }
    ///Trigger mass erase
    #[inline(always)]
    pub fn mass_erase(self) -> &'a mut crate::W<REG> {
        self.variant(MER::MassErase)
    }
}
///Field `PNB` reader - Page number selection
pub type PNB_R = crate::FieldReader;
///Field `PNB` writer - Page number selection
pub type PNB_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
/**Start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTR {
    ///0: Options modification completed or idle
    Done = 0,
}
impl From<STRTR> for bool {
    #[inline(always)]
    fn from(variant: STRTR) -> Self {
        variant as u8 != 0
    }
}
///Field `STRT` reader - Start
pub type STRT_R = crate::BitReader<STRTR>;
impl STRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STRTR> {
        match self.bits {
            false => Some(STRTR::Done),
            _ => None,
        }
    }
    ///Options modification completed or idle
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == STRTR::Done
    }
}
/**Start

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTW {
    ///1: Trigger options programming operation
    Start = 1,
}
impl From<STRTW> for bool {
    #[inline(always)]
    fn from(variant: STRTW) -> Self {
        variant as u8 != 0
    }
}
///Field `STRT` writer - Start
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG, STRTW>;
impl<'a, REG> STRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger options programming operation
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STRTW::Start)
    }
}
/**Fast programming

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSTPG {
    ///0: Fast programming disabled
    Disabled = 0,
    ///1: Fast programming enabled
    Enabled = 1,
}
impl From<FSTPG> for bool {
    #[inline(always)]
    fn from(variant: FSTPG) -> Self {
        variant as u8 != 0
    }
}
///Field `FSTPG` reader - Fast programming
pub type FSTPG_R = crate::BitReader<FSTPG>;
impl FSTPG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSTPG {
        match self.bits {
            false => FSTPG::Disabled,
            true => FSTPG::Enabled,
        }
    }
    ///Fast programming disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FSTPG::Disabled
    }
    ///Fast programming enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FSTPG::Enabled
    }
}
///Field `FSTPG` writer - Fast programming
pub type FSTPG_W<'a, REG> = crate::BitWriter<'a, REG, FSTPG>;
impl<'a, REG> FSTPG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Fast programming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSTPG::Disabled)
    }
    ///Fast programming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSTPG::Enabled)
    }
}
/**End of operation interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPIE {
    ///0: End of program interrupt disable
    Disabled = 0,
    ///1: End of program interrupt enable
    Enabled = 1,
}
impl From<EOPIE> for bool {
    #[inline(always)]
    fn from(variant: EOPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOPIE` reader - End of operation interrupt enable
pub type EOPIE_R = crate::BitReader<EOPIE>;
impl EOPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOPIE {
        match self.bits {
            false => EOPIE::Disabled,
            true => EOPIE::Enabled,
        }
    }
    ///End of program interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOPIE::Disabled
    }
    ///End of program interrupt enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOPIE::Enabled
    }
}
///Field `EOPIE` writer - End of operation interrupt enable
pub type EOPIE_W<'a, REG> = crate::BitWriter<'a, REG, EOPIE>;
impl<'a, REG> EOPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///End of program interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE::Disabled)
    }
    ///End of program interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOPIE::Enabled)
    }
}
/**Error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    ///0: OPERR Error interrupt disable
    Disabled = 0,
    ///1: OPERR Error interrupt enable
    Enabled = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRIE` reader - Error interrupt enable
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Disabled,
            true => ERRIE::Enabled,
        }
    }
    ///OPERR Error interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE::Disabled
    }
    ///OPERR Error interrupt enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE::Enabled
    }
}
///Field `ERRIE` writer - Error interrupt enable
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///OPERR Error interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Disabled)
    }
    ///OPERR Error interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Enabled)
    }
}
/**RDERRIE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRIE {
    ///0: PCROP read error interrupt disable
    Disabled = 0,
    ///1: PCROP read error interrupt enable
    Enabled = 1,
}
impl From<RDERRIE> for bool {
    #[inline(always)]
    fn from(variant: RDERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RDERRIE` reader - RDERRIE
pub type RDERRIE_R = crate::BitReader<RDERRIE>;
impl RDERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RDERRIE {
        match self.bits {
            false => RDERRIE::Disabled,
            true => RDERRIE::Enabled,
        }
    }
    ///PCROP read error interrupt disable
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDERRIE::Disabled
    }
    ///PCROP read error interrupt enable
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDERRIE::Enabled
    }
}
///Field `RDERRIE` writer - RDERRIE
pub type RDERRIE_W<'a, REG> = crate::BitWriter<'a, REG, RDERRIE>;
impl<'a, REG> RDERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PCROP read error interrupt disable
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRIE::Disabled)
    }
    ///PCROP read error interrupt enable
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDERRIE::Enabled)
    }
}
impl R {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Mass erase
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:9 - Page number selection
    #[inline(always)]
    pub fn pnb(&self) -> PNB_R {
        PNB_R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    pub fn fstpg(&self) -> FSTPG_R {
        FSTPG_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - RDERRIE
    #[inline(always)]
    pub fn rderrie(&self) -> RDERRIE_R {
        RDERRIE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2CR")
            .field("pg", &self.pg())
            .field("per", &self.per())
            .field("mer", &self.mer())
            .field("pnb", &self.pnb())
            .field("strt", &self.strt())
            .field("fstpg", &self.fstpg())
            .field("eopie", &self.eopie())
            .field("errie", &self.errie())
            .field("rderrie", &self.rderrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Programming
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W<'_, C2CRrs> {
        PG_W::new(self, 0)
    }
    ///Bit 1 - Page erase
    #[inline(always)]
    pub fn per(&mut self) -> PER_W<'_, C2CRrs> {
        PER_W::new(self, 1)
    }
    ///Bit 2 - Mass erase
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W<'_, C2CRrs> {
        MER_W::new(self, 2)
    }
    ///Bits 3:9 - Page number selection
    #[inline(always)]
    pub fn pnb(&mut self) -> PNB_W<'_, C2CRrs> {
        PNB_W::new(self, 3)
    }
    ///Bit 16 - Start
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<'_, C2CRrs> {
        STRT_W::new(self, 16)
    }
    ///Bit 18 - Fast programming
    #[inline(always)]
    pub fn fstpg(&mut self) -> FSTPG_W<'_, C2CRrs> {
        FSTPG_W::new(self, 18)
    }
    ///Bit 24 - End of operation interrupt enable
    #[inline(always)]
    pub fn eopie(&mut self) -> EOPIE_W<'_, C2CRrs> {
        EOPIE_W::new(self, 24)
    }
    ///Bit 25 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, C2CRrs> {
        ERRIE_W::new(self, 25)
    }
    ///Bit 26 - RDERRIE
    #[inline(always)]
    pub fn rderrie(&mut self) -> RDERRIE_W<'_, C2CRrs> {
        RDERRIE_W::new(self, 26)
    }
}
/**Flash CPU2 control register

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#FLASH:C2CR)*/
pub struct C2CRrs;
impl crate::RegisterSpec for C2CRrs {
    type Ux = u32;
}
///`read()` method returns [`c2cr::R`](R) reader structure
impl crate::Readable for C2CRrs {}
///`write(|w| ..)` method takes [`c2cr::W`](W) writer structure
impl crate::Writable for C2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2CR to value 0xc000_0000
impl crate::Resettable for C2CRrs {
    const RESET_VALUE: u32 = 0xc000_0000;
}
