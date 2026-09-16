///Register `MACRxQC2R` reader
pub type R = crate::R<MACRX_QC2Rrs>;
///Register `MACRxQC2R` writer
pub type W = crate::W<MACRX_QC2Rrs>;
///Field `PSRQ0` reader - PSRQ0
pub type PSRQ0_R = crate::FieldReader;
///Field `PSRQ0` writer - PSRQ0
pub type PSRQ0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PSRQ1` reader - PSRQ1
pub type PSRQ1_R = crate::FieldReader;
///Field `PSRQ1` writer - PSRQ1
pub type PSRQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - PSRQ0
    #[inline(always)]
    pub fn psrq0(&self) -> PSRQ0_R {
        PSRQ0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - PSRQ1
    #[inline(always)]
    pub fn psrq1(&self) -> PSRQ1_R {
        PSRQ1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACRxQC2R")
            .field("psrq0", &self.psrq0())
            .field("psrq1", &self.psrq1())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - PSRQ0
    #[inline(always)]
    pub fn psrq0(&mut self) -> PSRQ0_W<'_, MACRX_QC2Rrs> {
        PSRQ0_W::new(self, 0)
    }
    ///Bits 8:15 - PSRQ1
    #[inline(always)]
    pub fn psrq1(&mut self) -> PSRQ1_W<'_, MACRX_QC2Rrs> {
        PSRQ1_W::new(self, 8)
    }
}
/**This register controls the routing of tagged packets based on the USP (user priority) field of the received packets to the Rx queue 0 and 1.

You can [`read`](crate::Reg::read) this register and get [`macrx_qc2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrx_qc2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MAC_MMC:MACRxQC2R)*/
pub struct MACRX_QC2Rrs;
impl crate::RegisterSpec for MACRX_QC2Rrs {
    type Ux = u32;
}
///`read()` method returns [`macrx_qc2r::R`](R) reader structure
impl crate::Readable for MACRX_QC2Rrs {}
///`write(|w| ..)` method takes [`macrx_qc2r::W`](W) writer structure
impl crate::Writable for MACRX_QC2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACRxQC2R to value 0
impl crate::Resettable for MACRX_QC2Rrs {}
