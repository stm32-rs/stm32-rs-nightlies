///Register `MTLISR` reader
pub type R = crate::R<MTLISRrs>;
///Field `Q0IS` reader - Q0IS
pub type Q0IS_R = crate::BitReader;
///Field `Q1IS` reader - Q1IS
pub type Q1IS_R = crate::BitReader;
impl R {
    ///Bit 0 - Q0IS
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Q1IS
    #[inline(always)]
    pub fn q1is(&self) -> Q1IS_R {
        Q1IS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLISR")
            .field("q0is", &self.q0is())
            .field("q1is", &self.q1is())
            .finish()
    }
}
/**The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.

You can [`read`](crate::Reg::read) this register and get [`mtlisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLISR)*/
pub struct MTLISRrs;
impl crate::RegisterSpec for MTLISRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlisr::R`](R) reader structure
impl crate::Readable for MTLISRrs {}
///`reset()` method sets MTLISR to value 0
impl crate::Resettable for MTLISRrs {}
