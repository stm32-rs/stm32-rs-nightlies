///Register `JTAG_ID` reader
pub type R = crate::R<JTAG_IDrs>;
///Field `MANUF_ID` reader - Manufacturer ID
pub type MANUF_ID_R = crate::FieldReader<u16>;
///Field `PART_NUMBER` reader - Part number
pub type PART_NUMBER_R = crate::FieldReader<u16>;
///Field `VERSION_NUMBER` reader - Version
pub type VERSION_NUMBER_R = crate::FieldReader;
impl R {
    ///Bits 1:11 - Manufacturer ID
    #[inline(always)]
    pub fn manuf_id(&self) -> MANUF_ID_R {
        MANUF_ID_R::new(((self.bits >> 1) & 0x07ff) as u16)
    }
    ///Bits 12:27 - Part number
    #[inline(always)]
    pub fn part_number(&self) -> PART_NUMBER_R {
        PART_NUMBER_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    ///Bits 28:31 - Version
    #[inline(always)]
    pub fn version_number(&self) -> VERSION_NUMBER_R {
        VERSION_NUMBER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JTAG_ID")
            .field("manuf_id", &self.manuf_id())
            .field("part_number", &self.part_number())
            .field("version_number", &self.version_number())
            .finish()
    }
}
/**JTAG_ID register

You can [`read`](crate::Reg::read) this register and get [`jtag_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#SYSTEM_CTRL:JTAG_ID)*/
pub struct JTAG_IDrs;
impl crate::RegisterSpec for JTAG_IDrs {
    type Ux = u32;
}
///`read()` method returns [`jtag_id::R`](R) reader structure
impl crate::Readable for JTAG_IDrs {}
///`reset()` method sets JTAG_ID to value 0x0201_e041
impl crate::Resettable for JTAG_IDrs {
    const RESET_VALUE: u32 = 0x0201_e041;
}
