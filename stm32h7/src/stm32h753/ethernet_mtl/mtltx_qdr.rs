#[doc = "Register `MTLTxQDR` reader"]
pub type R = crate::R<MTLTX_QDRrs>;
#[doc = "Field `TXQPAUSED` reader - Transmit Queue in Pause"]
pub type TXQPAUSED_R = crate::BitReader;
#[doc = "Field `TRCSTS` reader - MTL Tx Queue Read Controller Status"]
pub type TRCSTS_R = crate::FieldReader;
#[doc = "Field `TWCSTS` reader - MTL Tx Queue Write Controller Status"]
pub type TWCSTS_R = crate::BitReader;
#[doc = "Field `TXQSTS` reader - MTL Tx Queue Not Empty Status"]
pub type TXQSTS_R = crate::BitReader;
#[doc = "Field `TXSTSFSTS` reader - MTL Tx Status FIFO Full Status"]
pub type TXSTSFSTS_R = crate::BitReader;
#[doc = "Field `PTXQ` reader - Number of Packets in the Transmit Queue"]
pub type PTXQ_R = crate::FieldReader;
#[doc = "Field `STXSTSF` reader - Number of Status Words in Tx Status FIFO of Queue"]
pub type STXSTSF_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit Queue in Pause"]
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MTL Tx Queue Read Controller Status"]
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - MTL Tx Queue Write Controller Status"]
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MTL Tx Queue Not Empty Status"]
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MTL Tx Status FIFO Full Status"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Number of Packets in the Transmit Queue"]
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue"]
    #[inline(always)]
    pub fn stxstsf(&self) -> STXSTSF_R {
        STXSTSF_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[doc = "Tx queue debug Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtltx_qdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLTX_QDRrs;
impl crate::RegisterSpec for MTLTX_QDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtltx_qdr::R`](R) reader structure"]
impl crate::Readable for MTLTX_QDRrs {}
#[doc = "`reset()` method sets MTLTxQDR to value 0"]
impl crate::Resettable for MTLTX_QDRrs {
    const RESET_VALUE: u32 = 0;
}
