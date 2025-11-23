///Register `MC_AHB3ENCLRR` reader
pub type R = crate::R<MC_AHB3ENCLRRrs>;
///Register `MC_AHB3ENCLRR` writer
pub type W = crate::W<MC_AHB3ENCLRRrs>;
///Field `DCMIEN` reader - DCMIEN
pub type DCMIEN_R = crate::BitReader;
///Field `DCMIEN` writer - DCMIEN
pub type DCMIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYP2EN` reader - CRYP2EN
pub type CRYP2EN_R = crate::BitReader;
///Field `CRYP2EN` writer - CRYP2EN
pub type CRYP2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASH2EN` reader - HASH2EN
pub type HASH2EN_R = crate::BitReader;
///Field `HASH2EN` writer - HASH2EN
pub type HASH2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNG2EN` reader - RNG2EN
pub type RNG2EN_R = crate::BitReader;
///Field `RNG2EN` writer - RNG2EN
pub type RNG2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRC2EN` reader - CRC2EN
pub type CRC2EN_R = crate::BitReader;
///Field `CRC2EN` writer - CRC2EN
pub type CRC2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEMEN` reader - HSEMEN
pub type HSEMEN_R = crate::BitReader;
///Field `HSEMEN` writer - HSEMEN
pub type HSEMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPCCEN` reader - IPCCEN
pub type IPCCEN_R = crate::BitReader;
///Field `IPCCEN` writer - IPCCEN
pub type IPCCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DCMIEN
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CRYP2EN
    #[inline(always)]
    pub fn cryp2en(&self) -> CRYP2EN_R {
        CRYP2EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH2EN
    #[inline(always)]
    pub fn hash2en(&self) -> HASH2EN_R {
        HASH2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG2EN
    #[inline(always)]
    pub fn rng2en(&self) -> RNG2EN_R {
        RNG2EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC2EN
    #[inline(always)]
    pub fn crc2en(&self) -> CRC2EN_R {
        CRC2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - HSEMEN
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - IPCCEN
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MC_AHB3ENCLRR")
            .field("dcmien", &self.dcmien())
            .field("cryp2en", &self.cryp2en())
            .field("hash2en", &self.hash2en())
            .field("rng2en", &self.rng2en())
            .field("crc2en", &self.crc2en())
            .field("hsemen", &self.hsemen())
            .field("ipccen", &self.ipccen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DCMIEN
    #[inline(always)]
    pub fn dcmien(&mut self) -> DCMIEN_W<'_, MC_AHB3ENCLRRrs> {
        DCMIEN_W::new(self, 0)
    }
    ///Bit 4 - CRYP2EN
    #[inline(always)]
    pub fn cryp2en(&mut self) -> CRYP2EN_W<'_, MC_AHB3ENCLRRrs> {
        CRYP2EN_W::new(self, 4)
    }
    ///Bit 5 - HASH2EN
    #[inline(always)]
    pub fn hash2en(&mut self) -> HASH2EN_W<'_, MC_AHB3ENCLRRrs> {
        HASH2EN_W::new(self, 5)
    }
    ///Bit 6 - RNG2EN
    #[inline(always)]
    pub fn rng2en(&mut self) -> RNG2EN_W<'_, MC_AHB3ENCLRRrs> {
        RNG2EN_W::new(self, 6)
    }
    ///Bit 7 - CRC2EN
    #[inline(always)]
    pub fn crc2en(&mut self) -> CRC2EN_W<'_, MC_AHB3ENCLRRrs> {
        CRC2EN_W::new(self, 7)
    }
    ///Bit 11 - HSEMEN
    #[inline(always)]
    pub fn hsemen(&mut self) -> HSEMEN_W<'_, MC_AHB3ENCLRRrs> {
        HSEMEN_W::new(self, 11)
    }
    ///Bit 12 - IPCCEN
    #[inline(always)]
    pub fn ipccen(&mut self) -> IPCCEN_W<'_, MC_AHB3ENCLRRrs> {
        IPCCEN_W::new(self, 12)
    }
}
/**This register is used to clear the peripheral clock enable bit

You can [`read`](crate::Reg::read) this register and get [`mc_ahb3enclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc_ahb3enclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:MC_AHB3ENCLRR)*/
pub struct MC_AHB3ENCLRRrs;
impl crate::RegisterSpec for MC_AHB3ENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`mc_ahb3enclrr::R`](R) reader structure
impl crate::Readable for MC_AHB3ENCLRRrs {}
///`write(|w| ..)` method takes [`mc_ahb3enclrr::W`](W) writer structure
impl crate::Writable for MC_AHB3ENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MC_AHB3ENCLRR to value 0
impl crate::Resettable for MC_AHB3ENCLRRrs {}
