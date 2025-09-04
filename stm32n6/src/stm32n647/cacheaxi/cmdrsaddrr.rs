///Register `CMDRSADDRR` reader
pub type R = crate::R<CMDRSADDRRrs>;
///Register `CMDRSADDRR` writer
pub type W = crate::W<CMDRSADDRRrs>;
///Field `CMDSTARTADDR` reader - start address of range to which the cache maintenance command specified in CACHEAXI_CR2.CACHECMD field applies
pub type CMDSTARTADDR_R = crate::FieldReader<u32>;
///Field `CMDSTARTADDR` writer - start address of range to which the cache maintenance command specified in CACHEAXI_CR2.CACHECMD field applies
pub type CMDSTARTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 6:31 - start address of range to which the cache maintenance command specified in CACHEAXI_CR2.CACHECMD field applies
    #[inline(always)]
    pub fn cmdstartaddr(&self) -> CMDSTARTADDR_R {
        CMDSTARTADDR_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDRSADDRR")
            .field("cmdstartaddr", &self.cmdstartaddr())
            .finish()
    }
}
impl W {
    ///Bits 6:31 - start address of range to which the cache maintenance command specified in CACHEAXI_CR2.CACHECMD field applies
    #[inline(always)]
    pub fn cmdstartaddr(&mut self) -> CMDSTARTADDR_W<CMDRSADDRRrs> {
        CMDSTARTADDR_W::new(self, 6)
    }
}
/**CACHEAXI command range start address register

You can [`read`](crate::Reg::read) this register and get [`cmdrsaddrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdrsaddrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#CACHEAXI:CMDRSADDRR)*/
pub struct CMDRSADDRRrs;
impl crate::RegisterSpec for CMDRSADDRRrs {
    type Ux = u32;
}
///`read()` method returns [`cmdrsaddrr::R`](R) reader structure
impl crate::Readable for CMDRSADDRRrs {}
///`write(|w| ..)` method takes [`cmdrsaddrr::W`](W) writer structure
impl crate::Writable for CMDRSADDRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMDRSADDRR to value 0
impl crate::Resettable for CMDRSADDRRrs {}
