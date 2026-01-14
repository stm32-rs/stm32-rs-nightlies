///Register `PSR` reader
pub type R = crate::R<PSRrs>;
///Field `PD` reader - PHY Direction
pub type PD_R = crate::BitReader;
///Field `PSSC` reader - PHY Stop State Clock lane
pub type PSSC_R = crate::BitReader;
///Field `UANC` reader - ULPS Active Not Clock lane
pub type UANC_R = crate::BitReader;
///Field `PSS0` reader - PHY Stop State lane 0
pub type PSS0_R = crate::BitReader;
///Field `UAN0` reader - ULPS Active Not lane 1
pub type UAN0_R = crate::BitReader;
///Field `RUE0` reader - RX ULPS Escape lane 0
pub type RUE0_R = crate::BitReader;
///Field `PSS1` reader - PHY Stop State lane 1
pub type PSS1_R = crate::BitReader;
///Field `UAN1` reader - ULPS Active Not lane 1
pub type UAN1_R = crate::BitReader;
impl R {
    ///Bit 1 - PHY Direction
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PHY Stop State Clock lane
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ULPS Active Not Clock lane
    #[inline(always)]
    pub fn uanc(&self) -> UANC_R {
        UANC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PHY Stop State lane 0
    #[inline(always)]
    pub fn pss0(&self) -> PSS0_R {
        PSS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ULPS Active Not lane 1
    #[inline(always)]
    pub fn uan0(&self) -> UAN0_R {
        UAN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RX ULPS Escape lane 0
    #[inline(always)]
    pub fn rue0(&self) -> RUE0_R {
        RUE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PHY Stop State lane 1
    #[inline(always)]
    pub fn pss1(&self) -> PSS1_R {
        PSS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ULPS Active Not lane 1
    #[inline(always)]
    pub fn uan1(&self) -> UAN1_R {
        UAN1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("pd", &self.pd())
            .field("pssc", &self.pssc())
            .field("uanc", &self.uanc())
            .field("pss0", &self.pss0())
            .field("uan0", &self.uan0())
            .field("rue0", &self.rue0())
            .field("pss1", &self.pss1())
            .field("uan1", &self.uan1())
            .finish()
    }
}
/**DSI Host PHY Status Register

You can [`read`](crate::Reg::read) this register and get [`psr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F778.html#DSI:PSR)*/
pub struct PSRrs;
impl crate::RegisterSpec for PSRrs {
    type Ux = u32;
}
///`read()` method returns [`psr::R`](R) reader structure
impl crate::Readable for PSRrs {}
///`reset()` method sets PSR to value 0
impl crate::Resettable for PSRrs {}
