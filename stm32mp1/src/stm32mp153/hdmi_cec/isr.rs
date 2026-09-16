///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
///Field `RXBR` reader - RXBR
pub type RXBR_R = crate::BitReader;
///Field `RXBR` writer - RXBR
pub type RXBR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXEND` reader - RXEND
pub type RXEND_R = crate::BitReader;
///Field `RXEND` writer - RXEND
pub type RXEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXOVR` reader - RXOVR
pub type RXOVR_R = crate::BitReader;
///Field `RXOVR` writer - RXOVR
pub type RXOVR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BRE` reader - BRE
pub type BRE_R = crate::BitReader;
///Field `BRE` writer - BRE
pub type BRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBPE` reader - SBPE
pub type SBPE_R = crate::BitReader;
///Field `SBPE` writer - SBPE
pub type SBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LBPE` reader - LBPE
pub type LBPE_R = crate::BitReader;
///Field `LBPE` writer - LBPE
pub type LBPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXACKE` reader - RXACKE
pub type RXACKE_R = crate::BitReader;
///Field `RXACKE` writer - RXACKE
pub type RXACKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBLST` reader - ARBLST
pub type ARBLST_R = crate::BitReader;
///Field `ARBLST` writer - ARBLST
pub type ARBLST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXBR` reader - TXBR
pub type TXBR_R = crate::BitReader;
///Field `TXBR` writer - TXBR
pub type TXBR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXEND` reader - TXEND
pub type TXEND_R = crate::BitReader;
///Field `TXEND` writer - TXEND
pub type TXEND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXUDR` reader - TXUDR
pub type TXUDR_R = crate::BitReader;
///Field `TXUDR` writer - TXUDR
pub type TXUDR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXERR` reader - TXERR
pub type TXERR_R = crate::BitReader;
///Field `TXERR` writer - TXERR
pub type TXERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXACKE` reader - TXACKE
pub type TXACKE_R = crate::BitReader;
///Field `TXACKE` writer - TXACKE
pub type TXACKE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RXBR
    #[inline(always)]
    pub fn rxbr(&self) -> RXBR_R {
        RXBR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RXEND
    #[inline(always)]
    pub fn rxend(&self) -> RXEND_R {
        RXEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RXOVR
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BRE
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SBPE
    #[inline(always)]
    pub fn sbpe(&self) -> SBPE_R {
        SBPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LBPE
    #[inline(always)]
    pub fn lbpe(&self) -> LBPE_R {
        LBPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RXACKE
    #[inline(always)]
    pub fn rxacke(&self) -> RXACKE_R {
        RXACKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ARBLST
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TXBR
    #[inline(always)]
    pub fn txbr(&self) -> TXBR_R {
        TXBR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TXEND
    #[inline(always)]
    pub fn txend(&self) -> TXEND_R {
        TXEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - TXUDR
    #[inline(always)]
    pub fn txudr(&self) -> TXUDR_R {
        TXUDR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TXERR
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TXACKE
    #[inline(always)]
    pub fn txacke(&self) -> TXACKE_R {
        TXACKE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("rxbr", &self.rxbr())
            .field("rxend", &self.rxend())
            .field("rxovr", &self.rxovr())
            .field("bre", &self.bre())
            .field("sbpe", &self.sbpe())
            .field("lbpe", &self.lbpe())
            .field("rxacke", &self.rxacke())
            .field("arblst", &self.arblst())
            .field("txbr", &self.txbr())
            .field("txend", &self.txend())
            .field("txudr", &self.txudr())
            .field("txerr", &self.txerr())
            .field("txacke", &self.txacke())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXBR
    #[inline(always)]
    pub fn rxbr(&mut self) -> RXBR_W<'_, ISRrs> {
        RXBR_W::new(self, 0)
    }
    ///Bit 1 - RXEND
    #[inline(always)]
    pub fn rxend(&mut self) -> RXEND_W<'_, ISRrs> {
        RXEND_W::new(self, 1)
    }
    ///Bit 2 - RXOVR
    #[inline(always)]
    pub fn rxovr(&mut self) -> RXOVR_W<'_, ISRrs> {
        RXOVR_W::new(self, 2)
    }
    ///Bit 3 - BRE
    #[inline(always)]
    pub fn bre(&mut self) -> BRE_W<'_, ISRrs> {
        BRE_W::new(self, 3)
    }
    ///Bit 4 - SBPE
    #[inline(always)]
    pub fn sbpe(&mut self) -> SBPE_W<'_, ISRrs> {
        SBPE_W::new(self, 4)
    }
    ///Bit 5 - LBPE
    #[inline(always)]
    pub fn lbpe(&mut self) -> LBPE_W<'_, ISRrs> {
        LBPE_W::new(self, 5)
    }
    ///Bit 6 - RXACKE
    #[inline(always)]
    pub fn rxacke(&mut self) -> RXACKE_W<'_, ISRrs> {
        RXACKE_W::new(self, 6)
    }
    ///Bit 7 - ARBLST
    #[inline(always)]
    pub fn arblst(&mut self) -> ARBLST_W<'_, ISRrs> {
        ARBLST_W::new(self, 7)
    }
    ///Bit 8 - TXBR
    #[inline(always)]
    pub fn txbr(&mut self) -> TXBR_W<'_, ISRrs> {
        TXBR_W::new(self, 8)
    }
    ///Bit 9 - TXEND
    #[inline(always)]
    pub fn txend(&mut self) -> TXEND_W<'_, ISRrs> {
        TXEND_W::new(self, 9)
    }
    ///Bit 10 - TXUDR
    #[inline(always)]
    pub fn txudr(&mut self) -> TXUDR_W<'_, ISRrs> {
        TXUDR_W::new(self, 10)
    }
    ///Bit 11 - TXERR
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W<'_, ISRrs> {
        TXERR_W::new(self, 11)
    }
    ///Bit 12 - TXACKE
    #[inline(always)]
    pub fn txacke(&mut self) -> TXACKE_W<'_, ISRrs> {
        TXACKE_W::new(self, 12)
    }
}
/**CEC Interrupt and Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HDMI_CEC:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
