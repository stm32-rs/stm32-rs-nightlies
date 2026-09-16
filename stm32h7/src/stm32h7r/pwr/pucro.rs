///Register `PUCRO` reader
pub type R = crate::R<PUCROrs>;
///Register `PUCRO` writer
pub type W = crate::W<PUCROrs>;
///Field `PUO0` reader - (n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set.
pub type PUO0_R = crate::BitReader;
///Field `PUO0` writer - (n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set.
pub type PUO0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PUO1` reader - (n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set.
pub type PUO1_R = crate::BitReader;
///Field `PUO1` writer - (n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set.
pub type PUO1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PUO4` reader - Port O pull-up bit 4 When set activates the pull-up on PO4 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits PDO4 in PWR_PDCRO is also set.
pub type PUO4_R = crate::BitReader;
///Field `PUO4` writer - Port O pull-up bit 4 When set activates the pull-up on PO4 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits PDO4 in PWR_PDCRO is also set.
pub type PUO4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - (n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set.
    #[inline(always)]
    pub fn puo0(&self) -> PUO0_R {
        PUO0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - (n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set.
    #[inline(always)]
    pub fn puo1(&self) -> PUO1_R {
        PUO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Port O pull-up bit 4 When set activates the pull-up on PO4 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits PDO4 in PWR_PDCRO is also set.
    #[inline(always)]
    pub fn puo4(&self) -> PUO4_R {
        PUO4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRO")
            .field("puo0", &self.puo0())
            .field("puo1", &self.puo1())
            .field("puo4", &self.puo4())
            .finish()
    }
}
impl W {
    ///Bit 0 - (n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set.
    #[inline(always)]
    pub fn puo0(&mut self) -> PUO0_W<'_, PUCROrs> {
        PUO0_W::new(self, 0)
    }
    ///Bit 1 - (n = 1 to 0) Port O pull-up bits When set, each bit activates the pull-up on POy when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits in PWR_PDCRO is also set.
    #[inline(always)]
    pub fn puo1(&mut self) -> PUO1_W<'_, PUCROrs> {
        PUO1_W::new(self, 1)
    }
    ///Bit 4 - Port O pull-up bit 4 When set activates the pull-up on PO4 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding bits PDO4 in PWR_PDCRO is also set.
    #[inline(always)]
    pub fn puo4(&mut self) -> PUO4_W<'_, PUCROrs> {
        PUO4_W::new(self, 4)
    }
}
/**PWR port O pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucro::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucro::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:PUCRO)*/
pub struct PUCROrs;
impl crate::RegisterSpec for PUCROrs {
    type Ux = u32;
}
///`read()` method returns [`pucro::R`](R) reader structure
impl crate::Readable for PUCROrs {}
///`write(|w| ..)` method takes [`pucro::W`](W) writer structure
impl crate::Writable for PUCROrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRO to value 0
impl crate::Resettable for PUCROrs {}
