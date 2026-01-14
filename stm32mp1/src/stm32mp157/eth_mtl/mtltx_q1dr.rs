///Register `MTLTxQ1DR` reader
pub type R = crate::R<MTLTX_Q1DRrs>;
///Field `TXQPAUSED` reader - TXQPAUSED
pub type TXQPAUSED_R = crate::BitReader;
///Field `TRCSTS` reader - TRCSTS
pub type TRCSTS_R = crate::FieldReader;
///Field `TWCSTS` reader - TWCSTS
pub type TWCSTS_R = crate::BitReader;
///Field `TXQSTS` reader - TXQSTS
pub type TXQSTS_R = crate::BitReader;
///Field `TXSTSFSTS` reader - TXSTSFSTS
pub type TXSTSFSTS_R = crate::BitReader;
///Field `PTXQ` reader - PTXQ
pub type PTXQ_R = crate::FieldReader;
///Field `STXSTSF` reader - STXSTSF
pub type STXSTSF_R = crate::FieldReader;
impl R {
    ///Bit 0 - TXQPAUSED
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - TRCSTS
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - TWCSTS
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXQSTS
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TXSTSFSTS
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 16:18 - PTXQ
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - STXSTSF
    #[inline(always)]
    pub fn stxstsf(&self) -> STXSTSF_R {
        STXSTSF_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTxQ1DR")
            .field("txqpaused", &self.txqpaused())
            .field("trcsts", &self.trcsts())
            .field("twcsts", &self.twcsts())
            .field("txqsts", &self.txqsts())
            .field("txstsfsts", &self.txstsfsts())
            .field("ptxq", &self.ptxq())
            .field("stxstsf", &self.stxstsf())
            .finish()
    }
}
/**Tx queue 1 underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltx_q1dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MTL:MTLTxQ1DR)*/
pub struct MTLTX_Q1DRrs;
impl crate::RegisterSpec for MTLTX_Q1DRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltx_q1dr::R`](R) reader structure
impl crate::Readable for MTLTX_Q1DRrs {}
///`reset()` method sets MTLTxQ1DR to value 0
impl crate::Resettable for MTLTX_Q1DRrs {}
