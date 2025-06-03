///Register `IAIER` reader
pub type R = crate::R<IAIERrs>;
///Field `IAEIE` reader - Illegal access error interrupt enable
pub type IAEIE_R = crate::BitReader;
impl R {
    ///Bit 1 - Illegal access error interrupt enable
    #[inline(always)]
    pub fn iaeie(&self) -> IAEIE_R {
        IAEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IAIER")
            .field("iaeie", &self.iaeie())
            .finish()
    }
}
/**MCE illegal access interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`iaier::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#MCE1:IAIER)*/
pub struct IAIERrs;
impl crate::RegisterSpec for IAIERrs {
    type Ux = u32;
}
///`read()` method returns [`iaier::R`](R) reader structure
impl crate::Readable for IAIERrs {}
///`reset()` method sets IAIER to value 0
impl crate::Resettable for IAIERrs {}
