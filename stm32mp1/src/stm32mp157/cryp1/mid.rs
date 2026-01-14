///Register `MID` reader
pub type R = crate::R<MIDrs>;
///Field `MID` reader - MID
pub type MID_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - MID
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MID").field("mid", &self.mid()).finish()
    }
}
/**CRYP HW Magic ID

You can [`read`](crate::Reg::read) this register and get [`mid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:MID)*/
pub struct MIDrs;
impl crate::RegisterSpec for MIDrs {
    type Ux = u32;
}
///`read()` method returns [`mid::R`](R) reader structure
impl crate::Readable for MIDrs {}
///`reset()` method sets MID to value 0xa3c5_dd01
impl crate::Resettable for MIDrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
