///Register `MTLTxQ1UR` reader
pub type R = crate::R<MTLTX_Q1URrs>;
///Field `UFFRMCNT` reader - UFFRMCNT
pub type UFFRMCNT_R = crate::FieldReader<u16>;
///Field `UFCNTOVF` reader - UFCNTOVF
pub type UFCNTOVF_R = crate::BitReader;
impl R {
    ///Bits 0:10 - UFFRMCNT
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 11 - UFCNTOVF
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTxQ1UR")
            .field("uffrmcnt", &self.uffrmcnt())
            .field("ufcntovf", &self.ufcntovf())
            .finish()
    }
}
/**Tx queue 1 underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1ur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLTxQ1UR)*/
pub struct MTLTX_Q1URrs;
impl crate::RegisterSpec for MTLTX_Q1URrs {
    type Ux = u32;
}
///`read()` method returns [`mtltx_q1ur::R`](R) reader structure
impl crate::Readable for MTLTX_Q1URrs {}
///`reset()` method sets MTLTxQ1UR to value 0
impl crate::Resettable for MTLTX_Q1URrs {}
