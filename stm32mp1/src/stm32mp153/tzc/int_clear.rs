///Register `INT_CLEAR` reader
pub type R = crate::R<INT_CLEARrs>;
///Register `INT_CLEAR` writer
pub type W = crate::W<INT_CLEARrs>;
///Field `CLEAR` writer - CLEAR
pub type CLEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CLEAR").finish()
    }
}
impl W {
    ///Bits 0:1 - CLEAR
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W<'_, INT_CLEARrs> {
        CLEAR_W::new(self, 0)
    }
}
/**Interrupt clear for each filter.

You can [`read`](crate::Reg::read) this register and get [`int_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:INT_CLEAR)*/
pub struct INT_CLEARrs;
impl crate::RegisterSpec for INT_CLEARrs {
    type Ux = u32;
}
///`read()` method returns [`int_clear::R`](R) reader structure
impl crate::Readable for INT_CLEARrs {}
///`write(|w| ..)` method takes [`int_clear::W`](W) writer structure
impl crate::Writable for INT_CLEARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INT_CLEAR to value 0
impl crate::Resettable for INT_CLEARrs {}
