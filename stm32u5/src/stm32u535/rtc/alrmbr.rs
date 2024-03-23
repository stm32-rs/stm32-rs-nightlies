#[doc = "Register `ALRMBR` reader"]
pub type R = crate::R<ALRMBRrs>;
#[doc = "Register `ALRMBR` writer"]
pub type W = crate::W<ALRMBRrs>;
#[doc = "Field `SU` reader - Second units in BCD format"]
pub type SU_R = crate::FieldReader;
#[doc = "Field `SU` writer - Second units in BCD format"]
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ST` reader - Second tens in BCD format"]
pub type ST_R = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens in BCD format"]
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSK1` reader - Alarm B seconds mask"]
pub type MSK1_R = crate::BitReader;
#[doc = "Field `MSK1` writer - Alarm B seconds mask"]
pub type MSK1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub type MNU_R = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD format"]
pub type MNU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub type MNT_R = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD format"]
pub type MNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSK2` reader - Alarm B minutes mask"]
pub type MSK2_R = crate::BitReader;
#[doc = "Field `MSK2` writer - Alarm B minutes mask"]
pub type MSK2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HU` reader - Hour units in BCD format"]
pub type HU_R = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units in BCD format"]
pub type HU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HT` reader - Hour tens in BCD format"]
pub type HT_R = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens in BCD format"]
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - AM/PM notation"]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSK3` reader - Alarm B hours mask"]
pub type MSK3_R = crate::BitReader;
#[doc = "Field `MSK3` writer - Alarm B hours mask"]
pub type MSK3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DU` reader - Date units or day in BCD format"]
pub type DU_R = crate::FieldReader;
#[doc = "Field `DU` writer - Date units or day in BCD format"]
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens in BCD format"]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WDSEL` reader - Week day selection"]
pub type WDSEL_R = crate::BitReader;
#[doc = "Field `WDSEL` writer - Week day selection"]
pub type WDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSK4` reader - Alarm B date mask"]
pub type MSK4_R = crate::BitReader;
#[doc = "Field `MSK4` writer - Alarm B date mask"]
pub type MSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm B seconds mask"]
    #[inline(always)]
    pub fn msk1(&self) -> MSK1_R {
        MSK1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm B minutes mask"]
    #[inline(always)]
    pub fn msk2(&self) -> MSK2_R {
        MSK2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Alarm B hours mask"]
    #[inline(always)]
    pub fn msk3(&self) -> MSK3_R {
        MSK3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn wdsel(&self) -> WDSEL_R {
        WDSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm B date mask"]
    #[inline(always)]
    pub fn msk4(&self) -> MSK4_R {
        MSK4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SU_W<ALRMBRrs> {
        SU_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<ALRMBRrs> {
        ST_W::new(self, 4)
    }
    #[doc = "Bit 7 - Alarm B seconds mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk1(&mut self) -> MSK1_W<ALRMBRrs> {
        MSK1_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mnu(&mut self) -> MNU_W<ALRMBRrs> {
        MNU_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mnt(&mut self) -> MNT_W<ALRMBRrs> {
        MNT_W::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm B minutes mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk2(&mut self) -> MSK2_W<ALRMBRrs> {
        MSK2_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<ALRMBRrs> {
        HU_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<ALRMBRrs> {
        HT_W::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<ALRMBRrs> {
        PM_W::new(self, 22)
    }
    #[doc = "Bit 23 - Alarm B hours mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk3(&mut self) -> MSK3_W<ALRMBRrs> {
        MSK3_W::new(self, 23)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<ALRMBRrs> {
        DU_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<ALRMBRrs> {
        DT_W::new(self, 28)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    #[must_use]
    pub fn wdsel(&mut self) -> WDSEL_W<ALRMBRrs> {
        WDSEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - Alarm B date mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk4(&mut self) -> MSK4_W<ALRMBRrs> {
        MSK4_W::new(self, 31)
    }
}
#[doc = "alarm B register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmbr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmbr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRMBRrs;
impl crate::RegisterSpec for ALRMBRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrmbr::R`](R) reader structure"]
impl crate::Readable for ALRMBRrs {}
#[doc = "`write(|w| ..)` method takes [`alrmbr::W`](W) writer structure"]
impl crate::Writable for ALRMBRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALRMBR to value 0"]
impl crate::Resettable for ALRMBRrs {
    const RESET_VALUE: u32 = 0;
}
