///Register `CSGCM6R` reader
pub type R = crate::R<CSGCM6Rrs>;
///Register `CSGCM6R` writer
pub type W = crate::W<CSGCM6Rrs>;
///Field `CSGCM` reader - Context swap for GCM/GMAC modes
pub type CSGCM_R = crate::FieldReader<u32>;
///Field `CSGCM` writer - Context swap for GCM/GMAC modes
pub type CSGCM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap for GCM/GMAC modes
    #[inline(always)]
    pub fn csgcm(&self) -> CSGCM_R {
        CSGCM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCM6R")
            .field("csgcm", &self.csgcm())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap for GCM/GMAC modes
    #[inline(always)]
    pub fn csgcm(&mut self) -> CSGCM_W<CSGCM6Rrs> {
        CSGCM_W::new(self, 0)
    }
}
/**CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcm6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCM6R)*/
pub struct CSGCM6Rrs;
impl crate::RegisterSpec for CSGCM6Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcm6r::R`](R) reader structure
impl crate::Readable for CSGCM6Rrs {}
///`write(|w| ..)` method takes [`csgcm6r::W`](W) writer structure
impl crate::Writable for CSGCM6Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCM6R to value 0
impl crate::Resettable for CSGCM6Rrs {}
