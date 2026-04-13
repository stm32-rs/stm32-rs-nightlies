///Register `CSGCMCCM6R` reader
pub type R = crate::R<CSGCMCCM6Rrs>;
///Register `CSGCMCCM6R` writer
pub type W = crate::W<CSGCMCCM6Rrs>;
///Field `CSGCMCCM` reader - Context swap for GCM/GMAC and CCM modes
pub type CSGCMCCM_R = crate::FieldReader<u32>;
///Field `CSGCMCCM` writer - Context swap for GCM/GMAC and CCM modes
pub type CSGCMCCM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap for GCM/GMAC and CCM modes
    #[inline(always)]
    pub fn csgcmccm(&self) -> CSGCMCCM_R {
        CSGCMCCM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMCCM6R")
            .field("csgcmccm", &self.csgcmccm())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap for GCM/GMAC and CCM modes
    #[inline(always)]
    pub fn csgcmccm(&mut self) -> CSGCMCCM_W<'_, CSGCMCCM6Rrs> {
        CSGCMCCM_W::new(self, 0)
    }
}
/**CRYP context swap GCM-CCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmccm6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#CRYP:CSGCMCCM6R)*/
pub struct CSGCMCCM6Rrs;
impl crate::RegisterSpec for CSGCMCCM6Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccm6r::R`](R) reader structure
impl crate::Readable for CSGCMCCM6Rrs {}
///`write(|w| ..)` method takes [`csgcmccm6r::W`](W) writer structure
impl crate::Writable for CSGCMCCM6Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCMCCM6R to value 0
impl crate::Resettable for CSGCMCCM6Rrs {}
