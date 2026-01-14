///Register `RTC_ALRMAR` reader
pub type R = crate::R<RTC_ALRMARrs>;
///Register `RTC_ALRMAR` writer
pub type W = crate::W<RTC_ALRMARrs>;
///Field `SU` reader - Second units in BCD format.
pub type SU_R = crate::FieldReader;
///Field `SU` writer - Second units in BCD format.
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ST` reader - Second tens in BCD format.
pub type ST_R = crate::FieldReader;
///Field `ST` writer - Second tens in BCD format.
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Alarm A seconds mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK1 {
    ///0: Alarm A set if the seconds match
    B0x0 = 0,
    ///1: Seconds don t care in alarm A comparison
    B0x1 = 1,
}
impl From<MSK1> for bool {
    #[inline(always)]
    fn from(variant: MSK1) -> Self {
        variant as u8 != 0
    }
}
///Field `MSK1` reader - Alarm A seconds mask
pub type MSK1_R = crate::BitReader<MSK1>;
impl MSK1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSK1 {
        match self.bits {
            false => MSK1::B0x0,
            true => MSK1::B0x1,
        }
    }
    ///Alarm A set if the seconds match
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSK1::B0x0
    }
    ///Seconds don t care in alarm A comparison
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSK1::B0x1
    }
}
///Field `MSK1` writer - Alarm A seconds mask
pub type MSK1_W<'a, REG> = crate::BitWriter<'a, REG, MSK1>;
impl<'a, REG> MSK1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alarm A set if the seconds match
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSK1::B0x0)
    }
    ///Seconds don t care in alarm A comparison
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSK1::B0x1)
    }
}
///Field `MNU` reader - Minute units in BCD format
pub type MNU_R = crate::FieldReader;
///Field `MNU` writer - Minute units in BCD format
pub type MNU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MNT` reader - Minute tens in BCD format
pub type MNT_R = crate::FieldReader;
///Field `MNT` writer - Minute tens in BCD format
pub type MNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/**Alarm A minutes mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK2 {
    ///0: Alarm A set if the minutes match
    B0x0 = 0,
    ///1: Minutes don t care in alarm A comparison
    B0x1 = 1,
}
impl From<MSK2> for bool {
    #[inline(always)]
    fn from(variant: MSK2) -> Self {
        variant as u8 != 0
    }
}
///Field `MSK2` reader - Alarm A minutes mask
pub type MSK2_R = crate::BitReader<MSK2>;
impl MSK2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSK2 {
        match self.bits {
            false => MSK2::B0x0,
            true => MSK2::B0x1,
        }
    }
    ///Alarm A set if the minutes match
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSK2::B0x0
    }
    ///Minutes don t care in alarm A comparison
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSK2::B0x1
    }
}
///Field `MSK2` writer - Alarm A minutes mask
pub type MSK2_W<'a, REG> = crate::BitWriter<'a, REG, MSK2>;
impl<'a, REG> MSK2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alarm A set if the minutes match
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSK2::B0x0)
    }
    ///Minutes don t care in alarm A comparison
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSK2::B0x1)
    }
}
///Field `HU` reader - Hour units in BCD format
pub type HU_R = crate::FieldReader;
///Field `HU` writer - Hour units in BCD format
pub type HU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HT` reader - Hour tens in BCD format
pub type HT_R = crate::FieldReader;
///Field `HT` writer - Hour tens in BCD format
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**AM/PM notation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM {
    ///0: AM or 24-hour format
    B0x0 = 0,
    ///1: PM
    B0x1 = 1,
}
impl From<PM> for bool {
    #[inline(always)]
    fn from(variant: PM) -> Self {
        variant as u8 != 0
    }
}
///Field `PM` reader - AM/PM notation
pub type PM_R = crate::BitReader<PM>;
impl PM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PM {
        match self.bits {
            false => PM::B0x0,
            true => PM::B0x1,
        }
    }
    ///AM or 24-hour format
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PM::B0x0
    }
    ///PM
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PM::B0x1
    }
}
///Field `PM` writer - AM/PM notation
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG, PM>;
impl<'a, REG> PM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AM or 24-hour format
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PM::B0x0)
    }
    ///PM
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PM::B0x1)
    }
}
/**Alarm A hours mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK3 {
    ///0: Alarm A set if the hours match
    B0x0 = 0,
    ///1: Hours don t care in alarm A comparison
    B0x1 = 1,
}
impl From<MSK3> for bool {
    #[inline(always)]
    fn from(variant: MSK3) -> Self {
        variant as u8 != 0
    }
}
///Field `MSK3` reader - Alarm A hours mask
pub type MSK3_R = crate::BitReader<MSK3>;
impl MSK3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSK3 {
        match self.bits {
            false => MSK3::B0x0,
            true => MSK3::B0x1,
        }
    }
    ///Alarm A set if the hours match
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSK3::B0x0
    }
    ///Hours don t care in alarm A comparison
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSK3::B0x1
    }
}
///Field `MSK3` writer - Alarm A hours mask
pub type MSK3_W<'a, REG> = crate::BitWriter<'a, REG, MSK3>;
impl<'a, REG> MSK3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alarm A set if the hours match
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSK3::B0x0)
    }
    ///Hours don t care in alarm A comparison
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSK3::B0x1)
    }
}
///Field `DU` reader - Date units or day in BCD format
pub type DU_R = crate::FieldReader;
///Field `DU` writer - Date units or day in BCD format
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DT` reader - Date tens in BCD format
pub type DT_R = crate::FieldReader;
///Field `DT` writer - Date tens in BCD format
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
/**Week day selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDSEL {
    ///0: DU\[3:0\] represents the date units
    B0x0 = 0,
    ///1: DU\[3:0\] represents the week day. DT\[1:0\] is don t care.
    B0x1 = 1,
}
impl From<WDSEL> for bool {
    #[inline(always)]
    fn from(variant: WDSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `WDSEL` reader - Week day selection
pub type WDSEL_R = crate::BitReader<WDSEL>;
impl WDSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDSEL {
        match self.bits {
            false => WDSEL::B0x0,
            true => WDSEL::B0x1,
        }
    }
    ///DU\[3:0\] represents the date units
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDSEL::B0x0
    }
    ///DU\[3:0\] represents the week day. DT\[1:0\] is don t care.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDSEL::B0x1
    }
}
///Field `WDSEL` writer - Week day selection
pub type WDSEL_W<'a, REG> = crate::BitWriter<'a, REG, WDSEL>;
impl<'a, REG> WDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DU\[3:0\] represents the date units
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDSEL::B0x0)
    }
    ///DU\[3:0\] represents the week day. DT\[1:0\] is don t care.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDSEL::B0x1)
    }
}
/**Alarm A date mask

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSK4 {
    ///0: Alarm A set if the date/day match
    B0x0 = 0,
    ///1: Date/day don t care in alarm A comparison
    B0x1 = 1,
}
impl From<MSK4> for bool {
    #[inline(always)]
    fn from(variant: MSK4) -> Self {
        variant as u8 != 0
    }
}
///Field `MSK4` reader - Alarm A date mask
pub type MSK4_R = crate::BitReader<MSK4>;
impl MSK4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSK4 {
        match self.bits {
            false => MSK4::B0x0,
            true => MSK4::B0x1,
        }
    }
    ///Alarm A set if the date/day match
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSK4::B0x0
    }
    ///Date/day don t care in alarm A comparison
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSK4::B0x1
    }
}
///Field `MSK4` writer - Alarm A date mask
pub type MSK4_W<'a, REG> = crate::BitWriter<'a, REG, MSK4>;
impl<'a, REG> MSK4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alarm A set if the date/day match
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSK4::B0x0)
    }
    ///Date/day don t care in alarm A comparison
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSK4::B0x1)
    }
}
impl R {
    ///Bits 0:3 - Second units in BCD format.
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - Second tens in BCD format.
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Alarm A seconds mask
    #[inline(always)]
    pub fn msk1(&self) -> MSK1_R {
        MSK1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - Minute units in BCD format
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - Minute tens in BCD format
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Alarm A minutes mask
    #[inline(always)]
    pub fn msk2(&self) -> MSK2_R {
        MSK2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Hour units in BCD format
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:21 - Hour tens in BCD format
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - AM/PM notation
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Alarm A hours mask
    #[inline(always)]
    pub fn msk3(&self) -> MSK3_R {
        MSK3_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:27 - Date units or day in BCD format
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:29 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - Week day selection
    #[inline(always)]
    pub fn wdsel(&self) -> WDSEL_R {
        WDSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Alarm A date mask
    #[inline(always)]
    pub fn msk4(&self) -> MSK4_R {
        MSK4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_ALRMAR")
            .field("su", &self.su())
            .field("st", &self.st())
            .field("msk1", &self.msk1())
            .field("mnu", &self.mnu())
            .field("mnt", &self.mnt())
            .field("msk2", &self.msk2())
            .field("hu", &self.hu())
            .field("ht", &self.ht())
            .field("pm", &self.pm())
            .field("msk3", &self.msk3())
            .field("du", &self.du())
            .field("dt", &self.dt())
            .field("wdsel", &self.wdsel())
            .field("msk4", &self.msk4())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Second units in BCD format.
    #[inline(always)]
    pub fn su(&mut self) -> SU_W<'_, RTC_ALRMARrs> {
        SU_W::new(self, 0)
    }
    ///Bits 4:6 - Second tens in BCD format.
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<'_, RTC_ALRMARrs> {
        ST_W::new(self, 4)
    }
    ///Bit 7 - Alarm A seconds mask
    #[inline(always)]
    pub fn msk1(&mut self) -> MSK1_W<'_, RTC_ALRMARrs> {
        MSK1_W::new(self, 7)
    }
    ///Bits 8:11 - Minute units in BCD format
    #[inline(always)]
    pub fn mnu(&mut self) -> MNU_W<'_, RTC_ALRMARrs> {
        MNU_W::new(self, 8)
    }
    ///Bits 12:14 - Minute tens in BCD format
    #[inline(always)]
    pub fn mnt(&mut self) -> MNT_W<'_, RTC_ALRMARrs> {
        MNT_W::new(self, 12)
    }
    ///Bit 15 - Alarm A minutes mask
    #[inline(always)]
    pub fn msk2(&mut self) -> MSK2_W<'_, RTC_ALRMARrs> {
        MSK2_W::new(self, 15)
    }
    ///Bits 16:19 - Hour units in BCD format
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W<'_, RTC_ALRMARrs> {
        HU_W::new(self, 16)
    }
    ///Bits 20:21 - Hour tens in BCD format
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W<'_, RTC_ALRMARrs> {
        HT_W::new(self, 20)
    }
    ///Bit 22 - AM/PM notation
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<'_, RTC_ALRMARrs> {
        PM_W::new(self, 22)
    }
    ///Bit 23 - Alarm A hours mask
    #[inline(always)]
    pub fn msk3(&mut self) -> MSK3_W<'_, RTC_ALRMARrs> {
        MSK3_W::new(self, 23)
    }
    ///Bits 24:27 - Date units or day in BCD format
    #[inline(always)]
    pub fn du(&mut self) -> DU_W<'_, RTC_ALRMARrs> {
        DU_W::new(self, 24)
    }
    ///Bits 28:29 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<'_, RTC_ALRMARrs> {
        DT_W::new(self, 28)
    }
    ///Bit 30 - Week day selection
    #[inline(always)]
    pub fn wdsel(&mut self) -> WDSEL_W<'_, RTC_ALRMARrs> {
        WDSEL_W::new(self, 30)
    }
    ///Bit 31 - Alarm A date mask
    #[inline(always)]
    pub fn msk4(&mut self) -> MSK4_W<'_, RTC_ALRMARrs> {
        MSK4_W::new(self, 31)
    }
}
/**RTC alarm A register

You can [`read`](crate::Reg::read) this register and get [`rtc_alrmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#RTC:RTC_ALRMAR)*/
pub struct RTC_ALRMARrs;
impl crate::RegisterSpec for RTC_ALRMARrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_alrmar::R`](R) reader structure
impl crate::Readable for RTC_ALRMARrs {}
///`write(|w| ..)` method takes [`rtc_alrmar::W`](W) writer structure
impl crate::Writable for RTC_ALRMARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTC_ALRMAR to value 0
impl crate::Resettable for RTC_ALRMARrs {}
