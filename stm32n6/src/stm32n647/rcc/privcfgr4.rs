///Register `PRIVCFGR4` reader
pub type R = crate::R<PRIVCFGR4rs>;
///Register `PRIVCFGR4` writer
pub type W = crate::W<PRIVCFGR4rs>;
///Field `ACLKNPV` reader - Defines the privilege protection of the ACLKN bus configuration bits.
pub type ACLKNPV_R = crate::BitReader;
///Field `ACLKNPV` writer - Defines the privilege protection of the ACLKN bus configuration bits.
pub type ACLKNPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCPV` reader - Defines the privilege protection of the ACLKNC bus configuration bits.
pub type ACLKNCPV_R = crate::BitReader;
///Field `ACLKNCPV` writer - Defines the privilege protection of the ACLKNC bus configuration bits.
pub type ACLKNCPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMPV` reader - Defines the privilege protection of the AHBM bus configuration bits.
pub type AHBMPV_R = crate::BitReader;
///Field `AHBMPV` writer - Defines the privilege protection of the AHBM bus configuration bits.
pub type AHBMPV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1PV` reader - Defines the privilege protection of the AHB1 bus configuration bits.
pub type AHB1PV_R = crate::BitReader;
///Field `AHB1PV` writer - Defines the privilege protection of the AHB1 bus configuration bits.
pub type AHB1PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2PV` reader - Defines the privilege protection of the AHB2 bus configuration bits.
pub type AHB2PV_R = crate::BitReader;
///Field `AHB2PV` writer - Defines the privilege protection of the AHB2 bus configuration bits.
pub type AHB2PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3PV` reader - Defines the privilege protection of the AHB3 bus configuration bits.
pub type AHB3PV_R = crate::BitReader;
///Field `AHB3PV` writer - Defines the privilege protection of the AHB3 bus configuration bits.
pub type AHB3PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4PV` reader - Defines the privilege protection of the AHB4 bus configuration bits.
pub type AHB4PV_R = crate::BitReader;
///Field `AHB4PV` writer - Defines the privilege protection of the AHB4 bus configuration bits.
pub type AHB4PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5PV` reader - Defines the privilege protection of the AHB5 bus configuration bits.
pub type AHB5PV_R = crate::BitReader;
///Field `AHB5PV` writer - Defines the privilege protection of the AHB5 bus configuration bits.
pub type AHB5PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1PV` reader - Defines the privilege protection of the APB1 bus configuration bits.
pub type APB1PV_R = crate::BitReader;
///Field `APB1PV` writer - Defines the privilege protection of the APB1 bus configuration bits.
pub type APB1PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2PV` reader - Defines the privilege protection of the APB2 bus configuration bits.
pub type APB2PV_R = crate::BitReader;
///Field `APB2PV` writer - Defines the privilege protection of the APB2 bus configuration bits.
pub type APB2PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3PV` reader - Defines the privilege protection of the APB3 bus configuration bits.
pub type APB3PV_R = crate::BitReader;
///Field `APB3PV` writer - Defines the privilege protection of the APB3 bus configuration bits.
pub type APB3PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4PV` reader - Defines the privilege protection of the APB4 bus configuration bits.
pub type APB4PV_R = crate::BitReader;
///Field `APB4PV` writer - Defines the privilege protection of the APB4 bus configuration bits.
pub type APB4PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5PV` reader - Defines the privilege protection of the APB5 bus configuration bits.
pub type APB5PV_R = crate::BitReader;
///Field `APB5PV` writer - Defines the privilege protection of the APB5 bus configuration bits.
pub type APB5PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOCPV` reader - Defines the privilege protection of the NOC bus configuration bits.
pub type NOCPV_R = crate::BitReader;
///Field `NOCPV` writer - Defines the privilege protection of the NOC bus configuration bits.
pub type NOCPV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the privilege protection of the ACLKN bus configuration bits.
    #[inline(always)]
    pub fn aclknpv(&self) -> ACLKNPV_R {
        ACLKNPV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the privilege protection of the ACLKNC bus configuration bits.
    #[inline(always)]
    pub fn aclkncpv(&self) -> ACLKNCPV_R {
        ACLKNCPV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the privilege protection of the AHBM bus configuration bits.
    #[inline(always)]
    pub fn ahbmpv(&self) -> AHBMPV_R {
        AHBMPV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the privilege protection of the AHB1 bus configuration bits.
    #[inline(always)]
    pub fn ahb1pv(&self) -> AHB1PV_R {
        AHB1PV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Defines the privilege protection of the AHB2 bus configuration bits.
    #[inline(always)]
    pub fn ahb2pv(&self) -> AHB2PV_R {
        AHB2PV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Defines the privilege protection of the AHB3 bus configuration bits.
    #[inline(always)]
    pub fn ahb3pv(&self) -> AHB3PV_R {
        AHB3PV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Defines the privilege protection of the AHB4 bus configuration bits.
    #[inline(always)]
    pub fn ahb4pv(&self) -> AHB4PV_R {
        AHB4PV_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Defines the privilege protection of the AHB5 bus configuration bits.
    #[inline(always)]
    pub fn ahb5pv(&self) -> AHB5PV_R {
        AHB5PV_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Defines the privilege protection of the APB1 bus configuration bits.
    #[inline(always)]
    pub fn apb1pv(&self) -> APB1PV_R {
        APB1PV_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Defines the privilege protection of the APB2 bus configuration bits.
    #[inline(always)]
    pub fn apb2pv(&self) -> APB2PV_R {
        APB2PV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Defines the privilege protection of the APB3 bus configuration bits.
    #[inline(always)]
    pub fn apb3pv(&self) -> APB3PV_R {
        APB3PV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Defines the privilege protection of the APB4 bus configuration bits.
    #[inline(always)]
    pub fn apb4pv(&self) -> APB4PV_R {
        APB4PV_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Defines the privilege protection of the APB5 bus configuration bits.
    #[inline(always)]
    pub fn apb5pv(&self) -> APB5PV_R {
        APB5PV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Defines the privilege protection of the NOC bus configuration bits.
    #[inline(always)]
    pub fn nocpv(&self) -> NOCPV_R {
        NOCPV_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR4")
            .field("aclknpv", &self.aclknpv())
            .field("aclkncpv", &self.aclkncpv())
            .field("ahbmpv", &self.ahbmpv())
            .field("ahb1pv", &self.ahb1pv())
            .field("ahb2pv", &self.ahb2pv())
            .field("ahb3pv", &self.ahb3pv())
            .field("ahb4pv", &self.ahb4pv())
            .field("ahb5pv", &self.ahb5pv())
            .field("apb1pv", &self.apb1pv())
            .field("apb2pv", &self.apb2pv())
            .field("apb3pv", &self.apb3pv())
            .field("apb4pv", &self.apb4pv())
            .field("apb5pv", &self.apb5pv())
            .field("nocpv", &self.nocpv())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the ACLKN bus configuration bits.
    #[inline(always)]
    pub fn aclknpv(&mut self) -> ACLKNPV_W<'_, PRIVCFGR4rs> {
        ACLKNPV_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the ACLKNC bus configuration bits.
    #[inline(always)]
    pub fn aclkncpv(&mut self) -> ACLKNCPV_W<'_, PRIVCFGR4rs> {
        ACLKNCPV_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the AHBM bus configuration bits.
    #[inline(always)]
    pub fn ahbmpv(&mut self) -> AHBMPV_W<'_, PRIVCFGR4rs> {
        AHBMPV_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the AHB1 bus configuration bits.
    #[inline(always)]
    pub fn ahb1pv(&mut self) -> AHB1PV_W<'_, PRIVCFGR4rs> {
        AHB1PV_W::new(self, 3)
    }
    ///Bit 4 - Defines the privilege protection of the AHB2 bus configuration bits.
    #[inline(always)]
    pub fn ahb2pv(&mut self) -> AHB2PV_W<'_, PRIVCFGR4rs> {
        AHB2PV_W::new(self, 4)
    }
    ///Bit 5 - Defines the privilege protection of the AHB3 bus configuration bits.
    #[inline(always)]
    pub fn ahb3pv(&mut self) -> AHB3PV_W<'_, PRIVCFGR4rs> {
        AHB3PV_W::new(self, 5)
    }
    ///Bit 6 - Defines the privilege protection of the AHB4 bus configuration bits.
    #[inline(always)]
    pub fn ahb4pv(&mut self) -> AHB4PV_W<'_, PRIVCFGR4rs> {
        AHB4PV_W::new(self, 6)
    }
    ///Bit 7 - Defines the privilege protection of the AHB5 bus configuration bits.
    #[inline(always)]
    pub fn ahb5pv(&mut self) -> AHB5PV_W<'_, PRIVCFGR4rs> {
        AHB5PV_W::new(self, 7)
    }
    ///Bit 8 - Defines the privilege protection of the APB1 bus configuration bits.
    #[inline(always)]
    pub fn apb1pv(&mut self) -> APB1PV_W<'_, PRIVCFGR4rs> {
        APB1PV_W::new(self, 8)
    }
    ///Bit 9 - Defines the privilege protection of the APB2 bus configuration bits.
    #[inline(always)]
    pub fn apb2pv(&mut self) -> APB2PV_W<'_, PRIVCFGR4rs> {
        APB2PV_W::new(self, 9)
    }
    ///Bit 10 - Defines the privilege protection of the APB3 bus configuration bits.
    #[inline(always)]
    pub fn apb3pv(&mut self) -> APB3PV_W<'_, PRIVCFGR4rs> {
        APB3PV_W::new(self, 10)
    }
    ///Bit 11 - Defines the privilege protection of the APB4 bus configuration bits.
    #[inline(always)]
    pub fn apb4pv(&mut self) -> APB4PV_W<'_, PRIVCFGR4rs> {
        APB4PV_W::new(self, 11)
    }
    ///Bit 12 - Defines the privilege protection of the APB5 bus configuration bits.
    #[inline(always)]
    pub fn apb5pv(&mut self) -> APB5PV_W<'_, PRIVCFGR4rs> {
        APB5PV_W::new(self, 12)
    }
    ///Bit 13 - Defines the privilege protection of the NOC bus configuration bits.
    #[inline(always)]
    pub fn nocpv(&mut self) -> NOCPV_W<'_, PRIVCFGR4rs> {
        NOCPV_W::new(self, 13)
    }
}
/**RCC bus privilege configuration register4

You can [`read`](crate::Reg::read) this register and get [`privcfgr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGR4)*/
pub struct PRIVCFGR4rs;
impl crate::RegisterSpec for PRIVCFGR4rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr4::R`](R) reader structure
impl crate::Readable for PRIVCFGR4rs {}
///`write(|w| ..)` method takes [`privcfgr4::W`](W) writer structure
impl crate::Writable for PRIVCFGR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR4 to value 0
impl crate::Resettable for PRIVCFGR4rs {}
