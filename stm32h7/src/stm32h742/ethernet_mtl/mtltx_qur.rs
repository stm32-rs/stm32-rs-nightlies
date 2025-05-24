///Register `MTLTxQUR` reader
pub type R = crate::R<MTLTX_QURrs>;
///Field `UFFRMCNT` reader - Underflow Packet Counter
pub type UFFRMCNT_R = crate::FieldReader<u16>;
///Field `UFCNTOVF` reader - Overflow Bit for Underflow Packet Counter
pub type UFCNTOVF_R = crate::BitReader;
impl R {
    ///Bits 0:10 - Underflow Packet Counter
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 11 - Overflow Bit for Underflow Packet Counter
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTxQUR")
            .field("uffrmcnt", &self.uffrmcnt())
            .field("ufcntovf", &self.ufcntovf())
            .finish()
    }
}
/**Tx queue underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltx_qur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#Ethernet_MTL:MTLTxQUR)*/
pub struct MTLTX_QURrs;
impl crate::RegisterSpec for MTLTX_QURrs {
    type Ux = u32;
}
///`read()` method returns [`mtltx_qur::R`](R) reader structure
impl crate::Readable for MTLTX_QURrs {}
///`reset()` method sets MTLTxQUR to value 0
impl crate::Resettable for MTLTX_QURrs {}
