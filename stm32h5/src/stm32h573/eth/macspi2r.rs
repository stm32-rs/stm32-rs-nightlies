#[doc = "Register `MACSPI2R` reader"]
pub type R = crate::R<MACSPI2Rrs>;
#[doc = "Register `MACSPI2R` writer"]
pub type W = crate::W<MACSPI2Rrs>;
#[doc = "Field `SPI2` reader - Source Port Identity 2 This field indicates bits \\[79:64\\]
of sourcePortIdentity of PTP node."]
pub type SPI2_R = crate::FieldReader<u16>;
#[doc = "Field `SPI2` writer - Source Port Identity 2 This field indicates bits \\[79:64\\]
of sourcePortIdentity of PTP node."]
pub type SPI2_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Source Port Identity 2 This field indicates bits \\[79:64\\]
of sourcePortIdentity of PTP node."]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source Port Identity 2 This field indicates bits \\[79:64\\]
of sourcePortIdentity of PTP node."]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<MACSPI2Rrs> {
        SPI2_W::new(self, 0)
    }
}
#[doc = "PTP Source port identity 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macspi2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macspi2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACSPI2Rrs;
impl crate::RegisterSpec for MACSPI2Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macspi2r::R`](R) reader structure"]
impl crate::Readable for MACSPI2Rrs {}
#[doc = "`write(|w| ..)` method takes [`macspi2r::W`](W) writer structure"]
impl crate::Writable for MACSPI2Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACSPI2R to value 0"]
impl crate::Resettable for MACSPI2Rrs {
    const RESET_VALUE: u32 = 0;
}
