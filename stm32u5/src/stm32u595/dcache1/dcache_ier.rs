///Register `DCACHE_IER` reader
pub type R = crate::R<DCACHE_IERrs>;
///Register `DCACHE_IER` writer
pub type W = crate::W<DCACHE_IERrs>;
///Field `BSYENDIE` reader - BSYENDIE
pub type BSYENDIE_R = crate::BitReader;
///Field `BSYENDIE` writer - BSYENDIE
pub type BSYENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - ERRIE
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - ERRIE
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMDENDIE` reader - CMDENDIE
pub type CMDENDIE_R = crate::BitReader;
///Field `CMDENDIE` writer - CMDENDIE
pub type CMDENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 4 - CMDENDIE
    #[inline(always)]
    pub fn cmdendie(&self) -> CMDENDIE_R {
        CMDENDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_IER")
            .field("bsyendie", &self.bsyendie())
            .field("errie", &self.errie())
            .field("cmdendie", &self.cmdendie())
            .finish()
    }
}
impl W {
    ///Bit 1 - BSYENDIE
    #[inline(always)]
    #[must_use]
    pub fn bsyendie(&mut self) -> BSYENDIE_W<DCACHE_IERrs> {
        BSYENDIE_W::new(self, 1)
    }
    ///Bit 2 - ERRIE
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<DCACHE_IERrs> {
        ERRIE_W::new(self, 2)
    }
    ///Bit 4 - CMDENDIE
    #[inline(always)]
    #[must_use]
    pub fn cmdendie(&mut self) -> CMDENDIE_W<DCACHE_IERrs> {
        CMDENDIE_W::new(self, 4)
    }
}
/**DCACHE interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dcache_ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#DCACHE1:DCACHE_IER)*/
pub struct DCACHE_IERrs;
impl crate::RegisterSpec for DCACHE_IERrs {
    type Ux = u32;
}
///`read()` method returns [`dcache_ier::R`](R) reader structure
impl crate::Readable for DCACHE_IERrs {}
///`write(|w| ..)` method takes [`dcache_ier::W`](W) writer structure
impl crate::Writable for DCACHE_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCACHE_IER to value 0
impl crate::Resettable for DCACHE_IERrs {
    const RESET_VALUE: u32 = 0;
}
