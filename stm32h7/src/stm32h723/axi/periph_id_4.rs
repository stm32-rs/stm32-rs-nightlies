///Register `PERIPH_ID_4` reader
pub type R = crate::R<PERIPH_ID_4rs>;
///Field `JEP106CON` reader - JEP106 continuation code
pub type JEP106CON_R = crate::FieldReader;
///Field `KCOUNT4` reader - Register file size
pub type KCOUNT4_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - JEP106 continuation code
    #[inline(always)]
    pub fn jep106con(&self) -> JEP106CON_R {
        JEP106CON_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Register file size
    #[inline(always)]
    pub fn kcount4(&self) -> KCOUNT4_R {
        KCOUNT4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIPH_ID_4")
            .field("jep106con", &self.jep106con())
            .field("kcount4", &self.kcount4())
            .finish()
    }
}
/**AXI interconnect - peripheral ID4 register

You can [`read`](crate::Reg::read) this register and get [`periph_id_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#AXI:PERIPH_ID_4)*/
pub struct PERIPH_ID_4rs;
impl crate::RegisterSpec for PERIPH_ID_4rs {
    type Ux = u32;
}
///`read()` method returns [`periph_id_4::R`](R) reader structure
impl crate::Readable for PERIPH_ID_4rs {}
///`reset()` method sets PERIPH_ID_4 to value 0x04
impl crate::Resettable for PERIPH_ID_4rs {
    const RESET_VALUE: u32 = 0x04;
}
