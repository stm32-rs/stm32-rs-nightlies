///Register `FLT0AWSR` reader
pub type R = crate::R<FLT0AWSRrs>;
///Field `AWLTF` reader - AWLTF
pub type AWLTF_R = crate::FieldReader;
///Field `AWHTF` reader - AWHTF
pub type AWHTF_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - AWLTF
    #[inline(always)]
    pub fn awltf(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - AWHTF
    #[inline(always)]
    pub fn awhtf(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLT0AWSR")
            .field("awltf", &self.awltf())
            .field("awhtf", &self.awhtf())
            .finish()
    }
}
/**DFSDM filter 0 analog watchdog status register

You can [`read`](crate::Reg::read) this register and get [`flt0awsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DFSDM1:FLT0AWSR)*/
pub struct FLT0AWSRrs;
impl crate::RegisterSpec for FLT0AWSRrs {
    type Ux = u32;
}
///`read()` method returns [`flt0awsr::R`](R) reader structure
impl crate::Readable for FLT0AWSRrs {}
///`reset()` method sets FLT0AWSR to value 0
impl crate::Resettable for FLT0AWSRrs {}
