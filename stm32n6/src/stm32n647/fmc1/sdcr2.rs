///Register `SDCR2` reader
pub type R = crate::R<SDCR2rs>;
///Register `SDCR2` writer
pub type W = crate::W<SDCR2rs>;
///Field `NC` reader - Number of column address bits
pub type NC_R = crate::FieldReader;
///Field `NC` writer - Number of column address bits
pub type NC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NR` reader - Number of row address bits
pub type NR_R = crate::FieldReader;
///Field `NR` writer - Number of row address bits
pub type NR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MWID` reader - Memory data bus width.
pub type MWID_R = crate::FieldReader;
///Field `MWID` writer - Memory data bus width.
pub type MWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NB` reader - Number of banks
pub type NB_R = crate::BitReader;
///Field `NB` writer - Number of banks
pub type NB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAS` reader - CAS Latency
pub type CAS_R = crate::FieldReader;
///Field `CAS` writer - CAS Latency
pub type CAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WP` reader - Write protection
pub type WP_R = crate::BitReader;
///Field `WP` writer - Write protection
pub type WP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDEN` reader - SDRAM device enable
pub type SDEN_R = crate::BitReader;
///Field `SDEN` writer - SDRAM device enable
pub type SDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDINIT` reader - SDRAM device initialization
pub type SDINIT_R = crate::BitReader;
///Field `SDINIT` writer - SDRAM device initialization
pub type SDINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 6 - Number of banks
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
    ///Bit 16 - SDRAM device enable
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SDRAM device initialization
    #[inline(always)]
    pub fn sdinit(&self) -> SDINIT_R {
        SDINIT_R::new(((self.bits >> 17) & 1) != 0)
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
            .field("sden", &self.sden())
            .field("sdinit", &self.sdinit())
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
    ///Bits 4:5 - Memory data bus width.
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W<'_, SDCR2rs> {
        MWID_W::new(self, 4)
    }
    ///Bit 6 - Number of banks
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W<'_, SDCR2rs> {
        NB_W::new(self, 6)
    }
    ///Bits 7:8 - CAS Latency
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W<'_, SDCR2rs> {
        CAS_W::new(self, 7)
    }
    ///Bit 9 - Write protection
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W<'_, SDCR2rs> {
        WP_W::new(self, 9)
    }
    ///Bit 16 - SDRAM device enable
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W<'_, SDCR2rs> {
        SDEN_W::new(self, 16)
    }
    ///Bit 17 - SDRAM device initialization
    #[inline(always)]
    pub fn sdinit(&mut self) -> SDINIT_W<'_, SDCR2rs> {
        SDINIT_W::new(self, 17)
    }
}
/**SDRAM control registers for SDRAM device 2

You can [`read`](crate::Reg::read) this register and get [`sdcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#FMC1:SDCR2)*/
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
