///Register `CID1` reader
pub type R = crate::R<CID1rs>;
///Field `COMP_ID_1` reader - COMP_ID_1
pub type COMP_ID_1_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - COMP_ID_1
    #[inline(always)]
    pub fn comp_id_1(&self) -> COMP_ID_1_R {
        COMP_ID_1_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CID1")
            .field("comp_id_1", &self.comp_id_1())
            .finish()
    }
}
/**Component ID 1.

You can [`read`](crate::Reg::read) this register and get [`cid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:CID1)*/
pub struct CID1rs;
impl crate::RegisterSpec for CID1rs {
    type Ux = u32;
}
///`read()` method returns [`cid1::R`](R) reader structure
impl crate::Readable for CID1rs {}
///`reset()` method sets CID1 to value 0xf0
impl crate::Resettable for CID1rs {
    const RESET_VALUE: u32 = 0xf0;
}
