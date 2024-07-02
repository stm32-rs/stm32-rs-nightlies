///Register `PSR` reader
pub type R = crate::R<PSRrs>;
///Register `PSR` writer
pub type W = crate::W<PSRrs>;
///Field `PD` reader - PHY direction
pub type PD_R = crate::BitReader;
///Field `PD` writer - PHY direction
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSSC` reader - PHY stop state clock lane
pub type PSSC_R = crate::BitReader;
///Field `PSSC` writer - PHY stop state clock lane
pub type PSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UANC` reader - ULPS active not clock lane
pub type UANC_R = crate::BitReader;
///Field `UANC` writer - ULPS active not clock lane
pub type UANC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSS0` reader - PHY stop state lane 0
pub type PSS0_R = crate::BitReader;
///Field `PSS0` writer - PHY stop state lane 0
pub type PSS0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UAN0` reader - ULPS active not lane 1
pub type UAN0_R = crate::BitReader;
///Field `UAN0` writer - ULPS active not lane 1
pub type UAN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RUE0` reader - RX ULPS escape lane 0
pub type RUE0_R = crate::BitReader;
///Field `RUE0` writer - RX ULPS escape lane 0
pub type RUE0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSS1` reader - PHY stop state lane 1
pub type PSS1_R = crate::BitReader;
///Field `PSS1` writer - PHY stop state lane 1
pub type PSS1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UAN1` reader - ULPS active not lane 1
pub type UAN1_R = crate::BitReader;
///Field `UAN1` writer - ULPS active not lane 1
pub type UAN1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - PHY direction
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PHY stop state clock lane
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ULPS active not clock lane
    #[inline(always)]
    pub fn uanc(&self) -> UANC_R {
        UANC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PHY stop state lane 0
    #[inline(always)]
    pub fn pss0(&self) -> PSS0_R {
        PSS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ULPS active not lane 1
    #[inline(always)]
    pub fn uan0(&self) -> UAN0_R {
        UAN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RX ULPS escape lane 0
    #[inline(always)]
    pub fn rue0(&self) -> RUE0_R {
        RUE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PHY stop state lane 1
    #[inline(always)]
    pub fn pss1(&self) -> PSS1_R {
        PSS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ULPS active not lane 1
    #[inline(always)]
    pub fn uan1(&self) -> UAN1_R {
        UAN1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("uan1", &self.uan1())
            .field("pss1", &self.pss1())
            .field("rue0", &self.rue0())
            .field("uan0", &self.uan0())
            .field("pss0", &self.pss0())
            .field("uanc", &self.uanc())
            .field("pssc", &self.pssc())
            .field("pd", &self.pd())
            .finish()
    }
}
impl W {
    ///Bit 1 - PHY direction
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<PSRrs> {
        PD_W::new(self, 1)
    }
    ///Bit 2 - PHY stop state clock lane
    #[inline(always)]
    #[must_use]
    pub fn pssc(&mut self) -> PSSC_W<PSRrs> {
        PSSC_W::new(self, 2)
    }
    ///Bit 3 - ULPS active not clock lane
    #[inline(always)]
    #[must_use]
    pub fn uanc(&mut self) -> UANC_W<PSRrs> {
        UANC_W::new(self, 3)
    }
    ///Bit 4 - PHY stop state lane 0
    #[inline(always)]
    #[must_use]
    pub fn pss0(&mut self) -> PSS0_W<PSRrs> {
        PSS0_W::new(self, 4)
    }
    ///Bit 5 - ULPS active not lane 1
    #[inline(always)]
    #[must_use]
    pub fn uan0(&mut self) -> UAN0_W<PSRrs> {
        UAN0_W::new(self, 5)
    }
    ///Bit 6 - RX ULPS escape lane 0
    #[inline(always)]
    #[must_use]
    pub fn rue0(&mut self) -> RUE0_W<PSRrs> {
        RUE0_W::new(self, 6)
    }
    ///Bit 7 - PHY stop state lane 1
    #[inline(always)]
    #[must_use]
    pub fn pss1(&mut self) -> PSS1_W<PSRrs> {
        PSS1_W::new(self, 7)
    }
    ///Bit 8 - ULPS active not lane 1
    #[inline(always)]
    #[must_use]
    pub fn uan1(&mut self) -> UAN1_W<PSRrs> {
        UAN1_W::new(self, 8)
    }
}
/**DSI Host PHY status register

You can [`read`](crate::Reg::read) this register and get [`psr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#DSIHOST:PSR)*/
pub struct PSRrs;
impl crate::RegisterSpec for PSRrs {
    type Ux = u32;
}
///`read()` method returns [`psr::R`](R) reader structure
impl crate::Readable for PSRrs {}
///`write(|w| ..)` method takes [`psr::W`](W) writer structure
impl crate::Writable for PSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PSR to value 0x1528
impl crate::Resettable for PSRrs {
    const RESET_VALUE: u32 = 0x1528;
}
