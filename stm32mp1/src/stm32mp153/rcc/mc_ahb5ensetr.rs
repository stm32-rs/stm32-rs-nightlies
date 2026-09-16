///Register `MC_AHB5ENSETR` reader
pub type R = crate::R<MC_AHB5ENSETRrs>;
///Register `MC_AHB5ENSETR` writer
pub type W = crate::W<MC_AHB5ENSETRrs>;
///Field `GPIOZEN` reader - GPIOZEN
pub type GPIOZEN_R = crate::BitReader;
///Field `GPIOZEN` writer - GPIOZEN
pub type GPIOZEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYP1EN` reader - CRYP1EN
pub type CRYP1EN_R = crate::BitReader;
///Field `CRYP1EN` writer - CRYP1EN
pub type CRYP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH1EN` reader - HASH1EN
pub type HASH1EN_R = crate::BitReader;
///Field `HASH1EN` writer - HASH1EN
pub type HASH1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNG1EN` reader - RNG1EN
pub type RNG1EN_R = crate::BitReader;
///Field `RNG1EN` writer - RNG1EN
pub type RNG1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKPSRAMEN` reader - BKPSRAMEN
pub type BKPSRAMEN_R = crate::BitReader;
///Field `BKPSRAMEN` writer - BKPSRAMEN
pub type BKPSRAMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIOZEN
    #[inline(always)]
    pub fn gpiozen(&self) -> GPIOZEN_R {
        GPIOZEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CRYP1EN
    #[inline(always)]
    pub fn cryp1en(&self) -> CRYP1EN_R {
        CRYP1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH1EN
    #[inline(always)]
    pub fn hash1en(&self) -> HASH1EN_R {
        HASH1EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG1EN
    #[inline(always)]
    pub fn rng1en(&self) -> RNG1EN_R {
        RNG1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - BKPSRAMEN
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_AHB5ENSETR")
            .field("gpiozen", &self.gpiozen())
            .field("cryp1en", &self.cryp1en())
            .field("hash1en", &self.hash1en())
            .field("rng1en", &self.rng1en())
            .field("bkpsramen", &self.bkpsramen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOZEN
    #[inline(always)]
    pub fn gpiozen(&mut self) -> GPIOZEN_W<'_, MC_AHB5ENSETRrs> {
        GPIOZEN_W::new(self, 0)
    }
    ///Bit 4 - CRYP1EN
    #[inline(always)]
    pub fn cryp1en(&mut self) -> CRYP1EN_W<'_, MC_AHB5ENSETRrs> {
        CRYP1EN_W::new(self, 4)
    }
    ///Bit 5 - HASH1EN
    #[inline(always)]
    pub fn hash1en(&mut self) -> HASH1EN_W<'_, MC_AHB5ENSETRrs> {
        HASH1EN_W::new(self, 5)
    }
    ///Bit 6 - RNG1EN
    #[inline(always)]
    pub fn rng1en(&mut self) -> RNG1EN_W<'_, MC_AHB5ENSETRrs> {
        RNG1EN_W::new(self, 6)
    }
    ///Bit 8 - BKPSRAMEN
    #[inline(always)]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W<'_, MC_AHB5ENSETRrs> {
        BKPSRAMEN_W::new(self, 8)
    }
}
/**This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mc_ahb5ensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb5ensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:MC_AHB5ENSETR)*/
pub struct MC_AHB5ENSETRrs;
impl crate::RegisterSpec for MC_AHB5ENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_ahb5ensetr::R`](R) reader structure
impl crate::Readable for MC_AHB5ENSETRrs {}
///`write(|w| ..)` method takes [`mc_ahb5ensetr::W`](W) writer structure
impl crate::Writable for MC_AHB5ENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_AHB5ENSETR to value 0
impl crate::Resettable for MC_AHB5ENSETRrs {}
