///Register `MANAESSTATREG` reader
pub type R = crate::R<MANAESSTATREGrs>;
///Field `BUSY` reader - AES manual encryption busy status
pub type BUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - AES manual encryption busy status
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MANAESSTATREG")
            .field("busy", &self.busy())
            .finish()
    }
}
/**MANAESSTATREG register

You can [`read`](crate::Reg::read) this register and get [`manaesstatreg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#BLUE:MANAESSTATREG)*/
pub struct MANAESSTATREGrs;
impl crate::RegisterSpec for MANAESSTATREGrs {
    type Ux = u32;
}
///`read()` method returns [`manaesstatreg::R`](R) reader structure
impl crate::Readable for MANAESSTATREGrs {}
///`reset()` method sets MANAESSTATREG to value 0
impl crate::Resettable for MANAESSTATREGrs {}
