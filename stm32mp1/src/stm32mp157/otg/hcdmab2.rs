///Register `HCDMAB2` reader
pub type R = crate::R<HCDMAB2rs>;
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
        f.debug_struct("HCDMAB2")
            .field("hcdmab", &self.hcdmab())
            .finish()
    }
}
/**OTG host channel-n DMA address buffer register

You can [`read`](crate::Reg::read) this register and get [`hcdmab2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#OTG:HCDMAB2)*/
pub struct HCDMAB2rs;
impl crate::RegisterSpec for HCDMAB2rs {
    type Ux = u32;
}
///`read()` method returns [`hcdmab2::R`](R) reader structure
impl crate::Readable for HCDMAB2rs {}
///`reset()` method sets HCDMAB2 to value 0
impl crate::Resettable for HCDMAB2rs {}
