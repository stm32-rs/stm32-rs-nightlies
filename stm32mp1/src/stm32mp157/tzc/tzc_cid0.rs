///Register `TZC_CID0` reader
pub type R = crate::R<TZC_CID0rs>;
///Field `COMP_ID_0` reader - COMP_ID_0
pub type COMP_ID_0_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - COMP_ID_0
    #[inline(always)]
    pub fn comp_id_0(&self) -> COMP_ID_0_R {
        COMP_ID_0_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZC_CID0")
            .field("comp_id_0", &self.comp_id_0())
            .finish()
    }
}
/**Component ID 0.

You can [`read`](crate::Reg::read) this register and get [`tzc_cid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:TZC_CID0)*/
pub struct TZC_CID0rs;
impl crate::RegisterSpec for TZC_CID0rs {
    type Ux = u32;
}
///`read()` method returns [`tzc_cid0::R`](R) reader structure
impl crate::Readable for TZC_CID0rs {}
///`reset()` method sets TZC_CID0 to value 0x0d
impl crate::Resettable for TZC_CID0rs {
    const RESET_VALUE: u32 = 0x0d;
}
