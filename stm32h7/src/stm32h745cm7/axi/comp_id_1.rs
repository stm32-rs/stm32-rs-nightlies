///Register `COMP_ID_1` reader
pub type R = crate::R<COMP_ID_1rs>;
///Field `PREAMBLE` reader - Preamble bits 8 to 11
pub type PREAMBLE_R = crate::FieldReader;
///Field `CLASS` reader - Component class
pub type CLASS_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - Preamble bits 8 to 11
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Component class
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_ID_1")
            .field("preamble", &self.preamble())
            .field("class", &self.class())
            .finish()
    }
}
/**AXI interconnect - component ID1 register

You can [`read`](crate::Reg::read) this register and get [`comp_id_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#AXI:COMP_ID_1)*/
pub struct COMP_ID_1rs;
impl crate::RegisterSpec for COMP_ID_1rs {
    type Ux = u32;
}
///`read()` method returns [`comp_id_1::R`](R) reader structure
impl crate::Readable for COMP_ID_1rs {}
///`reset()` method sets COMP_ID_1 to value 0x04
impl crate::Resettable for COMP_ID_1rs {
    const RESET_VALUE: u32 = 0x04;
}
