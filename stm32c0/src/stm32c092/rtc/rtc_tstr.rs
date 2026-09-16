///Register `RTC_TSTR` reader
pub type R = crate::R<RTC_TSTRrs>;
///Field `SU` reader - Second units in BCD format.
pub type SU_R = crate::FieldReader;
///Field `ST` reader - Second tens in BCD format.
pub type ST_R = crate::FieldReader;
///Field `MNU` reader - Minute units in BCD format.
pub type MNU_R = crate::FieldReader;
///Field `MNT` reader - Minute tens in BCD format.
pub type MNT_R = crate::FieldReader;
///Field `HU` reader - Hour units in BCD format.
pub type HU_R = crate::FieldReader;
///Field `HT` reader - Hour tens in BCD format.
pub type HT_R = crate::FieldReader;
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
    ///Bits 8:11 - Minute units in BCD format.
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - Minute tens in BCD format.
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:19 - Hour units in BCD format.
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:21 - Hour tens in BCD format.
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - AM/PM notation
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_TSTR")
            .field("su", &self.su())
            .field("st", &self.st())
            .field("mnu", &self.mnu())
            .field("mnt", &self.mnt())
            .field("hu", &self.hu())
            .field("ht", &self.ht())
            .field("pm", &self.pm())
            .finish()
    }
}
/**RTC timestamp time register

You can [`read`](crate::Reg::read) this register and get [`rtc_tstr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#RTC:RTC_TSTR)*/
pub struct RTC_TSTRrs;
impl crate::RegisterSpec for RTC_TSTRrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_tstr::R`](R) reader structure
impl crate::Readable for RTC_TSTRrs {}
///`reset()` method sets RTC_TSTR to value 0
impl crate::Resettable for RTC_TSTRrs {}
