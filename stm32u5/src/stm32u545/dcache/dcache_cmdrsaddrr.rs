///Register `DCACHE_CMDRSADDRR` reader
pub type R = crate::R<DCACHE_CMDRSADDRRrs>;
///Register `DCACHE_CMDRSADDRR` writer
pub type W = crate::W<DCACHE_CMDRSADDRRrs>;
///Field `CMDSTARTADDR` reader - CMDSTARTADDR
pub type CMDSTARTADDR_R = crate::FieldReader<u32>;
///Field `CMDSTARTADDR` writer - CMDSTARTADDR
pub type CMDSTARTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    ///Bits 4:31 - CMDSTARTADDR
    #[inline(always)]
    pub fn cmdstartaddr(&self) -> CMDSTARTADDR_R {
        CMDSTARTADDR_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_CMDRSADDRR")
            .field("cmdstartaddr", &self.cmdstartaddr())
            .finish()
    }
}
impl W {
    ///Bits 4:31 - CMDSTARTADDR
    #[inline(always)]
    #[must_use]
    pub fn cmdstartaddr(&mut self) -> CMDSTARTADDR_W<DCACHE_CMDRSADDRRrs> {
        CMDSTARTADDR_W::new(self, 4)
    }
}
/**command range start address register

You can [`read`](crate::Reg::read) this register and get [`dcache_cmdrsaddrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_cmdrsaddrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#DCACHE:DCACHE_CMDRSADDRR)*/
pub struct DCACHE_CMDRSADDRRrs;
impl crate::RegisterSpec for DCACHE_CMDRSADDRRrs {
    type Ux = u32;
}
///`read()` method returns [`dcache_cmdrsaddrr::R`](R) reader structure
impl crate::Readable for DCACHE_CMDRSADDRRrs {}
///`write(|w| ..)` method takes [`dcache_cmdrsaddrr::W`](W) writer structure
impl crate::Writable for DCACHE_CMDRSADDRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCACHE_CMDRSADDRR to value 0
impl crate::Resettable for DCACHE_CMDRSADDRRrs {
    const RESET_VALUE: u32 = 0;
}
