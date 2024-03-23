#[doc = "Register `FMC_PMEM` reader"]
pub type R = crate::R<FMC_PMEMrs>;
#[doc = "Register `FMC_PMEM` writer"]
pub type W = crate::W<FMC_PMEMrs>;
#[doc = "Field `MEMSET` reader - MEMSET"]
pub type MEMSET_R = crate::FieldReader;
#[doc = "Field `MEMSET` writer - MEMSET"]
pub type MEMSET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMWAIT` reader - MEMWAIT"]
pub type MEMWAIT_R = crate::FieldReader;
#[doc = "Field `MEMWAIT` writer - MEMWAIT"]
pub type MEMWAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMHOLD` reader - MEMHOLD"]
pub type MEMHOLD_R = crate::FieldReader;
#[doc = "Field `MEMHOLD` writer - MEMHOLD"]
pub type MEMHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEMHIZ` reader - MEMHIZ"]
pub type MEMHIZ_R = crate::FieldReader;
#[doc = "Field `MEMHIZ` writer - MEMHIZ"]
pub type MEMHIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MEMSET"]
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MEMWAIT"]
    #[inline(always)]
    pub fn memwait(&self) -> MEMWAIT_R {
        MEMWAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MEMHOLD"]
    #[inline(always)]
    pub fn memhold(&self) -> MEMHOLD_R {
        MEMHOLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - MEMHIZ"]
    #[inline(always)]
    pub fn memhiz(&self) -> MEMHIZ_R {
        MEMHIZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MEMSET"]
    #[inline(always)]
    #[must_use]
    pub fn memset(&mut self) -> MEMSET_W<FMC_PMEMrs> {
        MEMSET_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - MEMWAIT"]
    #[inline(always)]
    #[must_use]
    pub fn memwait(&mut self) -> MEMWAIT_W<FMC_PMEMrs> {
        MEMWAIT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - MEMHOLD"]
    #[inline(always)]
    #[must_use]
    pub fn memhold(&mut self) -> MEMHOLD_W<FMC_PMEMrs> {
        MEMHOLD_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - MEMHIZ"]
    #[inline(always)]
    #[must_use]
    pub fn memhiz(&mut self) -> MEMHIZ_W<FMC_PMEMrs> {
        MEMHIZ_W::new(self, 24)
    }
}
#[doc = "The FMC_PMEM read/write register contains NAND Flash memory bank timing information. This information is used to access the NAND Flash common memory space for command, address write accesses or data read/write accesses.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_pmem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_pmem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_PMEMrs;
impl crate::RegisterSpec for FMC_PMEMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_pmem::R`](R) reader structure"]
impl crate::Readable for FMC_PMEMrs {}
#[doc = "`write(|w| ..)` method takes [`fmc_pmem::W`](W) writer structure"]
impl crate::Writable for FMC_PMEMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_PMEM to value 0x0a0a_0a0a"]
impl crate::Resettable for FMC_PMEMrs {
    const RESET_VALUE: u32 = 0x0a0a_0a0a;
}
