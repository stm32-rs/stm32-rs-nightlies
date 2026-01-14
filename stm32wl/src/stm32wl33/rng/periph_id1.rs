///Register `PeriphID1` reader
pub type R = crate::R<PERIPH_ID1rs>;
///Field `PartNumber1` reader - These bits are read back as 0x05
pub type PART_NUMBER1_R = crate::FieldReader;
///Field `Designer0` reader - These bits are read back as 0x00
pub type DESIGNER0_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - These bits are read back as 0x05
    #[inline(always)]
    pub fn part_number1(&self) -> PART_NUMBER1_R {
        PART_NUMBER1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - These bits are read back as 0x00
    #[inline(always)]
    pub fn designer0(&self) -> DESIGNER0_R {
        DESIGNER0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PeriphID1")
            .field("part_number1", &self.part_number1())
            .field("designer0", &self.designer0())
            .finish()
    }
}
/**RNGPeriphID1 register

You can [`read`](crate::Reg::read) this register and get [`periph_id1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PeriphID1)*/
pub struct PERIPH_ID1rs;
impl crate::RegisterSpec for PERIPH_ID1rs {
    type Ux = u32;
}
///`read()` method returns [`periph_id1::R`](R) reader structure
impl crate::Readable for PERIPH_ID1rs {}
///`reset()` method sets PeriphID1 to value 0x05
impl crate::Resettable for PERIPH_ID1rs {
    const RESET_VALUE: u32 = 0x05;
}
