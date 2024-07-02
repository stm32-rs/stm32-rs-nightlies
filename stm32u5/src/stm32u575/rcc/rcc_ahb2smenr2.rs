///Register `RCC_AHB2SMENR2` reader
pub type R = crate::R<RCC_AHB2SMENR2rs>;
///Register `RCC_AHB2SMENR2` writer
pub type W = crate::W<RCC_AHB2SMENR2rs>;
///Field `FSMCSMEN` reader - FSMC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type FSMCSMEN_R = crate::BitReader;
///Field `FSMCSMEN` writer - FSMC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type FSMCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI1SMEN` reader - OCTOSPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type OCTOSPI1SMEN_R = crate::BitReader;
///Field `OCTOSPI1SMEN` writer - OCTOSPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type OCTOSPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OCTOSPI2SMEN` reader - OCTOSPI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type OCTOSPI2SMEN_R = crate::BitReader;
///Field `OCTOSPI2SMEN` writer - OCTOSPI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type OCTOSPI2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSPI1SMEN` reader - HSPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type HSPI1SMEN_R = crate::BitReader;
///Field `HSPI1SMEN` writer - HSPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type HSPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM6SMEN` reader - SRAM6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type SRAM6SMEN_R = crate::BitReader;
///Field `SRAM6SMEN` writer - SRAM6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type SRAM6SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM5SMEN` reader - SRAM5 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type SRAM5SMEN_R = crate::BitReader;
///Field `SRAM5SMEN` writer - SRAM5 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
pub type SRAM5SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - FSMC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn fsmcsmen(&self) -> FSMCSMEN_R {
        FSMCSMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - OCTOSPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn octospi1smen(&self) -> OCTOSPI1SMEN_R {
        OCTOSPI1SMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - OCTOSPI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn octospi2smen(&self) -> OCTOSPI2SMEN_R {
        OCTOSPI2SMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - HSPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn hspi1smen(&self) -> HSPI1SMEN_R {
        HSPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 30 - SRAM6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sram6smen(&self) -> SRAM6SMEN_R {
        SRAM6SMEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM5 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    pub fn sram5smen(&self) -> SRAM5SMEN_R {
        SRAM5SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB2SMENR2")
            .field("fsmcsmen", &self.fsmcsmen())
            .field("octospi1smen", &self.octospi1smen())
            .field("octospi2smen", &self.octospi2smen())
            .field("hspi1smen", &self.hspi1smen())
            .field("sram6smen", &self.sram6smen())
            .field("sram5smen", &self.sram5smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - FSMC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn fsmcsmen(&mut self) -> FSMCSMEN_W<RCC_AHB2SMENR2rs> {
        FSMCSMEN_W::new(self, 0)
    }
    ///Bit 4 - OCTOSPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn octospi1smen(&mut self) -> OCTOSPI1SMEN_W<RCC_AHB2SMENR2rs> {
        OCTOSPI1SMEN_W::new(self, 4)
    }
    ///Bit 8 - OCTOSPI2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn octospi2smen(&mut self) -> OCTOSPI2SMEN_W<RCC_AHB2SMENR2rs> {
        OCTOSPI2SMEN_W::new(self, 8)
    }
    ///Bit 12 - HSPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn hspi1smen(&mut self) -> HSPI1SMEN_W<RCC_AHB2SMENR2rs> {
        HSPI1SMEN_W::new(self, 12)
    }
    ///Bit 30 - SRAM6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn sram6smen(&mut self) -> SRAM6SMEN_W<RCC_AHB2SMENR2rs> {
        SRAM6SMEN_W::new(self, 30)
    }
    ///Bit 31 - SRAM5 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.
    #[inline(always)]
    #[must_use]
    pub fn sram5smen(&mut self) -> SRAM5SMEN_W<RCC_AHB2SMENR2rs> {
        SRAM5SMEN_W::new(self, 31)
    }
}
/**RCC AHB2 peripheral clock enable in Sleep and Stop modes register 2

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb2smenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb2smenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RCC:RCC_AHB2SMENR2)*/
pub struct RCC_AHB2SMENR2rs;
impl crate::RegisterSpec for RCC_AHB2SMENR2rs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ahb2smenr2::R`](R) reader structure
impl crate::Readable for RCC_AHB2SMENR2rs {}
///`write(|w| ..)` method takes [`rcc_ahb2smenr2::W`](W) writer structure
impl crate::Writable for RCC_AHB2SMENR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_AHB2SMENR2 to value 0xffff_ffff
impl crate::Resettable for RCC_AHB2SMENR2rs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
