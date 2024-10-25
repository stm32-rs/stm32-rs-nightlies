///Register `ACTRL` reader
pub type R = crate::R<ACTRLrs>;
///Register `ACTRL` writer
pub type W = crate::W<ACTRLrs>;
///Field `DISMCYCINT` reader - DISMCYCINT
pub type DISMCYCINT_R = crate::BitReader;
///Field `DISMCYCINT` writer - DISMCYCINT
pub type DISMCYCINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISDEFWBUF` reader - DISDEFWBUF
pub type DISDEFWBUF_R = crate::BitReader;
///Field `DISDEFWBUF` writer - DISDEFWBUF
pub type DISDEFWBUF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISFOLD` reader - DISFOLD
pub type DISFOLD_R = crate::BitReader;
///Field `DISFOLD` writer - DISFOLD
pub type DISFOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISFPCA` reader - DISFPCA
pub type DISFPCA_R = crate::BitReader;
///Field `DISFPCA` writer - DISFPCA
pub type DISFPCA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISOOFP` reader - DISOOFP
pub type DISOOFP_R = crate::BitReader;
///Field `DISOOFP` writer - DISOOFP
pub type DISOOFP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DISMCYCINT
    #[inline(always)]
    pub fn dismcycint(&self) -> DISMCYCINT_R {
        DISMCYCINT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DISDEFWBUF
    #[inline(always)]
    pub fn disdefwbuf(&self) -> DISDEFWBUF_R {
        DISDEFWBUF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DISFOLD
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - DISFPCA
    #[inline(always)]
    pub fn disfpca(&self) -> DISFPCA_R {
        DISFPCA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DISOOFP
    #[inline(always)]
    pub fn disoofp(&self) -> DISOOFP_R {
        DISOOFP_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTRL")
            .field("dismcycint", &self.dismcycint())
            .field("disdefwbuf", &self.disdefwbuf())
            .field("disfold", &self.disfold())
            .field("disfpca", &self.disfpca())
            .field("disoofp", &self.disoofp())
            .finish()
    }
}
impl W {
    ///Bit 0 - DISMCYCINT
    #[inline(always)]
    #[must_use]
    pub fn dismcycint(&mut self) -> DISMCYCINT_W<ACTRLrs> {
        DISMCYCINT_W::new(self, 0)
    }
    ///Bit 1 - DISDEFWBUF
    #[inline(always)]
    #[must_use]
    pub fn disdefwbuf(&mut self) -> DISDEFWBUF_W<ACTRLrs> {
        DISDEFWBUF_W::new(self, 1)
    }
    ///Bit 2 - DISFOLD
    #[inline(always)]
    #[must_use]
    pub fn disfold(&mut self) -> DISFOLD_W<ACTRLrs> {
        DISFOLD_W::new(self, 2)
    }
    ///Bit 8 - DISFPCA
    #[inline(always)]
    #[must_use]
    pub fn disfpca(&mut self) -> DISFPCA_W<ACTRLrs> {
        DISFPCA_W::new(self, 8)
    }
    ///Bit 9 - DISOOFP
    #[inline(always)]
    #[must_use]
    pub fn disoofp(&mut self) -> DISOOFP_W<ACTRLrs> {
        DISOOFP_W::new(self, 9)
    }
}
/**Auxiliary control register

You can [`read`](crate::Reg::read) this register and get [`actrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F427.html#SCB_ACTRL:ACTRL)*/
pub struct ACTRLrs;
impl crate::RegisterSpec for ACTRLrs {
    type Ux = u32;
}
///`read()` method returns [`actrl::R`](R) reader structure
impl crate::Readable for ACTRLrs {}
///`write(|w| ..)` method takes [`actrl::W`](W) writer structure
impl crate::Writable for ACTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ACTRL to value 0
impl crate::Resettable for ACTRLrs {
    const RESET_VALUE: u32 = 0;
}
