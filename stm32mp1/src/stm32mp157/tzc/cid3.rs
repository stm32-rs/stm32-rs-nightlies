///Register `CID3` reader
pub type R = crate::R<CID3rs>;
///Field `COMP_ID_3` reader - COMP_ID_3
pub type COMP_ID_3_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - COMP_ID_3
    #[inline(always)]
    pub fn comp_id_3(&self) -> COMP_ID_3_R {
        COMP_ID_3_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CID3")
            .field("comp_id_3", &self.comp_id_3())
            .finish()
    }
}
/**Component ID 3.

You can [`read`](crate::Reg::read) this register and get [`cid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:CID3)*/
pub struct CID3rs;
impl crate::RegisterSpec for CID3rs {
    type Ux = u32;
}
///`read()` method returns [`cid3::R`](R) reader structure
impl crate::Readable for CID3rs {}
///`reset()` method sets CID3 to value 0xb1
impl crate::Resettable for CID3rs {
    const RESET_VALUE: u32 = 0xb1;
}
