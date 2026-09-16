///Register `CSGCM0R` reader
pub type R = crate::R<CSGCM0Rrs>;
///Register `CSGCM0R` writer
pub type W = crate::W<CSGCM0Rrs>;
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
        f.debug_struct("CSGCM0R")
            .field("csgcm", &self.csgcm())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap for GCM/GMAC modes
    #[inline(always)]
    pub fn csgcm(&mut self) -> CSGCM_W<'_, CSGCM0Rrs> {
        CSGCM_W::new(self, 0)
    }
}
/**CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcm0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCM0R)*/
pub struct CSGCM0Rrs;
impl crate::RegisterSpec for CSGCM0Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcm0r::R`](R) reader structure
impl crate::Readable for CSGCM0Rrs {}
///`write(|w| ..)` method takes [`csgcm0r::W`](W) writer structure
impl crate::Writable for CSGCM0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCM0R to value 0
impl crate::Resettable for CSGCM0Rrs {}
