///Register `CSGCMCCM4R` reader
pub type R = crate::R<CSGCMCCM4Rrs>;
///Register `CSGCMCCM4R` writer
pub type W = crate::W<CSGCMCCM4Rrs>;
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
        f.debug_struct("CSGCMCCM4R")
            .field("csgcmccm", &self.csgcmccm())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap for GCM/GMAC and CCM modes
    #[inline(always)]
    pub fn csgcmccm(&mut self) -> CSGCMCCM_W<CSGCMCCM4Rrs> {
        CSGCMCCM_W::new(self, 0)
    }
}
/**CRYP context swap GCM-CCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmccm4r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm4r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#CRYP:CSGCMCCM4R)*/
pub struct CSGCMCCM4Rrs;
impl crate::RegisterSpec for CSGCMCCM4Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccm4r::R`](R) reader structure
impl crate::Readable for CSGCMCCM4Rrs {}
///`write(|w| ..)` method takes [`csgcmccm4r::W`](W) writer structure
impl crate::Writable for CSGCMCCM4Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCMCCM4R to value 0
impl crate::Resettable for CSGCMCCM4Rrs {}
