#[doc = "Register `ETH_MTLTxQ0DR` reader"]
pub type R = crate::R<ETH_MTLTX_Q0DRrs>;
#[doc = "Field `TXQPAUSED` reader - TXQPAUSED"]
pub type TXQPAUSED_R = crate::BitReader;
#[doc = "Field `TRCSTS` reader - TRCSTS"]
pub type TRCSTS_R = crate::FieldReader;
#[doc = "Field `TWCSTS` reader - TWCSTS"]
pub type TWCSTS_R = crate::BitReader;
#[doc = "Field `TXQSTS` reader - TXQSTS"]
pub type TXQSTS_R = crate::BitReader;
#[doc = "Field `TXSTSFSTS` reader - TXSTSFSTS"]
pub type TXSTSFSTS_R = crate::BitReader;
#[doc = "Field `PTXQ` reader - PTXQ"]
pub type PTXQ_R = crate::FieldReader;
#[doc = "Field `STXSTSF` reader - STXSTSF"]
pub type STXSTSF_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - TXQPAUSED"]
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - TRCSTS"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - TWCSTS"]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXQSTS"]
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXSTSFSTS"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:18 - PTXQ"]
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - STXSTSF"]
    #[inline(always)]
    pub fn stxstsf(&self) -> STXSTSF_R {
        STXSTSF_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[doc = "Tx queue 0 underflow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mtltx_q0dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MTLTX_Q0DRrs;
impl crate::RegisterSpec for ETH_MTLTX_Q0DRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_mtltx_q0dr::R`](R) reader structure"]
impl crate::Readable for ETH_MTLTX_Q0DRrs {}
#[doc = "`reset()` method sets ETH_MTLTxQ0DR to value 0"]
impl crate::Resettable for ETH_MTLTX_Q0DRrs {
    const RESET_VALUE: u32 = 0;
}
