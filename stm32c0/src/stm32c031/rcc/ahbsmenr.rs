///Register `AHBSMENR` reader
pub type R = crate::R<AHBSMENRrs>;
///Register `AHBSMENR` writer
pub type W = crate::W<AHBSMENRrs>;
///Field `DMA1SMEN` reader - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
pub type DMA1SMEN_R = crate::BitReader;
///Field `DMA1SMEN` writer - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
pub type DMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHSMEN` reader - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.
pub type FLASHSMEN_R = crate::BitReader;
///Field `FLASHSMEN` writer - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.
pub type FLASHSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAMSMEN` reader - SRAM clock enable during Sleep mode Set and cleared by software.
pub type SRAMSMEN_R = crate::BitReader;
///Field `SRAMSMEN` writer - SRAM clock enable during Sleep mode Set and cleared by software.
pub type SRAMSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCSMEN` reader - CRC clock enable during Sleep mode Set and cleared by software.
pub type CRCSMEN_R = crate::BitReader;
///Field `CRCSMEN` writer - CRC clock enable during Sleep mode Set and cleared by software.
pub type CRCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn sramsmen(&self) -> SRAMSMEN_R {
        SRAMSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBSMENR")
            .field("dma1smen", &self.dma1smen())
            .field("flashsmen", &self.flashsmen())
            .field("sramsmen", &self.sramsmen())
            .field("crcsmen", &self.crcsmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA1 and DMAMUX clock enable during Sleep mode Set and cleared by software. Clock to DMAMUX during Sleep mode is enabled as long as the clock in Sleep mode is enabled to at least one DMA peripheral.
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W<AHBSMENRrs> {
        DMA1SMEN_W::new(self, 0)
    }
    ///Bit 8 - Flash memory interface clock enable during Sleep mode Set and cleared by software. This bit can be activated only when the Flash memory is in power down mode.
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W<AHBSMENRrs> {
        FLASHSMEN_W::new(self, 8)
    }
    ///Bit 9 - SRAM clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn sramsmen(&mut self) -> SRAMSMEN_W<AHBSMENRrs> {
        SRAMSMEN_W::new(self, 9)
    }
    ///Bit 12 - CRC clock enable during Sleep mode Set and cleared by software.
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W<AHBSMENRrs> {
        CRCSMEN_W::new(self, 12)
    }
}
/**RCC AHB peripheral clock enable in Sleep/Stop mode register

You can [`read`](crate::Reg::read) this register and get [`ahbsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#RCC:AHBSMENR)*/
pub struct AHBSMENRrs;
impl crate::RegisterSpec for AHBSMENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbsmenr::R`](R) reader structure
impl crate::Readable for AHBSMENRrs {}
///`write(|w| ..)` method takes [`ahbsmenr::W`](W) writer structure
impl crate::Writable for AHBSMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHBSMENR to value 0x0005_1303
impl crate::Resettable for AHBSMENRrs {
    const RESET_VALUE: u32 = 0x0005_1303;
}
