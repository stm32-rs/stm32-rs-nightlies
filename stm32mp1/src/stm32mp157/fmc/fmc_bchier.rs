#[doc = "Register `FMC_BCHIER` reader"]
pub type R = crate::R<FMC_BCHIERrs>;
#[doc = "Register `FMC_BCHIER` writer"]
pub type W = crate::W<FMC_BCHIERrs>;
#[doc = "Field `DUEIE` reader - DUEIE"]
pub type DUEIE_R = crate::BitReader;
#[doc = "Field `DUEIE` writer - DUEIE"]
pub type DUEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DERIE` reader - DERIE"]
pub type DERIE_R = crate::BitReader;
#[doc = "Field `DERIE` writer - DERIE"]
pub type DERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFIE` reader - DEFIE"]
pub type DEFIE_R = crate::BitReader;
#[doc = "Field `DEFIE` writer - DEFIE"]
pub type DEFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSRIE` reader - DSRIE"]
pub type DSRIE_R = crate::BitReader;
#[doc = "Field `DSRIE` writer - DSRIE"]
pub type DSRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPBRIE` reader - EPBRIE"]
pub type EPBRIE_R = crate::BitReader;
#[doc = "Field `EPBRIE` writer - EPBRIE"]
pub type EPBRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DUEIE"]
    #[inline(always)]
    pub fn dueie(&self) -> DUEIE_R {
        DUEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DERIE"]
    #[inline(always)]
    pub fn derie(&self) -> DERIE_R {
        DERIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DEFIE"]
    #[inline(always)]
    pub fn defie(&self) -> DEFIE_R {
        DEFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSRIE"]
    #[inline(always)]
    pub fn dsrie(&self) -> DSRIE_R {
        DSRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EPBRIE"]
    #[inline(always)]
    pub fn epbrie(&self) -> EPBRIE_R {
        EPBRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DUEIE"]
    #[inline(always)]
    #[must_use]
    pub fn dueie(&mut self) -> DUEIE_W<FMC_BCHIERrs> {
        DUEIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - DERIE"]
    #[inline(always)]
    #[must_use]
    pub fn derie(&mut self) -> DERIE_W<FMC_BCHIERrs> {
        DERIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - DEFIE"]
    #[inline(always)]
    #[must_use]
    pub fn defie(&mut self) -> DEFIE_W<FMC_BCHIERrs> {
        DEFIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - DSRIE"]
    #[inline(always)]
    #[must_use]
    pub fn dsrie(&mut self) -> DSRIE_W<FMC_BCHIERrs> {
        DSRIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - EPBRIE"]
    #[inline(always)]
    #[must_use]
    pub fn epbrie(&mut self) -> EPBRIE_W<FMC_BCHIERrs> {
        EPBRIE_W::new(self, 4)
    }
}
#[doc = "FMC BCH Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_bchier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_bchier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_BCHIERrs;
impl crate::RegisterSpec for FMC_BCHIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_bchier::R`](R) reader structure"]
impl crate::Readable for FMC_BCHIERrs {}
#[doc = "`write(|w| ..)` method takes [`fmc_bchier::W`](W) writer structure"]
impl crate::Writable for FMC_BCHIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_BCHIER to value 0"]
impl crate::Resettable for FMC_BCHIERrs {
    const RESET_VALUE: u32 = 0;
}
