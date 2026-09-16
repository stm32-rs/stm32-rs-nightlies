///Register `HCDMAB9` reader
pub type R = crate::R<HCDMAB9rs>;
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
        f.debug_struct("HCDMAB9")
            .field("hcdmab", &self.hcdmab())
            .finish()
    }
}
/**OTG host channel-n DMA address buffer register

You can [`read`](crate::Reg::read) this register and get [`hcdmab9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:HCDMAB9)*/
pub struct HCDMAB9rs;
impl crate::RegisterSpec for HCDMAB9rs {
    type Ux = u32;
}
///`read()` method returns [`hcdmab9::R`](R) reader structure
impl crate::Readable for HCDMAB9rs {}
///`reset()` method sets HCDMAB9 to value 0
impl crate::Resettable for HCDMAB9rs {}
