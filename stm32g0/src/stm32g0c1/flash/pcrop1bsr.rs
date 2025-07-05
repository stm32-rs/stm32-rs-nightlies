///Register `PCROP1BSR` reader
pub type R = crate::R<PCROP1BSRrs>;
///Field `PCROP1B_STRT` reader - PCROP1B area start offset
pub type PCROP1B_STRT_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PCROP1B area start offset
    #[inline(always)]
    pub fn pcrop1b_strt(&self) -> PCROP1B_STRT_R {
        PCROP1B_STRT_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP1BSR")
            .field("pcrop1b_strt", &self.pcrop1b_strt())
            .finish()
    }
}
/**Flash PCROP zone B Start address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1bsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#FLASH:PCROP1BSR)*/
pub struct PCROP1BSRrs;
impl crate::RegisterSpec for PCROP1BSRrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1bsr::R`](R) reader structure
impl crate::Readable for PCROP1BSRrs {}
///`reset()` method sets PCROP1BSR to value 0xf000_0000
impl crate::Resettable for PCROP1BSRrs {
    const RESET_VALUE: u32 = 0xf000_0000;
}
