#[doc = "Register `UDRDR` reader"]
pub type R = crate::R<UDRDRrs>;
#[doc = "Register `UDRDR` writer"]
pub type W = crate::W<UDRDRrs>;
#[doc = "Field `UDRDR` reader - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\\[31-16\\]
bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
pub type UDRDR_R = crate::FieldReader<u32>;
#[doc = "Field `UDRDR` writer - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\\[31-16\\]
bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
pub type UDRDR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\\[31-16\\]
bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
    #[inline(always)]
    pub fn udrdr(&self) -> UDRDR_R {
        UDRDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data at slave underrun condition The register is taken into account in Slave mode and at underrun condition only. The number of bits considered depends on DSIZE bit settings of the SPI_CFG1 register. Underrun condition handling depends on setting UDRCFG bit at SPI_CFG1 register. Note: UDRDR\\[31-16\\]
bits are reserved at the peripheral instances with data size limited to 16-bit. There is no constraint when 32-bit access is applied at these addresses. Reserved bits 31-16 are always read zero while any write to them is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn udrdr(&mut self) -> UDRDR_W<UDRDRrs> {
        UDRDR_W::new(self, 0)
    }
}
#[doc = "SPI/I2S underrun data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udrdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udrdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDRDRrs;
impl crate::RegisterSpec for UDRDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udrdr::R`](R) reader structure"]
impl crate::Readable for UDRDRrs {}
#[doc = "`write(|w| ..)` method takes [`udrdr::W`](W) writer structure"]
impl crate::Writable for UDRDRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDRDR to value 0"]
impl crate::Resettable for UDRDRrs {
    const RESET_VALUE: u32 = 0;
}
