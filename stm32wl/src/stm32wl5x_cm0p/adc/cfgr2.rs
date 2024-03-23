#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<CFGR2rs>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<CFGR2rs>;
#[doc = "Field `OVSE` reader - OVSE"]
pub type OVSE_R = crate::BitReader;
#[doc = "Field `OVSE` writer - OVSE"]
pub type OVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSR0` reader - OVSR0"]
pub type OVSR0_R = crate::BitReader;
#[doc = "Field `OVSR0` writer - OVSR0"]
pub type OVSR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSR1` reader - OVSR1"]
pub type OVSR1_R = crate::BitReader;
#[doc = "Field `OVSR1` writer - OVSR1"]
pub type OVSR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSR2` reader - OVSR2"]
pub type OVSR2_R = crate::BitReader;
#[doc = "Field `OVSR2` writer - OVSR2"]
pub type OVSR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSS0` reader - OVSS0"]
pub type OVSS0_R = crate::BitReader;
#[doc = "Field `OVSS0` writer - OVSS0"]
pub type OVSS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSS1` reader - OVSS1"]
pub type OVSS1_R = crate::BitReader;
#[doc = "Field `OVSS1` writer - OVSS1"]
pub type OVSS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSS2` reader - OVSS2"]
pub type OVSS2_R = crate::BitReader;
#[doc = "Field `OVSS2` writer - OVSS2"]
pub type OVSS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSS3` reader - OVSS3"]
pub type OVSS3_R = crate::BitReader;
#[doc = "Field `OVSS3` writer - OVSS3"]
pub type OVSS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOVS` reader - TOVS"]
pub type TOVS_R = crate::BitReader;
#[doc = "Field `TOVS` writer - TOVS"]
pub type TOVS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFTRIG` reader - LFTRIG"]
pub type LFTRIG_R = crate::BitReader;
#[doc = "Field `LFTRIG` writer - LFTRIG"]
pub type LFTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKMODE` reader - CKMODE"]
pub type CKMODE_R = crate::FieldReader;
#[doc = "Field `CKMODE` writer - CKMODE"]
pub type CKMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - OVSE"]
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - OVSR0"]
    #[inline(always)]
    pub fn ovsr0(&self) -> OVSR0_R {
        OVSR0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OVSR1"]
    #[inline(always)]
    pub fn ovsr1(&self) -> OVSR1_R {
        OVSR1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OVSR2"]
    #[inline(always)]
    pub fn ovsr2(&self) -> OVSR2_R {
        OVSR2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OVSS0"]
    #[inline(always)]
    pub fn ovss0(&self) -> OVSS0_R {
        OVSS0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OVSS1"]
    #[inline(always)]
    pub fn ovss1(&self) -> OVSS1_R {
        OVSS1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OVSS2"]
    #[inline(always)]
    pub fn ovss2(&self) -> OVSS2_R {
        OVSS2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OVSS3"]
    #[inline(always)]
    pub fn ovss3(&self) -> OVSS3_R {
        OVSS3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TOVS"]
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - LFTRIG"]
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - CKMODE"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OVSE"]
    #[inline(always)]
    #[must_use]
    pub fn ovse(&mut self) -> OVSE_W<CFGR2rs> {
        OVSE_W::new(self, 0)
    }
    #[doc = "Bit 2 - OVSR0"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr0(&mut self) -> OVSR0_W<CFGR2rs> {
        OVSR0_W::new(self, 2)
    }
    #[doc = "Bit 3 - OVSR1"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr1(&mut self) -> OVSR1_W<CFGR2rs> {
        OVSR1_W::new(self, 3)
    }
    #[doc = "Bit 4 - OVSR2"]
    #[inline(always)]
    #[must_use]
    pub fn ovsr2(&mut self) -> OVSR2_W<CFGR2rs> {
        OVSR2_W::new(self, 4)
    }
    #[doc = "Bit 5 - OVSS0"]
    #[inline(always)]
    #[must_use]
    pub fn ovss0(&mut self) -> OVSS0_W<CFGR2rs> {
        OVSS0_W::new(self, 5)
    }
    #[doc = "Bit 6 - OVSS1"]
    #[inline(always)]
    #[must_use]
    pub fn ovss1(&mut self) -> OVSS1_W<CFGR2rs> {
        OVSS1_W::new(self, 6)
    }
    #[doc = "Bit 7 - OVSS2"]
    #[inline(always)]
    #[must_use]
    pub fn ovss2(&mut self) -> OVSS2_W<CFGR2rs> {
        OVSS2_W::new(self, 7)
    }
    #[doc = "Bit 8 - OVSS3"]
    #[inline(always)]
    #[must_use]
    pub fn ovss3(&mut self) -> OVSS3_W<CFGR2rs> {
        OVSS3_W::new(self, 8)
    }
    #[doc = "Bit 9 - TOVS"]
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TOVS_W<CFGR2rs> {
        TOVS_W::new(self, 9)
    }
    #[doc = "Bit 29 - LFTRIG"]
    #[inline(always)]
    #[must_use]
    pub fn lftrig(&mut self) -> LFTRIG_W<CFGR2rs> {
        LFTRIG_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - CKMODE"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<CFGR2rs> {
        CKMODE_W::new(self, 30)
    }
}
#[doc = "ADC configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR2rs;
impl crate::RegisterSpec for CFGR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for CFGR2rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
