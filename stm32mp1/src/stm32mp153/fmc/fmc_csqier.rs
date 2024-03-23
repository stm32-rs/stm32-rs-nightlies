#[doc = "Register `FMC_CSQIER` reader"]
pub type R = crate::R<FMC_CSQIERrs>;
#[doc = "Register `FMC_CSQIER` writer"]
pub type W = crate::W<FMC_CSQIERrs>;
#[doc = "Field `TCIE` reader - TCIE"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - TCIE"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCIE` reader - SCIE"]
pub type SCIE_R = crate::BitReader;
#[doc = "Field `SCIE` writer - SCIE"]
pub type SCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEIE` reader - SEIE"]
pub type SEIE_R = crate::BitReader;
#[doc = "Field `SEIE` writer - SEIE"]
pub type SEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUEIE` reader - SUEIE"]
pub type SUEIE_R = crate::BitReader;
#[doc = "Field `SUEIE` writer - SUEIE"]
pub type SUEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTCIE` reader - CMDTCIE"]
pub type CMDTCIE_R = crate::BitReader;
#[doc = "Field `CMDTCIE` writer - CMDTCIE"]
pub type CMDTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCIE"]
    #[inline(always)]
    pub fn scie(&self) -> SCIE_R {
        SCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SEIE"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SUEIE"]
    #[inline(always)]
    pub fn sueie(&self) -> SUEIE_R {
        SUEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMDTCIE"]
    #[inline(always)]
    pub fn cmdtcie(&self) -> CMDTCIE_R {
        CMDTCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<FMC_CSQIERrs> {
        TCIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SCIE"]
    #[inline(always)]
    #[must_use]
    pub fn scie(&mut self) -> SCIE_W<FMC_CSQIERrs> {
        SCIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - SEIE"]
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SEIE_W<FMC_CSQIERrs> {
        SEIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - SUEIE"]
    #[inline(always)]
    #[must_use]
    pub fn sueie(&mut self) -> SUEIE_W<FMC_CSQIERrs> {
        SUEIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - CMDTCIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtcie(&mut self) -> CMDTCIE_W<FMC_CSQIERrs> {
        CMDTCIE_W::new(self, 4)
    }
}
#[doc = "FMC NAND Command Sequencer Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_csqier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_CSQIERrs;
impl crate::RegisterSpec for FMC_CSQIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_csqier::R`](R) reader structure"]
impl crate::Readable for FMC_CSQIERrs {}
#[doc = "`write(|w| ..)` method takes [`fmc_csqier::W`](W) writer structure"]
impl crate::Writable for FMC_CSQIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_CSQIER to value 0"]
impl crate::Resettable for FMC_CSQIERrs {
    const RESET_VALUE: u32 = 0;
}
