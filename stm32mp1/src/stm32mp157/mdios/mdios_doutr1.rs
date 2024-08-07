///Register `MDIOS_DOUTR1` reader
pub type R = crate::R<MDIOS_DOUTR1rs>;
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
        f.debug_struct("MDIOS_DOUTR1")
            .field("dout", &self.dout())
            .finish()
    }
}
/**MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_doutr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDIOS:MDIOS_DOUTR1)*/
pub struct MDIOS_DOUTR1rs;
impl crate::RegisterSpec for MDIOS_DOUTR1rs {
    type Ux = u32;
}
///`read()` method returns [`mdios_doutr1::R`](R) reader structure
impl crate::Readable for MDIOS_DOUTR1rs {}
///`reset()` method sets MDIOS_DOUTR1 to value 0
impl crate::Resettable for MDIOS_DOUTR1rs {
    const RESET_VALUE: u32 = 0;
}
