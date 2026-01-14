///Register `TSTR` reader
pub type R = crate::R<TSTRrs>;
///Register `TSTR` writer
pub type W = crate::W<TSTRrs>;
///Field `SU` reader - Second units in BCD format.
pub type SU_R = crate::FieldReader;
///Field `SU` writer - Second units in BCD format.
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ST` reader - Second tens in BCD format.
pub type ST_R = crate::FieldReader;
///Field `ST` writer - Second tens in BCD format.
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MNU` reader - Minute units in BCD format.
pub type MNU_R = crate::FieldReader;
///Field `MNU` writer - Minute units in BCD format.
pub type MNU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MNT` reader - Minute tens in BCD format.
pub type MNT_R = crate::FieldReader;
///Field `MNT` writer - Minute tens in BCD format.
pub type MNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `HU` reader - Hour units in BCD format.
pub type HU_R = crate::FieldReader;
///Field `HU` writer - Hour units in BCD format.
pub type HU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HT` reader - Hour tens in BCD format.
pub type HT_R = crate::FieldReader;
///Field `HT` writer - Hour tens in BCD format.
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PM` reader - AM/PM notation 0: AM or 24-hour format 1: PM
pub type PM_R = crate::BitReader;
///Field `PM` writer - AM/PM notation 0: AM or 24-hour format 1: PM
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 22 - AM/PM notation 0: AM or 24-hour format 1: PM
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSTR")
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
impl W {
    ///Bits 0:3 - Second units in BCD format.
    #[inline(always)]
    pub fn su(&mut self) -> SU_W<'_, TSTRrs> {
        SU_W::new(self, 0)
    }
    ///Bits 4:6 - Second tens in BCD format.
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<'_, TSTRrs> {
        ST_W::new(self, 4)
    }
    ///Bits 8:11 - Minute units in BCD format.
    #[inline(always)]
    pub fn mnu(&mut self) -> MNU_W<'_, TSTRrs> {
        MNU_W::new(self, 8)
    }
    ///Bits 12:14 - Minute tens in BCD format.
    #[inline(always)]
    pub fn mnt(&mut self) -> MNT_W<'_, TSTRrs> {
        MNT_W::new(self, 12)
    }
    ///Bits 16:19 - Hour units in BCD format.
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W<'_, TSTRrs> {
        HU_W::new(self, 16)
    }
    ///Bits 20:21 - Hour tens in BCD format.
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W<'_, TSTRrs> {
        HT_W::new(self, 20)
    }
    ///Bit 22 - AM/PM notation 0: AM or 24-hour format 1: PM
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<'_, TSTRrs> {
        PM_W::new(self, 22)
    }
}
/**RTC_TSTR register

You can [`read`](crate::Reg::read) this register and get [`tstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RTC:TSTR)*/
pub struct TSTRrs;
impl crate::RegisterSpec for TSTRrs {
    type Ux = u32;
}
///`read()` method returns [`tstr::R`](R) reader structure
impl crate::Readable for TSTRrs {}
///`write(|w| ..)` method takes [`tstr::W`](W) writer structure
impl crate::Writable for TSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSTR to value 0
impl crate::Resettable for TSTRrs {}
