///Register `WRPROT2` reader
pub type R = crate::R<WRPROT2rs>;
///Field `WRPROT2` reader - Write protection
pub type WRPROT2_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Write protection
    #[inline(always)]
    pub fn wrprot2(&self) -> WRPROT2_R {
        WRPROT2_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRPROT2")
            .field("wrprot2", &self.wrprot2())
            .finish()
    }
}
/**Write protection register

You can [`read`](crate::Reg::read) this register and get [`wrprot2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#FLASH:WRPROT2)*/
pub struct WRPROT2rs;
impl crate::RegisterSpec for WRPROT2rs {
    type Ux = u32;
}
///`read()` method returns [`wrprot2::R`](R) reader structure
impl crate::Readable for WRPROT2rs {}
///`reset()` method sets WRPROT2 to value 0
impl crate::Resettable for WRPROT2rs {}
