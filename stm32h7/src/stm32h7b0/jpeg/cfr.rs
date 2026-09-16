///Register `CFR` reader
pub type R = crate::R<CFRrs>;
///Register `CFR` writer
pub type W = crate::W<CFRrs>;
///Field `CEOCF` reader - Clear End of Conversion Flag Writing 1 clears the End of Conversion Flag of the JPEG Status Register.
pub type CEOCF_R = crate::BitReader;
///Field `CEOCF` writer - Clear End of Conversion Flag Writing 1 clears the End of Conversion Flag of the JPEG Status Register.
pub type CEOCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CHPDF` reader - Clear Header Parsing Done Flag Writing 1 clears the Header Parsing Done Flag of the JPEG Status Register.
pub type CHPDF_R = crate::BitReader;
///Field `CHPDF` writer - Clear Header Parsing Done Flag Writing 1 clears the Header Parsing Done Flag of the JPEG Status Register.
pub type CHPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 5 - Clear End of Conversion Flag Writing 1 clears the End of Conversion Flag of the JPEG Status Register.
    #[inline(always)]
    pub fn ceocf(&self) -> CEOCF_R {
        CEOCF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Clear Header Parsing Done Flag Writing 1 clears the Header Parsing Done Flag of the JPEG Status Register.
    #[inline(always)]
    pub fn chpdf(&self) -> CHPDF_R {
        CHPDF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFR")
            .field("ceocf", &self.ceocf())
            .field("chpdf", &self.chpdf())
            .finish()
    }
}
impl W {
    ///Bit 5 - Clear End of Conversion Flag Writing 1 clears the End of Conversion Flag of the JPEG Status Register.
    #[inline(always)]
    pub fn ceocf(&mut self) -> CEOCF_W<'_, CFRrs> {
        CEOCF_W::new(self, 5)
    }
    ///Bit 6 - Clear Header Parsing Done Flag Writing 1 clears the Header Parsing Done Flag of the JPEG Status Register.
    #[inline(always)]
    pub fn chpdf(&mut self) -> CHPDF_W<'_, CFRrs> {
        CHPDF_W::new(self, 6)
    }
}
/**JPEG clear flag register

You can [`read`](crate::Reg::read) this register and get [`cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#JPEG:CFR)*/
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u32;
}
///`read()` method returns [`cfr::R`](R) reader structure
impl crate::Readable for CFRrs {}
///`write(|w| ..)` method takes [`cfr::W`](W) writer structure
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFR to value 0
impl crate::Resettable for CFRrs {}
