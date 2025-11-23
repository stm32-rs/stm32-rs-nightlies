///Register `PeriphID3` reader
pub type R = crate::R<PERIPH_ID3rs>;
///Field `Configuration` reader - These bits are read back as 0x00
pub type CONFIGURATION_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - These bits are read back as 0x00
    #[inline(always)]
    pub fn configuration(&self) -> CONFIGURATION_R {
        CONFIGURATION_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PeriphID3")
            .field("configuration", &self.configuration())
            .finish()
    }
}
/**RNGPeriphID3 register

You can [`read`](crate::Reg::read) this register and get [`periph_id3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PeriphID3)*/
pub struct PERIPH_ID3rs;
impl crate::RegisterSpec for PERIPH_ID3rs {
    type Ux = u32;
}
///`read()` method returns [`periph_id3::R`](R) reader structure
impl crate::Readable for PERIPH_ID3rs {}
///`reset()` method sets PeriphID3 to value 0
impl crate::Resettable for PERIPH_ID3rs {}
