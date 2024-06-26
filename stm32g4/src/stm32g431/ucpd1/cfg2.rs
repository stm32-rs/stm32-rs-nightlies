///Register `CFG2` reader
pub type R = crate::R<CFG2rs>;
///Register `CFG2` writer
pub type W = crate::W<CFG2rs>;
///Field `RXFILTDIS` reader - RXFILTDIS
pub type RXFILTDIS_R = crate::BitReader;
///Field `RXFILTDIS` writer - RXFILTDIS
pub type RXFILTDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFILT2N3` reader - RXFILT2N3
pub type RXFILT2N3_R = crate::BitReader;
///Field `RXFILT2N3` writer - RXFILT2N3
pub type RXFILT2N3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCECLK` reader - FORCECLK
pub type FORCECLK_R = crate::BitReader;
///Field `FORCECLK` writer - FORCECLK
pub type FORCECLK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WUPEN` reader - WUPEN
pub type WUPEN_R = crate::BitReader;
///Field `WUPEN` writer - WUPEN
pub type WUPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RXFILTDIS
    #[inline(always)]
    pub fn rxfiltdis(&self) -> RXFILTDIS_R {
        RXFILTDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RXFILT2N3
    #[inline(always)]
    pub fn rxfilt2n3(&self) -> RXFILT2N3_R {
        RXFILT2N3_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FORCECLK
    #[inline(always)]
    pub fn forceclk(&self) -> FORCECLK_R {
        FORCECLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - WUPEN
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG2")
            .field("rxfiltdis", &self.rxfiltdis())
            .field("rxfilt2n3", &self.rxfilt2n3())
            .field("forceclk", &self.forceclk())
            .field("wupen", &self.wupen())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXFILTDIS
    #[inline(always)]
    #[must_use]
    pub fn rxfiltdis(&mut self) -> RXFILTDIS_W<CFG2rs> {
        RXFILTDIS_W::new(self, 0)
    }
    ///Bit 1 - RXFILT2N3
    #[inline(always)]
    #[must_use]
    pub fn rxfilt2n3(&mut self) -> RXFILT2N3_W<CFG2rs> {
        RXFILT2N3_W::new(self, 1)
    }
    ///Bit 2 - FORCECLK
    #[inline(always)]
    #[must_use]
    pub fn forceclk(&mut self) -> FORCECLK_W<CFG2rs> {
        FORCECLK_W::new(self, 2)
    }
    ///Bit 3 - WUPEN
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WUPEN_W<CFG2rs> {
        WUPEN_W::new(self, 3)
    }
}
/**UCPD configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431xx.html#UCPD1:CFG2)*/
pub struct CFG2rs;
impl crate::RegisterSpec for CFG2rs {
    type Ux = u32;
}
///`read()` method returns [`cfg2::R`](R) reader structure
impl crate::Readable for CFG2rs {}
///`write(|w| ..)` method takes [`cfg2::W`](W) writer structure
impl crate::Writable for CFG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CFG2 to value 0
impl crate::Resettable for CFG2rs {
    const RESET_VALUE: u32 = 0;
}
