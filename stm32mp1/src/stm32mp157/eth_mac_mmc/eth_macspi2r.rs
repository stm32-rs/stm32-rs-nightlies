#[doc = "Register `ETH_MACSPI2R` reader"]
pub type R = crate::R<ETH_MACSPI2Rrs>;
#[doc = "Register `ETH_MACSPI2R` writer"]
pub type W = crate::W<ETH_MACSPI2Rrs>;
#[doc = "Field `SPI2` reader - SPI2"]
pub type SPI2_R = crate::FieldReader<u16>;
#[doc = "Field `SPI2` writer - SPI2"]
pub type SPI2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SPI2"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SPI2"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<ETH_MACSPI2Rrs> {
        SPI2_W::new(self, 0)
    }
}
#[doc = "This register contains Bits\\[79:64\\]
of the 80-bit Source Port Identity of the PTP node.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_macspi2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_macspi2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_MACSPI2Rrs;
impl crate::RegisterSpec for ETH_MACSPI2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_macspi2r::R`](R) reader structure"]
impl crate::Readable for ETH_MACSPI2Rrs {}
#[doc = "`write(|w| ..)` method takes [`eth_macspi2r::W`](W) writer structure"]
impl crate::Writable for ETH_MACSPI2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_MACSPI2R to value 0"]
impl crate::Resettable for ETH_MACSPI2Rrs {
    const RESET_VALUE: u32 = 0;
}
