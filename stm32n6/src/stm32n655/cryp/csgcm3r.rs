///Register `CSGCM3R` reader
pub type R = crate::R<CSGCM3Rrs>;
///Register `CSGCM3R` writer
pub type W = crate::W<CSGCM3Rrs>;
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
        f.debug_struct("CSGCM3R")
            .field("csgcm", &self.csgcm())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap for GCM/GMAC modes
    #[inline(always)]
    pub fn csgcm(&mut self) -> CSGCM_W<'_, CSGCM3Rrs> {
        CSGCM_W::new(self, 0)
    }
}
/**CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcm3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCM3R)*/
pub struct CSGCM3Rrs;
impl crate::RegisterSpec for CSGCM3Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcm3r::R`](R) reader structure
impl crate::Readable for CSGCM3Rrs {}
///`write(|w| ..)` method takes [`csgcm3r::W`](W) writer structure
impl crate::Writable for CSGCM3Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCM3R to value 0
impl crate::Resettable for CSGCM3Rrs {}
