#[doc = "Register `MACSPI0R` reader"]
pub type R = crate::R<MACSPI0Rrs>;
#[doc = "Register `MACSPI0R` writer"]
pub type W = crate::W<MACSPI0Rrs>;
#[doc = "Field `SPI0` reader - Source Port Identity 0 This field indicates bits \\[31:0\\]
of sourcePortIdentity of PTP node."]
pub type SPI0_R = crate::FieldReader<u32>;
#[doc = "Field `SPI0` writer - Source Port Identity 0 This field indicates bits \\[31:0\\]
of sourcePortIdentity of PTP node."]
pub type SPI0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Port Identity 0 This field indicates bits \\[31:0\\]
of sourcePortIdentity of PTP node."]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Port Identity 0 This field indicates bits \\[31:0\\]
of sourcePortIdentity of PTP node."]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<MACSPI0Rrs> {
        SPI0_W::new(self, 0)
    }
}
#[doc = "PTP Source Port Identity 0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macspi0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macspi0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSPI0Rrs;
impl crate::RegisterSpec for MACSPI0Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macspi0r::R`](R) reader structure"]
impl crate::Readable for MACSPI0Rrs {}
#[doc = "`write(|w| ..)` method takes [`macspi0r::W`](W) writer structure"]
impl crate::Writable for MACSPI0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACSPI0R to value 0"]
impl crate::Resettable for MACSPI0Rrs {
    const RESET_VALUE: u32 = 0;
}
