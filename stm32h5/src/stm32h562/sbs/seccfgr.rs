#[doc = "Register `SECCFGR` reader"]
pub type R = crate::R<SECCFGRrs>;
#[doc = "Register `SECCFGR` writer"]
pub type W = crate::W<SECCFGRrs>;
#[doc = "Field `SBSSEC` reader - SBS clock control, memory-erase status register and compensation cell register security enable"]
pub type SBSSEC_R = crate::BitReader;
#[doc = "Field `SBSSEC` writer - SBS clock control, memory-erase status register and compensation cell register security enable"]
pub type SBSSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLASSBSEC` reader - ClassB security enable"]
pub type CLASSBSEC_R = crate::BitReader;
#[doc = "Field `CLASSBSEC` writer - ClassB security enable"]
pub type CLASSBSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPUSEC` reader - FPU security enable Note: This bit can only be written through privilege transaction."]
pub type FPUSEC_R = crate::BitReader;
#[doc = "Field `FPUSEC` writer - FPU security enable Note: This bit can only be written through privilege transaction."]
pub type FPUSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDCE_SEC_EN` reader - control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR"]
pub type SDCE_SEC_EN_R = crate::BitReader;
#[doc = "Field `SDCE_SEC_EN` writer - control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR"]
pub type SDCE_SEC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SBS clock control, memory-erase status register and compensation cell register security enable"]
    #[inline(always)]
    pub fn sbssec(&self) -> SBSSEC_R {
        SBSSEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ClassB security enable"]
    #[inline(always)]
    pub fn classbsec(&self) -> CLASSBSEC_R {
        CLASSBSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - FPU security enable Note: This bit can only be written through privilege transaction."]
    #[inline(always)]
    pub fn fpusec(&self) -> FPUSEC_R {
        FPUSEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR"]
    #[inline(always)]
    pub fn sdce_sec_en(&self) -> SDCE_SEC_EN_R {
        SDCE_SEC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SBS clock control, memory-erase status register and compensation cell register security enable"]
    #[inline(always)]
    #[must_use]
    pub fn sbssec(&mut self) -> SBSSEC_W<SECCFGRrs> {
        SBSSEC_W::new(self, 0)
    }
    #[doc = "Bit 1 - ClassB security enable"]
    #[inline(always)]
    #[must_use]
    pub fn classbsec(&mut self) -> CLASSBSEC_W<SECCFGRrs> {
        CLASSBSEC_W::new(self, 1)
    }
    #[doc = "Bit 3 - FPU security enable Note: This bit can only be written through privilege transaction."]
    #[inline(always)]
    #[must_use]
    pub fn fpusec(&mut self) -> FPUSEC_W<SECCFGRrs> {
        FPUSEC_W::new(self, 3)
    }
    #[doc = "Bit 31 - control accessibility of SMPS_DIV_CLOCK _EN in SBS_PMCR"]
    #[inline(always)]
    #[must_use]
    pub fn sdce_sec_en(&mut self) -> SDCE_SEC_EN_W<SECCFGRrs> {
        SDCE_SEC_EN_W::new(self, 31)
    }
}
#[doc = "SBS security mode configuration control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seccfgr::R`](R) reader structure"]
impl crate::Readable for SECCFGRrs {}
#[doc = "`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure"]
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SECCFGR to value 0"]
impl crate::Resettable for SECCFGRrs {
    const RESET_VALUE: u32 = 0;
}
