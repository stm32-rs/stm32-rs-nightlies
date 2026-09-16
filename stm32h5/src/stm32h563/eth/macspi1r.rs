///Register `MACSPI1R` reader
pub type R = crate::R<MACSPI1Rrs>;
///Register `MACSPI1R` writer
pub type W = crate::W<MACSPI1Rrs>;
///Field `SPI1` reader - Source Port Identity 1 This field indicates bits \[63:32\] of sourcePortIdentity of PTP node.
pub type SPI1_R = crate::FieldReader<u32>;
///Field `SPI1` writer - Source Port Identity 1 This field indicates bits \[63:32\] of sourcePortIdentity of PTP node.
pub type SPI1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Source Port Identity 1 This field indicates bits \[63:32\] of sourcePortIdentity of PTP node.
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACSPI1R")
            .field("spi1", &self.spi1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Source Port Identity 1 This field indicates bits \[63:32\] of sourcePortIdentity of PTP node.
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W<'_, MACSPI1Rrs> {
        SPI1_W::new(self, 0)
    }
}
/**PTP Source port identity 1 register

You can [`read`](crate::Reg::read) this register and get [`macspi1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:MACSPI1R)*/
pub struct MACSPI1Rrs;
impl crate::RegisterSpec for MACSPI1Rrs {
    type Ux = u32;
}
///`read()` method returns [`macspi1r::R`](R) reader structure
impl crate::Readable for MACSPI1Rrs {}
///`write(|w| ..)` method takes [`macspi1r::W`](W) writer structure
impl crate::Writable for MACSPI1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACSPI1R to value 0
impl crate::Resettable for MACSPI1Rrs {}
