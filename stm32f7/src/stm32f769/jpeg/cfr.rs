///Register `CFR` writer
pub type W = crate::W<CFRrs>;
///Field `CEOCF` writer - Clear End of Conversion Flag
pub type CEOCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHPDF` writer - Clear Header Parsing Done Flag
pub type CHPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 5 - Clear End of Conversion Flag
    #[inline(always)]
    pub fn ceocf(&mut self) -> CEOCF_W<'_, CFRrs> {
        CEOCF_W::new(self, 5)
    }
    ///Bit 6 - Clear Header Parsing Done Flag
    #[inline(always)]
    pub fn chpdf(&mut self) -> CHPDF_W<'_, CFRrs> {
        CHPDF_W::new(self, 6)
    }
}
/**JPEG clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F769.html#JPEG:CFR)*/
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cfr::W`](W) writer structure
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFR to value 0
impl crate::Resettable for CFRrs {}
