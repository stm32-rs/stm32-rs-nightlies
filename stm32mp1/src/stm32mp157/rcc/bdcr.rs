///Register `BDCR` reader
pub type R = crate::R<BDCRrs>;
///Register `BDCR` writer
pub type W = crate::W<BDCRrs>;
///Field `LSEON` reader - LSEON
pub type LSEON_R = crate::BitReader;
///Field `LSEON` writer - LSEON
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEBYP` reader - LSEBYP
pub type LSEBYP_R = crate::BitReader;
///Field `LSEBYP` writer - LSEBYP
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDY` reader - LSERDY
pub type LSERDY_R = crate::BitReader;
///Field `DIGBYP` reader - DIGBYP
pub type DIGBYP_R = crate::BitReader;
///Field `DIGBYP` writer - DIGBYP
pub type DIGBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSEDRV` reader - LSEDRV
pub type LSEDRV_R = crate::FieldReader;
///Field `LSEDRV` writer - LSEDRV
pub type LSEDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `LSECSSON` reader - LSECSSON
pub type LSECSSON_R = crate::BitReader;
///Field `LSECSSON` writer - LSECSSON
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSECSSD` reader - LSECSSD
pub type LSECSSD_R = crate::BitReader;
///Field `RTCSRC` reader - RTCSRC
pub type RTCSRC_R = crate::FieldReader;
///Field `RTCCKEN` reader - RTCCKEN
pub type RTCCKEN_R = crate::BitReader;
///Field `RTCCKEN` writer - RTCCKEN
pub type RTCCKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSWRST` reader - VSWRST
pub type VSWRST_R = crate::BitReader;
///Field `VSWRST` writer - VSWRST
pub type VSWRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSEON
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSEBYP
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSERDY
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DIGBYP
    #[inline(always)]
    pub fn digbyp(&self) -> DIGBYP_R {
        DIGBYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - LSEDRV
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - LSECSSON
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LSECSSD
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 16:17 - RTCSRC
    #[inline(always)]
    pub fn rtcsrc(&self) -> RTCSRC_R {
        RTCSRC_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 20 - RTCCKEN
    #[inline(always)]
    pub fn rtccken(&self) -> RTCCKEN_R {
        RTCCKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 31 - VSWRST
    #[inline(always)]
    pub fn vswrst(&self) -> VSWRST_R {
        VSWRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR")
            .field("lseon", &self.lseon())
            .field("lsebyp", &self.lsebyp())
            .field("lserdy", &self.lserdy())
            .field("digbyp", &self.digbyp())
            .field("lsedrv", &self.lsedrv())
            .field("lsecsson", &self.lsecsson())
            .field("lsecssd", &self.lsecssd())
            .field("rtcsrc", &self.rtcsrc())
            .field("rtccken", &self.rtccken())
            .field("vswrst", &self.vswrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSEON
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<'_, BDCRrs> {
        LSEON_W::new(self, 0)
    }
    ///Bit 1 - LSEBYP
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<'_, BDCRrs> {
        LSEBYP_W::new(self, 1)
    }
    ///Bit 3 - DIGBYP
    #[inline(always)]
    pub fn digbyp(&mut self) -> DIGBYP_W<'_, BDCRrs> {
        DIGBYP_W::new(self, 3)
    }
    ///Bits 4:5 - LSEDRV
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W<'_, BDCRrs> {
        LSEDRV_W::new(self, 4)
    }
    ///Bit 8 - LSECSSON
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W<'_, BDCRrs> {
        LSECSSON_W::new(self, 8)
    }
    ///Bit 20 - RTCCKEN
    #[inline(always)]
    pub fn rtccken(&mut self) -> RTCCKEN_W<'_, BDCRrs> {
        RTCCKEN_W::new(self, 20)
    }
    ///Bit 31 - VSWRST
    #[inline(always)]
    pub fn vswrst(&mut self) -> VSWRST_W<'_, BDCRrs> {
        VSWRST_W::new(self, 31)
    }
}
/**This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:BDCR)*/
pub struct BDCRrs;
impl crate::RegisterSpec for BDCRrs {
    type Ux = u32;
}
///`read()` method returns [`bdcr::R`](R) reader structure
impl crate::Readable for BDCRrs {}
///`write(|w| ..)` method takes [`bdcr::W`](W) writer structure
impl crate::Writable for BDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BDCR to value 0x20
impl crate::Resettable for BDCRrs {
    const RESET_VALUE: u32 = 0x20;
}
