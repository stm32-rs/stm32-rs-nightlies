///Register `MDIOS_DOUTR24` reader
pub type R = crate::R<MDIOS_DOUTR24rs>;
///Field `DOUT` reader - DOUT
pub type DOUT_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - DOUT
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIOS_DOUTR24")
            .field("dout", &self.dout())
            .finish()
    }
}
/**MDIOS output data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr24::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR24)*/
pub struct MDIOS_DOUTR24rs;
impl crate::RegisterSpec for MDIOS_DOUTR24rs {
    type Ux = u32;
}
///`read()` method returns [`mdios_doutr24::R`](R) reader structure
impl crate::Readable for MDIOS_DOUTR24rs {}
///`reset()` method sets MDIOS_DOUTR24 to value 0
impl crate::Resettable for MDIOS_DOUTR24rs {
    const RESET_VALUE: u32 = 0;
}
