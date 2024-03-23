#[doc = "Register `TR` reader"]
pub type R = crate::R<TRrs>;
#[doc = "Register `TR` writer"]
pub type W = crate::W<TRrs>;
#[doc = "Field `SU` reader - SU"]
pub type SU_R = crate::FieldReader;
#[doc = "Field `SU` writer - SU"]
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ST` reader - ST"]
pub type ST_R = crate::FieldReader;
#[doc = "Field `ST` writer - ST"]
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MNU` reader - MNU"]
pub type MNU_R = crate::FieldReader;
#[doc = "Field `MNU` writer - MNU"]
pub type MNU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - MNT"]
pub type MNT_R = crate::FieldReader;
#[doc = "Field `MNT` writer - MNT"]
pub type MNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HU` reader - HU"]
pub type HU_R = crate::FieldReader;
#[doc = "Field `HU` writer - HU"]
pub type HU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HT` reader - HT"]
pub type HT_R = crate::FieldReader;
#[doc = "Field `HT` writer - HT"]
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PM` reader - PM"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - PM"]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - SU"]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - ST"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - MNU"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - MNT"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - HU"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - HT"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SU"]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SU_W<TRrs> {
        SU_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - ST"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<TRrs> {
        ST_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - MNU"]
    #[inline(always)]
    #[must_use]
    pub fn mnu(&mut self) -> MNU_W<TRrs> {
        MNU_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - MNT"]
    #[inline(always)]
    #[must_use]
    pub fn mnt(&mut self) -> MNT_W<TRrs> {
        MNT_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - HU"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<TRrs> {
        HU_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - HT"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<TRrs> {
        HT_W::new(self, 20)
    }
    #[doc = "Bit 22 - PM"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<TRrs> {
        PM_W::new(self, 22)
    }
}
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRrs;
impl crate::RegisterSpec for TRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr::R`](R) reader structure"]
impl crate::Readable for TRrs {}
#[doc = "`write(|w| ..)` method takes [`tr::W`](W) writer structure"]
impl crate::Writable for TRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TRrs {
    const RESET_VALUE: u32 = 0;
}
