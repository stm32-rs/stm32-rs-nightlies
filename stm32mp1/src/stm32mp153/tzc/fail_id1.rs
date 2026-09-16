///Register `FAIL_ID1` reader
pub type R = crate::R<FAIL_ID1rs>;
///Field `ID` reader - ID
pub type ID_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:10 - ID
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FAIL_ID1").field("id", &self.id()).finish()
    }
}
/**Contains the master AXI ARID or AWID of the first access that failed a region permission check in the associated filter unit. This occurs even if the ACTION register is set to not drive the interrupt signal. AXI ID mapping is described in Table4: NSAID definition table (TBD).

You can [`read`](crate::Reg::read) this register and get [`fail_id1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:FAIL_ID1)*/
pub struct FAIL_ID1rs;
impl crate::RegisterSpec for FAIL_ID1rs {
    type Ux = u32;
}
///`read()` method returns [`fail_id1::R`](R) reader structure
impl crate::Readable for FAIL_ID1rs {}
///`reset()` method sets FAIL_ID1 to value 0
impl crate::Resettable for FAIL_ID1rs {}
