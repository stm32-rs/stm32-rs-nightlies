///Register `CFGR` reader
pub type R = crate::R<CFGRrs>;
///Register `CFGR` writer
pub type W = crate::W<CFGRrs>;
///Field `CLKDIV` reader - Clock divide ratio (for FMC_CLK signal)
pub type CLKDIV_R = crate::FieldReader;
///Field `CLKDIV` writer - Clock divide ratio (for FMC_CLK signal)
pub type CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CCLKEN` reader - Continuous clock enable
pub type CCLKEN_R = crate::BitReader;
///Field `CCLKEN` writer - Continuous clock enable
pub type CCLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BMAP0` reader - FMC memory region mapping
pub type BMAP0_R = crate::BitReader;
///Field `BMAP0` writer - FMC memory region mapping
pub type BMAP0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BMAP1` reader - FMC memory region mapping
pub type BMAP1_R = crate::BitReader;
///Field `BMAP1` writer - FMC memory region mapping
pub type BMAP1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMCEN` reader - FMC controller enable
pub type FMCEN_R = crate::BitReader;
///Field `FMCEN` writer - FMC controller enable
pub type FMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 16:19 - Clock divide ratio (for FMC_CLK signal)
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - Continuous clock enable
    #[inline(always)]
    pub fn cclken(&self) -> CCLKEN_R {
        CCLKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - FMC memory region mapping
    #[inline(always)]
    pub fn bmap0(&self) -> BMAP0_R {
        BMAP0_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - FMC memory region mapping
    #[inline(always)]
    pub fn bmap1(&self) -> BMAP1_R {
        BMAP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 31 - FMC controller enable
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("clkdiv", &self.clkdiv())
            .field("cclken", &self.cclken())
            .field("bmap0", &self.bmap0())
            .field("bmap1", &self.bmap1())
            .field("fmcen", &self.fmcen())
            .finish()
    }
}
impl W {
    ///Bits 16:19 - Clock divide ratio (for FMC_CLK signal)
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W<'_, CFGRrs> {
        CLKDIV_W::new(self, 16)
    }
    ///Bit 20 - Continuous clock enable
    #[inline(always)]
    pub fn cclken(&mut self) -> CCLKEN_W<'_, CFGRrs> {
        CCLKEN_W::new(self, 20)
    }
    ///Bit 24 - FMC memory region mapping
    #[inline(always)]
    pub fn bmap0(&mut self) -> BMAP0_W<'_, CFGRrs> {
        BMAP0_W::new(self, 24)
    }
    ///Bit 25 - FMC memory region mapping
    #[inline(always)]
    pub fn bmap1(&mut self) -> BMAP1_W<'_, CFGRrs> {
        BMAP1_W::new(self, 25)
    }
    ///Bit 31 - FMC controller enable
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W<'_, CFGRrs> {
        FMCEN_W::new(self, 31)
    }
}
/**FMC common configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#FMC1:CFGR)*/
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
///`read()` method returns [`cfgr::R`](R) reader structure
impl crate::Readable for CFGRrs {}
///`write(|w| ..)` method takes [`cfgr::W`](W) writer structure
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGRrs {}
