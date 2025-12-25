///Register `PDCRO` reader
pub type R = crate::R<PDCROrs>;
///Register `PDCRO` writer
pub type W = crate::W<PDCROrs>;
///Field `PDO0` reader - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
pub type PDO0_R = crate::BitReader;
///Field `PDO0` writer - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
pub type PDO0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDO1` reader - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
pub type PDO1_R = crate::BitReader;
///Field `PDO1` writer - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
pub type PDO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDO2` reader - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
pub type PDO2_R = crate::BitReader;
///Field `PDO2` writer - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
pub type PDO2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDO3` reader - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
pub type PDO3_R = crate::BitReader;
///Field `PDO3` writer - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
pub type PDO3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDO4` reader - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
pub type PDO4_R = crate::BitReader;
///Field `PDO4` writer - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
pub type PDO4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdo0(&self) -> PDO0_R {
        PDO0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdo1(&self) -> PDO1_R {
        PDO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdo2(&self) -> PDO2_R {
        PDO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdo3(&self) -> PDO3_R {
        PDO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdo4(&self) -> PDO4_R {
        PDO4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRO")
            .field("pdo0", &self.pdo0())
            .field("pdo1", &self.pdo1())
            .field("pdo2", &self.pdo2())
            .field("pdo3", &self.pdo3())
            .field("pdo4", &self.pdo4())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdo0(&mut self) -> PDO0_W<'_, PDCROrs> {
        PDO0_W::new(self, 0)
    }
    ///Bit 1 - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdo1(&mut self) -> PDO1_W<'_, PDCROrs> {
        PDO1_W::new(self, 1)
    }
    ///Bit 2 - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdo2(&mut self) -> PDO2_W<'_, PDCROrs> {
        PDO2_W::new(self, 2)
    }
    ///Bit 3 - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdo3(&mut self) -> PDO3_W<'_, PDCROrs> {
        PDO3_W::new(self, 3)
    }
    ///Bit 4 - Port O pull-down bit y When set, each bit activates the pull-down on POy when the APC bit is set in PWR_APCR.
    #[inline(always)]
    pub fn pdo4(&mut self) -> PDO4_W<'_, PDCROrs> {
        PDO4_W::new(self, 4)
    }
}
/**PWR port O pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcro::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcro::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:PDCRO)*/
pub struct PDCROrs;
impl crate::RegisterSpec for PDCROrs {
    type Ux = u32;
}
///`read()` method returns [`pdcro::R`](R) reader structure
impl crate::Readable for PDCROrs {}
///`write(|w| ..)` method takes [`pdcro::W`](W) writer structure
impl crate::Writable for PDCROrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRO to value 0
impl crate::Resettable for PDCROrs {}
