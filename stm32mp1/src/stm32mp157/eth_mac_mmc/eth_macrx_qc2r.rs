#[doc = "Register `ETH_MACRxQC2R` reader"]
pub type R = crate::R<ETH_MACRX_QC2Rrs>;
#[doc = "Register `ETH_MACRxQC2R` writer"]
pub type W = crate::W<ETH_MACRX_QC2Rrs>;
#[doc = "Field `PSRQ0` reader - PSRQ0"]
pub type PSRQ0_R = crate::FieldReader;
#[doc = "Field `PSRQ0` writer - PSRQ0"]
pub type PSRQ0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PSRQ1` reader - PSRQ1"]
pub type PSRQ1_R = crate::FieldReader;
#[doc = "Field `PSRQ1` writer - PSRQ1"]
pub type PSRQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PSRQ0"]
    #[inline(always)]
    pub fn psrq0(&self) -> PSRQ0_R {
        PSRQ0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PSRQ1"]
    #[inline(always)]
    pub fn psrq1(&self) -> PSRQ1_R {
        PSRQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PSRQ0"]
    #[inline(always)]
    #[must_use]
    pub fn psrq0(&mut self) -> PSRQ0_W<ETH_MACRX_QC2Rrs> {
        PSRQ0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - PSRQ1"]
    #[inline(always)]
    #[must_use]
    pub fn psrq1(&mut self) -> PSRQ1_W<ETH_MACRX_QC2Rrs> {
        PSRQ1_W::new(self, 8)
    }
}
#[doc = "This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macrx_qc2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macrx_qc2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACRX_QC2Rrs;
impl crate::RegisterSpec for ETH_MACRX_QC2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macrx_qc2r::R`](R) reader structure"]
impl crate::Readable for ETH_MACRX_QC2Rrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macrx_qc2r::W`](W) writer structure"]
impl crate::Writable for ETH_MACRX_QC2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACRxQC2R to value 0"]
impl crate::Resettable for ETH_MACRX_QC2Rrs {
    const RESET_VALUE: u32 = 0;
}
