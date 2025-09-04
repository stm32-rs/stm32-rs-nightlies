///Register `TSTR` reader
pub type R = crate::R<TSTRrs>;
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
    Am = 0,
    ///1: PM
    Pm = 1,
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
            false => PM::Am,
            true => PM::Pm,
        }
    }
    ///AM or 24-hour format
    #[inline(always)]
    pub fn is_am(&self) -> bool {
        *self == PM::Am
    }
    ///PM
    #[inline(always)]
    pub fn is_pm(&self) -> bool {
        *self == PM::Pm
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
        f.debug_struct("TSTR")
            .field("pm", &self.pm())
            .field("ht", &self.ht())
            .field("hu", &self.hu())
            .field("mnt", &self.mnt())
            .field("mnu", &self.mnu())
            .field("st", &self.st())
            .field("su", &self.su())
            .finish()
    }
}
/**Timestamp time register

You can [`read`](crate::Reg::read) this register and get [`tstr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#RTC:TSTR)*/
pub struct TSTRrs;
impl crate::RegisterSpec for TSTRrs {
    type Ux = u32;
}
///`read()` method returns [`tstr::R`](R) reader structure
impl crate::Readable for TSTRrs {}
///`reset()` method sets TSTR to value 0
impl crate::Resettable for TSTRrs {}
