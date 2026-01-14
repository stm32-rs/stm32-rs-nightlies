///Register `WRPROT1` reader
pub type R = crate::R<WRPROT1rs>;
///Field `WRPROT1` reader - Write protection
pub type WRPROT1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Write protection
    #[inline(always)]
    pub fn wrprot1(&self) -> WRPROT1_R {
        WRPROT1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPROT1")
            .field("wrprot1", &self.wrprot1())
            .finish()
    }
}
/**Write protection register

You can [`read`](crate::Reg::read) this register and get [`wrprot1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:WRPROT1)*/
pub struct WRPROT1rs;
impl crate::RegisterSpec for WRPROT1rs {
    type Ux = u32;
}
///`read()` method returns [`wrprot1::R`](R) reader structure
impl crate::Readable for WRPROT1rs {}
///`reset()` method sets WRPROT1 to value 0
impl crate::Resettable for WRPROT1rs {}
