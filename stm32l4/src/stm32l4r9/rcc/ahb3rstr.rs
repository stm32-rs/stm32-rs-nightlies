///Register `AHB3RSTR` reader
pub type R = crate::R<AHB3RSTRrs>;
///Register `AHB3RSTR` writer
pub type W = crate::W<AHB3RSTRrs>;
///Field `FMCRST` reader - Flexible memory controller reset
pub type FMCRST_R = crate::BitReader;
///Field `FMCRST` writer - Flexible memory controller reset
pub type FMCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSPI2RST` reader - OctOSPI2 memory interface reset
pub type OSPI2RST_R = crate::BitReader;
///Field `OSPI2RST` writer - OctOSPI2 memory interface reset
pub type OSPI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Flexible memory controller reset
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 9 - OctOSPI2 memory interface reset
    #[inline(always)]
    pub fn ospi2rst(&self) -> OSPI2RST_R {
        OSPI2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3RSTR")
            .field("fmcrst", &self.fmcrst())
            .field("ospi2rst", &self.ospi2rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flexible memory controller reset
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W<'_, AHB3RSTRrs> {
        FMCRST_W::new(self, 0)
    }
    ///Bit 9 - OctOSPI2 memory interface reset
    #[inline(always)]
    pub fn ospi2rst(&mut self) -> OSPI2RST_W<'_, AHB3RSTRrs> {
        OSPI2RST_W::new(self, 9)
    }
}
/**AHB3 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#RCC:AHB3RSTR)*/
pub struct AHB3RSTRrs;
impl crate::RegisterSpec for AHB3RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3rstr::R`](R) reader structure
impl crate::Readable for AHB3RSTRrs {}
///`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure
impl crate::Writable for AHB3RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3RSTR to value 0
impl crate::Resettable for AHB3RSTRrs {}
