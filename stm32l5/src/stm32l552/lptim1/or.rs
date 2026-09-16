///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `OR_0` reader - Option register bit 0
pub type OR_0_R = crate::BitReader;
///Field `OR_0` writer - Option register bit 0
pub type OR_0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OR_1` reader - Option register bit 1
pub type OR_1_R = crate::BitReader;
///Field `OR_1` writer - Option register bit 1
pub type OR_1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Option register bit 0
    #[inline(always)]
    pub fn or_0(&self) -> OR_0_R {
        OR_0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Option register bit 1
    #[inline(always)]
    pub fn or_1(&self) -> OR_1_R {
        OR_1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("or_0", &self.or_0())
            .field("or_1", &self.or_1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Option register bit 0
    #[inline(always)]
    pub fn or_0(&mut self) -> OR_0_W<'_, ORrs> {
        OR_0_W::new(self, 0)
    }
    ///Bit 1 - Option register bit 1
    #[inline(always)]
    pub fn or_1(&mut self) -> OR_1_W<'_, ORrs> {
        OR_1_W::new(self, 1)
    }
}
/**LPTIM option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#LPTIM1:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
