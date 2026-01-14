///Register `SECCFGR` reader
pub type R = crate::R<SECCFGRrs>;
///Register `SECCFGR` writer
pub type W = crate::W<SECCFGRrs>;
///Field `HSISEC` reader - HSISEC
pub type HSISEC_R = crate::BitReader;
///Field `HSISEC` writer - HSISEC
pub type HSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSESEC` reader - HSESEC
pub type HSESEC_R = crate::BitReader;
///Field `HSESEC` writer - HSESEC
pub type HSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MSISEC` reader - MSISEC
pub type MSISEC_R = crate::BitReader;
///Field `MSISEC` writer - MSISEC
pub type MSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSISEC` reader - LSISEC
pub type LSISEC_R = crate::BitReader;
///Field `LSISEC` writer - LSISEC
pub type LSISEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSESEC` reader - LSESEC
pub type LSESEC_R = crate::BitReader;
///Field `LSESEC` writer - LSESEC
pub type LSESEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCLKSEC` reader - SYSCLKSEC
pub type SYSCLKSEC_R = crate::BitReader;
///Field `SYSCLKSEC` writer - SYSCLKSEC
pub type SYSCLKSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRESCSEC` reader - PRESCSEC
pub type PRESCSEC_R = crate::BitReader;
///Field `PRESCSEC` writer - PRESCSEC
pub type PRESCSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSEC` reader - PLLSEC
pub type PLLSEC_R = crate::BitReader;
///Field `PLLSEC` writer - PLLSEC
pub type PLLSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI1SEC` reader - PLLSAI1SEC
pub type PLLSAI1SEC_R = crate::BitReader;
///Field `PLLSAI1SEC` writer - PLLSAI1SEC
pub type PLLSAI1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLSAI2SEC` reader - PLLSAI2SEC
pub type PLLSAI2SEC_R = crate::BitReader;
///Field `PLLSAI2SEC` writer - PLLSAI2SEC
pub type PLLSAI2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK48MSEC` reader - CLK48MSEC
pub type CLK48MSEC_R = crate::BitReader;
///Field `CLK48MSEC` writer - CLK48MSEC
pub type CLK48MSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSI48SEC` reader - HSI48SEC
pub type HSI48SEC_R = crate::BitReader;
///Field `HSI48SEC` writer - HSI48SEC
pub type HSI48SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMVFSEC` reader - RMVFSEC
pub type RMVFSEC_R = crate::BitReader;
///Field `RMVFSEC` writer - RMVFSEC
pub type RMVFSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HSISEC
    #[inline(always)]
    pub fn hsisec(&self) -> HSISEC_R {
        HSISEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HSESEC
    #[inline(always)]
    pub fn hsesec(&self) -> HSESEC_R {
        HSESEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSISEC
    #[inline(always)]
    pub fn msisec(&self) -> MSISEC_R {
        MSISEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LSISEC
    #[inline(always)]
    pub fn lsisec(&self) -> LSISEC_R {
        LSISEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LSESEC
    #[inline(always)]
    pub fn lsesec(&self) -> LSESEC_R {
        LSESEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYSCLKSEC
    #[inline(always)]
    pub fn sysclksec(&self) -> SYSCLKSEC_R {
        SYSCLKSEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PRESCSEC
    #[inline(always)]
    pub fn prescsec(&self) -> PRESCSEC_R {
        PRESCSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PLLSEC
    #[inline(always)]
    pub fn pllsec(&self) -> PLLSEC_R {
        PLLSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PLLSAI1SEC
    #[inline(always)]
    pub fn pllsai1sec(&self) -> PLLSAI1SEC_R {
        PLLSAI1SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PLLSAI2SEC
    #[inline(always)]
    pub fn pllsai2sec(&self) -> PLLSAI2SEC_R {
        PLLSAI2SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CLK48MSEC
    #[inline(always)]
    pub fn clk48msec(&self) -> CLK48MSEC_R {
        CLK48MSEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSI48SEC
    #[inline(always)]
    pub fn hsi48sec(&self) -> HSI48SEC_R {
        HSI48SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RMVFSEC
    #[inline(always)]
    pub fn rmvfsec(&self) -> RMVFSEC_R {
        RMVFSEC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR")
            .field("hsisec", &self.hsisec())
            .field("hsesec", &self.hsesec())
            .field("msisec", &self.msisec())
            .field("lsisec", &self.lsisec())
            .field("lsesec", &self.lsesec())
            .field("sysclksec", &self.sysclksec())
            .field("prescsec", &self.prescsec())
            .field("pllsec", &self.pllsec())
            .field("pllsai1sec", &self.pllsai1sec())
            .field("pllsai2sec", &self.pllsai2sec())
            .field("clk48msec", &self.clk48msec())
            .field("hsi48sec", &self.hsi48sec())
            .field("rmvfsec", &self.rmvfsec())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSISEC
    #[inline(always)]
    pub fn hsisec(&mut self) -> HSISEC_W<'_, SECCFGRrs> {
        HSISEC_W::new(self, 0)
    }
    ///Bit 1 - HSESEC
    #[inline(always)]
    pub fn hsesec(&mut self) -> HSESEC_W<'_, SECCFGRrs> {
        HSESEC_W::new(self, 1)
    }
    ///Bit 2 - MSISEC
    #[inline(always)]
    pub fn msisec(&mut self) -> MSISEC_W<'_, SECCFGRrs> {
        MSISEC_W::new(self, 2)
    }
    ///Bit 3 - LSISEC
    #[inline(always)]
    pub fn lsisec(&mut self) -> LSISEC_W<'_, SECCFGRrs> {
        LSISEC_W::new(self, 3)
    }
    ///Bit 4 - LSESEC
    #[inline(always)]
    pub fn lsesec(&mut self) -> LSESEC_W<'_, SECCFGRrs> {
        LSESEC_W::new(self, 4)
    }
    ///Bit 5 - SYSCLKSEC
    #[inline(always)]
    pub fn sysclksec(&mut self) -> SYSCLKSEC_W<'_, SECCFGRrs> {
        SYSCLKSEC_W::new(self, 5)
    }
    ///Bit 6 - PRESCSEC
    #[inline(always)]
    pub fn prescsec(&mut self) -> PRESCSEC_W<'_, SECCFGRrs> {
        PRESCSEC_W::new(self, 6)
    }
    ///Bit 7 - PLLSEC
    #[inline(always)]
    pub fn pllsec(&mut self) -> PLLSEC_W<'_, SECCFGRrs> {
        PLLSEC_W::new(self, 7)
    }
    ///Bit 8 - PLLSAI1SEC
    #[inline(always)]
    pub fn pllsai1sec(&mut self) -> PLLSAI1SEC_W<'_, SECCFGRrs> {
        PLLSAI1SEC_W::new(self, 8)
    }
    ///Bit 9 - PLLSAI2SEC
    #[inline(always)]
    pub fn pllsai2sec(&mut self) -> PLLSAI2SEC_W<'_, SECCFGRrs> {
        PLLSAI2SEC_W::new(self, 9)
    }
    ///Bit 10 - CLK48MSEC
    #[inline(always)]
    pub fn clk48msec(&mut self) -> CLK48MSEC_W<'_, SECCFGRrs> {
        CLK48MSEC_W::new(self, 10)
    }
    ///Bit 11 - HSI48SEC
    #[inline(always)]
    pub fn hsi48sec(&mut self) -> HSI48SEC_W<'_, SECCFGRrs> {
        HSI48SEC_W::new(self, 11)
    }
    ///Bit 12 - RMVFSEC
    #[inline(always)]
    pub fn rmvfsec(&mut self) -> RMVFSEC_W<'_, SECCFGRrs> {
        RMVFSEC_W::new(self, 12)
    }
}
/**RCC secure configuration register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#RCC:SECCFGR)*/
pub struct SECCFGRrs;
impl crate::RegisterSpec for SECCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr::R`](R) reader structure
impl crate::Readable for SECCFGRrs {}
///`write(|w| ..)` method takes [`seccfgr::W`](W) writer structure
impl crate::Writable for SECCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR to value 0
impl crate::Resettable for SECCFGRrs {}
