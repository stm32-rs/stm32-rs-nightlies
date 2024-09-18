///Register `MDIOS_DOUTR0` reader
pub type R = crate::R<MDIOS_DOUTR0rs>;
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
        f.debug_struct("MDIOS_DOUTR0")
            .field("dout", &self.dout())
            .finish()
    }
}
/**MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDIOS:MDIOS_DOUTR0)*/
pub struct MDIOS_DOUTR0rs;
impl crate::RegisterSpec for MDIOS_DOUTR0rs {
    type Ux = u32;
}
///`read()` method returns [`mdios_doutr0::R`](R) reader structure
impl crate::Readable for MDIOS_DOUTR0rs {}
///`reset()` method sets MDIOS_DOUTR0 to value 0
impl crate::Resettable for MDIOS_DOUTR0rs {
    const RESET_VALUE: u32 = 0;
}
