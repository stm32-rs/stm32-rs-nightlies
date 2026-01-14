///Register `IER3` reader
pub type R = crate::R<IER3rs>;
///Register `IER3` writer
pub type W = crate::W<IER3rs>;
///Field `CRCIE` reader - illegal access interrupt enable for CRC
pub type CRCIE_R = crate::BitReader;
///Field `CRCIE` writer - illegal access interrupt enable for CRC
pub type CRCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TSCIE` reader - illegal access interrupt enable for TSC
pub type TSCIE_R = crate::BitReader;
///Field `TSCIE` writer - illegal access interrupt enable for TSC
pub type TSCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ICACHEIE` reader - illegal access interrupt enable for ICACHE registers
pub type ICACHEIE_R = crate::BitReader;
///Field `ICACHEIE` writer - illegal access interrupt enable for ICACHE registers
pub type ICACHEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AESIE` reader - illegal access interrupt enable for AES
pub type AESIE_R = crate::BitReader;
///Field `AESIE` writer - illegal access interrupt enable for AES
pub type AESIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HASHIE` reader - illegal access interrupt enable for HASH
pub type HASHIE_R = crate::BitReader;
///Field `HASHIE` writer - illegal access interrupt enable for HASH
pub type HASHIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGIE` reader - illegal access interrupt enable for RNG
pub type RNGIE_R = crate::BitReader;
///Field `RNGIE` writer - illegal access interrupt enable for RNG
pub type RNGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAESIE` reader - illegal access interrupt enable for SAES
pub type SAESIE_R = crate::BitReader;
///Field `SAESIE` writer - illegal access interrupt enable for SAES
pub type SAESIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEMIE` reader - illegal access interrupt enable for HSEM
pub type HSEMIE_R = crate::BitReader;
///Field `HSEMIE` writer - illegal access interrupt enable for HSEM
pub type HSEMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAIE` reader - illegal access interrupt enable for PKA
pub type PKAIE_R = crate::BitReader;
///Field `PKAIE` writer - illegal access interrupt enable for PKA
pub type PKAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMCFGIE` reader - illegal access interrupt enable for RAMCFG
pub type RAMCFGIE_R = crate::BitReader;
///Field `RAMCFGIE` writer - illegal access interrupt enable for RAMCFG
pub type RAMCFGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RADIOIE` reader - illegal access interrupt enable for 2.4 GHz RADIO
pub type RADIOIE_R = crate::BitReader;
///Field `RADIOIE` writer - illegal access interrupt enable for 2.4 GHz RADIO
pub type RADIOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - illegal access interrupt enable for CRC
    #[inline(always)]
    pub fn crcie(&self) -> CRCIE_R {
        CRCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access interrupt enable for TSC
    #[inline(always)]
    pub fn tscie(&self) -> TSCIE_R {
        TSCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - illegal access interrupt enable for ICACHE registers
    #[inline(always)]
    pub fn icacheie(&self) -> ICACHEIE_R {
        ICACHEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 11 - illegal access interrupt enable for AES
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access interrupt enable for HASH
    #[inline(always)]
    pub fn hashie(&self) -> HASHIE_R {
        HASHIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access interrupt enable for RNG
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access interrupt enable for SAES
    #[inline(always)]
    pub fn saesie(&self) -> SAESIE_R {
        SAESIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access interrupt enable for HSEM
    #[inline(always)]
    pub fn hsemie(&self) -> HSEMIE_R {
        HSEMIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access interrupt enable for PKA
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 22 - illegal access interrupt enable for RAMCFG
    #[inline(always)]
    pub fn ramcfgie(&self) -> RAMCFGIE_R {
        RAMCFGIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - illegal access interrupt enable for 2.4 GHz RADIO
    #[inline(always)]
    pub fn radioie(&self) -> RADIOIE_R {
        RADIOIE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER3")
            .field("crcie", &self.crcie())
            .field("tscie", &self.tscie())
            .field("icacheie", &self.icacheie())
            .field("aesie", &self.aesie())
            .field("hashie", &self.hashie())
            .field("rngie", &self.rngie())
            .field("saesie", &self.saesie())
            .field("hsemie", &self.hsemie())
            .field("pkaie", &self.pkaie())
            .field("ramcfgie", &self.ramcfgie())
            .field("radioie", &self.radioie())
            .finish()
    }
}
impl W {
    ///Bit 3 - illegal access interrupt enable for CRC
    #[inline(always)]
    pub fn crcie(&mut self) -> CRCIE_W<'_, IER3rs> {
        CRCIE_W::new(self, 3)
    }
    ///Bit 4 - illegal access interrupt enable for TSC
    #[inline(always)]
    pub fn tscie(&mut self) -> TSCIE_W<'_, IER3rs> {
        TSCIE_W::new(self, 4)
    }
    ///Bit 6 - illegal access interrupt enable for ICACHE registers
    #[inline(always)]
    pub fn icacheie(&mut self) -> ICACHEIE_W<'_, IER3rs> {
        ICACHEIE_W::new(self, 6)
    }
    ///Bit 11 - illegal access interrupt enable for AES
    #[inline(always)]
    pub fn aesie(&mut self) -> AESIE_W<'_, IER3rs> {
        AESIE_W::new(self, 11)
    }
    ///Bit 12 - illegal access interrupt enable for HASH
    #[inline(always)]
    pub fn hashie(&mut self) -> HASHIE_W<'_, IER3rs> {
        HASHIE_W::new(self, 12)
    }
    ///Bit 13 - illegal access interrupt enable for RNG
    #[inline(always)]
    pub fn rngie(&mut self) -> RNGIE_W<'_, IER3rs> {
        RNGIE_W::new(self, 13)
    }
    ///Bit 14 - illegal access interrupt enable for SAES
    #[inline(always)]
    pub fn saesie(&mut self) -> SAESIE_W<'_, IER3rs> {
        SAESIE_W::new(self, 14)
    }
    ///Bit 15 - illegal access interrupt enable for HSEM
    #[inline(always)]
    pub fn hsemie(&mut self) -> HSEMIE_W<'_, IER3rs> {
        HSEMIE_W::new(self, 15)
    }
    ///Bit 16 - illegal access interrupt enable for PKA
    #[inline(always)]
    pub fn pkaie(&mut self) -> PKAIE_W<'_, IER3rs> {
        PKAIE_W::new(self, 16)
    }
    ///Bit 22 - illegal access interrupt enable for RAMCFG
    #[inline(always)]
    pub fn ramcfgie(&mut self) -> RAMCFGIE_W<'_, IER3rs> {
        RAMCFGIE_W::new(self, 22)
    }
    ///Bit 23 - illegal access interrupt enable for 2.4 GHz RADIO
    #[inline(always)]
    pub fn radioie(&mut self) -> RADIOIE_W<'_, IER3rs> {
        RADIOIE_W::new(self, 23)
    }
}
/**GTZC1 TZIC interrupt enable register 3

You can [`read`](crate::Reg::read) this register and get [`ier3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#GTZC1_TZIC:IER3)*/
pub struct IER3rs;
impl crate::RegisterSpec for IER3rs {
    type Ux = u32;
}
///`read()` method returns [`ier3::R`](R) reader structure
impl crate::Readable for IER3rs {}
///`write(|w| ..)` method takes [`ier3::W`](W) writer structure
impl crate::Writable for IER3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER3 to value 0
impl crate::Resettable for IER3rs {}
