///Register `TZC_CID3` reader
pub type R = crate::R<TZC_CID3rs>;
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
        f.debug_struct("TZC_CID3")
            .field("comp_id_3", &self.comp_id_3())
            .finish()
    }
}
/**Component ID 3.

You can [`read`](crate::Reg::read) this register and get [`tzc_cid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:TZC_CID3)*/
pub struct TZC_CID3rs;
impl crate::RegisterSpec for TZC_CID3rs {
    type Ux = u32;
}
///`read()` method returns [`tzc_cid3::R`](R) reader structure
impl crate::Readable for TZC_CID3rs {}
///`reset()` method sets TZC_CID3 to value 0xb1
impl crate::Resettable for TZC_CID3rs {
    const RESET_VALUE: u32 = 0xb1;
}
