///Register `SPISR1` reader
pub type R = crate::R<SPISR1rs>;
///Field `SPISR1` reader - SPISR1
pub type SPISR1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SPISR1
    #[inline(always)]
    pub fn spisr1(&self) -> SPISR1_R {
        SPISR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPISR1")
            .field("spisr1", &self.spisr1())
            .finish()
    }
}
/**For interrupts ID = SPI number+32, from SPI \[x*32+31\] to SPI \[x*32\]

You can [`read`](crate::Reg::read) this register and get [`spisr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPISR1)*/
pub struct SPISR1rs;
impl crate::RegisterSpec for SPISR1rs {
    type Ux = u32;
}
///`read()` method returns [`spisr1::R`](R) reader structure
impl crate::Readable for SPISR1rs {}
///`reset()` method sets SPISR1 to value 0
impl crate::Resettable for SPISR1rs {}
