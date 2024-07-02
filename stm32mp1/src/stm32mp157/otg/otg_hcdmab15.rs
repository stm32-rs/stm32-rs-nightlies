///Register `OTG_HCDMAB15` reader
pub type R = crate::R<OTG_HCDMAB15rs>;
///Field `HCDMAB` reader - HCDMAB
pub type HCDMAB_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - HCDMAB
    #[inline(always)]
    pub fn hcdmab(&self) -> HCDMAB_R {
        HCDMAB_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HCDMAB15")
            .field("hcdmab", &self.hcdmab())
            .finish()
    }
}
/**OTG host channel-n DMA address buffer register

You can [`read`](crate::Reg::read) this register and get [`otg_hcdmab15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#OTG:OTG_HCDMAB15)*/
pub struct OTG_HCDMAB15rs;
impl crate::RegisterSpec for OTG_HCDMAB15rs {
    type Ux = u32;
}
///`read()` method returns [`otg_hcdmab15::R`](R) reader structure
impl crate::Readable for OTG_HCDMAB15rs {}
///`reset()` method sets OTG_HCDMAB15 to value 0
impl crate::Resettable for OTG_HCDMAB15rs {
    const RESET_VALUE: u32 = 0;
}
