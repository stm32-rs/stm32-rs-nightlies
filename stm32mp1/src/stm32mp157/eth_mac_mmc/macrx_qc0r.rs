///Register `MACRxQC0R` reader
pub type R = crate::R<MACRX_QC0Rrs>;
///Register `MACRxQC0R` writer
pub type W = crate::W<MACRX_QC0Rrs>;
///Field `RXQ0EN` reader - RXQ0EN
pub type RXQ0EN_R = crate::FieldReader;
///Field `RXQ0EN` writer - RXQ0EN
pub type RXQ0EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RXQ1EN` reader - RXQ1EN
pub type RXQ1EN_R = crate::FieldReader;
///Field `RXQ1EN` writer - RXQ1EN
pub type RXQ1EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - RXQ0EN
    #[inline(always)]
    pub fn rxq0en(&self) -> RXQ0EN_R {
        RXQ0EN_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - RXQ1EN
    #[inline(always)]
    pub fn rxq1en(&self) -> RXQ1EN_R {
        RXQ1EN_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACRxQC0R")
            .field("rxq0en", &self.rxq0en())
            .field("rxq1en", &self.rxq1en())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - RXQ0EN
    #[inline(always)]
    pub fn rxq0en(&mut self) -> RXQ0EN_W<'_, MACRX_QC0Rrs> {
        RXQ0EN_W::new(self, 0)
    }
    ///Bits 2:3 - RXQ1EN
    #[inline(always)]
    pub fn rxq1en(&mut self) -> RXQ1EN_W<'_, MACRX_QC0Rrs> {
        RXQ1EN_W::new(self, 2)
    }
}
/**The Receive Queue Control 0 register controls the queue management in the MAC Receiver.

You can [`read`](crate::Reg::read) this register and get [`macrx_qc0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrx_qc0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MACRxQC0R)*/
pub struct MACRX_QC0Rrs;
impl crate::RegisterSpec for MACRX_QC0Rrs {
    type Ux = u32;
}
///`read()` method returns [`macrx_qc0r::R`](R) reader structure
impl crate::Readable for MACRX_QC0Rrs {}
///`write(|w| ..)` method takes [`macrx_qc0r::W`](W) writer structure
impl crate::Writable for MACRX_QC0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACRxQC0R to value 0
impl crate::Resettable for MACRX_QC0Rrs {}
