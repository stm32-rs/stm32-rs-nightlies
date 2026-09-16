///Register `OCENSETR` reader
pub type R = crate::R<OCENSETRrs>;
///Register `OCENSETR` writer
pub type W = crate::W<OCENSETRrs>;
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
///Field `HSECSSON` reader - HSECSSON
pub type HSECSSON_R = crate::BitReader;
///Field `HSECSSON` writer - HSECSSON
pub type HSECSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 11 - HSECSSON
    #[inline(always)]
    pub fn hsecsson(&self) -> HSECSSON_R {
        HSECSSON_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCENSETR")
            .field("hsion", &self.hsion())
            .field("hsikeron", &self.hsikeron())
            .field("csion", &self.csion())
            .field("csikeron", &self.csikeron())
            .field("digbyp", &self.digbyp())
            .field("hseon", &self.hseon())
            .field("hsekeron", &self.hsekeron())
            .field("hsebyp", &self.hsebyp())
            .field("hsecsson", &self.hsecsson())
            .finish()
    }
}
impl W {
    ///Bit 0 - HSION
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W<'_, OCENSETRrs> {
        HSION_W::new(self, 0)
    }
    ///Bit 1 - HSIKERON
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W<'_, OCENSETRrs> {
        HSIKERON_W::new(self, 1)
    }
    ///Bit 4 - CSION
    #[inline(always)]
    pub fn csion(&mut self) -> CSION_W<'_, OCENSETRrs> {
        CSION_W::new(self, 4)
    }
    ///Bit 5 - CSIKERON
    #[inline(always)]
    pub fn csikeron(&mut self) -> CSIKERON_W<'_, OCENSETRrs> {
        CSIKERON_W::new(self, 5)
    }
    ///Bit 7 - DIGBYP
    #[inline(always)]
    pub fn digbyp(&mut self) -> DIGBYP_W<'_, OCENSETRrs> {
        DIGBYP_W::new(self, 7)
    }
    ///Bit 8 - HSEON
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W<'_, OCENSETRrs> {
        HSEON_W::new(self, 8)
    }
    ///Bit 9 - HSEKERON
    #[inline(always)]
    pub fn hsekeron(&mut self) -> HSEKERON_W<'_, OCENSETRrs> {
        HSEKERON_W::new(self, 9)
    }
    ///Bit 10 - HSEBYP
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W<'_, OCENSETRrs> {
        HSEBYP_W::new(self, 10)
    }
    ///Bit 11 - HSECSSON
    #[inline(always)]
    pub fn hsecsson(&mut self) -> HSECSSON_W<'_, OCENSETRrs> {
        HSECSSON_W::new(self, 11)
    }
}
/**This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`ocensetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocensetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:OCENSETR)*/
pub struct OCENSETRrs;
impl crate::RegisterSpec for OCENSETRrs {
    type Ux = u32;
}
///`read()` method returns [`ocensetr::R`](R) reader structure
impl crate::Readable for OCENSETRrs {}
///`write(|w| ..)` method takes [`ocensetr::W`](W) writer structure
impl crate::Writable for OCENSETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OCENSETR to value 0x01
impl crate::Resettable for OCENSETRrs {
    const RESET_VALUE: u32 = 0x01;
}
