///Register `UR18` reader
pub type R = crate::R<UR18rs>;
///Field `CPU_FREQ_BOOST` reader - CPU maximum frequency boost
pub type CPU_FREQ_BOOST_R = crate::BitReader;
impl R {
    ///Bit 0 - CPU maximum frequency boost
    #[inline(always)]
    pub fn cpu_freq_boost(&self) -> CPU_FREQ_BOOST_R {
        CPU_FREQ_BOOST_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UR18")
            .field("cpu_freq_boost", &self.cpu_freq_boost())
            .finish()
    }
}
/**SYSCFG user register 18

You can [`read`](crate::Reg::read) this register and get [`ur18::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#SYSCFG:UR18)*/
pub struct UR18rs;
impl crate::RegisterSpec for UR18rs {
    type Ux = u32;
}
///`read()` method returns [`ur18::R`](R) reader structure
impl crate::Readable for UR18rs {}
///`reset()` method sets UR18 to value 0
impl crate::Resettable for UR18rs {}
