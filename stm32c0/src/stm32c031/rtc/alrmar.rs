///Register `ALRMAR` reader
pub type R = crate::R<ALRMARrs>;
///Register `ALRMAR` writer
pub type W = crate::W<ALRMARrs>;
///Field `SU` reader - Second units in BCD format
pub type SU_R = crate::FieldReader;
///Field `SU` writer - Second units in BCD format
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ST` reader - Second tens in BCD format
pub type ST_R = crate::FieldReader;
///Field `ST` writer - Second tens in BCD format
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MSK1` reader - Alarm seconds mask
pub type MSK1_R = crate::BitReader;
///Field `MSK1` writer - Alarm seconds mask
pub type MSK1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MNU` reader - Minute units in BCD format
pub type MNU_R = crate::FieldReader;
///Field `MNU` writer - Minute units in BCD format
pub type MNU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MNT` reader - Minute tens in BCD format
pub type MNT_R = crate::FieldReader;
///Field `MNT` writer - Minute tens in BCD format
pub type MNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `MSK2` reader - Alarm minutes mask
pub type MSK2_R = crate::BitReader;
///Field `MSK2` writer - Alarm minutes mask
pub type MSK2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HU` reader - Hour units in BCD format
pub type HU_R = crate::FieldReader;
///Field `HU` writer - Hour units in BCD format
pub type HU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HT` reader - Hour tens in BCD format
pub type HT_R = crate::FieldReader;
///Field `HT` writer - Hour tens in BCD format
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PM` reader - AM/PM notation
pub type PM_R = crate::BitReader;
///Field `PM` writer - AM/PM notation
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSK3` reader - Alarm hours mask
pub type MSK3_R = crate::BitReader;
///Field `MSK3` writer - Alarm hours mask
pub type MSK3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DU` reader - Date units or day in BCD format
pub type DU_R = crate::FieldReader;
///Field `DU` writer - Date units or day in BCD format
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DT` reader - Date tens in BCD format
pub type DT_R = crate::FieldReader;
///Field `DT` writer - Date tens in BCD format
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WDSEL` reader - Week day selection
pub type WDSEL_R = crate::BitReader;
///Field `WDSEL` writer - Week day selection
pub type WDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSK4` reader - Alarm date mask
pub type MSK4_R = crate::BitReader;
///Field `MSK4` writer - Alarm date mask
pub type MSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - Second units in BCD format
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - Second tens in BCD format
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Alarm seconds mask
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
    ///Bit 15 - Alarm minutes mask
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
    ///Bit 23 - Alarm hours mask
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
    ///Bit 31 - Alarm date mask
    #[inline(always)]
    pub fn msk4(&self) -> MSK4_R {
        MSK4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALRMAR")
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
    ///Bits 0:3 - Second units in BCD format
    #[inline(always)]
    pub fn su(&mut self) -> SU_W<ALRMARrs> {
        SU_W::new(self, 0)
    }
    ///Bits 4:6 - Second tens in BCD format
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<ALRMARrs> {
        ST_W::new(self, 4)
    }
    ///Bit 7 - Alarm seconds mask
    #[inline(always)]
    pub fn msk1(&mut self) -> MSK1_W<ALRMARrs> {
        MSK1_W::new(self, 7)
    }
    ///Bits 8:11 - Minute units in BCD format
    #[inline(always)]
    pub fn mnu(&mut self) -> MNU_W<ALRMARrs> {
        MNU_W::new(self, 8)
    }
    ///Bits 12:14 - Minute tens in BCD format
    #[inline(always)]
    pub fn mnt(&mut self) -> MNT_W<ALRMARrs> {
        MNT_W::new(self, 12)
    }
    ///Bit 15 - Alarm minutes mask
    #[inline(always)]
    pub fn msk2(&mut self) -> MSK2_W<ALRMARrs> {
        MSK2_W::new(self, 15)
    }
    ///Bits 16:19 - Hour units in BCD format
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W<ALRMARrs> {
        HU_W::new(self, 16)
    }
    ///Bits 20:21 - Hour tens in BCD format
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W<ALRMARrs> {
        HT_W::new(self, 20)
    }
    ///Bit 22 - AM/PM notation
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<ALRMARrs> {
        PM_W::new(self, 22)
    }
    ///Bit 23 - Alarm hours mask
    #[inline(always)]
    pub fn msk3(&mut self) -> MSK3_W<ALRMARrs> {
        MSK3_W::new(self, 23)
    }
    ///Bits 24:27 - Date units or day in BCD format
    #[inline(always)]
    pub fn du(&mut self) -> DU_W<ALRMARrs> {
        DU_W::new(self, 24)
    }
    ///Bits 28:29 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<ALRMARrs> {
        DT_W::new(self, 28)
    }
    ///Bit 30 - Week day selection
    #[inline(always)]
    pub fn wdsel(&mut self) -> WDSEL_W<ALRMARrs> {
        WDSEL_W::new(self, 30)
    }
    ///Bit 31 - Alarm date mask
    #[inline(always)]
    pub fn msk4(&mut self) -> MSK4_W<ALRMARrs> {
        MSK4_W::new(self, 31)
    }
}
/**Alarm register

You can [`read`](crate::Reg::read) this register and get [`alrmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#RTC:ALRMAR)*/
pub struct ALRMARrs;
impl crate::RegisterSpec for ALRMARrs {
    type Ux = u32;
}
///`read()` method returns [`alrmar::R`](R) reader structure
impl crate::Readable for ALRMARrs {}
///`write(|w| ..)` method takes [`alrmar::W`](W) writer structure
impl crate::Writable for ALRMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ALRMAR to value 0
impl crate::Resettable for ALRMARrs {
    const RESET_VALUE: u32 = 0;
}
