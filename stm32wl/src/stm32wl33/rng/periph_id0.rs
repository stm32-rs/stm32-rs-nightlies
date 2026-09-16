///Register `PeriphID0` reader
pub type R = crate::R<PERIPH_ID0rs>;
///Field `PartNumber0` reader - These bits are read back as 0xE1
pub type PART_NUMBER0_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - These bits are read back as 0xE1
    #[inline(always)]
    pub fn part_number0(&self) -> PART_NUMBER0_R {
        PART_NUMBER0_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PeriphID0")
            .field("part_number0", &self.part_number0())
            .finish()
    }
}
/**RNGPeriphID0 register

You can [`read`](crate::Reg::read) this register and get [`periph_id0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PeriphID0)*/
pub struct PERIPH_ID0rs;
impl crate::RegisterSpec for PERIPH_ID0rs {
    type Ux = u32;
}
///`read()` method returns [`periph_id0::R`](R) reader structure
impl crate::Readable for PERIPH_ID0rs {}
///`reset()` method sets PeriphID0 to value 0xe1
impl crate::Resettable for PERIPH_ID0rs {
    const RESET_VALUE: u32 = 0xe1;
}
