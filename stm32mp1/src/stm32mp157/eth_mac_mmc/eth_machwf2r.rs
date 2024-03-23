#[doc = "Register `ETH_MACHWF2R` reader"]
pub type R = crate::R<ETH_MACHWF2Rrs>;
#[doc = "Field `RXQCNT` reader - RXQCNT"]
pub type RXQCNT_R = crate::FieldReader;
#[doc = "Field `TXQCNT` reader - TXQCNT"]
pub type TXQCNT_R = crate::FieldReader;
#[doc = "Field `RXCHCNT` reader - RXCHCNT"]
pub type RXCHCNT_R = crate::FieldReader;
#[doc = "Field `TXCHCNT` reader - TXCHCNT"]
pub type TXCHCNT_R = crate::FieldReader;
#[doc = "Field `PPSOUTNUM` reader - PPSOUTNUM"]
pub type PPSOUTNUM_R = crate::FieldReader;
#[doc = "Field `AUXSNAPNUM` reader - AUXSNAPNUM"]
pub type AUXSNAPNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - RXQCNT"]
    #[inline(always)]
    pub fn rxqcnt(&self) -> RXQCNT_R {
        RXQCNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - TXQCNT"]
    #[inline(always)]
    pub fn txqcnt(&self) -> TXQCNT_R {
        TXQCNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RXCHCNT"]
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - TXCHCNT"]
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - PPSOUTNUM"]
    #[inline(always)]
    pub fn ppsoutnum(&self) -> PPSOUTNUM_R {
        PPSOUTNUM_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - AUXSNAPNUM"]
    #[inline(always)]
    pub fn auxsnapnum(&self) -> AUXSNAPNUM_R {
        AUXSNAPNUM_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "This register indicates the presence of third set of the optional features or functions of the Ethernet peripheral. The software driver can use this register to dynamically enable or disable the programs related to the optional blocks.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_machwf2r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACHWF2Rrs;
impl crate::RegisterSpec for ETH_MACHWF2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_machwf2r::R`](R) reader structure"]
impl crate::Readable for ETH_MACHWF2Rrs {}
#[doc = "`reset()` method sets ETH_MACHWF2R to value 0x4104_0041"]
impl crate::Resettable for ETH_MACHWF2Rrs {
    const RESET_VALUE: u32 = 0x4104_0041;
}
