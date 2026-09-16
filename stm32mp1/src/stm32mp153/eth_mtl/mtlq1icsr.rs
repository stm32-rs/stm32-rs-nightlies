///Register `MTLQ1ICSR` reader
pub type R = crate::R<MTLQ1ICSRrs>;
///Register `MTLQ1ICSR` writer
pub type W = crate::W<MTLQ1ICSRrs>;
///Field `TXUNFIS` reader - TXUNFIS
pub type TXUNFIS_R = crate::BitReader;
///Field `ABPSIS` reader - ABPSIS
pub type ABPSIS_R = crate::BitReader;
///Field `ABPSIS` writer - ABPSIS
pub type ABPSIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUIE` reader - TXUIE
pub type TXUIE_R = crate::BitReader;
///Field `TXUIE` writer - TXUIE
pub type TXUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABPSIE` reader - ABPSIE
pub type ABPSIE_R = crate::BitReader;
///Field `ABPSIE` writer - ABPSIE
pub type ABPSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVFIS` reader - RXOVFIS
pub type RXOVFIS_R = crate::BitReader;
///Field `RXOVFIS` writer - RXOVFIS
pub type RXOVFIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOIE` reader - RXOIE
pub type RXOIE_R = crate::BitReader;
///Field `RXOIE` writer - RXOIE
pub type RXOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TXUNFIS
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ABPSIS
    #[inline(always)]
    pub fn abpsis(&self) -> ABPSIS_R {
        ABPSIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - TXUIE
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ABPSIE
    #[inline(always)]
    pub fn abpsie(&self) -> ABPSIE_R {
        ABPSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - RXOVFIS
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - RXOIE
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLQ1ICSR")
            .field("txunfis", &self.txunfis())
            .field("abpsis", &self.abpsis())
            .field("txuie", &self.txuie())
            .field("abpsie", &self.abpsie())
            .field("rxovfis", &self.rxovfis())
            .field("rxoie", &self.rxoie())
            .finish()
    }
}
impl W {
    ///Bit 1 - ABPSIS
    #[inline(always)]
    pub fn abpsis(&mut self) -> ABPSIS_W<'_, MTLQ1ICSRrs> {
        ABPSIS_W::new(self, 1)
    }
    ///Bit 8 - TXUIE
    #[inline(always)]
    pub fn txuie(&mut self) -> TXUIE_W<'_, MTLQ1ICSRrs> {
        TXUIE_W::new(self, 8)
    }
    ///Bit 9 - ABPSIE
    #[inline(always)]
    pub fn abpsie(&mut self) -> ABPSIE_W<'_, MTLQ1ICSRrs> {
        ABPSIE_W::new(self, 9)
    }
    ///Bit 16 - RXOVFIS
    #[inline(always)]
    pub fn rxovfis(&mut self) -> RXOVFIS_W<'_, MTLQ1ICSRrs> {
        RXOVFIS_W::new(self, 16)
    }
    ///Bit 24 - RXOIE
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W<'_, MTLQ1ICSRrs> {
        RXOIE_W::new(self, 24)
    }
}
/**Queue 1 interrupt control status Register

You can [`read`](crate::Reg::read) this register and get [`mtlq1icsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlq1icsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:MTLQ1ICSR)*/
pub struct MTLQ1ICSRrs;
impl crate::RegisterSpec for MTLQ1ICSRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlq1icsr::R`](R) reader structure
impl crate::Readable for MTLQ1ICSRrs {}
///`write(|w| ..)` method takes [`mtlq1icsr::W`](W) writer structure
impl crate::Writable for MTLQ1ICSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLQ1ICSR to value 0
impl crate::Resettable for MTLQ1ICSRrs {}
