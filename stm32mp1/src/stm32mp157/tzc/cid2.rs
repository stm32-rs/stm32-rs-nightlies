///Register `CID2` reader
pub type R = crate::R<CID2rs>;
///Field `COMP_ID_2` reader - COMP_ID_2
pub type COMP_ID_2_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - COMP_ID_2
    #[inline(always)]
    pub fn comp_id_2(&self) -> COMP_ID_2_R {
        COMP_ID_2_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CID2")
            .field("comp_id_2", &self.comp_id_2())
            .finish()
    }
}
/**Component ID 2.

You can [`read`](crate::Reg::read) this register and get [`cid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:CID2)*/
pub struct CID2rs;
impl crate::RegisterSpec for CID2rs {
    type Ux = u32;
}
///`read()` method returns [`cid2::R`](R) reader structure
impl crate::Readable for CID2rs {}
///`reset()` method sets CID2 to value 0x05
impl crate::Resettable for CID2rs {
    const RESET_VALUE: u32 = 0x05;
}
