///Register `PERIPH_ID_0` reader
pub type R = crate::R<PERIPH_ID_0rs>;
///Field `PARTNUM` reader - Peripheral part number bits 0 to 7
pub type PARTNUM_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Peripheral part number bits 0 to 7
    #[inline(always)]
    pub fn partnum(&self) -> PARTNUM_R {
        PARTNUM_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIPH_ID_0")
            .field("partnum", &self.partnum())
            .finish()
    }
}
/**AXI interconnect - peripheral ID0 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#AXI:PERIPH_ID_0)*/
pub struct PERIPH_ID_0rs;
impl crate::RegisterSpec for PERIPH_ID_0rs {
    type Ux = u32;
}
///`read()` method returns [`periph_id_0::R`](R) reader structure
impl crate::Readable for PERIPH_ID_0rs {}
///`reset()` method sets PERIPH_ID_0 to value 0x04
impl crate::Resettable for PERIPH_ID_0rs {
    const RESET_VALUE: u32 = 0x04;
}
