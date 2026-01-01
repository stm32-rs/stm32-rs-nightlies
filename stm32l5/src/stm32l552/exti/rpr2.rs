///Register `RPR2` reader
pub type R = crate::R<RPR2rs>;
///Register `RPR2` writer
pub type W = crate::W<RPR2rs>;
///Field `RPIF35` reader - RPIF35
pub type RPIF35_R = crate::BitReader;
///Field `RPIF35` writer - RPIF35
pub type RPIF35_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF36` reader - RPIF36
pub type RPIF36_R = crate::BitReader;
///Field `RPIF36` writer - RPIF36
pub type RPIF36_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF37` reader - RPIF37
pub type RPIF37_R = crate::BitReader;
///Field `RPIF37` writer - RPIF37
pub type RPIF37_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RPIF38` reader - RPIF38
pub type RPIF38_R = crate::BitReader;
///Field `RPIF38` writer - RPIF38
pub type RPIF38_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - RPIF35
    #[inline(always)]
    pub fn rpif35(&self) -> RPIF35_R {
        RPIF35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RPIF36
    #[inline(always)]
    pub fn rpif36(&self) -> RPIF36_R {
        RPIF36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RPIF37
    #[inline(always)]
    pub fn rpif37(&self) -> RPIF37_R {
        RPIF37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RPIF38
    #[inline(always)]
    pub fn rpif38(&self) -> RPIF38_R {
        RPIF38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR2")
            .field("rpif35", &self.rpif35())
            .field("rpif36", &self.rpif36())
            .field("rpif37", &self.rpif37())
            .field("rpif38", &self.rpif38())
            .finish()
    }
}
impl W {
    ///Bit 3 - RPIF35
    #[inline(always)]
    pub fn rpif35(&mut self) -> RPIF35_W<'_, RPR2rs> {
        RPIF35_W::new(self, 3)
    }
    ///Bit 4 - RPIF36
    #[inline(always)]
    pub fn rpif36(&mut self) -> RPIF36_W<'_, RPR2rs> {
        RPIF36_W::new(self, 4)
    }
    ///Bit 5 - RPIF37
    #[inline(always)]
    pub fn rpif37(&mut self) -> RPIF37_W<'_, RPR2rs> {
        RPIF37_W::new(self, 5)
    }
    ///Bit 6 - RPIF38
    #[inline(always)]
    pub fn rpif38(&mut self) -> RPIF38_W<'_, RPR2rs> {
        RPIF38_W::new(self, 6)
    }
}
/**EXTI rising edge pending register

You can [`read`](crate::Reg::read) this register and get [`rpr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#EXTI:RPR2)*/
pub struct RPR2rs;
impl crate::RegisterSpec for RPR2rs {
    type Ux = u32;
}
///`read()` method returns [`rpr2::R`](R) reader structure
impl crate::Readable for RPR2rs {}
///`write(|w| ..)` method takes [`rpr2::W`](W) writer structure
impl crate::Writable for RPR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RPR2 to value 0
impl crate::Resettable for RPR2rs {}
