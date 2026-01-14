///Register `PUBCFGCR5` writer
pub type W = crate::W<PUBCFGCR5rs>;
///Field `AXISRAM3PUBC` writer - Defines the public protection of the AXISRAM3 configuration bits (enable, ready, divider).
pub type AXISRAM3PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4PUBC` writer - Defines the public protection of the AXISRAM4 configuration bits (enable, ready, divider).
pub type AXISRAM4PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM5PUBC` writer - Defines the public protection of the AXISRAM5 configuration bits (enable, ready, divider).
pub type AXISRAM5PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM6PUBC` writer - Defines the public protection of the AXISRAM6 configuration bits (enable, ready, divider).
pub type AXISRAM6PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM1PUBC` writer - Defines the public protection of the AHBSRAM1 configuration bits (enable, ready, divider).
pub type AHBSRAM1PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM2PUBC` writer - Defines the public protection of the AHBSRAM2 configuration bits (enable, ready, divider).
pub type AHBSRAM2PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMPUBC` writer - Defines the public protection of the BKPSRAM configuration bits (enable, ready, divider).
pub type BKPSRAMPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1PUBC` writer - Defines the public protection of the AXISRAM1 configuration bits (enable, ready, divider).
pub type AXISRAM1PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2PUBC` writer - Defines the public protection of the AXISRAM2 configuration bits (enable, ready, divider).
pub type AXISRAM2PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEXRAMPUBC` writer - Defines the public protection of the FLEXRAM configuration bits (enable, ready, divider).
pub type FLEXRAMPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHEAXIRAMPUBC` writer - Defines the public protection of the NPUCACHERAM configuration bits (enable, ready, divider).
pub type CACHEAXIRAMPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRAMPUBC` writer - Defines the public protection of the VENCRAM configuration bits (enable, ready, divider).
pub type VENCRAMPUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PUBCFGCR5rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the AXISRAM3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn axisram3pubc(&mut self) -> AXISRAM3PUBC_W<'_, PUBCFGCR5rs> {
        AXISRAM3PUBC_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the AXISRAM4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn axisram4pubc(&mut self) -> AXISRAM4PUBC_W<'_, PUBCFGCR5rs> {
        AXISRAM4PUBC_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the AXISRAM5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn axisram5pubc(&mut self) -> AXISRAM5PUBC_W<'_, PUBCFGCR5rs> {
        AXISRAM5PUBC_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the AXISRAM6 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn axisram6pubc(&mut self) -> AXISRAM6PUBC_W<'_, PUBCFGCR5rs> {
        AXISRAM6PUBC_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the AHBSRAM1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahbsram1pubc(&mut self) -> AHBSRAM1PUBC_W<'_, PUBCFGCR5rs> {
        AHBSRAM1PUBC_W::new(self, 4)
    }
    ///Bit 5 - Defines the public protection of the AHBSRAM2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahbsram2pubc(&mut self) -> AHBSRAM2PUBC_W<'_, PUBCFGCR5rs> {
        AHBSRAM2PUBC_W::new(self, 5)
    }
    ///Bit 6 - Defines the public protection of the BKPSRAM configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn bkpsrampubc(&mut self) -> BKPSRAMPUBC_W<'_, PUBCFGCR5rs> {
        BKPSRAMPUBC_W::new(self, 6)
    }
    ///Bit 7 - Defines the public protection of the AXISRAM1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn axisram1pubc(&mut self) -> AXISRAM1PUBC_W<'_, PUBCFGCR5rs> {
        AXISRAM1PUBC_W::new(self, 7)
    }
    ///Bit 8 - Defines the public protection of the AXISRAM2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn axisram2pubc(&mut self) -> AXISRAM2PUBC_W<'_, PUBCFGCR5rs> {
        AXISRAM2PUBC_W::new(self, 8)
    }
    ///Bit 9 - Defines the public protection of the FLEXRAM configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn flexrampubc(&mut self) -> FLEXRAMPUBC_W<'_, PUBCFGCR5rs> {
        FLEXRAMPUBC_W::new(self, 9)
    }
    ///Bit 10 - Defines the public protection of the NPUCACHERAM configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn cacheaxirampubc(&mut self) -> CACHEAXIRAMPUBC_W<'_, PUBCFGCR5rs> {
        CACHEAXIRAMPUBC_W::new(self, 10)
    }
    ///Bit 11 - Defines the public protection of the VENCRAM configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn vencrampubc(&mut self) -> VENCRAMPUBC_W<'_, PUBCFGCR5rs> {
        VENCRAMPUBC_W::new(self, 11)
    }
}
/**RCC public configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgcr5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:PUBCFGCR5)*/
pub struct PUBCFGCR5rs;
impl crate::RegisterSpec for PUBCFGCR5rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pubcfgcr5::W`](W) writer structure
impl crate::Writable for PUBCFGCR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGCR5 to value 0
impl crate::Resettable for PUBCFGCR5rs {}
