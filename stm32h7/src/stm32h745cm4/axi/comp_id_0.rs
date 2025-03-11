///Register `COMP_ID_0` reader
pub type R = crate::R<COMP_ID_0rs>;
///Field `PREAMBLE` reader - Preamble bits 0 to 7
pub type PREAMBLE_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Preamble bits 0 to 7
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_ID_0")
            .field("preamble", &self.preamble())
            .finish()
    }
}
/**AXI interconnect - component ID0 register

You can [`read`](crate::Reg::read) this register and get [`comp_id_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#AXI:COMP_ID_0)*/
pub struct COMP_ID_0rs;
impl crate::RegisterSpec for COMP_ID_0rs {
    type Ux = u32;
}
///`read()` method returns [`comp_id_0::R`](R) reader structure
impl crate::Readable for COMP_ID_0rs {}
///`reset()` method sets COMP_ID_0 to value 0x04
impl crate::Resettable for COMP_ID_0rs {
    const RESET_VALUE: u32 = 0x04;
}
