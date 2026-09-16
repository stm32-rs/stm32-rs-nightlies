///Register `MACSPI0R` reader
pub type R = crate::R<MACSPI0Rrs>;
///Register `MACSPI0R` writer
pub type W = crate::W<MACSPI0Rrs>;
///Field `SPI0` reader - Source Port Identity 0
pub type SPI0_R = crate::FieldReader<u32>;
///Field `SPI0` writer - Source Port Identity 0
pub type SPI0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Source Port Identity 0
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACSPI0R")
            .field("spi0", &self.spi0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Source Port Identity 0
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W<'_, MACSPI0Rrs> {
        SPI0_W::new(self, 0)
    }
}
/**PTP Source Port Identity 0 Register

You can [`read`](crate::Reg::read) this register and get [`macspi0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MACSPI0R)*/
pub struct MACSPI0Rrs;
impl crate::RegisterSpec for MACSPI0Rrs {
    type Ux = u32;
}
///`read()` method returns [`macspi0r::R`](R) reader structure
impl crate::Readable for MACSPI0Rrs {}
///`write(|w| ..)` method takes [`macspi0r::W`](W) writer structure
impl crate::Writable for MACSPI0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACSPI0R to value 0
impl crate::Resettable for MACSPI0Rrs {}
