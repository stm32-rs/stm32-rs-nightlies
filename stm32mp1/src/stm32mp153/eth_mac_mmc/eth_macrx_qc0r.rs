#[doc = "Register `ETH_MACRxQC0R` reader"]
pub type R = crate::R<ETH_MACRX_QC0Rrs>;
#[doc = "Register `ETH_MACRxQC0R` writer"]
pub type W = crate::W<ETH_MACRX_QC0Rrs>;
#[doc = "Field `RXQ0EN` reader - RXQ0EN"]
pub type RXQ0EN_R = crate::FieldReader;
#[doc = "Field `RXQ0EN` writer - RXQ0EN"]
pub type RXQ0EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXQ1EN` reader - RXQ1EN"]
pub type RXQ1EN_R = crate::FieldReader;
#[doc = "Field `RXQ1EN` writer - RXQ1EN"]
pub type RXQ1EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - RXQ0EN"]
    #[inline(always)]
    pub fn rxq0en(&self) -> RXQ0EN_R {
        RXQ0EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - RXQ1EN"]
    #[inline(always)]
    pub fn rxq1en(&self) -> RXQ1EN_R {
        RXQ1EN_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RXQ0EN"]
    #[inline(always)]
    #[must_use]
    pub fn rxq0en(&mut self) -> RXQ0EN_W<ETH_MACRX_QC0Rrs> {
        RXQ0EN_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - RXQ1EN"]
    #[inline(always)]
    #[must_use]
    pub fn rxq1en(&mut self) -> RXQ1EN_W<ETH_MACRX_QC0Rrs> {
        RXQ1EN_W::new(self, 2)
    }
}
#[doc = "The Receive Queue Control 0 register controls the queue management in the MAC Receiver.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macrx_qc0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macrx_qc0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACRX_QC0Rrs;
impl crate::RegisterSpec for ETH_MACRX_QC0Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macrx_qc0r::R`](R) reader structure"]
impl crate::Readable for ETH_MACRX_QC0Rrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macrx_qc0r::W`](W) writer structure"]
impl crate::Writable for ETH_MACRX_QC0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACRxQC0R to value 0"]
impl crate::Resettable for ETH_MACRX_QC0Rrs {
    const RESET_VALUE: u32 = 0;
}
