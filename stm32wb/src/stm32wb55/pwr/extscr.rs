///Register `EXTSCR` reader
pub type R = crate::R<EXTSCRrs>;
///Register `EXTSCR` writer
pub type W = crate::W<EXTSCRrs>;
///Field `C1CSSF` writer - Clear CPU1 Stop Standby flags
pub type C1CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `C2CSSF` writer - Clear CPU2 Stop Standby flags
pub type C2CSSF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CCRPF` writer - Clear Critical Radio system phase
pub type CCRPF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `C1SBF` reader - System Standby flag for CPU1
pub type C1SBF_R = crate::BitReader;
///Field `C1STOPF` reader - System Stop flag for CPU1
pub type C1STOPF_R = crate::BitReader;
///Field `C2SBF` reader - System Standby flag for CPU2
pub type C2SBF_R = crate::BitReader;
///Field `C2STOPF` reader - System Stop flag for CPU2
pub type C2STOPF_R = crate::BitReader;
///Field `CRPF` reader - Critical Radio system phase
pub type CRPF_R = crate::BitReader;
///Field `C1DS` reader - CPU1 deepsleep mode
pub type C1DS_R = crate::BitReader;
///Field `C2DS` reader - CPU2 deepsleep mode
pub type C2DS_R = crate::BitReader;
impl R {
    ///Bit 8 - System Standby flag for CPU1
    #[inline(always)]
    pub fn c1sbf(&self) -> C1SBF_R {
        C1SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - System Stop flag for CPU1
    #[inline(always)]
    pub fn c1stopf(&self) -> C1STOPF_R {
        C1STOPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - System Standby flag for CPU2
    #[inline(always)]
    pub fn c2sbf(&self) -> C2SBF_R {
        C2SBF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - System Stop flag for CPU2
    #[inline(always)]
    pub fn c2stopf(&self) -> C2STOPF_R {
        C2STOPF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - Critical Radio system phase
    #[inline(always)]
    pub fn crpf(&self) -> CRPF_R {
        CRPF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CPU1 deepsleep mode
    #[inline(always)]
    pub fn c1ds(&self) -> C1DS_R {
        C1DS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CPU2 deepsleep mode
    #[inline(always)]
    pub fn c2ds(&self) -> C2DS_R {
        C2DS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTSCR")
            .field("c2ds", &self.c2ds())
            .field("c1ds", &self.c1ds())
            .field("crpf", &self.crpf())
            .field("c2stopf", &self.c2stopf())
            .field("c2sbf", &self.c2sbf())
            .field("c1stopf", &self.c1stopf())
            .field("c1sbf", &self.c1sbf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear CPU1 Stop Standby flags
    #[inline(always)]
    pub fn c1cssf(&mut self) -> C1CSSF_W<'_, EXTSCRrs> {
        C1CSSF_W::new(self, 0)
    }
    ///Bit 1 - Clear CPU2 Stop Standby flags
    #[inline(always)]
    pub fn c2cssf(&mut self) -> C2CSSF_W<'_, EXTSCRrs> {
        C2CSSF_W::new(self, 1)
    }
    ///Bit 2 - Clear Critical Radio system phase
    #[inline(always)]
    pub fn ccrpf(&mut self) -> CCRPF_W<'_, EXTSCRrs> {
        CCRPF_W::new(self, 2)
    }
}
/**Power status clear register

You can [`read`](crate::Reg::read) this register and get [`extscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PWR:EXTSCR)*/
pub struct EXTSCRrs;
impl crate::RegisterSpec for EXTSCRrs {
    type Ux = u32;
}
///`read()` method returns [`extscr::R`](R) reader structure
impl crate::Readable for EXTSCRrs {}
///`write(|w| ..)` method takes [`extscr::W`](W) writer structure
impl crate::Writable for EXTSCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTSCR to value 0
impl crate::Resettable for EXTSCRrs {}
