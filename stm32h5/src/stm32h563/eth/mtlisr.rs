///Register `MTLISR` reader
pub type R = crate::R<MTLISRrs>;
///Field `Q0IS` reader - Queue interrupt status This bit indicates that an interrupt has been generated by Queue. To reset this bit, read ETH_MTLQICSR register to identify the interrupt cause and clear the source.
pub type Q0IS_R = crate::BitReader;
impl R {
    ///Bit 0 - Queue interrupt status This bit indicates that an interrupt has been generated by Queue. To reset this bit, read ETH_MTLQICSR register to identify the interrupt cause and clear the source.
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLISR")
            .field("q0is", &self.q0is())
            .finish()
    }
}
/**Interrupt status Register

You can [`read`](crate::Reg::read) this register and get [`mtlisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:MTLISR)*/
pub struct MTLISRrs;
impl crate::RegisterSpec for MTLISRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlisr::R`](R) reader structure
impl crate::Readable for MTLISRrs {}
///`reset()` method sets MTLISR to value 0
impl crate::Resettable for MTLISRrs {}
