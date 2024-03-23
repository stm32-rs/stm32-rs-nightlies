#[doc = "Register `EXTSCR` reader"]
pub type R = crate::R<EXTSCRrs>;
#[doc = "Register `EXTSCR` writer"]
pub type W = crate::W<EXTSCRrs>;
#[doc = "Field `C1CSSF` writer - Clear CPU1 Stop Standby flags"]
pub type C1CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2CSSF` writer - Clear CPU2 Stop Standby flags"]
pub type C2CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCRPF` writer - Clear Critical Radio system phase"]
pub type CCRPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1SBF` reader - System Standby flag for CPU1"]
pub type C1SBF_R = crate::BitReader;
#[doc = "Field `C1STOPF` reader - System Stop flag for CPU1"]
pub type C1STOPF_R = crate::BitReader;
#[doc = "Field `C2SBF` reader - System Standby flag for CPU2"]
pub type C2SBF_R = crate::BitReader;
#[doc = "Field `C2STOPF` reader - System Stop flag for CPU2"]
pub type C2STOPF_R = crate::BitReader;
#[doc = "Field `CRPF` reader - Critical Radio system phase"]
pub type CRPF_R = crate::BitReader;
#[doc = "Field `C1DS` reader - CPU1 deepsleep mode"]
pub type C1DS_R = crate::BitReader;
#[doc = "Field `C2DS` reader - CPU2 deepsleep mode"]
pub type C2DS_R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - System Standby flag for CPU1"]
    #[inline(always)]
    pub fn c1sbf(&self) -> C1SBF_R {
        C1SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - System Stop flag for CPU1"]
    #[inline(always)]
    pub fn c1stopf(&self) -> C1STOPF_R {
        C1STOPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - System Standby flag for CPU2"]
    #[inline(always)]
    pub fn c2sbf(&self) -> C2SBF_R {
        C2SBF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - System Stop flag for CPU2"]
    #[inline(always)]
    pub fn c2stopf(&self) -> C2STOPF_R {
        C2STOPF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Critical Radio system phase"]
    #[inline(always)]
    pub fn crpf(&self) -> CRPF_R {
        CRPF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU1 deepsleep mode"]
    #[inline(always)]
    pub fn c1ds(&self) -> C1DS_R {
        C1DS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU2 deepsleep mode"]
    #[inline(always)]
    pub fn c2ds(&self) -> C2DS_R {
        C2DS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear CPU1 Stop Standby flags"]
    #[inline(always)]
    #[must_use]
    pub fn c1cssf(&mut self) -> C1CSSF_W<EXTSCRrs> {
        C1CSSF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear CPU2 Stop Standby flags"]
    #[inline(always)]
    #[must_use]
    pub fn c2cssf(&mut self) -> C2CSSF_W<EXTSCRrs> {
        C2CSSF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Critical Radio system phase"]
    #[inline(always)]
    #[must_use]
    pub fn ccrpf(&mut self) -> CCRPF_W<EXTSCRrs> {
        CCRPF_W::new(self, 2)
    }
}
#[doc = "Power status clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extscr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extscr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTSCRrs;
impl crate::RegisterSpec for EXTSCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extscr::R`](R) reader structure"]
impl crate::Readable for EXTSCRrs {}
#[doc = "`write(|w| ..)` method takes [`extscr::W`](W) writer structure"]
impl crate::Writable for EXTSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTSCR to value 0"]
impl crate::Resettable for EXTSCRrs {
    const RESET_VALUE: u32 = 0;
}
