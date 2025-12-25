///Register `CM55RWMCR` reader
pub type R = crate::R<CM55RWMCRrs>;
///Register `CM55RWMCR` writer
pub type W = crate::W<CM55RWMCRrs>;
///Field `RME_TCM` reader - RW margin enable input for TCM memories
pub type RME_TCM_R = crate::BitReader;
///Field `RME_TCM` writer - RW margin enable input for TCM memories
pub type RME_TCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RM_TCM` reader - External RW margin inputs for TCM memories
pub type RM_TCM_R = crate::FieldReader;
///Field `RM_TCM` writer - External RW margin inputs for TCM memories
pub type RM_TCM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BC1_TCM` reader - Biasing level adjust input recommended for Vnom
pub type BC1_TCM_R = crate::BitReader;
///Field `BC1_TCM` writer - Biasing level adjust input recommended for Vnom
pub type BC1_TCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BC2_TCM` reader - Biasing level adjust input recommended for Vnom + 10%
pub type BC2_TCM_R = crate::BitReader;
///Field `BC2_TCM` writer - Biasing level adjust input recommended for Vnom + 10%
pub type BC2_TCM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RME_CACHE` reader - RW margin enable input for caches memories
pub type RME_CACHE_R = crate::BitReader;
///Field `RME_CACHE` writer - RW margin enable input for caches memories
pub type RME_CACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RM_CACHE` reader - External read/write (RW) margin inputs for caches memories
pub type RM_CACHE_R = crate::FieldReader;
///Field `RM_CACHE` writer - External read/write (RW) margin inputs for caches memories
pub type RM_CACHE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `BC1_CACHE` reader - Biasing level adjust input recommended for Vnom.
pub type BC1_CACHE_R = crate::BitReader;
///Field `BC1_CACHE` writer - Biasing level adjust input recommended for Vnom.
pub type BC1_CACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BC2_CACHE` reader - Biasing level adjust input recommended for Vnom + 10%
pub type BC2_CACHE_R = crate::BitReader;
///Field `BC2_CACHE` writer - Biasing level adjust input recommended for Vnom + 10%
pub type BC2_CACHE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RW margin enable input for TCM memories
    #[inline(always)]
    pub fn rme_tcm(&self) -> RME_TCM_R {
        RME_TCM_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - External RW margin inputs for TCM memories
    #[inline(always)]
    pub fn rm_tcm(&self) -> RM_TCM_R {
        RM_TCM_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 5 - Biasing level adjust input recommended for Vnom
    #[inline(always)]
    pub fn bc1_tcm(&self) -> BC1_TCM_R {
        BC1_TCM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Biasing level adjust input recommended for Vnom + 10%
    #[inline(always)]
    pub fn bc2_tcm(&self) -> BC2_TCM_R {
        BC2_TCM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - RW margin enable input for caches memories
    #[inline(always)]
    pub fn rme_cache(&self) -> RME_CACHE_R {
        RME_CACHE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - External read/write (RW) margin inputs for caches memories
    #[inline(always)]
    pub fn rm_cache(&self) -> RM_CACHE_R {
        RM_CACHE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Biasing level adjust input recommended for Vnom.
    #[inline(always)]
    pub fn bc1_cache(&self) -> BC1_CACHE_R {
        BC1_CACHE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Biasing level adjust input recommended for Vnom + 10%
    #[inline(always)]
    pub fn bc2_cache(&self) -> BC2_CACHE_R {
        BC2_CACHE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CM55RWMCR")
            .field("rme_tcm", &self.rme_tcm())
            .field("rm_tcm", &self.rm_tcm())
            .field("bc1_tcm", &self.bc1_tcm())
            .field("bc2_tcm", &self.bc2_tcm())
            .field("rme_cache", &self.rme_cache())
            .field("rm_cache", &self.rm_cache())
            .field("bc1_cache", &self.bc1_cache())
            .field("bc2_cache", &self.bc2_cache())
            .finish()
    }
}
impl W {
    ///Bit 0 - RW margin enable input for TCM memories
    #[inline(always)]
    pub fn rme_tcm(&mut self) -> RME_TCM_W<'_, CM55RWMCRrs> {
        RME_TCM_W::new(self, 0)
    }
    ///Bits 1:4 - External RW margin inputs for TCM memories
    #[inline(always)]
    pub fn rm_tcm(&mut self) -> RM_TCM_W<'_, CM55RWMCRrs> {
        RM_TCM_W::new(self, 1)
    }
    ///Bit 5 - Biasing level adjust input recommended for Vnom
    #[inline(always)]
    pub fn bc1_tcm(&mut self) -> BC1_TCM_W<'_, CM55RWMCRrs> {
        BC1_TCM_W::new(self, 5)
    }
    ///Bit 6 - Biasing level adjust input recommended for Vnom + 10%
    #[inline(always)]
    pub fn bc2_tcm(&mut self) -> BC2_TCM_W<'_, CM55RWMCRrs> {
        BC2_TCM_W::new(self, 6)
    }
    ///Bit 7 - RW margin enable input for caches memories
    #[inline(always)]
    pub fn rme_cache(&mut self) -> RME_CACHE_W<'_, CM55RWMCRrs> {
        RME_CACHE_W::new(self, 7)
    }
    ///Bits 8:11 - External read/write (RW) margin inputs for caches memories
    #[inline(always)]
    pub fn rm_cache(&mut self) -> RM_CACHE_W<'_, CM55RWMCRrs> {
        RM_CACHE_W::new(self, 8)
    }
    ///Bit 12 - Biasing level adjust input recommended for Vnom.
    #[inline(always)]
    pub fn bc1_cache(&mut self) -> BC1_CACHE_W<'_, CM55RWMCRrs> {
        BC1_CACHE_W::new(self, 12)
    }
    ///Bit 13 - Biasing level adjust input recommended for Vnom + 10%
    #[inline(always)]
    pub fn bc2_cache(&mut self) -> BC2_CACHE_W<'_, CM55RWMCRrs> {
        BC2_CACHE_W::new(self, 13)
    }
}
/**SYSCFG Cortex-CM55 memory RW margin register

You can [`read`](crate::Reg::read) this register and get [`cm55rwmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm55rwmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#SYSCFG:CM55RWMCR)*/
pub struct CM55RWMCRrs;
impl crate::RegisterSpec for CM55RWMCRrs {
    type Ux = u32;
}
///`read()` method returns [`cm55rwmcr::R`](R) reader structure
impl crate::Readable for CM55RWMCRrs {}
///`write(|w| ..)` method takes [`cm55rwmcr::W`](W) writer structure
impl crate::Writable for CM55RWMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CM55RWMCR to value 0x1020
impl crate::Resettable for CM55RWMCRrs {
    const RESET_VALUE: u32 = 0x1020;
}
