///Register `PLL3CSGR` reader
pub type R = crate::R<PLL3CSGRrs>;
///Register `PLL3CSGR` writer
pub type W = crate::W<PLL3CSGRrs>;
///Field `MOD_PER` reader - MOD_PER
pub type MOD_PER_R = crate::FieldReader<u16>;
///Field `MOD_PER` writer - MOD_PER
pub type MOD_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
///Field `TPDFN_DIS` reader - TPDFN_DIS
pub type TPDFN_DIS_R = crate::BitReader;
///Field `TPDFN_DIS` writer - TPDFN_DIS
pub type TPDFN_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPDFN_DIS` reader - RPDFN_DIS
pub type RPDFN_DIS_R = crate::BitReader;
///Field `RPDFN_DIS` writer - RPDFN_DIS
pub type RPDFN_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSCG_MODE` reader - SSCG_MODE
pub type SSCG_MODE_R = crate::BitReader;
///Field `SSCG_MODE` writer - SSCG_MODE
pub type SSCG_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INC_STEP` reader - INC_STEP
pub type INC_STEP_R = crate::FieldReader<u16>;
///Field `INC_STEP` writer - INC_STEP
pub type INC_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:12 - MOD_PER
    #[inline(always)]
    pub fn mod_per(&self) -> MOD_PER_R {
        MOD_PER_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bit 13 - TPDFN_DIS
    #[inline(always)]
    pub fn tpdfn_dis(&self) -> TPDFN_DIS_R {
        TPDFN_DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RPDFN_DIS
    #[inline(always)]
    pub fn rpdfn_dis(&self) -> RPDFN_DIS_R {
        RPDFN_DIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SSCG_MODE
    #[inline(always)]
    pub fn sscg_mode(&self) -> SSCG_MODE_R {
        SSCG_MODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:30 - INC_STEP
    #[inline(always)]
    pub fn inc_step(&self) -> INC_STEP_R {
        INC_STEP_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL3CSGR")
            .field("mod_per", &self.mod_per())
            .field("tpdfn_dis", &self.tpdfn_dis())
            .field("rpdfn_dis", &self.rpdfn_dis())
            .field("sscg_mode", &self.sscg_mode())
            .field("inc_step", &self.inc_step())
            .finish()
    }
}
impl W {
    ///Bits 0:12 - MOD_PER
    #[inline(always)]
    pub fn mod_per(&mut self) -> MOD_PER_W<PLL3CSGRrs> {
        MOD_PER_W::new(self, 0)
    }
    ///Bit 13 - TPDFN_DIS
    #[inline(always)]
    pub fn tpdfn_dis(&mut self) -> TPDFN_DIS_W<PLL3CSGRrs> {
        TPDFN_DIS_W::new(self, 13)
    }
    ///Bit 14 - RPDFN_DIS
    #[inline(always)]
    pub fn rpdfn_dis(&mut self) -> RPDFN_DIS_W<PLL3CSGRrs> {
        RPDFN_DIS_W::new(self, 14)
    }
    ///Bit 15 - SSCG_MODE
    #[inline(always)]
    pub fn sscg_mode(&mut self) -> SSCG_MODE_W<PLL3CSGRrs> {
        SSCG_MODE_W::new(self, 15)
    }
    ///Bits 16:30 - INC_STEP
    #[inline(always)]
    pub fn inc_step(&mut self) -> INC_STEP_W<PLL3CSGRrs> {
        INC_STEP_W::new(self, 16)
    }
}
/**This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`pll3csgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3csgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:PLL3CSGR)*/
pub struct PLL3CSGRrs;
impl crate::RegisterSpec for PLL3CSGRrs {
    type Ux = u32;
}
///`read()` method returns [`pll3csgr::R`](R) reader structure
impl crate::Readable for PLL3CSGRrs {}
///`write(|w| ..)` method takes [`pll3csgr::W`](W) writer structure
impl crate::Writable for PLL3CSGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL3CSGR to value 0
impl crate::Resettable for PLL3CSGRrs {}
