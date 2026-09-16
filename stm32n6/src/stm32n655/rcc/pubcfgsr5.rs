///Register `PUBCFGSR5` writer
pub type W = crate::W<PUBCFGSR5rs>;
///Field `AXISRAM3PUBS` writer - Defines the public protection of the AXISRAM3 configuration bits (enable, ready, divider).
pub type AXISRAM3PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4PUBS` writer - Defines the public protection of the AXISRAM4 configuration bits (enable, ready, divider).
pub type AXISRAM4PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM5PUBS` writer - Defines the public protection of the AXISRAM5 configuration bits (enable, ready, divider).
pub type AXISRAM5PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM6PUBS` writer - Defines the public protection of the AXISRAM6 configuration bits (enable, ready, divider).
pub type AXISRAM6PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM1PUBS` writer - Defines the public protection of the AHBSRAM1 configuration bits (enable, ready, divider).
pub type AHBSRAM1PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM2PUBS` writer - Defines the public protection of the AHBSRAM2 configuration bits (enable, ready, divider).
pub type AHBSRAM2PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMPUBS` writer - Defines the public protection of the BKPSRAM configuration bits (enable, ready, divider).
pub type BKPSRAMPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1PUBS` writer - Defines the public protection of the AXISRAM1 configuration bits (enable, ready, divider).
pub type AXISRAM1PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2PUBS` writer - Defines the public protection of the AXISRAM2 configuration bits (enable, ready, divider).
pub type AXISRAM2PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEXRAMPUBS` writer - Defines the public protection of the FLEXRAM configuration bits (enable, ready, divider).
pub type FLEXRAMPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERAMPUBS` writer - Defines the public protection of the NPUCACHERAM configuration bits (enable, ready, divider).
pub type NPUCACHERAMPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRAMPUBS` writer - Defines the public protection of the VENCRAM configuration bits (enable, ready, divider).
pub type VENCRAMPUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PUBCFGSR5rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the AXISRAM3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn axisram3pubs(&mut self) -> AXISRAM3PUBS_W<'_, PUBCFGSR5rs> {
        AXISRAM3PUBS_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the AXISRAM4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn axisram4pubs(&mut self) -> AXISRAM4PUBS_W<'_, PUBCFGSR5rs> {
        AXISRAM4PUBS_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the AXISRAM5 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn axisram5pubs(&mut self) -> AXISRAM5PUBS_W<'_, PUBCFGSR5rs> {
        AXISRAM5PUBS_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the AXISRAM6 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn axisram6pubs(&mut self) -> AXISRAM6PUBS_W<'_, PUBCFGSR5rs> {
        AXISRAM6PUBS_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the AHBSRAM1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahbsram1pubs(&mut self) -> AHBSRAM1PUBS_W<'_, PUBCFGSR5rs> {
        AHBSRAM1PUBS_W::new(self, 4)
    }
    ///Bit 5 - Defines the public protection of the AHBSRAM2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn ahbsram2pubs(&mut self) -> AHBSRAM2PUBS_W<'_, PUBCFGSR5rs> {
        AHBSRAM2PUBS_W::new(self, 5)
    }
    ///Bit 6 - Defines the public protection of the BKPSRAM configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn bkpsrampubs(&mut self) -> BKPSRAMPUBS_W<'_, PUBCFGSR5rs> {
        BKPSRAMPUBS_W::new(self, 6)
    }
    ///Bit 7 - Defines the public protection of the AXISRAM1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn axisram1pubs(&mut self) -> AXISRAM1PUBS_W<'_, PUBCFGSR5rs> {
        AXISRAM1PUBS_W::new(self, 7)
    }
    ///Bit 8 - Defines the public protection of the AXISRAM2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn axisram2pubs(&mut self) -> AXISRAM2PUBS_W<'_, PUBCFGSR5rs> {
        AXISRAM2PUBS_W::new(self, 8)
    }
    ///Bit 9 - Defines the public protection of the FLEXRAM configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn flexrampubs(&mut self) -> FLEXRAMPUBS_W<'_, PUBCFGSR5rs> {
        FLEXRAMPUBS_W::new(self, 9)
    }
    ///Bit 10 - Defines the public protection of the NPUCACHERAM configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn npucacherampubs(&mut self) -> NPUCACHERAMPUBS_W<'_, PUBCFGSR5rs> {
        NPUCACHERAMPUBS_W::new(self, 10)
    }
    ///Bit 11 - Defines the public protection of the VENCRAM configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn vencrampubs(&mut self) -> VENCRAMPUBS_W<'_, PUBCFGSR5rs> {
        VENCRAMPUBS_W::new(self, 11)
    }
}
/**RCC public configuration register4

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgsr5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:PUBCFGSR5)*/
pub struct PUBCFGSR5rs;
impl crate::RegisterSpec for PUBCFGSR5rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pubcfgsr5::W`](W) writer structure
impl crate::Writable for PUBCFGSR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGSR5 to value 0
impl crate::Resettable for PUBCFGSR5rs {}
