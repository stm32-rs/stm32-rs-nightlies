///Register `OCENCLRR` reader
pub type R = crate::R<OCENCLRRrs>;
///Register `OCENCLRR` writer
pub type W = crate::W<OCENCLRRrs>;
///Field `HSION` reader - HSION
pub type HSION_R = crate::BitReader;
///Field `HSION` writer - HSION
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIKERON` reader - HSIKERON
pub type HSIKERON_R = crate::BitReader;
///Field `HSIKERON` writer - HSIKERON
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSION` reader - CSION
pub type CSION_R = crate::BitReader;
///Field `CSION` writer - CSION
pub type CSION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSIKERON` reader - CSIKERON
pub type CSIKERON_R = crate::BitReader;
///Field `CSIKERON` writer - CSIKERON
pub type CSIKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIGBYP` reader - DIGBYP
pub type DIGBYP_R = crate::BitReader;
///Field `DIGBYP` writer - DIGBYP
pub type DIGBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEON` reader - HSEON
pub type HSEON_R = crate::BitReader;
///Field `HSEON` writer - HSEON
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEKERON` reader - HSEKERON
pub type HSEKERON_R = crate::BitReader;
///Field `HSEKERON` writer - HSEKERON
pub type HSEKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSEBYP` reader - HSEBYP
pub type HSEBYP_R = crate::BitReader;
///Field `HSEBYP` writer - HSEBYP
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HSION
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HSIKERON
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - CSION
    #[inline(always)]
    pub fn csion(&self) -> CSION_R {
        CSION_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CSIKERON
    #[inline(always)]
    pub fn csikeron(&self) -> CSIKERON_R {
        CSIKERON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - DIGBYP
    #[inline(always)]
    pub fn digbyp(&self) -> DIGBYP_R {
        DIGBYP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HSEON
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSEKERON
    #[inline(always)]
    pub fn hsekeron(&self) -> HSEKERON_R {
        HSEKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSEBYP
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCENCLRR")
            .field("hsion", &self.hsion())
            .field("hsikeron", &self.hsikeron())
            .field("csion", &self.csion())
            .field("csikeron", &self.csikeron())
            .field("digbyp", &self.digbyp())
            .field("hseon", &self.hseon())
            .field("hsekeron", &self.hsekeron())
            .field("hsebyp", &self.hsebyp())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSION
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<'_, OCENCLRRrs> {
        HSION_W::new(self, 0)
    }
    ///Bit 1 - HSIKERON
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W<'_, OCENCLRRrs> {
        HSIKERON_W::new(self, 1)
    }
    ///Bit 4 - CSION
    #[inline(always)]
    pub fn csion(&mut self) -> CSION_W<'_, OCENCLRRrs> {
        CSION_W::new(self, 4)
    }
    ///Bit 5 - CSIKERON
    #[inline(always)]
    pub fn csikeron(&mut self) -> CSIKERON_W<'_, OCENCLRRrs> {
        CSIKERON_W::new(self, 5)
    }
    ///Bit 7 - DIGBYP
    #[inline(always)]
    pub fn digbyp(&mut self) -> DIGBYP_W<'_, OCENCLRRrs> {
        DIGBYP_W::new(self, 7)
    }
    ///Bit 8 - HSEON
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, OCENCLRRrs> {
        HSEON_W::new(self, 8)
    }
    ///Bit 9 - HSEKERON
    #[inline(always)]
    pub fn hsekeron(&mut self) -> HSEKERON_W<'_, OCENCLRRrs> {
        HSEKERON_W::new(self, 9)
    }
    ///Bit 10 - HSEBYP
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<'_, OCENCLRRrs> {
        HSEBYP_W::new(self, 10)
    }
}
/**This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`ocenclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocenclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:OCENCLRR)*/
pub struct OCENCLRRrs;
impl crate::RegisterSpec for OCENCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`ocenclrr::R`](R) reader structure
impl crate::Readable for OCENCLRRrs {}
///`write(|w| ..)` method takes [`ocenclrr::W`](W) writer structure
impl crate::Writable for OCENCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OCENCLRR to value 0x01
impl crate::Resettable for OCENCLRRrs {
    const RESET_VALUE: u32 = 0x01;
}
