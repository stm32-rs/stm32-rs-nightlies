///Register `DTR` reader
pub type R = crate::R<DTRrs>;
///Register `DTR` writer
pub type W = crate::W<DTRrs>;
///Field `DTR` reader - Deadtime Rising value
pub type DTR_R = crate::FieldReader<u16>;
///Field `DTR` writer - Deadtime Rising value
pub type DTR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16, crate::Safe>;
/**Sign Deadtime Rising value

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTR {
    ///0: Positive deadtime on rising edge
    Positive = 0,
    ///1: Negative deadtime on rising edge
    Negative = 1,
}
impl From<SDTR> for bool {
    #[inline(always)]
    fn from(variant: SDTR) -> Self {
        variant as u8 != 0
    }
}
///Field `SDTR` reader - Sign Deadtime Rising value
pub type SDTR_R = crate::BitReader<SDTR>;
impl SDTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDTR {
        match self.bits {
            false => SDTR::Positive,
            true => SDTR::Negative,
        }
    }
    ///Positive deadtime on rising edge
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == SDTR::Positive
    }
    ///Negative deadtime on rising edge
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SDTR::Negative
    }
}
///Field `SDTR` writer - Sign Deadtime Rising value
pub type SDTR_W<'a, REG> = crate::BitWriter<'a, REG, SDTR>;
impl<'a, REG> SDTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Positive deadtime on rising edge
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(SDTR::Positive)
    }
    ///Negative deadtime on rising edge
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(SDTR::Negative)
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
pub enum DTRSLK {
    ///0: Deadtime rising sign is writable
    Unlocked = 0,
    ///1: Deadtime rising sign is read-only
    Locked = 1,
}
impl From<DTRSLK> for bool {
    #[inline(always)]
    fn from(variant: DTRSLK) -> Self {
        variant as u8 != 0
    }
}
///Field `DTRSLK` reader - Deadtime Rising Sign Lock
pub type DTRSLK_R = crate::BitReader<DTRSLK>;
impl DTRSLK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTRSLK {
        match self.bits {
            false => DTRSLK::Unlocked,
            true => DTRSLK::Locked,
        }
    }
    ///Deadtime rising sign is writable
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTRSLK::Unlocked
    }
    ///Deadtime rising sign is read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTRSLK::Locked
    }
}
///Field `DTRSLK` writer - Deadtime Rising Sign Lock
pub type DTRSLK_W<'a, REG> = crate::BitWriter<'a, REG, DTRSLK>;
impl<'a, REG> DTRSLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Deadtime rising sign is writable
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DTRSLK::Unlocked)
    }
    ///Deadtime rising sign is read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DTRSLK::Locked)
    }
}
/**Deadtime Rising Lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTRLK {
    ///0: Deadtime rising value and sign is writable
    Unlocked = 0,
    ///1: Deadtime rising value and sign is read-only
    Locked = 1,
}
impl From<DTRLK> for bool {
    #[inline(always)]
    fn from(variant: DTRLK) -> Self {
        variant as u8 != 0
    }
}
///Field `DTRLK` reader - Deadtime Rising Lock
pub type DTRLK_R = crate::BitReader<DTRLK>;
impl DTRLK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTRLK {
        match self.bits {
            false => DTRLK::Unlocked,
            true => DTRLK::Locked,
        }
    }
    ///Deadtime rising value and sign is writable
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTRLK::Unlocked
    }
    ///Deadtime rising value and sign is read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTRLK::Locked
    }
}
///Field `DTRLK` writer - Deadtime Rising Lock
pub type DTRLK_W<'a, REG> = crate::BitWriter<'a, REG, DTRLK>;
impl<'a, REG> DTRLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Deadtime rising value and sign is writable
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DTRLK::Unlocked)
    }
    ///Deadtime rising value and sign is read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DTRLK::Locked)
    }
}
///Field `DTF` reader - Deadtime Falling value
pub type DTF_R = crate::FieldReader<u16>;
///Field `DTF` writer - Deadtime Falling value
pub type DTF_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16, crate::Safe>;
/**Sign Deadtime Falling value

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDTF {
    ///0: Positive deadtime on falling edge
    Positive = 0,
    ///1: Negative deadtime on falling edge
    Negative = 1,
}
impl From<SDTF> for bool {
    #[inline(always)]
    fn from(variant: SDTF) -> Self {
        variant as u8 != 0
    }
}
///Field `SDTF` reader - Sign Deadtime Falling value
pub type SDTF_R = crate::BitReader<SDTF>;
impl SDTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SDTF {
        match self.bits {
            false => SDTF::Positive,
            true => SDTF::Negative,
        }
    }
    ///Positive deadtime on falling edge
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == SDTF::Positive
    }
    ///Negative deadtime on falling edge
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == SDTF::Negative
    }
}
///Field `SDTF` writer - Sign Deadtime Falling value
pub type SDTF_W<'a, REG> = crate::BitWriter<'a, REG, SDTF>;
impl<'a, REG> SDTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Positive deadtime on falling edge
    #[inline(always)]
    pub fn positive(self) -> &'a mut crate::W<REG> {
        self.variant(SDTF::Positive)
    }
    ///Negative deadtime on falling edge
    #[inline(always)]
    pub fn negative(self) -> &'a mut crate::W<REG> {
        self.variant(SDTF::Negative)
    }
}
/**Deadtime Falling Sign Lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTFSLK {
    ///0: Deadtime falling sign is writable
    Unlocked = 0,
    ///1: Deadtime falling sign is read-only
    Locked = 1,
}
impl From<DTFSLK> for bool {
    #[inline(always)]
    fn from(variant: DTFSLK) -> Self {
        variant as u8 != 0
    }
}
///Field `DTFSLK` reader - Deadtime Falling Sign Lock
pub type DTFSLK_R = crate::BitReader<DTFSLK>;
impl DTFSLK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTFSLK {
        match self.bits {
            false => DTFSLK::Unlocked,
            true => DTFSLK::Locked,
        }
    }
    ///Deadtime falling sign is writable
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTFSLK::Unlocked
    }
    ///Deadtime falling sign is read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTFSLK::Locked
    }
}
///Field `DTFSLK` writer - Deadtime Falling Sign Lock
pub type DTFSLK_W<'a, REG> = crate::BitWriter<'a, REG, DTFSLK>;
impl<'a, REG> DTFSLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Deadtime falling sign is writable
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DTFSLK::Unlocked)
    }
    ///Deadtime falling sign is read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DTFSLK::Locked)
    }
}
/**Deadtime Falling Lock

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTFLK {
    ///0: Deadtime falling value and sign is writable
    Unlocked = 0,
    ///1: Deadtime falling value and sign is read-only
    Locked = 1,
}
impl From<DTFLK> for bool {
    #[inline(always)]
    fn from(variant: DTFLK) -> Self {
        variant as u8 != 0
    }
}
///Field `DTFLK` reader - Deadtime Falling Lock
pub type DTFLK_R = crate::BitReader<DTFLK>;
impl DTFLK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTFLK {
        match self.bits {
            false => DTFLK::Unlocked,
            true => DTFLK::Locked,
        }
    }
    ///Deadtime falling value and sign is writable
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DTFLK::Unlocked
    }
    ///Deadtime falling value and sign is read-only
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DTFLK::Locked
    }
}
///Field `DTFLK` writer - Deadtime Falling Lock
pub type DTFLK_W<'a, REG> = crate::BitWriter<'a, REG, DTFLK>;
impl<'a, REG> DTFLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Deadtime falling value and sign is writable
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(DTFLK::Unlocked)
    }
    ///Deadtime falling value and sign is read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(DTFLK::Locked)
    }
}
impl R {
    ///Bits 0:8 - Deadtime Rising value
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 9 - Sign Deadtime Rising value
    #[inline(always)]
    pub fn sdtr(&self) -> SDTR_R {
        SDTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:12 - Deadtime Prescaler
    #[inline(always)]
    pub fn dtprsc(&self) -> DTPRSC_R {
        DTPRSC_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 14 - Deadtime Rising Sign Lock
    #[inline(always)]
    pub fn dtrslk(&self) -> DTRSLK_R {
        DTRSLK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Deadtime Rising Lock
    #[inline(always)]
    pub fn dtrlk(&self) -> DTRLK_R {
        DTRLK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:24 - Deadtime Falling value
    #[inline(always)]
    pub fn dtf(&self) -> DTF_R {
        DTF_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bit 25 - Sign Deadtime Falling value
    #[inline(always)]
    pub fn sdtf(&self) -> SDTF_R {
        SDTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - Deadtime Falling Sign Lock
    #[inline(always)]
    pub fn dtfslk(&self) -> DTFSLK_R {
        DTFSLK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Deadtime Falling Lock
    #[inline(always)]
    pub fn dtflk(&self) -> DTFLK_R {
        DTFLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTR")
            .field("dtflk", &self.dtflk())
            .field("dtfslk", &self.dtfslk())
            .field("sdtf", &self.sdtf())
            .field("dtf", &self.dtf())
            .field("dtrlk", &self.dtrlk())
            .field("dtrslk", &self.dtrslk())
            .field("dtprsc", &self.dtprsc())
            .field("sdtr", &self.sdtr())
            .field("dtr", &self.dtr())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Deadtime Rising value
    #[inline(always)]
    pub fn dtr(&mut self) -> DTR_W<'_, DTRrs> {
        DTR_W::new(self, 0)
    }
    ///Bit 9 - Sign Deadtime Rising value
    #[inline(always)]
    pub fn sdtr(&mut self) -> SDTR_W<'_, DTRrs> {
        SDTR_W::new(self, 9)
    }
    ///Bits 10:12 - Deadtime Prescaler
    #[inline(always)]
    pub fn dtprsc(&mut self) -> DTPRSC_W<'_, DTRrs> {
        DTPRSC_W::new(self, 10)
    }
    ///Bit 14 - Deadtime Rising Sign Lock
    #[inline(always)]
    pub fn dtrslk(&mut self) -> DTRSLK_W<'_, DTRrs> {
        DTRSLK_W::new(self, 14)
    }
    ///Bit 15 - Deadtime Rising Lock
    #[inline(always)]
    pub fn dtrlk(&mut self) -> DTRLK_W<'_, DTRrs> {
        DTRLK_W::new(self, 15)
    }
    ///Bits 16:24 - Deadtime Falling value
    #[inline(always)]
    pub fn dtf(&mut self) -> DTF_W<'_, DTRrs> {
        DTF_W::new(self, 16)
    }
    ///Bit 25 - Sign Deadtime Falling value
    #[inline(always)]
    pub fn sdtf(&mut self) -> SDTF_W<'_, DTRrs> {
        SDTF_W::new(self, 25)
    }
    ///Bit 30 - Deadtime Falling Sign Lock
    #[inline(always)]
    pub fn dtfslk(&mut self) -> DTFSLK_W<'_, DTRrs> {
        DTFSLK_W::new(self, 30)
    }
    ///Bit 31 - Deadtime Falling Lock
    #[inline(always)]
    pub fn dtflk(&mut self) -> DTFLK_W<'_, DTRrs> {
        DTFLK_W::new(self, 31)
    }
}
/**Timerx Deadtime Register

You can [`read`](crate::Reg::read) this register and get [`dtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#HRTIM_TIMA:DTR)*/
pub struct DTRrs;
impl crate::RegisterSpec for DTRrs {
    type Ux = u32;
}
///`read()` method returns [`dtr::R`](R) reader structure
impl crate::Readable for DTRrs {}
///`write(|w| ..)` method takes [`dtr::W`](W) writer structure
impl crate::Writable for DTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DTR to value 0
impl crate::Resettable for DTRrs {}
