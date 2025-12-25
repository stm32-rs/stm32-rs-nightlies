///Register `AHB1SMENR` reader
pub type R = crate::R<AHB1SMENRrs>;
///Register `AHB1SMENR` writer
pub type W = crate::W<AHB1SMENRrs>;
///Field `GPDMA1SMEN` reader - GPDMA1 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type GPDMA1SMEN_R = crate::BitReader;
///Field `GPDMA1SMEN` writer - GPDMA1 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type GPDMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHSMEN` reader - FLASH bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type FLASHSMEN_R = crate::BitReader;
///Field `FLASHSMEN` writer - FLASH bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCSMEN` reader - CRC bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type CRCSMEN_R = crate::BitReader;
///Field `CRCSMEN` writer - CRC bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCSMEN` reader - TSC bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..
pub type TSCSMEN_R = crate::BitReader;
///Field `TSCSMEN` writer - TSC bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..
pub type TSCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGSMEN` reader - RAMCFG bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RAMCFGSMEN_R = crate::BitReader;
///Field `RAMCFGSMEN` writer - RAMCFG bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type RAMCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTZC1SMEN` reader - GTZC1 bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GTZC1SMEN_R = crate::BitReader;
///Field `GTZC1SMEN` writer - GTZC1 bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type GTZC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHESMEN` reader - ICACHE bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC ICACHE_REGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..
pub type ICACHESMEN_R = crate::BitReader;
///Field `ICACHESMEN` writer - ICACHE bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC ICACHE_REGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..
pub type ICACHESMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM1SMEN` reader - SRAM1 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SRAM1SMEN_R = crate::BitReader;
///Field `SRAM1SMEN` writer - SRAM1 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
pub type SRAM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPDMA1 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn gpdma1smen(&self) -> GPDMA1SMEN_R {
        GPDMA1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - FLASH bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - TSC bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RAMCFG bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn ramcfgsmen(&self) -> RAMCFGSMEN_R {
        RAMCFGSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - GTZC1 bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gtzc1smen(&self) -> GTZC1SMEN_R {
        GTZC1SMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 29 - ICACHE bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC ICACHE_REGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..
    #[inline(always)]
    pub fn icachesmen(&self) -> ICACHESMEN_R {
        ICACHESMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - SRAM1 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB1SMENR")
            .field("gpdma1smen", &self.gpdma1smen())
            .field("flashsmen", &self.flashsmen())
            .field("crcsmen", &self.crcsmen())
            .field("tscsmen", &self.tscsmen())
            .field("ramcfgsmen", &self.ramcfgsmen())
            .field("gtzc1smen", &self.gtzc1smen())
            .field("icachesmen", &self.icachesmen())
            .field("sram1smen", &self.sram1smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPDMA1 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn gpdma1smen(&mut self) -> GPDMA1SMEN_W<'_, AHB1SMENRrs> {
        GPDMA1SMEN_W::new(self, 0)
    }
    ///Bit 8 - FLASH bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<'_, AHB1SMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    ///Bit 12 - CRC bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<'_, AHB1SMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
    ///Bit 16 - TSC bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..
    #[inline(always)]
    pub fn tscsmen(&mut self) -> TSCSMEN_W<'_, AHB1SMENRrs> {
        TSCSMEN_W::new(self, 16)
    }
    ///Bit 17 - RAMCFG bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn ramcfgsmen(&mut self) -> RAMCFGSMEN_W<'_, AHB1SMENRrs> {
        RAMCFGSMEN_W::new(self, 17)
    }
    ///Bit 24 - GTZC1 bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn gtzc1smen(&mut self) -> GTZC1SMEN_W<'_, AHB1SMENRrs> {
        GTZC1SMEN_W::new(self, 24)
    }
    ///Bit 29 - ICACHE bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC ICACHE_REGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV..
    #[inline(always)]
    pub fn icachesmen(&mut self) -> ICACHESMEN_W<'_, AHB1SMENRrs> {
        ICACHESMEN_W::new(self, 29)
    }
    ///Bit 31 - SRAM1 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.
    #[inline(always)]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W<'_, AHB1SMENRrs> {
        SRAM1SMEN_W::new(self, 31)
    }
}
/**RCC AHB1 peripheral clocks enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`ahb1smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RCC:AHB1SMENR)*/
pub struct AHB1SMENRrs;
impl crate::RegisterSpec for AHB1SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb1smenr::R`](R) reader structure
impl crate::Readable for AHB1SMENRrs {}
///`write(|w| ..)` method takes [`ahb1smenr::W`](W) writer structure
impl crate::Writable for AHB1SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB1SMENR to value 0xffff_ffff
impl crate::Resettable for AHB1SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
