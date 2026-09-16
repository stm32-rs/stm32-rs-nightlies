///Register `HCDMAB3` reader
pub type R = crate::R<HCDMAB3rs>;
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
        f.debug_struct("HCDMAB3")
            .field("hcdmab", &self.hcdmab())
            .finish()
    }
}
/**OTG host channel-n DMA address buffer register

You can [`read`](crate::Reg::read) this register and get [`hcdmab3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#OTG:HCDMAB3)*/
pub struct HCDMAB3rs;
impl crate::RegisterSpec for HCDMAB3rs {
    type Ux = u32;
}
///`read()` method returns [`hcdmab3::R`](R) reader structure
impl crate::Readable for HCDMAB3rs {}
///`reset()` method sets HCDMAB3 to value 0
impl crate::Resettable for HCDMAB3rs {}
