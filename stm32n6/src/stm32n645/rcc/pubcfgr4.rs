///Register `PUBCFGR4` reader
pub type R = crate::R<PUBCFGR4rs>;
///Register `PUBCFGR4` writer
pub type W = crate::W<PUBCFGR4rs>;
///Field `ACLKNPUB` reader - Defines the public protection of the ACLKN bus configuration bits.
pub type ACLKNPUB_R = crate::BitReader;
///Field `ACLKNPUB` writer - Defines the public protection of the ACLKN bus configuration bits.
pub type ACLKNPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCPUB` reader - Defines the public protection of the ACLKNC bus configuration bits.
pub type ACLKNCPUB_R = crate::BitReader;
///Field `ACLKNCPUB` writer - Defines the public protection of the ACLKNC bus configuration bits.
pub type ACLKNCPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMPUB` reader - Defines the public protection of the AHBM bus configuration bits.
pub type AHBMPUB_R = crate::BitReader;
///Field `AHBMPUB` writer - Defines the public protection of the AHBM bus configuration bits.
pub type AHBMPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1PUB` reader - Defines the public protection of the AHB1 bus configuration bits.
pub type AHB1PUB_R = crate::BitReader;
///Field `AHB1PUB` writer - Defines the public protection of the AHB1 bus configuration bits.
pub type AHB1PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2PUB` reader - Defines the public protection of the AHB2 bus configuration bits.
pub type AHB2PUB_R = crate::BitReader;
///Field `AHB2PUB` writer - Defines the public protection of the AHB2 bus configuration bits.
pub type AHB2PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3PUB` reader - Defines the public protection of the AHB3 bus configuration bits.
pub type AHB3PUB_R = crate::BitReader;
///Field `AHB3PUB` writer - Defines the public protection of the AHB3 bus configuration bits.
pub type AHB3PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4PUB` reader - Defines the public protection of the AHB4 bus configuration bits.
pub type AHB4PUB_R = crate::BitReader;
///Field `AHB4PUB` writer - Defines the public protection of the AHB4 bus configuration bits.
pub type AHB4PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5PUB` reader - Defines the public protection of the AHB5 bus configuration bits.
pub type AHB5PUB_R = crate::BitReader;
///Field `AHB5PUB` writer - Defines the public protection of the AHB5 bus configuration bits.
pub type AHB5PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1PUB` reader - Defines the public protection of the APB1 bus configuration bits.
pub type APB1PUB_R = crate::BitReader;
///Field `APB1PUB` writer - Defines the public protection of the APB1 bus configuration bits.
pub type APB1PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2PUB` reader - Defines the public protection of the APB2 bus configuration bits.
pub type APB2PUB_R = crate::BitReader;
///Field `APB2PUB` writer - Defines the public protection of the APB2 bus configuration bits.
pub type APB2PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3PUB` reader - Defines the public protection of the APB3 bus configuration bits.
pub type APB3PUB_R = crate::BitReader;
///Field `APB3PUB` writer - Defines the public protection of the APB3 bus configuration bits.
pub type APB3PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4PUB` reader - Defines the public protection of the APB4 bus configuration bits.
pub type APB4PUB_R = crate::BitReader;
///Field `APB4PUB` writer - Defines the public protection of the APB4 bus configuration bits.
pub type APB4PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5PUB` reader - Defines the public protection of the APB5 bus configuration bits.
pub type APB5PUB_R = crate::BitReader;
///Field `APB5PUB` writer - Defines the public protection of the APB5 bus configuration bits.
pub type APB5PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOCPUB` reader - Defines the public protection of the NOC bus configuration bits.
pub type NOCPUB_R = crate::BitReader;
///Field `NOCPUB` writer - Defines the public protection of the NOC bus configuration bits.
pub type NOCPUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the public protection of the ACLKN bus configuration bits.
    #[inline(always)]
    pub fn aclknpub(&self) -> ACLKNPUB_R {
        ACLKNPUB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the public protection of the ACLKNC bus configuration bits.
    #[inline(always)]
    pub fn aclkncpub(&self) -> ACLKNCPUB_R {
        ACLKNCPUB_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the public protection of the AHBM bus configuration bits.
    #[inline(always)]
    pub fn ahbmpub(&self) -> AHBMPUB_R {
        AHBMPUB_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the public protection of the AHB1 bus configuration bits.
    #[inline(always)]
    pub fn ahb1pub(&self) -> AHB1PUB_R {
        AHB1PUB_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Defines the public protection of the AHB2 bus configuration bits.
    #[inline(always)]
    pub fn ahb2pub(&self) -> AHB2PUB_R {
        AHB2PUB_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Defines the public protection of the AHB3 bus configuration bits.
    #[inline(always)]
    pub fn ahb3pub(&self) -> AHB3PUB_R {
        AHB3PUB_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Defines the public protection of the AHB4 bus configuration bits.
    #[inline(always)]
    pub fn ahb4pub(&self) -> AHB4PUB_R {
        AHB4PUB_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Defines the public protection of the AHB5 bus configuration bits.
    #[inline(always)]
    pub fn ahb5pub(&self) -> AHB5PUB_R {
        AHB5PUB_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Defines the public protection of the APB1 bus configuration bits.
    #[inline(always)]
    pub fn apb1pub(&self) -> APB1PUB_R {
        APB1PUB_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Defines the public protection of the APB2 bus configuration bits.
    #[inline(always)]
    pub fn apb2pub(&self) -> APB2PUB_R {
        APB2PUB_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Defines the public protection of the APB3 bus configuration bits.
    #[inline(always)]
    pub fn apb3pub(&self) -> APB3PUB_R {
        APB3PUB_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Defines the public protection of the APB4 bus configuration bits.
    #[inline(always)]
    pub fn apb4pub(&self) -> APB4PUB_R {
        APB4PUB_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Defines the public protection of the APB5 bus configuration bits.
    #[inline(always)]
    pub fn apb5pub(&self) -> APB5PUB_R {
        APB5PUB_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Defines the public protection of the NOC bus configuration bits.
    #[inline(always)]
    pub fn nocpub(&self) -> NOCPUB_R {
        NOCPUB_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUBCFGR4")
            .field("aclknpub", &self.aclknpub())
            .field("aclkncpub", &self.aclkncpub())
            .field("ahbmpub", &self.ahbmpub())
            .field("ahb1pub", &self.ahb1pub())
            .field("ahb2pub", &self.ahb2pub())
            .field("ahb3pub", &self.ahb3pub())
            .field("ahb4pub", &self.ahb4pub())
            .field("ahb5pub", &self.ahb5pub())
            .field("apb1pub", &self.apb1pub())
            .field("apb2pub", &self.apb2pub())
            .field("apb3pub", &self.apb3pub())
            .field("apb4pub", &self.apb4pub())
            .field("apb5pub", &self.apb5pub())
            .field("nocpub", &self.nocpub())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the ACLKN bus configuration bits.
    #[inline(always)]
    pub fn aclknpub(&mut self) -> ACLKNPUB_W<'_, PUBCFGR4rs> {
        ACLKNPUB_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the ACLKNC bus configuration bits.
    #[inline(always)]
    pub fn aclkncpub(&mut self) -> ACLKNCPUB_W<'_, PUBCFGR4rs> {
        ACLKNCPUB_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the AHBM bus configuration bits.
    #[inline(always)]
    pub fn ahbmpub(&mut self) -> AHBMPUB_W<'_, PUBCFGR4rs> {
        AHBMPUB_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the AHB1 bus configuration bits.
    #[inline(always)]
    pub fn ahb1pub(&mut self) -> AHB1PUB_W<'_, PUBCFGR4rs> {
        AHB1PUB_W::new(self, 3)
    }
    ///Bit 4 - Defines the public protection of the AHB2 bus configuration bits.
    #[inline(always)]
    pub fn ahb2pub(&mut self) -> AHB2PUB_W<'_, PUBCFGR4rs> {
        AHB2PUB_W::new(self, 4)
    }
    ///Bit 5 - Defines the public protection of the AHB3 bus configuration bits.
    #[inline(always)]
    pub fn ahb3pub(&mut self) -> AHB3PUB_W<'_, PUBCFGR4rs> {
        AHB3PUB_W::new(self, 5)
    }
    ///Bit 6 - Defines the public protection of the AHB4 bus configuration bits.
    #[inline(always)]
    pub fn ahb4pub(&mut self) -> AHB4PUB_W<'_, PUBCFGR4rs> {
        AHB4PUB_W::new(self, 6)
    }
    ///Bit 7 - Defines the public protection of the AHB5 bus configuration bits.
    #[inline(always)]
    pub fn ahb5pub(&mut self) -> AHB5PUB_W<'_, PUBCFGR4rs> {
        AHB5PUB_W::new(self, 7)
    }
    ///Bit 8 - Defines the public protection of the APB1 bus configuration bits.
    #[inline(always)]
    pub fn apb1pub(&mut self) -> APB1PUB_W<'_, PUBCFGR4rs> {
        APB1PUB_W::new(self, 8)
    }
    ///Bit 9 - Defines the public protection of the APB2 bus configuration bits.
    #[inline(always)]
    pub fn apb2pub(&mut self) -> APB2PUB_W<'_, PUBCFGR4rs> {
        APB2PUB_W::new(self, 9)
    }
    ///Bit 10 - Defines the public protection of the APB3 bus configuration bits.
    #[inline(always)]
    pub fn apb3pub(&mut self) -> APB3PUB_W<'_, PUBCFGR4rs> {
        APB3PUB_W::new(self, 10)
    }
    ///Bit 11 - Defines the public protection of the APB4 bus configuration bits.
    #[inline(always)]
    pub fn apb4pub(&mut self) -> APB4PUB_W<'_, PUBCFGR4rs> {
        APB4PUB_W::new(self, 11)
    }
    ///Bit 12 - Defines the public protection of the APB5 bus configuration bits.
    #[inline(always)]
    pub fn apb5pub(&mut self) -> APB5PUB_W<'_, PUBCFGR4rs> {
        APB5PUB_W::new(self, 12)
    }
    ///Bit 13 - Defines the public protection of the NOC bus configuration bits.
    #[inline(always)]
    pub fn nocpub(&mut self) -> NOCPUB_W<'_, PUBCFGR4rs> {
        NOCPUB_W::new(self, 13)
    }
}
/**RCC bus public configuration register4

You can [`read`](crate::Reg::read) this register and get [`pubcfgr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:PUBCFGR4)*/
pub struct PUBCFGR4rs;
impl crate::RegisterSpec for PUBCFGR4rs {
    type Ux = u32;
}
///`read()` method returns [`pubcfgr4::R`](R) reader structure
impl crate::Readable for PUBCFGR4rs {}
///`write(|w| ..)` method takes [`pubcfgr4::W`](W) writer structure
impl crate::Writable for PUBCFGR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGR4 to value 0
impl crate::Resettable for PUBCFGR4rs {}
