///Register `CRYP_CSGCMCCM3R` reader
pub type R = crate::R<CRYP_CSGCMCCM3Rrs>;
///Register `CRYP_CSGCMCCM3R` writer
pub type W = crate::W<CRYP_CSGCMCCM3Rrs>;
///Field `CSGCMCCM3` reader - CSGCMCCM3
pub type CSGCMCCM3_R = crate::FieldReader<u32>;
///Field `CSGCMCCM3` writer - CSGCMCCM3
pub type CSGCMCCM3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCMCCM3
    #[inline(always)]
    pub fn csgcmccm3(&self) -> CSGCMCCM3_R {
        CSGCMCCM3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRYP_CSGCMCCM3R")
            .field("csgcmccm3", &self.csgcmccm3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCMCCM3
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm3(&mut self) -> CSGCMCCM3_W<CRYP_CSGCMCCM3Rrs> {
        CSGCMCCM3_W::new(self, 0)
    }
}
/**These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.

You can [`read`](crate::Reg::read) this register and get [`cryp_csgcmccm3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cryp_csgcmccm3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CRYP_CSGCMCCM3R)*/
pub struct CRYP_CSGCMCCM3Rrs;
impl crate::RegisterSpec for CRYP_CSGCMCCM3Rrs {
    type Ux = u32;
}
///`read()` method returns [`cryp_csgcmccm3r::R`](R) reader structure
impl crate::Readable for CRYP_CSGCMCCM3Rrs {}
///`write(|w| ..)` method takes [`cryp_csgcmccm3r::W`](W) writer structure
impl crate::Writable for CRYP_CSGCMCCM3Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CRYP_CSGCMCCM3R to value 0
impl crate::Resettable for CRYP_CSGCMCCM3Rrs {
    const RESET_VALUE: u32 = 0;
}
