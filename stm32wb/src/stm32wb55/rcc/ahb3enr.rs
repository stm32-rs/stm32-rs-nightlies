///Register `AHB3ENR` reader
pub type R = crate::R<AHB3ENRrs>;
///Register `AHB3ENR` writer
pub type W = crate::W<AHB3ENRrs>;
///Field `QSPIEN` reader - QSPIEN
pub type QSPIEN_R = crate::BitReader;
///Field `QSPIEN` writer - QSPIEN
pub type QSPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PKAEN` reader - PKAEN
pub type PKAEN_R = crate::BitReader;
///Field `PKAEN` writer - PKAEN
pub type PKAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AES2EN` reader - AES2EN
pub type AES2EN_R = crate::BitReader;
///Field `AES2EN` writer - AES2EN
pub type AES2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RNGEN` reader - RNGEN
pub type RNGEN_R = crate::BitReader;
///Field `RNGEN` writer - RNGEN
pub type RNGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEMEN` reader - HSEMEN
pub type HSEMEN_R = crate::BitReader;
///Field `HSEMEN` writer - HSEMEN
pub type HSEMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IPCCEN` reader - IPCCEN
pub type IPCCEN_R = crate::BitReader;
///Field `IPCCEN` writer - IPCCEN
pub type IPCCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASHEN` reader - FLASHEN
pub type FLASHEN_R = crate::BitReader;
///Field `FLASHEN` writer - FLASHEN
pub type FLASHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - QSPIEN
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - PKAEN
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - AES2EN
    #[inline(always)]
    pub fn aes2en(&self) -> AES2EN_R {
        AES2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RNGEN
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - HSEMEN
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - IPCCEN
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 25 - FLASHEN
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3ENR")
            .field("flashen", &self.flashen())
            .field("ipccen", &self.ipccen())
            .field("hsemen", &self.hsemen())
            .field("rngen", &self.rngen())
            .field("aes2en", &self.aes2en())
            .field("pkaen", &self.pkaen())
            .field("qspien", &self.qspien())
            .finish()
    }
}
impl W {
    ///Bit 8 - QSPIEN
    #[inline(always)]
    #[must_use]
    pub fn qspien(&mut self) -> QSPIEN_W<AHB3ENRrs> {
        QSPIEN_W::new(self, 8)
    }
    ///Bit 16 - PKAEN
    #[inline(always)]
    #[must_use]
    pub fn pkaen(&mut self) -> PKAEN_W<AHB3ENRrs> {
        PKAEN_W::new(self, 16)
    }
    ///Bit 17 - AES2EN
    #[inline(always)]
    #[must_use]
    pub fn aes2en(&mut self) -> AES2EN_W<AHB3ENRrs> {
        AES2EN_W::new(self, 17)
    }
    ///Bit 18 - RNGEN
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<AHB3ENRrs> {
        RNGEN_W::new(self, 18)
    }
    ///Bit 19 - HSEMEN
    #[inline(always)]
    #[must_use]
    pub fn hsemen(&mut self) -> HSEMEN_W<AHB3ENRrs> {
        HSEMEN_W::new(self, 19)
    }
    ///Bit 20 - IPCCEN
    #[inline(always)]
    #[must_use]
    pub fn ipccen(&mut self) -> IPCCEN_W<AHB3ENRrs> {
        IPCCEN_W::new(self, 20)
    }
    ///Bit 25 - FLASHEN
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<AHB3ENRrs> {
        FLASHEN_W::new(self, 25)
    }
}
/**AHB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#RCC:AHB3ENR)*/
pub struct AHB3ENRrs;
impl crate::RegisterSpec for AHB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3enr::R`](R) reader structure
impl crate::Readable for AHB3ENRrs {}
///`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure
impl crate::Writable for AHB3ENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHB3ENR to value 0x0208_0000
impl crate::Resettable for AHB3ENRrs {
    const RESET_VALUE: u32 = 0x0208_0000;
}
