#[doc = "Register `ETH_MACRxQC1R` reader"]
pub type R = crate::R<ETH_MACRX_QC1Rrs>;
#[doc = "Register `ETH_MACRxQC1R` writer"]
pub type W = crate::W<ETH_MACRX_QC1Rrs>;
#[doc = "Field `AVCPQ` reader - AVCPQ"]
pub type AVCPQ_R = crate::FieldReader;
#[doc = "Field `AVCPQ` writer - AVCPQ"]
pub type AVCPQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AVPTPQ` reader - AVPTPQ"]
pub type AVPTPQ_R = crate::FieldReader;
#[doc = "Field `AVPTPQ` writer - AVPTPQ"]
pub type AVPTPQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UPQ` reader - UPQ"]
pub type UPQ_R = crate::FieldReader;
#[doc = "Field `UPQ` writer - UPQ"]
pub type UPQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCBCQ` reader - MCBCQ"]
pub type MCBCQ_R = crate::FieldReader;
#[doc = "Field `MCBCQ` writer - MCBCQ"]
pub type MCBCQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCBCQEN` reader - MCBCQEN"]
pub type MCBCQEN_R = crate::BitReader;
#[doc = "Field `MCBCQEN` writer - MCBCQEN"]
pub type MCBCQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TACPQE` reader - TACPQE"]
pub type TACPQE_R = crate::BitReader;
#[doc = "Field `TACPQE` writer - TACPQE"]
pub type TACPQE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - AVCPQ"]
    #[inline(always)]
    pub fn avcpq(&self) -> AVCPQ_R {
        AVCPQ_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - AVPTPQ"]
    #[inline(always)]
    pub fn avptpq(&self) -> AVPTPQ_R {
        AVPTPQ_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 12:14 - UPQ"]
    #[inline(always)]
    pub fn upq(&self) -> UPQ_R {
        UPQ_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - MCBCQ"]
    #[inline(always)]
    pub fn mcbcq(&self) -> MCBCQ_R {
        MCBCQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - MCBCQEN"]
    #[inline(always)]
    pub fn mcbcqen(&self) -> MCBCQEN_R {
        MCBCQEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TACPQE"]
    #[inline(always)]
    pub fn tacpqe(&self) -> TACPQE_R {
        TACPQE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AVCPQ"]
    #[inline(always)]
    #[must_use]
    pub fn avcpq(&mut self) -> AVCPQ_W<ETH_MACRX_QC1Rrs> {
        AVCPQ_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - AVPTPQ"]
    #[inline(always)]
    #[must_use]
    pub fn avptpq(&mut self) -> AVPTPQ_W<ETH_MACRX_QC1Rrs> {
        AVPTPQ_W::new(self, 4)
    }
    #[doc = "Bits 12:14 - UPQ"]
    #[inline(always)]
    #[must_use]
    pub fn upq(&mut self) -> UPQ_W<ETH_MACRX_QC1Rrs> {
        UPQ_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - MCBCQ"]
    #[inline(always)]
    #[must_use]
    pub fn mcbcq(&mut self) -> MCBCQ_W<ETH_MACRX_QC1Rrs> {
        MCBCQ_W::new(self, 16)
    }
    #[doc = "Bit 20 - MCBCQEN"]
    #[inline(always)]
    #[must_use]
    pub fn mcbcqen(&mut self) -> MCBCQEN_W<ETH_MACRX_QC1Rrs> {
        MCBCQEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - TACPQE"]
    #[inline(always)]
    #[must_use]
    pub fn tacpqe(&mut self) -> TACPQE_W<ETH_MACRX_QC1Rrs> {
        TACPQE_W::new(self, 21)
    }
}
#[doc = "The Receive Queue Control 1 register controls queue 1 management in the MAC receiver.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macrx_qc1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macrx_qc1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACRX_QC1Rrs;
impl crate::RegisterSpec for ETH_MACRX_QC1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macrx_qc1r::R`](R) reader structure"]
impl crate::Readable for ETH_MACRX_QC1Rrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macrx_qc1r::W`](W) writer structure"]
impl crate::Writable for ETH_MACRX_QC1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACRxQC1R to value 0"]
impl crate::Resettable for ETH_MACRX_QC1Rrs {
    const RESET_VALUE: u32 = 0;
}
