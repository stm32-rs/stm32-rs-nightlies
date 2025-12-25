///Register `PeriphID2` reader
pub type R = crate::R<PERIPH_ID2rs>;
///Field `Designer1` reader - These bits are read back as 0x08
pub type DESIGNER1_R = crate::FieldReader;
///Field `Revision` reader - These bits are read back as 0x02
pub type REVISION_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - These bits are read back as 0x08
    #[inline(always)]
    pub fn designer1(&self) -> DESIGNER1_R {
        DESIGNER1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - These bits are read back as 0x02
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PeriphID2")
            .field("designer1", &self.designer1())
            .field("revision", &self.revision())
            .finish()
    }
}
/**RNGPeriphID2 register

You can [`read`](crate::Reg::read) this register and get [`periph_id2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RNG:PeriphID2)*/
pub struct PERIPH_ID2rs;
impl crate::RegisterSpec for PERIPH_ID2rs {
    type Ux = u32;
}
///`read()` method returns [`periph_id2::R`](R) reader structure
impl crate::Readable for PERIPH_ID2rs {}
///`reset()` method sets PeriphID2 to value 0x28
impl crate::Resettable for PERIPH_ID2rs {
    const RESET_VALUE: u32 = 0x28;
}
