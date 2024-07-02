///Register `DTCR` reader
pub type R = crate::R<DTCRrs>;
///Register `DTCR` writer
pub type W = crate::W<DTCRrs>;
///Field `DTRx` reader - Deadtime Rising value
pub type DTRX_R = crate::FieldReader<u16>;
///Field `DTRx` writer - Deadtime Rising value
pub type DTRX_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16, crate::Safe>;
/**Sign Deadtime Rising value

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTRX {
    ///0: Positive deadtime on rising edge
    Positive = 0,
    ///1: Negative deadtime on rising edge
    Negative = 1,
}
impl From<SDTRX> for bool {
    #[inline(always)]
    fn from(variant: SDTRX) -> Self {
        variant as u8 != 0
    }
}
///Field `SDTRx` reader - Sign Deadtime Rising value
pub type SDTRX_R = crate::BitReader<SDTRX>;
impl SDTRX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDTRX {
        match self.bits {
            false => SDTRX::Positive,
            true => SDTRX::Negative,
        }
    }
    ///Positive deadtime on rising edge
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == SDTRX::Positive
    }
    ///Negative deadtime on rising edge
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SDTRX::Negative
    }
}
///Field `SDTRx` writer - Sign Deadtime Rising value
pub type SDTRX_W<'a, REG> = crate::BitWriter<'a, REG, SDTRX>;
impl<'a, REG> SDTRX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Positive deadtime on rising edge
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(SDTRX::Positive)
    }
    ///Negative deadtime on rising edge
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(SDTRX::Negative)
    }
}
///Field `DTPRSC` reader - Deadtime Prescaler
pub type DTPRSC_R = crate::FieldReader;
///Field `DTPRSC` writer - Deadtime Prescaler
pub type DTPRSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
/**Deadtime Rising Sign Lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRSLKX {
    ///0: Deadtime rising sign is writable
    Unlocked = 0,
    ///1: Deadtime rising sign is read-only
    Locked = 1,
}
impl From<DTRSLKX> for bool {
    #[inline(always)]
    fn from(variant: DTRSLKX) -> Self {
        variant as u8 != 0
    }
}
///Field `DTRSLKx` reader - Deadtime Rising Sign Lock
pub type DTRSLKX_R = crate::BitReader<DTRSLKX>;
impl DTRSLKX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTRSLKX {
        match self.bits {
            false => DTRSLKX::Unlocked,
            true => DTRSLKX::Locked,
        }
    }
    ///Deadtime rising sign is writable
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTRSLKX::Unlocked
    }
    ///Deadtime rising sign is read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTRSLKX::Locked
    }
}
///Field `DTRSLKx` writer - Deadtime Rising Sign Lock
pub type DTRSLKX_W<'a, REG> = crate::BitWriter<'a, REG, DTRSLKX>;
impl<'a, REG> DTRSLKX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Deadtime rising sign is writable
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DTRSLKX::Unlocked)
    }
    ///Deadtime rising sign is read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DTRSLKX::Locked)
    }
}
/**Deadtime Rising Lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRLKX {
    ///0: Deadtime rising value and sign is writable
    Unlocked = 0,
    ///1: Deadtime rising value and sign is read-only
    Locked = 1,
}
impl From<DTRLKX> for bool {
    #[inline(always)]
    fn from(variant: DTRLKX) -> Self {
        variant as u8 != 0
    }
}
///Field `DTRLKx` reader - Deadtime Rising Lock
pub type DTRLKX_R = crate::BitReader<DTRLKX>;
impl DTRLKX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTRLKX {
        match self.bits {
            false => DTRLKX::Unlocked,
            true => DTRLKX::Locked,
        }
    }
    ///Deadtime rising value and sign is writable
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTRLKX::Unlocked
    }
    ///Deadtime rising value and sign is read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTRLKX::Locked
    }
}
///Field `DTRLKx` writer - Deadtime Rising Lock
pub type DTRLKX_W<'a, REG> = crate::BitWriter<'a, REG, DTRLKX>;
impl<'a, REG> DTRLKX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Deadtime rising value and sign is writable
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DTRLKX::Unlocked)
    }
    ///Deadtime rising value and sign is read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DTRLKX::Locked)
    }
}
///Field `DTFx` reader - Deadtime Falling value
pub type DTFX_R = crate::FieldReader<u16>;
///Field `DTFx` writer - Deadtime Falling value
pub type DTFX_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16, crate::Safe>;
/**Sign Deadtime Falling value

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTFX {
    ///0: Positive deadtime on falling edge
    Positive = 0,
    ///1: Negative deadtime on falling edge
    Negative = 1,
}
impl From<SDTFX> for bool {
    #[inline(always)]
    fn from(variant: SDTFX) -> Self {
        variant as u8 != 0
    }
}
///Field `SDTFx` reader - Sign Deadtime Falling value
pub type SDTFX_R = crate::BitReader<SDTFX>;
impl SDTFX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDTFX {
        match self.bits {
            false => SDTFX::Positive,
            true => SDTFX::Negative,
        }
    }
    ///Positive deadtime on falling edge
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == SDTFX::Positive
    }
    ///Negative deadtime on falling edge
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SDTFX::Negative
    }
}
///Field `SDTFx` writer - Sign Deadtime Falling value
pub type SDTFX_W<'a, REG> = crate::BitWriter<'a, REG, SDTFX>;
impl<'a, REG> SDTFX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Positive deadtime on falling edge
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(SDTFX::Positive)
    }
    ///Negative deadtime on falling edge
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(SDTFX::Negative)
    }
}
/**Deadtime Falling Sign Lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTFSLKX {
    ///0: Deadtime falling sign is writable
    Unlocked = 0,
    ///1: Deadtime falling sign is read-only
    Locked = 1,
}
impl From<DTFSLKX> for bool {
    #[inline(always)]
    fn from(variant: DTFSLKX) -> Self {
        variant as u8 != 0
    }
}
///Field `DTFSLKx` reader - Deadtime Falling Sign Lock
pub type DTFSLKX_R = crate::BitReader<DTFSLKX>;
impl DTFSLKX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTFSLKX {
        match self.bits {
            false => DTFSLKX::Unlocked,
            true => DTFSLKX::Locked,
        }
    }
    ///Deadtime falling sign is writable
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTFSLKX::Unlocked
    }
    ///Deadtime falling sign is read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTFSLKX::Locked
    }
}
///Field `DTFSLKx` writer - Deadtime Falling Sign Lock
pub type DTFSLKX_W<'a, REG> = crate::BitWriter<'a, REG, DTFSLKX>;
impl<'a, REG> DTFSLKX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Deadtime falling sign is writable
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DTFSLKX::Unlocked)
    }
    ///Deadtime falling sign is read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DTFSLKX::Locked)
    }
}
/**Deadtime Falling Lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTFLKX {
    ///0: Deadtime falling value and sign is writable
    Unlocked = 0,
    ///1: Deadtime falling value and sign is read-only
    Locked = 1,
}
impl From<DTFLKX> for bool {
    #[inline(always)]
    fn from(variant: DTFLKX) -> Self {
        variant as u8 != 0
    }
}
///Field `DTFLKx` reader - Deadtime Falling Lock
pub type DTFLKX_R = crate::BitReader<DTFLKX>;
impl DTFLKX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTFLKX {
        match self.bits {
            false => DTFLKX::Unlocked,
            true => DTFLKX::Locked,
        }
    }
    ///Deadtime falling value and sign is writable
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTFLKX::Unlocked
    }
    ///Deadtime falling value and sign is read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTFLKX::Locked
    }
}
///Field `DTFLKx` writer - Deadtime Falling Lock
pub type DTFLKX_W<'a, REG> = crate::BitWriter<'a, REG, DTFLKX>;
impl<'a, REG> DTFLKX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Deadtime falling value and sign is writable
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DTFLKX::Unlocked)
    }
    ///Deadtime falling value and sign is read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DTFLKX::Locked)
    }
}
impl R {
    ///Bits 0:8 - Deadtime Rising value
    #[inline(always)]
    pub fn dtrx(&self) -> DTRX_R {
        DTRX_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 9 - Sign Deadtime Rising value
    #[inline(always)]
    pub fn sdtrx(&self) -> SDTRX_R {
        SDTRX_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:12 - Deadtime Prescaler
    #[inline(always)]
    pub fn dtprsc(&self) -> DTPRSC_R {
        DTPRSC_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 14 - Deadtime Rising Sign Lock
    #[inline(always)]
    pub fn dtrslkx(&self) -> DTRSLKX_R {
        DTRSLKX_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Deadtime Rising Lock
    #[inline(always)]
    pub fn dtrlkx(&self) -> DTRLKX_R {
        DTRLKX_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:24 - Deadtime Falling value
    #[inline(always)]
    pub fn dtfx(&self) -> DTFX_R {
        DTFX_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bit 25 - Sign Deadtime Falling value
    #[inline(always)]
    pub fn sdtfx(&self) -> SDTFX_R {
        SDTFX_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - Deadtime Falling Sign Lock
    #[inline(always)]
    pub fn dtfslkx(&self) -> DTFSLKX_R {
        DTFSLKX_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Deadtime Falling Lock
    #[inline(always)]
    pub fn dtflkx(&self) -> DTFLKX_R {
        DTFLKX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCR")
            .field("dtflkx", &self.dtflkx())
            .field("dtfslkx", &self.dtfslkx())
            .field("sdtfx", &self.sdtfx())
            .field("dtfx", &self.dtfx())
            .field("dtrlkx", &self.dtrlkx())
            .field("dtrslkx", &self.dtrslkx())
            .field("dtprsc", &self.dtprsc())
            .field("sdtrx", &self.sdtrx())
            .field("dtrx", &self.dtrx())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Deadtime Rising value
    #[inline(always)]
    #[must_use]
    pub fn dtrx(&mut self) -> DTRX_W<DTCRrs> {
        DTRX_W::new(self, 0)
    }
    ///Bit 9 - Sign Deadtime Rising value
    #[inline(always)]
    #[must_use]
    pub fn sdtrx(&mut self) -> SDTRX_W<DTCRrs> {
        SDTRX_W::new(self, 9)
    }
    ///Bits 10:12 - Deadtime Prescaler
    #[inline(always)]
    #[must_use]
    pub fn dtprsc(&mut self) -> DTPRSC_W<DTCRrs> {
        DTPRSC_W::new(self, 10)
    }
    ///Bit 14 - Deadtime Rising Sign Lock
    #[inline(always)]
    #[must_use]
    pub fn dtrslkx(&mut self) -> DTRSLKX_W<DTCRrs> {
        DTRSLKX_W::new(self, 14)
    }
    ///Bit 15 - Deadtime Rising Lock
    #[inline(always)]
    #[must_use]
    pub fn dtrlkx(&mut self) -> DTRLKX_W<DTCRrs> {
        DTRLKX_W::new(self, 15)
    }
    ///Bits 16:24 - Deadtime Falling value
    #[inline(always)]
    #[must_use]
    pub fn dtfx(&mut self) -> DTFX_W<DTCRrs> {
        DTFX_W::new(self, 16)
    }
    ///Bit 25 - Sign Deadtime Falling value
    #[inline(always)]
    #[must_use]
    pub fn sdtfx(&mut self) -> SDTFX_W<DTCRrs> {
        SDTFX_W::new(self, 25)
    }
    ///Bit 30 - Deadtime Falling Sign Lock
    #[inline(always)]
    #[must_use]
    pub fn dtfslkx(&mut self) -> DTFSLKX_W<DTCRrs> {
        DTFSLKX_W::new(self, 30)
    }
    ///Bit 31 - Deadtime Falling Lock
    #[inline(always)]
    #[must_use]
    pub fn dtflkx(&mut self) -> DTFLKX_W<DTCRrs> {
        DTFLKX_W::new(self, 31)
    }
}
/**Timerx Deadtime Register

You can [`read`](crate::Reg::read) this register and get [`dtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMC:DTCR)*/
pub struct DTCRrs;
impl crate::RegisterSpec for DTCRrs {
    type Ux = u32;
}
///`read()` method returns [`dtcr::R`](R) reader structure
impl crate::Readable for DTCRrs {}
///`write(|w| ..)` method takes [`dtcr::W`](W) writer structure
impl crate::Writable for DTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DTCR to value 0
impl crate::Resettable for DTCRrs {
    const RESET_VALUE: u32 = 0;
}
