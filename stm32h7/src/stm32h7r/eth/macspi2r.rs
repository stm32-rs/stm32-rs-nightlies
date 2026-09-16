///Register `MACSPI2R` reader
pub type R = crate::R<MACSPI2Rrs>;
///Register `MACSPI2R` writer
pub type W = crate::W<MACSPI2Rrs>;
///Field `SPI2` reader - Source Port Identity 2 This field indicates bits \[79:64\] of sourcePortIdentity of PTP node.
pub type SPI2_R = crate::FieldReader<u16>;
///Field `SPI2` writer - Source Port Identity 2 This field indicates bits \[79:64\] of sourcePortIdentity of PTP node.
pub type SPI2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Source Port Identity 2 This field indicates bits \[79:64\] of sourcePortIdentity of PTP node.
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACSPI2R")
            .field("spi2", &self.spi2())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Source Port Identity 2 This field indicates bits \[79:64\] of sourcePortIdentity of PTP node.
    #[inline(always)]
    pub fn spi2(&mut self) -> SPI2_W<'_, MACSPI2Rrs> {
        SPI2_W::new(self, 0)
    }
}
/**PTP Source port identity 2 register

You can [`read`](crate::Reg::read) this register and get [`macspi2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macspi2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#ETH:MACSPI2R)*/
pub struct MACSPI2Rrs;
impl crate::RegisterSpec for MACSPI2Rrs {
    type Ux = u32;
}
///`read()` method returns [`macspi2r::R`](R) reader structure
impl crate::Readable for MACSPI2Rrs {}
///`write(|w| ..)` method takes [`macspi2r::W`](W) writer structure
impl crate::Writable for MACSPI2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MACSPI2R to value 0
impl crate::Resettable for MACSPI2Rrs {}
