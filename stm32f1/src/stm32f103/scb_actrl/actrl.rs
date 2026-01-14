///Register `ACTRL` reader
pub type R = crate::R<ACTRLrs>;
///Register `ACTRL` writer
pub type W = crate::W<ACTRLrs>;
///Field `DISFOLD` reader - DISFOLD
pub type DISFOLD_R = crate::BitReader;
///Field `DISFOLD` writer - DISFOLD
pub type DISFOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FPEXCODIS` reader - FPEXCODIS
pub type FPEXCODIS_R = crate::BitReader;
///Field `FPEXCODIS` writer - FPEXCODIS
pub type FPEXCODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISRAMODE` reader - DISRAMODE
pub type DISRAMODE_R = crate::BitReader;
///Field `DISRAMODE` writer - DISRAMODE
pub type DISRAMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISITMATBFLUSH` reader - DISITMATBFLUSH
pub type DISITMATBFLUSH_R = crate::BitReader;
///Field `DISITMATBFLUSH` writer - DISITMATBFLUSH
pub type DISITMATBFLUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 2 - DISFOLD
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 10 - FPEXCODIS
    #[inline(always)]
    pub fn fpexcodis(&self) -> FPEXCODIS_R {
        FPEXCODIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DISRAMODE
    #[inline(always)]
    pub fn disramode(&self) -> DISRAMODE_R {
        DISRAMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DISITMATBFLUSH
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DISITMATBFLUSH_R {
        DISITMATBFLUSH_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTRL")
            .field("disfold", &self.disfold())
            .field("fpexcodis", &self.fpexcodis())
            .field("disramode", &self.disramode())
            .field("disitmatbflush", &self.disitmatbflush())
            .finish()
    }
}
impl W {
    ///Bit 2 - DISFOLD
    #[inline(always)]
    pub fn disfold(&mut self) -> DISFOLD_W<'_, ACTRLrs> {
        DISFOLD_W::new(self, 2)
    }
    ///Bit 10 - FPEXCODIS
    #[inline(always)]
    pub fn fpexcodis(&mut self) -> FPEXCODIS_W<'_, ACTRLrs> {
        FPEXCODIS_W::new(self, 10)
    }
    ///Bit 11 - DISRAMODE
    #[inline(always)]
    pub fn disramode(&mut self) -> DISRAMODE_W<'_, ACTRLrs> {
        DISRAMODE_W::new(self, 11)
    }
    ///Bit 12 - DISITMATBFLUSH
    #[inline(always)]
    pub fn disitmatbflush(&mut self) -> DISITMATBFLUSH_W<'_, ACTRLrs> {
        DISITMATBFLUSH_W::new(self, 12)
    }
}
/**Auxiliary control register

You can [`read`](crate::Reg::read) this register and get [`actrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F103.html#SCB_ACTRL:ACTRL)*/
pub struct ACTRLrs;
impl crate::RegisterSpec for ACTRLrs {
    type Ux = u32;
}
///`read()` method returns [`actrl::R`](R) reader structure
impl crate::Readable for ACTRLrs {}
///`write(|w| ..)` method takes [`actrl::W`](W) writer structure
impl crate::Writable for ACTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACTRL to value 0
impl crate::Resettable for ACTRLrs {}
