///Register `SDCR2` reader
pub type R = crate::R<SDCR2rs>;
///Register `SDCR2` writer
pub type W = crate::W<SDCR2rs>;
///CAS latency
pub use super::sdcr1::CAS;
///Field `CAS` reader - CAS latency
pub use super::sdcr1::CAS_R;
///Field `CAS` writer - CAS latency
pub use super::sdcr1::CAS_W;
///Memory data bus width
pub use super::sdcr1::MWID;
///Field `MWID` reader - Memory data bus width
pub use super::sdcr1::MWID_R;
///Field `MWID` writer - Memory data bus width
pub use super::sdcr1::MWID_W;
///Number of internal banks
pub use super::sdcr1::NB;
///Field `NB` reader - Number of internal banks
pub use super::sdcr1::NB_R;
///Field `NB` writer - Number of internal banks
pub use super::sdcr1::NB_W;
///Number of column address bits
pub use super::sdcr1::NC;
///Field `NC` reader - Number of column address bits
pub use super::sdcr1::NC_R;
///Field `NC` writer - Number of column address bits
pub use super::sdcr1::NC_W;
///Number of row address bits
pub use super::sdcr1::NR;
///Field `NR` reader - Number of row address bits
pub use super::sdcr1::NR_R;
///Field `NR` writer - Number of row address bits
pub use super::sdcr1::NR_W;
///SDRAM clock configuration
pub use super::sdcr1::SDCLK;
///Field `SDCLK` reader - SDRAM clock configuration
pub use super::sdcr1::SDCLK_R;
///Field `SDCLK` writer - SDRAM clock configuration
pub use super::sdcr1::SDCLK_W;
///Write protection
pub use super::sdcr1::WP;
///Field `WP` reader - Write protection
pub use super::sdcr1::WP_R;
///Field `WP` writer - Write protection
pub use super::sdcr1::WP_W;
/**Burst read

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBURST {
    ///0: Single read requests are not managed as bursts
    Disabled = 0,
    ///1: Single read requests are always managed as bursts
    Enabled = 1,
}
impl From<RBURST> for bool {
    #[inline(always)]
    fn from(variant: RBURST) -> Self {
        variant as u8 != 0
    }
}
///Field `RBURST` reader - Burst read
pub type RBURST_R = crate::BitReader<RBURST>;
impl RBURST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RBURST {
        match self.bits {
            false => RBURST::Disabled,
            true => RBURST::Enabled,
        }
    }
    ///Single read requests are not managed as bursts
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RBURST::Disabled
    }
    ///Single read requests are always managed as bursts
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RBURST::Enabled
    }
}
///Field `RBURST` writer - Burst read
pub type RBURST_W<'a, REG> = crate::BitWriter<'a, REG, RBURST>;
impl<'a, REG> RBURST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Single read requests are not managed as bursts
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RBURST::Disabled)
    }
    ///Single read requests are always managed as bursts
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RBURST::Enabled)
    }
}
impl R {
    ///Bits 0:1 - Number of column address bits
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Number of row address bits
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Memory data bus width
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Number of internal banks
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:8 - CAS latency
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - Write protection
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - SDRAM clock configuration
    #[inline(always)]
    pub fn sdclk(&self) -> SDCLK_R {
        SDCLK_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Burst read
    #[inline(always)]
    pub fn rburst(&self) -> RBURST_R {
        RBURST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDCR2")
            .field("nc", &self.nc())
            .field("nr", &self.nr())
            .field("mwid", &self.mwid())
            .field("nb", &self.nb())
            .field("cas", &self.cas())
            .field("wp", &self.wp())
            .field("sdclk", &self.sdclk())
            .field("rburst", &self.rburst())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Number of column address bits
    #[inline(always)]
    pub fn nc(&mut self) -> NC_W<'_, SDCR2rs> {
        NC_W::new(self, 0)
    }
    ///Bits 2:3 - Number of row address bits
    #[inline(always)]
    pub fn nr(&mut self) -> NR_W<'_, SDCR2rs> {
        NR_W::new(self, 2)
    }
    ///Bits 4:5 - Memory data bus width
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<'_, SDCR2rs> {
        MWID_W::new(self, 4)
    }
    ///Bit 6 - Number of internal banks
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W<'_, SDCR2rs> {
        NB_W::new(self, 6)
    }
    ///Bits 7:8 - CAS latency
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W<'_, SDCR2rs> {
        CAS_W::new(self, 7)
    }
    ///Bit 9 - Write protection
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W<'_, SDCR2rs> {
        WP_W::new(self, 9)
    }
    ///Bits 10:11 - SDRAM clock configuration
    #[inline(always)]
    pub fn sdclk(&mut self) -> SDCLK_W<'_, SDCR2rs> {
        SDCLK_W::new(self, 10)
    }
    ///Bit 12 - Burst read
    #[inline(always)]
    pub fn rburst(&mut self) -> RBURST_W<'_, SDCR2rs> {
        RBURST_W::new(self, 12)
    }
}
/**SDRAM Control Register 2

You can [`read`](crate::Reg::read) this register and get [`sdcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F723.html#FMC:SDCR2)*/
pub struct SDCR2rs;
impl crate::RegisterSpec for SDCR2rs {
    type Ux = u32;
}
///`read()` method returns [`sdcr2::R`](R) reader structure
impl crate::Readable for SDCR2rs {}
///`write(|w| ..)` method takes [`sdcr2::W`](W) writer structure
impl crate::Writable for SDCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SDCR2 to value 0x02d0
impl crate::Resettable for SDCR2rs {
    const RESET_VALUE: u32 = 0x02d0;
}
