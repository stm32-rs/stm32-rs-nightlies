///Register `SECCFGR4` reader
pub type R = crate::R<SECCFGR4rs>;
///Register `SECCFGR4` writer
pub type W = crate::W<SECCFGR4rs>;
///Field `ACLKNSEC` reader - Defines the secure protection of the ACLKN bus configuration bits.
pub type ACLKNSEC_R = crate::BitReader;
///Field `ACLKNSEC` writer - Defines the secure protection of the ACLKN bus configuration bits.
pub type ACLKNSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACLKNCSEC` reader - Defines the secure protection of the ACLKNC bus configuration bits.
pub type ACLKNCSEC_R = crate::BitReader;
///Field `ACLKNCSEC` writer - Defines the secure protection of the ACLKNC bus configuration bits.
pub type ACLKNCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHBMSEC` reader - Defines the secure protection of the AHBM bus configuration bits.
pub type AHBMSEC_R = crate::BitReader;
///Field `AHBMSEC` writer - Defines the secure protection of the AHBM bus configuration bits.
pub type AHBMSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB1SEC` reader - Defines the secure protection of the AHB1 bus configuration bits.
pub type AHB1SEC_R = crate::BitReader;
///Field `AHB1SEC` writer - Defines the secure protection of the AHB1 bus configuration bits.
pub type AHB1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB2SEC` reader - Defines the secure protection of the AHB2 bus configuration bits.
pub type AHB2SEC_R = crate::BitReader;
///Field `AHB2SEC` writer - Defines the secure protection of the AHB2 bus configuration bits.
pub type AHB2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB3SEC` reader - Defines the secure protection of the AHB3 bus configuration bits.
pub type AHB3SEC_R = crate::BitReader;
///Field `AHB3SEC` writer - Defines the secure protection of the AHB3 bus configuration bits.
pub type AHB3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB4SEC` reader - Defines the secure protection of the AHB4 bus configuration bits.
pub type AHB4SEC_R = crate::BitReader;
///Field `AHB4SEC` writer - Defines the secure protection of the AHB4 bus configuration bits.
pub type AHB4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB5SEC` reader - Defines the secure protection of the AHB5 bus configuration bits.
pub type AHB5SEC_R = crate::BitReader;
///Field `AHB5SEC` writer - Defines the secure protection of the AHB5 bus configuration bits.
pub type AHB5SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB1SEC` reader - Defines the secure protection of the APB1 bus configuration bits.
pub type APB1SEC_R = crate::BitReader;
///Field `APB1SEC` writer - Defines the secure protection of the APB1 bus configuration bits.
pub type APB1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB2SEC` reader - Defines the secure protection of the APB2 bus configuration bits.
pub type APB2SEC_R = crate::BitReader;
///Field `APB2SEC` writer - Defines the secure protection of the APB2 bus configuration bits.
pub type APB2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB3SEC` reader - Defines the secure protection of the APB3 bus configuration bits.
pub type APB3SEC_R = crate::BitReader;
///Field `APB3SEC` writer - Defines the secure protection of the APB3 bus configuration bits.
pub type APB3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB4SEC` reader - Defines the secure protection of the APB4 bus configuration bits.
pub type APB4SEC_R = crate::BitReader;
///Field `APB4SEC` writer - Defines the secure protection of the APB4 bus configuration bits.
pub type APB4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB5SEC` reader - Defines the secure protection of the APB5 bus configuration bits.
pub type APB5SEC_R = crate::BitReader;
///Field `APB5SEC` writer - Defines the secure protection of the APB5 bus configuration bits.
pub type APB5SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOCSEC` reader - Defines the secure protection of the NOC bus configuration bits.
pub type NOCSEC_R = crate::BitReader;
///Field `NOCSEC` writer - Defines the secure protection of the NOC bus configuration bits.
pub type NOCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the secure protection of the ACLKN bus configuration bits.
    #[inline(always)]
    pub fn aclknsec(&self) -> ACLKNSEC_R {
        ACLKNSEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the secure protection of the ACLKNC bus configuration bits.
    #[inline(always)]
    pub fn aclkncsec(&self) -> ACLKNCSEC_R {
        ACLKNCSEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the secure protection of the AHBM bus configuration bits.
    #[inline(always)]
    pub fn ahbmsec(&self) -> AHBMSEC_R {
        AHBMSEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the secure protection of the AHB1 bus configuration bits.
    #[inline(always)]
    pub fn ahb1sec(&self) -> AHB1SEC_R {
        AHB1SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Defines the secure protection of the AHB2 bus configuration bits.
    #[inline(always)]
    pub fn ahb2sec(&self) -> AHB2SEC_R {
        AHB2SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Defines the secure protection of the AHB3 bus configuration bits.
    #[inline(always)]
    pub fn ahb3sec(&self) -> AHB3SEC_R {
        AHB3SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Defines the secure protection of the AHB4 bus configuration bits.
    #[inline(always)]
    pub fn ahb4sec(&self) -> AHB4SEC_R {
        AHB4SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Defines the secure protection of the AHB5 bus configuration bits.
    #[inline(always)]
    pub fn ahb5sec(&self) -> AHB5SEC_R {
        AHB5SEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Defines the secure protection of the APB1 bus configuration bits.
    #[inline(always)]
    pub fn apb1sec(&self) -> APB1SEC_R {
        APB1SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Defines the secure protection of the APB2 bus configuration bits.
    #[inline(always)]
    pub fn apb2sec(&self) -> APB2SEC_R {
        APB2SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Defines the secure protection of the APB3 bus configuration bits.
    #[inline(always)]
    pub fn apb3sec(&self) -> APB3SEC_R {
        APB3SEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Defines the secure protection of the APB4 bus configuration bits.
    #[inline(always)]
    pub fn apb4sec(&self) -> APB4SEC_R {
        APB4SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Defines the secure protection of the APB5 bus configuration bits.
    #[inline(always)]
    pub fn apb5sec(&self) -> APB5SEC_R {
        APB5SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Defines the secure protection of the NOC bus configuration bits.
    #[inline(always)]
    pub fn nocsec(&self) -> NOCSEC_R {
        NOCSEC_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR4")
            .field("aclknsec", &self.aclknsec())
            .field("aclkncsec", &self.aclkncsec())
            .field("ahbmsec", &self.ahbmsec())
            .field("ahb1sec", &self.ahb1sec())
            .field("ahb2sec", &self.ahb2sec())
            .field("ahb3sec", &self.ahb3sec())
            .field("ahb4sec", &self.ahb4sec())
            .field("ahb5sec", &self.ahb5sec())
            .field("apb1sec", &self.apb1sec())
            .field("apb2sec", &self.apb2sec())
            .field("apb3sec", &self.apb3sec())
            .field("apb4sec", &self.apb4sec())
            .field("apb5sec", &self.apb5sec())
            .field("nocsec", &self.nocsec())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the secure protection of the ACLKN bus configuration bits.
    #[inline(always)]
    pub fn aclknsec(&mut self) -> ACLKNSEC_W<'_, SECCFGR4rs> {
        ACLKNSEC_W::new(self, 0)
    }
    ///Bit 1 - Defines the secure protection of the ACLKNC bus configuration bits.
    #[inline(always)]
    pub fn aclkncsec(&mut self) -> ACLKNCSEC_W<'_, SECCFGR4rs> {
        ACLKNCSEC_W::new(self, 1)
    }
    ///Bit 2 - Defines the secure protection of the AHBM bus configuration bits.
    #[inline(always)]
    pub fn ahbmsec(&mut self) -> AHBMSEC_W<'_, SECCFGR4rs> {
        AHBMSEC_W::new(self, 2)
    }
    ///Bit 3 - Defines the secure protection of the AHB1 bus configuration bits.
    #[inline(always)]
    pub fn ahb1sec(&mut self) -> AHB1SEC_W<'_, SECCFGR4rs> {
        AHB1SEC_W::new(self, 3)
    }
    ///Bit 4 - Defines the secure protection of the AHB2 bus configuration bits.
    #[inline(always)]
    pub fn ahb2sec(&mut self) -> AHB2SEC_W<'_, SECCFGR4rs> {
        AHB2SEC_W::new(self, 4)
    }
    ///Bit 5 - Defines the secure protection of the AHB3 bus configuration bits.
    #[inline(always)]
    pub fn ahb3sec(&mut self) -> AHB3SEC_W<'_, SECCFGR4rs> {
        AHB3SEC_W::new(self, 5)
    }
    ///Bit 6 - Defines the secure protection of the AHB4 bus configuration bits.
    #[inline(always)]
    pub fn ahb4sec(&mut self) -> AHB4SEC_W<'_, SECCFGR4rs> {
        AHB4SEC_W::new(self, 6)
    }
    ///Bit 7 - Defines the secure protection of the AHB5 bus configuration bits.
    #[inline(always)]
    pub fn ahb5sec(&mut self) -> AHB5SEC_W<'_, SECCFGR4rs> {
        AHB5SEC_W::new(self, 7)
    }
    ///Bit 8 - Defines the secure protection of the APB1 bus configuration bits.
    #[inline(always)]
    pub fn apb1sec(&mut self) -> APB1SEC_W<'_, SECCFGR4rs> {
        APB1SEC_W::new(self, 8)
    }
    ///Bit 9 - Defines the secure protection of the APB2 bus configuration bits.
    #[inline(always)]
    pub fn apb2sec(&mut self) -> APB2SEC_W<'_, SECCFGR4rs> {
        APB2SEC_W::new(self, 9)
    }
    ///Bit 10 - Defines the secure protection of the APB3 bus configuration bits.
    #[inline(always)]
    pub fn apb3sec(&mut self) -> APB3SEC_W<'_, SECCFGR4rs> {
        APB3SEC_W::new(self, 10)
    }
    ///Bit 11 - Defines the secure protection of the APB4 bus configuration bits.
    #[inline(always)]
    pub fn apb4sec(&mut self) -> APB4SEC_W<'_, SECCFGR4rs> {
        APB4SEC_W::new(self, 11)
    }
    ///Bit 12 - Defines the secure protection of the APB5 bus configuration bits.
    #[inline(always)]
    pub fn apb5sec(&mut self) -> APB5SEC_W<'_, SECCFGR4rs> {
        APB5SEC_W::new(self, 12)
    }
    ///Bit 13 - Defines the secure protection of the NOC bus configuration bits.
    #[inline(always)]
    pub fn nocsec(&mut self) -> NOCSEC_W<'_, SECCFGR4rs> {
        NOCSEC_W::new(self, 13)
    }
}
/**RCC bus secure configuration register4

You can [`read`](crate::Reg::read) this register and get [`seccfgr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:SECCFGR4)*/
pub struct SECCFGR4rs;
impl crate::RegisterSpec for SECCFGR4rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr4::R`](R) reader structure
impl crate::Readable for SECCFGR4rs {}
///`write(|w| ..)` method takes [`seccfgr4::W`](W) writer structure
impl crate::Writable for SECCFGR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR4 to value 0
impl crate::Resettable for SECCFGR4rs {}
