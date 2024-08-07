///Register `FMC_CSQIER` reader
pub type R = crate::R<FMC_CSQIERrs>;
///Register `FMC_CSQIER` writer
pub type W = crate::W<FMC_CSQIERrs>;
///Field `TCIE` reader - TCIE
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - TCIE
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCIE` reader - SCIE
pub type SCIE_R = crate::BitReader;
///Field `SCIE` writer - SCIE
pub type SCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEIE` reader - SEIE
pub type SEIE_R = crate::BitReader;
///Field `SEIE` writer - SEIE
pub type SEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUEIE` reader - SUEIE
pub type SUEIE_R = crate::BitReader;
///Field `SUEIE` writer - SUEIE
pub type SUEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDTCIE` reader - CMDTCIE
pub type CMDTCIE_R = crate::BitReader;
///Field `CMDTCIE` writer - CMDTCIE
pub type CMDTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TCIE
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SCIE
    #[inline(always)]
    pub fn scie(&self) -> SCIE_R {
        SCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SEIE
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SUEIE
    #[inline(always)]
    pub fn sueie(&self) -> SUEIE_R {
        SUEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CMDTCIE
    #[inline(always)]
    pub fn cmdtcie(&self) -> CMDTCIE_R {
        CMDTCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMC_CSQIER")
            .field("tcie", &self.tcie())
            .field("scie", &self.scie())
            .field("seie", &self.seie())
            .field("sueie", &self.sueie())
            .field("cmdtcie", &self.cmdtcie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TCIE
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<FMC_CSQIERrs> {
        TCIE_W::new(self, 0)
    }
    ///Bit 1 - SCIE
    #[inline(always)]
    #[must_use]
    pub fn scie(&mut self) -> SCIE_W<FMC_CSQIERrs> {
        SCIE_W::new(self, 1)
    }
    ///Bit 2 - SEIE
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SEIE_W<FMC_CSQIERrs> {
        SEIE_W::new(self, 2)
    }
    ///Bit 3 - SUEIE
    #[inline(always)]
    #[must_use]
    pub fn sueie(&mut self) -> SUEIE_W<FMC_CSQIERrs> {
        SUEIE_W::new(self, 3)
    }
    ///Bit 4 - CMDTCIE
    #[inline(always)]
    #[must_use]
    pub fn cmdtcie(&mut self) -> CMDTCIE_W<FMC_CSQIERrs> {
        CMDTCIE_W::new(self, 4)
    }
}
/**FMC NAND Command Sequencer Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`fmc_csqier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc_csqier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FMC:FMC_CSQIER)*/
pub struct FMC_CSQIERrs;
impl crate::RegisterSpec for FMC_CSQIERrs {
    type Ux = u32;
}
///`read()` method returns [`fmc_csqier::R`](R) reader structure
impl crate::Readable for FMC_CSQIERrs {}
///`write(|w| ..)` method takes [`fmc_csqier::W`](W) writer structure
impl crate::Writable for FMC_CSQIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FMC_CSQIER to value 0
impl crate::Resettable for FMC_CSQIERrs {
    const RESET_VALUE: u32 = 0;
}
