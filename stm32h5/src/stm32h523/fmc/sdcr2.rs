///Register `SDCR2` reader
pub type R = crate::R<SDCR2rs>;
///Register `SDCR2` writer
pub type W = crate::W<SDCR2rs>;
///CAS Latency
pub use super::sdcr1::CAS;
///Field `CAS` reader - CAS Latency
pub use super::sdcr1::CAS_R;
///Field `CAS` writer - CAS Latency
pub use super::sdcr1::CAS_W;
///Memory data bus width.
pub use super::sdcr1::MWID;
///Field `MWID` reader - Memory data bus width.
pub use super::sdcr1::MWID_R;
///Field `MWID` writer - Memory data bus width.
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
/**Read pipe

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RPIPE {
    ///0: No clock cycle delay
    NoDelay = 0,
    ///1: One clock cycle delay
    Clocks1 = 1,
    ///2: Two clock cycles delay
    Clocks2 = 2,
}
impl From<RPIPE> for u8 {
    #[inline(always)]
    fn from(variant: RPIPE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RPIPE {
    type Ux = u8;
}
impl crate::IsEnum for RPIPE {}
///Field `RPIPE` reader - Read pipe
pub type RPIPE_R = crate::FieldReader<RPIPE>;
impl RPIPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RPIPE> {
        match self.bits {
            0 => Some(RPIPE::NoDelay),
            1 => Some(RPIPE::Clocks1),
            2 => Some(RPIPE::Clocks2),
            _ => None,
        }
    }
    ///No clock cycle delay
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == RPIPE::NoDelay
    }
    ///One clock cycle delay
    #[inline(always)]
    pub fn is_clocks1(&self) -> bool {
        *self == RPIPE::Clocks1
    }
    ///Two clock cycles delay
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == RPIPE::Clocks2
    }
}
///Field `RPIPE` writer - Read pipe
pub type RPIPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, RPIPE>;
impl<'a, REG> RPIPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock cycle delay
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut crate::W<REG> {
        self.variant(RPIPE::NoDelay)
    }
    ///One clock cycle delay
    #[inline(always)]
    pub fn clocks1(self) -> &'a mut crate::W<REG> {
        self.variant(RPIPE::Clocks1)
    }
    ///Two clock cycles delay
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut crate::W<REG> {
        self.variant(RPIPE::Clocks2)
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
    ///Bits 4:5 - Memory data bus width.
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Number of internal banks
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:8 - CAS Latency
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
    ///Bits 13:14 - Read pipe
    #[inline(always)]
    pub fn rpipe(&self) -> RPIPE_R {
        RPIPE_R::new(((self.bits >> 13) & 3) as u8)
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
            .field("rpipe", &self.rpipe())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Number of column address bits
    #[inline(always)]
    pub fn nc(&mut self) -> NC_W<SDCR2rs> {
        NC_W::new(self, 0)
    }
    ///Bits 2:3 - Number of row address bits
    #[inline(always)]
    pub fn nr(&mut self) -> NR_W<SDCR2rs> {
        NR_W::new(self, 2)
    }
    ///Bits 4:5 - Memory data bus width.
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<SDCR2rs> {
        MWID_W::new(self, 4)
    }
    ///Bit 6 - Number of internal banks
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W<SDCR2rs> {
        NB_W::new(self, 6)
    }
    ///Bits 7:8 - CAS Latency
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W<SDCR2rs> {
        CAS_W::new(self, 7)
    }
    ///Bit 9 - Write protection
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W<SDCR2rs> {
        WP_W::new(self, 9)
    }
    ///Bits 10:11 - SDRAM clock configuration
    #[inline(always)]
    pub fn sdclk(&mut self) -> SDCLK_W<SDCR2rs> {
        SDCLK_W::new(self, 10)
    }
    ///Bit 12 - Burst read
    #[inline(always)]
    pub fn rburst(&mut self) -> RBURST_W<SDCR2rs> {
        RBURST_W::new(self, 12)
    }
    ///Bits 13:14 - Read pipe
    #[inline(always)]
    pub fn rpipe(&mut self) -> RPIPE_W<SDCR2rs> {
        RPIPE_W::new(self, 13)
    }
}
/**SDRAM control registers 1,2

You can [`read`](crate::Reg::read) this register and get [`sdcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H523.html#FMC:SDCR2)*/
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
