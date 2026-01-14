///Register `CCIPR2` reader
pub type R = crate::R<CCIPR2rs>;
///Register `CCIPR2` writer
pub type W = crate::W<CCIPR2rs>;
///Field `ETH1PTPSEL` reader - Source selection for the ETH1 kernel clock
pub type ETH1PTPSEL_R = crate::FieldReader;
///Field `ETH1PTPSEL` writer - Source selection for the ETH1 kernel clock
pub type ETH1PTPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ETH1PTPDIV` reader - ETH1 Kernel clock divider selection (for clock ck_ker_eth1ptp)
pub type ETH1PTPDIV_R = crate::FieldReader;
///Field `ETH1PTPDIV` writer - ETH1 Kernel clock divider selection (for clock ck_ker_eth1ptp)
pub type ETH1PTPDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ETH1PWRDOWNACK` reader - Set and reset by software.
pub type ETH1PWRDOWNACK_R = crate::BitReader;
///Field `ETH1CLKSEL` reader - Source selection for the ETH1 kernel clock
pub type ETH1CLKSEL_R = crate::FieldReader;
///Field `ETH1CLKSEL` writer - Source selection for the ETH1 kernel clock
pub type ETH1CLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ETH1SEL` reader - Set and reset by software
pub type ETH1SEL_R = crate::FieldReader;
///Field `ETH1SEL` writer - Set and reset by software
pub type ETH1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ETH1REFCLKSEL` reader - Set and reset by software
pub type ETH1REFCLKSEL_R = crate::BitReader;
///Field `ETH1REFCLKSEL` writer - Set and reset by software
pub type ETH1REFCLKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ETH1GTXCLKSEL` reader - Set and reset by software.
pub type ETH1GTXCLKSEL_R = crate::BitReader;
///Field `ETH1GTXCLKSEL` writer - Set and reset by software.
pub type ETH1GTXCLKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Source selection for the ETH1 kernel clock
    #[inline(always)]
    pub fn eth1ptpsel(&self) -> ETH1PTPSEL_R {
        ETH1PTPSEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:7 - ETH1 Kernel clock divider selection (for clock ck_ker_eth1ptp)
    #[inline(always)]
    pub fn eth1ptpdiv(&self) -> ETH1PTPDIV_R {
        ETH1PTPDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Set and reset by software.
    #[inline(always)]
    pub fn eth1pwrdownack(&self) -> ETH1PWRDOWNACK_R {
        ETH1PWRDOWNACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:13 - Source selection for the ETH1 kernel clock
    #[inline(always)]
    pub fn eth1clksel(&self) -> ETH1CLKSEL_R {
        ETH1CLKSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:18 - Set and reset by software
    #[inline(always)]
    pub fn eth1sel(&self) -> ETH1SEL_R {
        ETH1SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 20 - Set and reset by software
    #[inline(always)]
    pub fn eth1refclksel(&self) -> ETH1REFCLKSEL_R {
        ETH1REFCLKSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - Set and reset by software.
    #[inline(always)]
    pub fn eth1gtxclksel(&self) -> ETH1GTXCLKSEL_R {
        ETH1GTXCLKSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCIPR2")
            .field("eth1ptpsel", &self.eth1ptpsel())
            .field("eth1ptpdiv", &self.eth1ptpdiv())
            .field("eth1pwrdownack", &self.eth1pwrdownack())
            .field("eth1clksel", &self.eth1clksel())
            .field("eth1sel", &self.eth1sel())
            .field("eth1refclksel", &self.eth1refclksel())
            .field("eth1gtxclksel", &self.eth1gtxclksel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Source selection for the ETH1 kernel clock
    #[inline(always)]
    pub fn eth1ptpsel(&mut self) -> ETH1PTPSEL_W<'_, CCIPR2rs> {
        ETH1PTPSEL_W::new(self, 0)
    }
    ///Bits 4:7 - ETH1 Kernel clock divider selection (for clock ck_ker_eth1ptp)
    #[inline(always)]
    pub fn eth1ptpdiv(&mut self) -> ETH1PTPDIV_W<'_, CCIPR2rs> {
        ETH1PTPDIV_W::new(self, 4)
    }
    ///Bits 12:13 - Source selection for the ETH1 kernel clock
    #[inline(always)]
    pub fn eth1clksel(&mut self) -> ETH1CLKSEL_W<'_, CCIPR2rs> {
        ETH1CLKSEL_W::new(self, 12)
    }
    ///Bits 16:18 - Set and reset by software
    #[inline(always)]
    pub fn eth1sel(&mut self) -> ETH1SEL_W<'_, CCIPR2rs> {
        ETH1SEL_W::new(self, 16)
    }
    ///Bit 20 - Set and reset by software
    #[inline(always)]
    pub fn eth1refclksel(&mut self) -> ETH1REFCLKSEL_W<'_, CCIPR2rs> {
        ETH1REFCLKSEL_W::new(self, 20)
    }
    ///Bit 24 - Set and reset by software.
    #[inline(always)]
    pub fn eth1gtxclksel(&mut self) -> ETH1GTXCLKSEL_W<'_, CCIPR2rs> {
        ETH1GTXCLKSEL_W::new(self, 24)
    }
}
/**RCC clock configuration for independent peripheral register 2

You can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:CCIPR2)*/
pub struct CCIPR2rs;
impl crate::RegisterSpec for CCIPR2rs {
    type Ux = u32;
}
///`read()` method returns [`ccipr2::R`](R) reader structure
impl crate::Readable for CCIPR2rs {}
///`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure
impl crate::Writable for CCIPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCIPR2 to value 0
impl crate::Resettable for CCIPR2rs {}
