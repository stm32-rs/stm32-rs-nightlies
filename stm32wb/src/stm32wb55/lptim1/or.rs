///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
///Field `OR1` reader - Option register bit 1
pub type OR1_R = crate::BitReader;
///Field `OR1` writer - Option register bit 1
pub type OR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OR2` reader - Option register bit 2
pub type OR2_R = crate::BitReader;
///Field `OR2` writer - Option register bit 2
pub type OR2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Option register bit 1
    #[inline(always)]
    pub fn or1(&self) -> OR1_R {
        OR1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Option register bit 2
    #[inline(always)]
    pub fn or2(&self) -> OR2_R {
        OR2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("or1", &self.or1())
            .field("or2", &self.or2())
            .finish()
    }
}
impl W {
    ///Bit 0 - Option register bit 1
    #[inline(always)]
    pub fn or1(&mut self) -> OR1_W<'_, ORrs> {
        OR1_W::new(self, 0)
    }
    ///Bit 1 - Option register bit 2
    #[inline(always)]
    pub fn or2(&mut self) -> OR2_W<'_, ORrs> {
        OR2_W::new(self, 1)
    }
}
/**Option Register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#LPTIM1:OR)*/
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
