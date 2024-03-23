#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "TAMP1NOER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1NOER {
    #[doc = "0: Tamper x event erases the backup registers"]
    Erase = 0,
    #[doc = "1: Tamper x event does not erase the backup registers"]
    NotErase = 1,
}
impl From<TAMP1NOER> for bool {
    #[inline(always)]
    fn from(variant: TAMP1NOER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1NOER` reader - TAMP1NOER"]
pub type TAMP1NOER_R = crate::BitReader<TAMP1NOER>;
impl TAMP1NOER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1NOER {
        match self.bits {
            false => TAMP1NOER::Erase,
            true => TAMP1NOER::NotErase,
        }
    }
    #[doc = "Tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == TAMP1NOER::Erase
    }
    #[doc = "Tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn is_not_erase(&self) -> bool {
        *self == TAMP1NOER::NotErase
    }
}
#[doc = "Field `TAMP1NOER` writer - TAMP1NOER"]
pub type TAMP1NOER_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1NOER>;
impl<'a, REG> TAMP1NOER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper x event erases the backup registers"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1NOER::Erase)
    }
    #[doc = "Tamper x event does not erase the backup registers"]
    #[inline(always)]
    pub fn not_erase(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1NOER::NotErase)
    }
}
#[doc = "Field `TAMP2NOER` reader - TAMP2NOER"]
pub use TAMP1NOER_R as TAMP2NOER_R;
#[doc = "Field `TAMP3NOER` reader - TAMP3NOER"]
pub use TAMP1NOER_R as TAMP3NOER_R;
#[doc = "Field `TAMP2NOER` writer - TAMP2NOER"]
pub use TAMP1NOER_W as TAMP2NOER_W;
#[doc = "Field `TAMP3NOER` writer - TAMP3NOER"]
pub use TAMP1NOER_W as TAMP3NOER_W;
#[doc = "TAMP1MSK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1MSK {
    #[doc = "0: Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    ResetBySoftware = 0,
    #[doc = "1: Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased. The tamper x interrupt must not be enabled when TAMP3MSK is set"]
    ResetByHardware = 1,
}
impl From<TAMP1MSK> for bool {
    #[inline(always)]
    fn from(variant: TAMP1MSK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1MSK` reader - TAMP1MSK"]
pub type TAMP1MSK_R = crate::BitReader<TAMP1MSK>;
impl TAMP1MSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1MSK {
        match self.bits {
            false => TAMP1MSK::ResetBySoftware,
            true => TAMP1MSK::ResetByHardware,
        }
    }
    #[doc = "Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    #[inline(always)]
    pub fn is_reset_by_software(&self) -> bool {
        *self == TAMP1MSK::ResetBySoftware
    }
    #[doc = "Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased. The tamper x interrupt must not be enabled when TAMP3MSK is set"]
    #[inline(always)]
    pub fn is_reset_by_hardware(&self) -> bool {
        *self == TAMP1MSK::ResetByHardware
    }
}
#[doc = "Field `TAMP1MSK` writer - TAMP1MSK"]
pub type TAMP1MSK_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1MSK>;
impl<'a, REG> TAMP1MSK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper x event generates a trigger event and TAMPxF must be cleared by software to allow next tamper event detection"]
    #[inline(always)]
    pub fn reset_by_software(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1MSK::ResetBySoftware)
    }
    #[doc = "Tamper x event generates a trigger event. TAMPxF is masked and internally cleared by hardware. The backup registers are not erased. The tamper x interrupt must not be enabled when TAMP3MSK is set"]
    #[inline(always)]
    pub fn reset_by_hardware(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1MSK::ResetByHardware)
    }
}
#[doc = "Field `TAMP2MSK` reader - TAMP2MSK"]
pub use TAMP1MSK_R as TAMP2MSK_R;
#[doc = "Field `TAMP3MSK` reader - TAMP3MSK"]
pub use TAMP1MSK_R as TAMP3MSK_R;
#[doc = "Field `TAMP2MSK` writer - TAMP2MSK"]
pub use TAMP1MSK_W as TAMP2MSK_W;
#[doc = "Field `TAMP3MSK` writer - TAMP3MSK"]
pub use TAMP1MSK_W as TAMP3MSK_W;
#[doc = "Backup registerserase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKERASEW {
    #[doc = "1: Reset backup registers"]
    Reset = 1,
}
impl From<BKERASEW> for bool {
    #[inline(always)]
    fn from(variant: BKERASEW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKERASE` reader - Backup registerserase"]
pub type BKERASE_R = crate::BitReader<BKERASEW>;
impl BKERASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BKERASEW> {
        match self.bits {
            true => Some(BKERASEW::Reset),
            _ => None,
        }
    }
    #[doc = "Reset backup registers"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BKERASEW::Reset
    }
}
#[doc = "Field `BKERASE` writer - Backup registerserase"]
pub type BKERASE_W<'a, REG> = crate::BitWriter<'a, REG, BKERASEW>;
impl<'a, REG> BKERASE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset backup registers"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BKERASEW::Reset)
    }
}
#[doc = "TAMP1TRG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1TRG {
    #[doc = "0: If TAMPFLT != 00 Tamper x input staying low triggers a tamper detection event. If TAMPFLT = 00 Tamper x input rising edge and high level triggers a tamper detection event"]
    FilteredLowOrUnfilteredHigh = 0,
    #[doc = "1: If TAMPFLT != 00 Tamper x input staying high triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge and low level triggers a tamper detection event"]
    FilteredHighOrUnfilteredLow = 1,
}
impl From<TAMP1TRG> for bool {
    #[inline(always)]
    fn from(variant: TAMP1TRG) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1TRG` reader - TAMP1TRG"]
pub type TAMP1TRG_R = crate::BitReader<TAMP1TRG>;
impl TAMP1TRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1TRG {
        match self.bits {
            false => TAMP1TRG::FilteredLowOrUnfilteredHigh,
            true => TAMP1TRG::FilteredHighOrUnfilteredLow,
        }
    }
    #[doc = "If TAMPFLT != 00 Tamper x input staying low triggers a tamper detection event. If TAMPFLT = 00 Tamper x input rising edge and high level triggers a tamper detection event"]
    #[inline(always)]
    pub fn is_filtered_low_or_unfiltered_high(&self) -> bool {
        *self == TAMP1TRG::FilteredLowOrUnfilteredHigh
    }
    #[doc = "If TAMPFLT != 00 Tamper x input staying high triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge and low level triggers a tamper detection event"]
    #[inline(always)]
    pub fn is_filtered_high_or_unfiltered_low(&self) -> bool {
        *self == TAMP1TRG::FilteredHighOrUnfilteredLow
    }
}
#[doc = "Field `TAMP1TRG` writer - TAMP1TRG"]
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1TRG>;
impl<'a, REG> TAMP1TRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If TAMPFLT != 00 Tamper x input staying low triggers a tamper detection event. If TAMPFLT = 00 Tamper x input rising edge and high level triggers a tamper detection event"]
    #[inline(always)]
    pub fn filtered_low_or_unfiltered_high(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1TRG::FilteredLowOrUnfilteredHigh)
    }
    #[doc = "If TAMPFLT != 00 Tamper x input staying high triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge and low level triggers a tamper detection event"]
    #[inline(always)]
    pub fn filtered_high_or_unfiltered_low(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1TRG::FilteredHighOrUnfilteredLow)
    }
}
#[doc = "Field `TAMP2TRG` reader - TAMP2TRG"]
pub use TAMP1TRG_R as TAMP2TRG_R;
#[doc = "Field `TAMP3TRG` reader - TAMP3TRG"]
pub use TAMP1TRG_R as TAMP3TRG_R;
#[doc = "Field `TAMP2TRG` writer - TAMP2TRG"]
pub use TAMP1TRG_W as TAMP2TRG_W;
#[doc = "Field `TAMP3TRG` writer - TAMP3TRG"]
pub use TAMP1TRG_W as TAMP3TRG_W;
impl R {
    #[doc = "Bit 0 - TAMP1NOER"]
    #[inline(always)]
    pub fn tamp1noer(&self) -> TAMP1NOER_R {
        TAMP1NOER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2NOER"]
    #[inline(always)]
    pub fn tamp2noer(&self) -> TAMP2NOER_R {
        TAMP2NOER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3NOER"]
    #[inline(always)]
    pub fn tamp3noer(&self) -> TAMP3NOER_R {
        TAMP3NOER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - TAMP1MSK"]
    #[inline(always)]
    pub fn tamp1msk(&self) -> TAMP1MSK_R {
        TAMP1MSK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TAMP2MSK"]
    #[inline(always)]
    pub fn tamp2msk(&self) -> TAMP2MSK_R {
        TAMP2MSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TAMP3MSK"]
    #[inline(always)]
    pub fn tamp3msk(&self) -> TAMP3MSK_R {
        TAMP3MSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 23 - Backup registerserase"]
    #[inline(always)]
    pub fn bkerase(&self) -> BKERASE_R {
        BKERASE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TAMP1TRG"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TAMP2TRG"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TAMP3TRG"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1noer(&mut self) -> TAMP1NOER_W<CR2rs> {
        TAMP1NOER_W::new(self, 0)
    }
    #[doc = "Bit 1 - TAMP2NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2noer(&mut self) -> TAMP2NOER_W<CR2rs> {
        TAMP2NOER_W::new(self, 1)
    }
    #[doc = "Bit 2 - TAMP3NOER"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3noer(&mut self) -> TAMP3NOER_W<CR2rs> {
        TAMP3NOER_W::new(self, 2)
    }
    #[doc = "Bit 16 - TAMP1MSK"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1msk(&mut self) -> TAMP1MSK_W<CR2rs> {
        TAMP1MSK_W::new(self, 16)
    }
    #[doc = "Bit 17 - TAMP2MSK"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2msk(&mut self) -> TAMP2MSK_W<CR2rs> {
        TAMP2MSK_W::new(self, 17)
    }
    #[doc = "Bit 18 - TAMP3MSK"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3msk(&mut self) -> TAMP3MSK_W<CR2rs> {
        TAMP3MSK_W::new(self, 18)
    }
    #[doc = "Bit 23 - Backup registerserase"]
    #[inline(always)]
    #[must_use]
    pub fn bkerase(&mut self) -> BKERASE_W<CR2rs> {
        BKERASE_W::new(self, 23)
    }
    #[doc = "Bit 24 - TAMP1TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<CR2rs> {
        TAMP1TRG_W::new(self, 24)
    }
    #[doc = "Bit 25 - TAMP2TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<CR2rs> {
        TAMP2TRG_W::new(self, 25)
    }
    #[doc = "Bit 26 - TAMP3TRG"]
    #[inline(always)]
    #[must_use]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<CR2rs> {
        TAMP3TRG_W::new(self, 26)
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
