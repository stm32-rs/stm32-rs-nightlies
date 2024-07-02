///Register `ICACHE_IER` reader
pub type R = crate::R<ICACHE_IERrs>;
///Register `ICACHE_IER` writer
pub type W = crate::W<ICACHE_IERrs>;
///Field `BSYENDIE` reader - BSYENDIE
pub type BSYENDIE_R = crate::BitReader;
///Field `BSYENDIE` writer - BSYENDIE
pub type BSYENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - ERRIE
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - ERRIE
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - BSYENDIE
    #[inline(always)]
    pub fn bsyendie(&self) -> BSYENDIE_R {
        BSYENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ERRIE
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_IER")
            .field("bsyendie", &self.bsyendie())
            .field("errie", &self.errie())
            .finish()
    }
}
impl W {
    ///Bit 1 - BSYENDIE
    #[inline(always)]
    #[must_use]
    pub fn bsyendie(&mut self) -> BSYENDIE_W<ICACHE_IERrs> {
        BSYENDIE_W::new(self, 1)
    }
    ///Bit 2 - ERRIE
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<ICACHE_IERrs> {
        ERRIE_W::new(self, 2)
    }
}
/**ICACHE interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`icache_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icache_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ICache:ICACHE_IER)*/
pub struct ICACHE_IERrs;
impl crate::RegisterSpec for ICACHE_IERrs {
    type Ux = u32;
}
///`read()` method returns [`icache_ier::R`](R) reader structure
impl crate::Readable for ICACHE_IERrs {}
///`write(|w| ..)` method takes [`icache_ier::W`](W) writer structure
impl crate::Writable for ICACHE_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICACHE_IER to value 0
impl crate::Resettable for ICACHE_IERrs {
    const RESET_VALUE: u32 = 0;
}
