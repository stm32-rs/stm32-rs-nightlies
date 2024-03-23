#[doc = "Register `DTAR` reader"]
pub type R = crate::R<DTARrs>;
#[doc = "Register `DTAR` writer"]
pub type W = crate::W<DTARrs>;
#[doc = "Field `DTRx` reader - Deadtime Rising value"]
pub type DTRX_R = crate::FieldReader<u16>;
#[doc = "Field `DTRx` writer - Deadtime Rising value"]
pub type DTRX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 9, u16>;
#[doc = "Sign Deadtime Rising value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTRX {
    #[doc = "0: Positive deadtime on rising edge"]
    Positive = 0,
    #[doc = "1: Negative deadtime on rising edge"]
    Negative = 1,
}
impl From<SDTRX> for bool {
    #[inline(always)]
    fn from(variant: SDTRX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDTRx` reader - Sign Deadtime Rising value"]
pub type SDTRX_R = crate::BitReader<SDTRX>;
impl SDTRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDTRX {
        match self.bits {
            false => SDTRX::Positive,
            true => SDTRX::Negative,
        }
    }
    #[doc = "Positive deadtime on rising edge"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == SDTRX::Positive
    }
    #[doc = "Negative deadtime on rising edge"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SDTRX::Negative
    }
}
#[doc = "Field `SDTRx` writer - Sign Deadtime Rising value"]
pub type SDTRX_W<'a, REG> = crate::BitWriter<'a, REG, SDTRX>;
impl<'a, REG> SDTRX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Positive deadtime on rising edge"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(SDTRX::Positive)
    }
    #[doc = "Negative deadtime on rising edge"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(SDTRX::Negative)
    }
}
#[doc = "Field `DTPRSC` reader - Deadtime Prescaler"]
pub type DTPRSC_R = crate::FieldReader;
#[doc = "Field `DTPRSC` writer - Deadtime Prescaler"]
pub type DTPRSC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
#[doc = "Deadtime Rising Sign Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRSLKX {
    #[doc = "0: Deadtime rising sign is writable"]
    Unlocked = 0,
    #[doc = "1: Deadtime rising sign is read-only"]
    Locked = 1,
}
impl From<DTRSLKX> for bool {
    #[inline(always)]
    fn from(variant: DTRSLKX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTRSLKx` reader - Deadtime Rising Sign Lock"]
pub type DTRSLKX_R = crate::BitReader<DTRSLKX>;
impl DTRSLKX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTRSLKX {
        match self.bits {
            false => DTRSLKX::Unlocked,
            true => DTRSLKX::Locked,
        }
    }
    #[doc = "Deadtime rising sign is writable"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTRSLKX::Unlocked
    }
    #[doc = "Deadtime rising sign is read-only"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTRSLKX::Locked
    }
}
#[doc = "Field `DTRSLKx` writer - Deadtime Rising Sign Lock"]
pub type DTRSLKX_W<'a, REG> = crate::BitWriter<'a, REG, DTRSLKX>;
impl<'a, REG> DTRSLKX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deadtime rising sign is writable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DTRSLKX::Unlocked)
    }
    #[doc = "Deadtime rising sign is read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DTRSLKX::Locked)
    }
}
#[doc = "Deadtime Rising Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRLKX {
    #[doc = "0: Deadtime rising value and sign is writable"]
    Unlocked = 0,
    #[doc = "1: Deadtime rising value and sign is read-only"]
    Locked = 1,
}
impl From<DTRLKX> for bool {
    #[inline(always)]
    fn from(variant: DTRLKX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTRLKx` reader - Deadtime Rising Lock"]
pub type DTRLKX_R = crate::BitReader<DTRLKX>;
impl DTRLKX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTRLKX {
        match self.bits {
            false => DTRLKX::Unlocked,
            true => DTRLKX::Locked,
        }
    }
    #[doc = "Deadtime rising value and sign is writable"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTRLKX::Unlocked
    }
    #[doc = "Deadtime rising value and sign is read-only"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTRLKX::Locked
    }
}
#[doc = "Field `DTRLKx` writer - Deadtime Rising Lock"]
pub type DTRLKX_W<'a, REG> = crate::BitWriter<'a, REG, DTRLKX>;
impl<'a, REG> DTRLKX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deadtime rising value and sign is writable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DTRLKX::Unlocked)
    }
    #[doc = "Deadtime rising value and sign is read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DTRLKX::Locked)
    }
}
#[doc = "Field `DTFx` reader - Deadtime Falling value"]
pub type DTFX_R = crate::FieldReader<u16>;
#[doc = "Field `DTFx` writer - Deadtime Falling value"]
pub type DTFX_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 9, u16>;
#[doc = "Sign Deadtime Falling value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTFX {
    #[doc = "0: Positive deadtime on falling edge"]
    Positive = 0,
    #[doc = "1: Negative deadtime on falling edge"]
    Negative = 1,
}
impl From<SDTFX> for bool {
    #[inline(always)]
    fn from(variant: SDTFX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDTFx` reader - Sign Deadtime Falling value"]
pub type SDTFX_R = crate::BitReader<SDTFX>;
impl SDTFX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SDTFX {
        match self.bits {
            false => SDTFX::Positive,
            true => SDTFX::Negative,
        }
    }
    #[doc = "Positive deadtime on falling edge"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == SDTFX::Positive
    }
    #[doc = "Negative deadtime on falling edge"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SDTFX::Negative
    }
}
#[doc = "Field `SDTFx` writer - Sign Deadtime Falling value"]
pub type SDTFX_W<'a, REG> = crate::BitWriter<'a, REG, SDTFX>;
impl<'a, REG> SDTFX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Positive deadtime on falling edge"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(SDTFX::Positive)
    }
    #[doc = "Negative deadtime on falling edge"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(SDTFX::Negative)
    }
}
#[doc = "Deadtime Falling Sign Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTFSLKX {
    #[doc = "0: Deadtime falling sign is writable"]
    Unlocked = 0,
    #[doc = "1: Deadtime falling sign is read-only"]
    Locked = 1,
}
impl From<DTFSLKX> for bool {
    #[inline(always)]
    fn from(variant: DTFSLKX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTFSLKx` reader - Deadtime Falling Sign Lock"]
pub type DTFSLKX_R = crate::BitReader<DTFSLKX>;
impl DTFSLKX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTFSLKX {
        match self.bits {
            false => DTFSLKX::Unlocked,
            true => DTFSLKX::Locked,
        }
    }
    #[doc = "Deadtime falling sign is writable"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTFSLKX::Unlocked
    }
    #[doc = "Deadtime falling sign is read-only"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTFSLKX::Locked
    }
}
#[doc = "Field `DTFSLKx` writer - Deadtime Falling Sign Lock"]
pub type DTFSLKX_W<'a, REG> = crate::BitWriter<'a, REG, DTFSLKX>;
impl<'a, REG> DTFSLKX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deadtime falling sign is writable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DTFSLKX::Unlocked)
    }
    #[doc = "Deadtime falling sign is read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DTFSLKX::Locked)
    }
}
#[doc = "Deadtime Falling Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTFLKX {
    #[doc = "0: Deadtime falling value and sign is writable"]
    Unlocked = 0,
    #[doc = "1: Deadtime falling value and sign is read-only"]
    Locked = 1,
}
impl From<DTFLKX> for bool {
    #[inline(always)]
    fn from(variant: DTFLKX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTFLKx` reader - Deadtime Falling Lock"]
pub type DTFLKX_R = crate::BitReader<DTFLKX>;
impl DTFLKX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTFLKX {
        match self.bits {
            false => DTFLKX::Unlocked,
            true => DTFLKX::Locked,
        }
    }
    #[doc = "Deadtime falling value and sign is writable"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTFLKX::Unlocked
    }
    #[doc = "Deadtime falling value and sign is read-only"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTFLKX::Locked
    }
}
#[doc = "Field `DTFLKx` writer - Deadtime Falling Lock"]
pub type DTFLKX_W<'a, REG> = crate::BitWriter<'a, REG, DTFLKX>;
impl<'a, REG> DTFLKX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Deadtime falling value and sign is writable"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DTFLKX::Unlocked)
    }
    #[doc = "Deadtime falling value and sign is read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DTFLKX::Locked)
    }
}
impl R {
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    pub fn dtrx(&self) -> DTRX_R {
        DTRX_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    pub fn sdtrx(&self) -> SDTRX_R {
        SDTRX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    pub fn dtprsc(&self) -> DTPRSC_R {
        DTPRSC_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    pub fn dtrslkx(&self) -> DTRSLKX_R {
        DTRSLKX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    pub fn dtrlkx(&self) -> DTRLKX_R {
        DTRLKX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    pub fn dtfx(&self) -> DTFX_R {
        DTFX_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    pub fn sdtfx(&self) -> SDTFX_R {
        SDTFX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    pub fn dtfslkx(&self) -> DTFSLKX_R {
        DTFSLKX_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    pub fn dtflkx(&self) -> DTFLKX_R {
        DTFLKX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    #[must_use]
    pub fn dtrx(&mut self) -> DTRX_W<DTARrs> {
        DTRX_W::new(self, 0)
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    #[must_use]
    pub fn sdtrx(&mut self) -> SDTRX_W<DTARrs> {
        SDTRX_W::new(self, 9)
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn dtprsc(&mut self) -> DTPRSC_W<DTARrs> {
        DTPRSC_W::new(self, 10)
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtrslkx(&mut self) -> DTRSLKX_W<DTARrs> {
        DTRSLKX_W::new(self, 14)
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtrlkx(&mut self) -> DTRLKX_W<DTARrs> {
        DTRLKX_W::new(self, 15)
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    #[must_use]
    pub fn dtfx(&mut self) -> DTFX_W<DTARrs> {
        DTFX_W::new(self, 16)
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    #[must_use]
    pub fn sdtfx(&mut self) -> SDTFX_W<DTARrs> {
        SDTFX_W::new(self, 25)
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtfslkx(&mut self) -> DTFSLKX_W<DTARrs> {
        DTFSLKX_W::new(self, 30)
    }
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    #[must_use]
    pub fn dtflkx(&mut self) -> DTFLKX_W<DTARrs> {
        DTFLKX_W::new(self, 31)
    }
}
#[doc = "Timerx Deadtime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTARrs;
impl crate::RegisterSpec for DTARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtar::R`](R) reader structure"]
impl crate::Readable for DTARrs {}
#[doc = "`write(|w| ..)` method takes [`dtar::W`](W) writer structure"]
impl crate::Writable for DTARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTAR to value 0"]
impl crate::Resettable for DTARrs {
    const RESET_VALUE: u32 = 0;
}
