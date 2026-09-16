///Register `AHB1ENR` reader
pub type R = crate::R<AHB1ENRrs>;
///Register `AHB1ENR` writer
pub type W = crate::W<AHB1ENRrs>;
///Field `GPDMA1EN` reader - GPDMA1 bus clock enable Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GPDMA1EN_R = crate::BitReader;
///Field `GPDMA1EN` writer - GPDMA1 bus clock enable Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GPDMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHEN` reader - FLASH bus clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode. Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type FLASHEN_R = crate::BitReader;
///Field `FLASHEN` writer - FLASH bus clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode. Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type FLASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEN` reader - CRC bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type CRCEN_R = crate::BitReader;
///Field `CRCEN` writer - CRC bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCEN` reader - Touch sensing controller bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TSCEN_R = crate::BitReader;
///Field `TSCEN` writer - Touch sensing controller bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type TSCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGEN` reader - RAMCFG bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RAMCFGEN_R = crate::BitReader;
///Field `RAMCFGEN` writer - RAMCFG bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RAMCFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTZC1EN` reader - GTZC1 bus clock enable Set and reset by software. Can only be accessed secure when device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GTZC1EN_R = crate::BitReader;
///Field `GTZC1EN` writer - GTZC1 bus clock enable Set and reset by software. Can only be accessed secure when device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GTZC1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1EN` reader - SRAM1 bus clock enable Set and reset by software. Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SRAM1EN_R = crate::BitReader;
///Field `SRAM1EN` writer - SRAM1 bus clock enable Set and reset by software. Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SRAM1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPDMA1 bus clock enable Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gpdma1en(&self) -> GPDMA1EN_R {
        GPDMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - FLASH bus clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode. Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Touch sensing controller bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RAMCFG bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn ramcfgen(&self) -> RAMCFGEN_R {
        RAMCFGEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - GTZC1 bus clock enable Set and reset by software. Can only be accessed secure when device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gtzc1en(&self) -> GTZC1EN_R {
        GTZC1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 31 - SRAM1 bus clock enable Set and reset by software. Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn sram1en(&self) -> SRAM1EN_R {
        SRAM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1ENR")
            .field("gpdma1en", &self.gpdma1en())
            .field("flashen", &self.flashen())
            .field("crcen", &self.crcen())
            .field("tscen", &self.tscen())
            .field("ramcfgen", &self.ramcfgen())
            .field("gtzc1en", &self.gtzc1en())
            .field("sram1en", &self.sram1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPDMA1 bus clock enable Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gpdma1en(&mut self) -> GPDMA1EN_W<'_, AHB1ENRrs> {
        GPDMA1EN_W::new(self, 0)
    }
    ///Bit 8 - FLASH bus clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode. Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn flashen(&mut self) -> FLASHEN_W<'_, AHB1ENRrs> {
        FLASHEN_W::new(self, 8)
    }
    ///Bit 12 - CRC bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, AHB1ENRrs> {
        CRCEN_W::new(self, 12)
    }
    ///Bit 16 - Touch sensing controller bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn tscen(&mut self) -> TSCEN_W<'_, AHB1ENRrs> {
        TSCEN_W::new(self, 16)
    }
    ///Bit 17 - RAMCFG bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn ramcfgen(&mut self) -> RAMCFGEN_W<'_, AHB1ENRrs> {
        RAMCFGEN_W::new(self, 17)
    }
    ///Bit 24 - GTZC1 bus clock enable Set and reset by software. Can only be accessed secure when device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gtzc1en(&mut self) -> GTZC1EN_W<'_, AHB1ENRrs> {
        GTZC1EN_W::new(self, 24)
    }
    ///Bit 31 - SRAM1 bus clock enable Set and reset by software. Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn sram1en(&mut self) -> SRAM1EN_W<'_, AHB1ENRrs> {
        SRAM1EN_W::new(self, 31)
    }
}
/**RCC AHB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RCC:AHB1ENR)*/
pub struct AHB1ENRrs;
impl crate::RegisterSpec for AHB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1enr::R`](R) reader structure
impl crate::Readable for AHB1ENRrs {}
///`write(|w| ..)` method takes [`ahb1enr::W`](W) writer structure
impl crate::Writable for AHB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1ENR to value 0x8000_0100
impl crate::Resettable for AHB1ENRrs {
    const RESET_VALUE: u32 = 0x8000_0100;
}
