///Register `HASH_MID` reader
pub type R = crate::R<HASH_MIDrs>;
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
        f.debug_struct("HASH_MID")
            .field("mid", &self.mid())
            .finish()
    }
}
/**HASH Hardware Magic ID

You can [`read`](crate::Reg::read) this register and get [`hash_mid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HASH2:HASH_MID)*/
pub struct HASH_MIDrs;
impl crate::RegisterSpec for HASH_MIDrs {
    type Ux = u32;
}
///`read()` method returns [`hash_mid::R`](R) reader structure
impl crate::Readable for HASH_MIDrs {}
///`reset()` method sets HASH_MID to value 0xa3c5_dd01
impl crate::Resettable for HASH_MIDrs {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
