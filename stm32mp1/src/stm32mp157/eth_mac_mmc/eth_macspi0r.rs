///Register `ETH_MACSPI0R` reader
pub type R = crate::R<ETH_MACSPI0Rrs>;
///Register `ETH_MACSPI0R` writer
pub type W = crate::W<ETH_MACSPI0Rrs>;
///Field `SPI0` reader - SPI0
pub type SPI0_R = crate::FieldReader<u32>;
///Field `SPI0` writer - SPI0
pub type SPI0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - SPI0
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MACSPI0R")
            .field("spi0", &self.spi0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - SPI0
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<ETH_MACSPI0Rrs> {
        SPI0_W::new(self, 0)
    }
}
/**This register contains Bits\[31:0\]
of the 80-bit Source Port Identity of the PTP node. This register is available only when the Enable PTP Timestamp Offload feature is selected.

You can [`read`](crate::Reg::read) this register and get [`eth_macspi0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eth_macspi0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:ETH_MACSPI0R)*/
pub struct ETH_MACSPI0Rrs;
impl crate::RegisterSpec for ETH_MACSPI0Rrs {
    type Ux = u32;
}
///`read()` method returns [`eth_macspi0r::R`](R) reader structure
impl crate::Readable for ETH_MACSPI0Rrs {}
///`write(|w| ..)` method takes [`eth_macspi0r::W`](W) writer structure
impl crate::Writable for ETH_MACSPI0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETH_MACSPI0R to value 0
impl crate::Resettable for ETH_MACSPI0Rrs {
    const RESET_VALUE: u32 = 0;
}
