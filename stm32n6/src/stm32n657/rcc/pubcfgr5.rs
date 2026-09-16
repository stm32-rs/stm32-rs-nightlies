///Register `PUBCFGR5` reader
pub type R = crate::R<PUBCFGR5rs>;
///Register `PUBCFGR5` writer
pub type W = crate::W<PUBCFGR5rs>;
///Field `AXISRAM3PUB` reader - Defines the public protection of the AXISRAM3 bus configuration bits.
pub type AXISRAM3PUB_R = crate::BitReader;
///Field `AXISRAM3PUB` writer - Defines the public protection of the AXISRAM3 bus configuration bits.
pub type AXISRAM3PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM4PUB` reader - Defines the public protection of the AXISRAM4 bus configuration bits.
pub type AXISRAM4PUB_R = crate::BitReader;
///Field `AXISRAM4PUB` writer - Defines the public protection of the AXISRAM4 bus configuration bits.
pub type AXISRAM4PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM5PUB` reader - Defines the public protection of the AXISRAM5 bus configuration bits.
pub type AXISRAM5PUB_R = crate::BitReader;
///Field `AXISRAM5PUB` writer - Defines the public protection of the AXISRAM5 bus configuration bits.
pub type AXISRAM5PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM6PUB` reader - Defines the public protection of the AXISRAM6 bus configuration bits.
pub type AXISRAM6PUB_R = crate::BitReader;
///Field `AXISRAM6PUB` writer - Defines the public protection of the AXISRAM6 bus configuration bits.
pub type AXISRAM6PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM1PUB` reader - Defines the public protection of the AHBSRAM1 bus configuration bits.
pub type AHBSRAM1PUB_R = crate::BitReader;
///Field `AHBSRAM1PUB` writer - Defines the public protection of the AHBSRAM1 bus configuration bits.
pub type AHBSRAM1PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBSRAM2PUB` reader - Defines the public protection of the AHBSRAM2 bus configuration bits.
pub type AHBSRAM2PUB_R = crate::BitReader;
///Field `AHBSRAM2PUB` writer - Defines the public protection of the AHBSRAM2 bus configuration bits.
pub type AHBSRAM2PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMPUB` reader - Defines the public protection of the BKPSRAM bus configuration bits.
pub type BKPSRAMPUB_R = crate::BitReader;
///Field `BKPSRAMPUB` writer - Defines the public protection of the BKPSRAM bus configuration bits.
pub type BKPSRAMPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM1PUB` reader - Defines the public protection of the AXISRAM1 bus configuration bits.
pub type AXISRAM1PUB_R = crate::BitReader;
///Field `AXISRAM1PUB` writer - Defines the public protection of the AXISRAM1 bus configuration bits.
pub type AXISRAM1PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXISRAM2PUB` reader - Defines the public protection of the AXISRAM2 bus configuration bits.
pub type AXISRAM2PUB_R = crate::BitReader;
///Field `AXISRAM2PUB` writer - Defines the public protection of the AXISRAM2 bus configuration bits.
pub type AXISRAM2PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLEXRAMPUB` reader - Defines the public protection of the FLEXRAM bus configuration bits.
pub type FLEXRAMPUB_R = crate::BitReader;
///Field `FLEXRAMPUB` writer - Defines the public protection of the FLEXRAM bus configuration bits.
pub type FLEXRAMPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPUCACHERAMPUB` reader - Defines the public protection of the NPUCACHERAM bus configuration bits.
pub type NPUCACHERAMPUB_R = crate::BitReader;
///Field `NPUCACHERAMPUB` writer - Defines the public protection of the NPUCACHERAM bus configuration bits.
pub type NPUCACHERAMPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VENCRAMPUB` reader - Defines the public protection of the VENCRAM bus configuration bits.
pub type VENCRAMPUB_R = crate::BitReader;
///Field `VENCRAMPUB` writer - Defines the public protection of the VENCRAM bus configuration bits.
pub type VENCRAMPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the public protection of the AXISRAM3 bus configuration bits.
    #[inline(always)]
    pub fn axisram3pub(&self) -> AXISRAM3PUB_R {
        AXISRAM3PUB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the public protection of the AXISRAM4 bus configuration bits.
    #[inline(always)]
    pub fn axisram4pub(&self) -> AXISRAM4PUB_R {
        AXISRAM4PUB_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the public protection of the AXISRAM5 bus configuration bits.
    #[inline(always)]
    pub fn axisram5pub(&self) -> AXISRAM5PUB_R {
        AXISRAM5PUB_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the public protection of the AXISRAM6 bus configuration bits.
    #[inline(always)]
    pub fn axisram6pub(&self) -> AXISRAM6PUB_R {
        AXISRAM6PUB_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Defines the public protection of the AHBSRAM1 bus configuration bits.
    #[inline(always)]
    pub fn ahbsram1pub(&self) -> AHBSRAM1PUB_R {
        AHBSRAM1PUB_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Defines the public protection of the AHBSRAM2 bus configuration bits.
    #[inline(always)]
    pub fn ahbsram2pub(&self) -> AHBSRAM2PUB_R {
        AHBSRAM2PUB_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Defines the public protection of the BKPSRAM bus configuration bits.
    #[inline(always)]
    pub fn bkpsrampub(&self) -> BKPSRAMPUB_R {
        BKPSRAMPUB_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Defines the public protection of the AXISRAM1 bus configuration bits.
    #[inline(always)]
    pub fn axisram1pub(&self) -> AXISRAM1PUB_R {
        AXISRAM1PUB_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Defines the public protection of the AXISRAM2 bus configuration bits.
    #[inline(always)]
    pub fn axisram2pub(&self) -> AXISRAM2PUB_R {
        AXISRAM2PUB_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Defines the public protection of the FLEXRAM bus configuration bits.
    #[inline(always)]
    pub fn flexrampub(&self) -> FLEXRAMPUB_R {
        FLEXRAMPUB_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Defines the public protection of the NPUCACHERAM bus configuration bits.
    #[inline(always)]
    pub fn npucacherampub(&self) -> NPUCACHERAMPUB_R {
        NPUCACHERAMPUB_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Defines the public protection of the VENCRAM bus configuration bits.
    #[inline(always)]
    pub fn vencrampub(&self) -> VENCRAMPUB_R {
        VENCRAMPUB_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUBCFGR5")
            .field("axisram3pub", &self.axisram3pub())
            .field("axisram4pub", &self.axisram4pub())
            .field("axisram5pub", &self.axisram5pub())
            .field("axisram6pub", &self.axisram6pub())
            .field("ahbsram1pub", &self.ahbsram1pub())
            .field("ahbsram2pub", &self.ahbsram2pub())
            .field("bkpsrampub", &self.bkpsrampub())
            .field("axisram1pub", &self.axisram1pub())
            .field("axisram2pub", &self.axisram2pub())
            .field("flexrampub", &self.flexrampub())
            .field("npucacherampub", &self.npucacherampub())
            .field("vencrampub", &self.vencrampub())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the AXISRAM3 bus configuration bits.
    #[inline(always)]
    pub fn axisram3pub(&mut self) -> AXISRAM3PUB_W<'_, PUBCFGR5rs> {
        AXISRAM3PUB_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the AXISRAM4 bus configuration bits.
    #[inline(always)]
    pub fn axisram4pub(&mut self) -> AXISRAM4PUB_W<'_, PUBCFGR5rs> {
        AXISRAM4PUB_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the AXISRAM5 bus configuration bits.
    #[inline(always)]
    pub fn axisram5pub(&mut self) -> AXISRAM5PUB_W<'_, PUBCFGR5rs> {
        AXISRAM5PUB_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the AXISRAM6 bus configuration bits.
    #[inline(always)]
    pub fn axisram6pub(&mut self) -> AXISRAM6PUB_W<'_, PUBCFGR5rs> {
        AXISRAM6PUB_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the AHBSRAM1 bus configuration bits.
    #[inline(always)]
    pub fn ahbsram1pub(&mut self) -> AHBSRAM1PUB_W<'_, PUBCFGR5rs> {
        AHBSRAM1PUB_W::new(self, 4)
    }
    ///Bit 5 - Defines the public protection of the AHBSRAM2 bus configuration bits.
    #[inline(always)]
    pub fn ahbsram2pub(&mut self) -> AHBSRAM2PUB_W<'_, PUBCFGR5rs> {
        AHBSRAM2PUB_W::new(self, 5)
    }
    ///Bit 6 - Defines the public protection of the BKPSRAM bus configuration bits.
    #[inline(always)]
    pub fn bkpsrampub(&mut self) -> BKPSRAMPUB_W<'_, PUBCFGR5rs> {
        BKPSRAMPUB_W::new(self, 6)
    }
    ///Bit 7 - Defines the public protection of the AXISRAM1 bus configuration bits.
    #[inline(always)]
    pub fn axisram1pub(&mut self) -> AXISRAM1PUB_W<'_, PUBCFGR5rs> {
        AXISRAM1PUB_W::new(self, 7)
    }
    ///Bit 8 - Defines the public protection of the AXISRAM2 bus configuration bits.
    #[inline(always)]
    pub fn axisram2pub(&mut self) -> AXISRAM2PUB_W<'_, PUBCFGR5rs> {
        AXISRAM2PUB_W::new(self, 8)
    }
    ///Bit 9 - Defines the public protection of the FLEXRAM bus configuration bits.
    #[inline(always)]
    pub fn flexrampub(&mut self) -> FLEXRAMPUB_W<'_, PUBCFGR5rs> {
        FLEXRAMPUB_W::new(self, 9)
    }
    ///Bit 10 - Defines the public protection of the NPUCACHERAM bus configuration bits.
    #[inline(always)]
    pub fn npucacherampub(&mut self) -> NPUCACHERAMPUB_W<'_, PUBCFGR5rs> {
        NPUCACHERAMPUB_W::new(self, 10)
    }
    ///Bit 11 - Defines the public protection of the VENCRAM bus configuration bits.
    #[inline(always)]
    pub fn vencrampub(&mut self) -> VENCRAMPUB_W<'_, PUBCFGR5rs> {
        VENCRAMPUB_W::new(self, 11)
    }
}
/**RCC bus public configuration register4

You can [`read`](crate::Reg::read) this register and get [`pubcfgr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:PUBCFGR5)*/
pub struct PUBCFGR5rs;
impl crate::RegisterSpec for PUBCFGR5rs {
    type Ux = u32;
}
///`read()` method returns [`pubcfgr5::R`](R) reader structure
impl crate::Readable for PUBCFGR5rs {}
///`write(|w| ..)` method takes [`pubcfgr5::W`](W) writer structure
impl crate::Writable for PUBCFGR5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGR5 to value 0
impl crate::Resettable for PUBCFGR5rs {}
