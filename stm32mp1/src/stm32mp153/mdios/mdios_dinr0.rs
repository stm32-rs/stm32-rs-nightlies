///Register `MDIOS_DINR0` reader
pub type R = crate::R<MDIOS_DINR0rs>;
///Field `DIN` reader - DIN
pub type DIN_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - DIN
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDIOS_DINR0")
            .field("din", &self.din())
            .finish()
    }
}
/**MDIOS input data register

You can [`read`](crate::Reg::read) this register and get [`mdios_dinr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDIOS:MDIOS_DINR0)*/
pub struct MDIOS_DINR0rs;
impl crate::RegisterSpec for MDIOS_DINR0rs {
    type Ux = u32;
}
///`read()` method returns [`mdios_dinr0::R`](R) reader structure
impl crate::Readable for MDIOS_DINR0rs {}
///`reset()` method sets MDIOS_DINR0 to value 0
impl crate::Resettable for MDIOS_DINR0rs {
    const RESET_VALUE: u32 = 0;
}
