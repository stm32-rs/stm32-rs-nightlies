///Register `PUCRN` reader
pub type R = crate::R<PUCRNrs>;
///Register `PUCRN` writer
pub type W = crate::W<PUCRNrs>;
///Field `PUN1` reader - Port N pull-up bit 1 When set, each bit activates the pull-up on PN1 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD1 bit is also set.
pub type PUN1_R = crate::BitReader;
///Field `PUN1` writer - Port N pull-up bit 1 When set, each bit activates the pull-up on PN1 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD1 bit is also set.
pub type PUN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PUN6` reader - Port N pull-up bit 6 When set activates the pull-up on PN6 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PDN6 bit is also set.
pub type PUN6_R = crate::BitReader;
///Field `PUN6` writer - Port N pull-up bit 6 When set activates the pull-up on PN6 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PDN6 bit is also set.
pub type PUN6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PUN12` reader - Port N pull-up bit 12 When set, each bit activates the pull-up on PN12 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD12 bit is also set.
pub type PUN12_R = crate::BitReader;
///Field `PUN12` writer - Port N pull-up bit 12 When set, each bit activates the pull-up on PN12 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD12 bit is also set.
pub type PUN12_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Port N pull-up bit 1 When set, each bit activates the pull-up on PN1 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD1 bit is also set.
    #[inline(always)]
    pub fn pun1(&self) -> PUN1_R {
        PUN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 6 - Port N pull-up bit 6 When set activates the pull-up on PN6 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PDN6 bit is also set.
    #[inline(always)]
    pub fn pun6(&self) -> PUN6_R {
        PUN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 12 - Port N pull-up bit 12 When set, each bit activates the pull-up on PN12 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD12 bit is also set.
    #[inline(always)]
    pub fn pun12(&self) -> PUN12_R {
        PUN12_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRN")
            .field("pun1", &self.pun1())
            .field("pun6", &self.pun6())
            .field("pun12", &self.pun12())
            .finish()
    }
}
impl W {
    ///Bit 1 - Port N pull-up bit 1 When set, each bit activates the pull-up on PN1 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD1 bit is also set.
    #[inline(always)]
    pub fn pun1(&mut self) -> PUN1_W<'_, PUCRNrs> {
        PUN1_W::new(self, 1)
    }
    ///Bit 6 - Port N pull-up bit 6 When set activates the pull-up on PN6 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PDN6 bit is also set.
    #[inline(always)]
    pub fn pun6(&mut self) -> PUN6_W<'_, PUCRNrs> {
        PUN6_W::new(self, 6)
    }
    ///Bit 12 - Port N pull-up bit 12 When set, each bit activates the pull-up on PN12 when the APC bit is set in PWR_APCR. The pull-up is not activated if the corresponding PD12 bit is also set.
    #[inline(always)]
    pub fn pun12(&mut self) -> PUN12_W<'_, PUCRNrs> {
        PUN12_W::new(self, 12)
    }
}
/**PWR port N pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#PWR:PUCRN)*/
pub struct PUCRNrs;
impl crate::RegisterSpec for PUCRNrs {
    type Ux = u32;
}
///`read()` method returns [`pucrn::R`](R) reader structure
impl crate::Readable for PUCRNrs {}
///`write(|w| ..)` method takes [`pucrn::W`](W) writer structure
impl crate::Writable for PUCRNrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRN to value 0
impl crate::Resettable for PUCRNrs {}
