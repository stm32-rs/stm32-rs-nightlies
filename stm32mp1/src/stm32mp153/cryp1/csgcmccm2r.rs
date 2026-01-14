///Register `CSGCMCCM2R` reader
pub type R = crate::R<CSGCMCCM2Rrs>;
///Register `CSGCMCCM2R` writer
pub type W = crate::W<CSGCMCCM2Rrs>;
///Field `CSGCMCCM2` reader - CSGCMCCM2
pub type CSGCMCCM2_R = crate::FieldReader<u32>;
///Field `CSGCMCCM2` writer - CSGCMCCM2
pub type CSGCMCCM2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCMCCM2
    #[inline(always)]
    pub fn csgcmccm2(&self) -> CSGCMCCM2_R {
        CSGCMCCM2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMCCM2R")
            .field("csgcmccm2", &self.csgcmccm2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCMCCM2
    #[inline(always)]
    pub fn csgcmccm2(&mut self) -> CSGCMCCM2_W<'_, CSGCMCCM2Rrs> {
        CSGCMCCM2_W::new(self, 0)
    }
}
/**These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.

You can [`read`](crate::Reg::read) this register and get [`csgcmccm2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CRYP1:CSGCMCCM2R)*/
pub struct CSGCMCCM2Rrs;
impl crate::RegisterSpec for CSGCMCCM2Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccm2r::R`](R) reader structure
impl crate::Readable for CSGCMCCM2Rrs {}
///`write(|w| ..)` method takes [`csgcmccm2r::W`](W) writer structure
impl crate::Writable for CSGCMCCM2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCMCCM2R to value 0
impl crate::Resettable for CSGCMCCM2Rrs {}
