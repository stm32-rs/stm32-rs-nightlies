#[doc = "Register `ETH_DMADSR` reader"]
pub type R = crate::R<ETH_DMADSRrs>;
#[doc = "Field `AXWHSTS` reader - AHB Master Write Channel"]
pub type AXWHSTS_R = crate::BitReader;
#[doc = "Field `AXRHSTS` reader - AXRHSTS"]
pub type AXRHSTS_R = crate::BitReader;
#[doc = "Field `RPS0` reader - RPS0"]
pub type RPS0_R = crate::FieldReader;
#[doc = "Field `TPS0` reader - TPS0"]
pub type TPS0_R = crate::FieldReader;
#[doc = "Field `RPS1` reader - RPS1"]
pub type RPS1_R = crate::FieldReader;
#[doc = "Field `TPS1` reader - TPS1"]
pub type TPS1_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - AHB Master Write Channel"]
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AXRHSTS"]
    #[inline(always)]
    pub fn axrhsts(&self) -> AXRHSTS_R {
        AXRHSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - RPS0"]
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TPS0"]
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RPS1"]
    #[inline(always)]
    pub fn rps1(&self) -> RPS1_R {
        RPS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - TPS1"]
    #[inline(always)]
    pub fn tps1(&self) -> TPS1_R {
        TPS1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "Debug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_dmadsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_DMADSRrs;
impl crate::RegisterSpec for ETH_DMADSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_dmadsr::R`](R) reader structure"]
impl crate::Readable for ETH_DMADSRrs {}
#[doc = "`reset()` method sets ETH_DMADSR to value 0"]
impl crate::Resettable for ETH_DMADSRrs {
    const RESET_VALUE: u32 = 0;
}
