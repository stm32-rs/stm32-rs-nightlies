///Register `MP_AHB5ENSETR` reader
pub type R = crate::R<MP_AHB5ENSETRrs>;
///Register `MP_AHB5ENSETR` writer
pub type W = crate::W<MP_AHB5ENSETRrs>;
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
///Field `AXIMCEN` reader - AXIMCEN
pub type AXIMCEN_R = crate::BitReader;
///Field `AXIMCEN` writer - AXIMCEN
pub type AXIMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 16 - AXIMCEN
    #[inline(always)]
    pub fn aximcen(&self) -> AXIMCEN_R {
        AXIMCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MP_AHB5ENSETR")
            .field("gpiozen", &self.gpiozen())
            .field("cryp1en", &self.cryp1en())
            .field("hash1en", &self.hash1en())
            .field("rng1en", &self.rng1en())
            .field("bkpsramen", &self.bkpsramen())
            .field("aximcen", &self.aximcen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOZEN
    #[inline(always)]
    pub fn gpiozen(&mut self) -> GPIOZEN_W<'_, MP_AHB5ENSETRrs> {
        GPIOZEN_W::new(self, 0)
    }
    ///Bit 4 - CRYP1EN
    #[inline(always)]
    pub fn cryp1en(&mut self) -> CRYP1EN_W<'_, MP_AHB5ENSETRrs> {
        CRYP1EN_W::new(self, 4)
    }
    ///Bit 5 - HASH1EN
    #[inline(always)]
    pub fn hash1en(&mut self) -> HASH1EN_W<'_, MP_AHB5ENSETRrs> {
        HASH1EN_W::new(self, 5)
    }
    ///Bit 6 - RNG1EN
    #[inline(always)]
    pub fn rng1en(&mut self) -> RNG1EN_W<'_, MP_AHB5ENSETRrs> {
        RNG1EN_W::new(self, 6)
    }
    ///Bit 8 - BKPSRAMEN
    #[inline(always)]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W<'_, MP_AHB5ENSETRrs> {
        BKPSRAMEN_W::new(self, 8)
    }
    ///Bit 16 - AXIMCEN
    #[inline(always)]
    pub fn aximcen(&mut self) -> AXIMCEN_W<'_, MP_AHB5ENSETRrs> {
        AXIMCEN_W::new(self, 16)
    }
}
/**This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`mp_ahb5ensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mp_ahb5ensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MP_AHB5ENSETR)*/
pub struct MP_AHB5ENSETRrs;
impl crate::RegisterSpec for MP_AHB5ENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`mp_ahb5ensetr::R`](R) reader structure
impl crate::Readable for MP_AHB5ENSETRrs {}
///`write(|w| ..)` method takes [`mp_ahb5ensetr::W`](W) writer structure
impl crate::Writable for MP_AHB5ENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MP_AHB5ENSETR to value 0x0001_0000
impl crate::Resettable for MP_AHB5ENSETRrs {
    const RESET_VALUE: u32 = 0x0001_0000;
}
