///Register `DFSDM_FLT0AWSR` reader
pub type R = crate::R<DFSDM_FLT0AWSRrs>;
///Field `AWLTF` reader - Analog watchdog low threshold flag
pub type AWLTF_R = crate::FieldReader;
///Field `AWHTF` reader - Analog watchdog high threshold flag
pub type AWHTF_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Analog watchdog low threshold flag
    #[inline(always)]
    pub fn awltf(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Analog watchdog high threshold flag
    #[inline(always)]
    pub fn awhtf(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFSDM_FLT0AWSR")
            .field("awhtf", &self.awhtf())
            .field("awltf", &self.awltf())
            .finish()
    }
}
/**analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#DFSDM1:DFSDM_FLT0AWSR)*/
pub struct DFSDM_FLT0AWSRrs;
impl crate::RegisterSpec for DFSDM_FLT0AWSRrs {
    type Ux = u32;
}
///`read()` method returns [`dfsdm_flt0awsr::R`](R) reader structure
impl crate::Readable for DFSDM_FLT0AWSRrs {}
///`reset()` method sets DFSDM_FLT0AWSR to value 0
impl crate::Resettable for DFSDM_FLT0AWSRrs {
    const RESET_VALUE: u32 = 0;
}
