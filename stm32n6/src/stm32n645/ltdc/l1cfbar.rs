///Register `L1CFBAR` reader
pub type R = crate::R<L1CFBARrs>;
///Register `L1CFBAR` writer
pub type W = crate::W<L1CFBARrs>;
///Field `CFBADD` reader - color frame buffer start address
pub type CFBADD_R = crate::FieldReader<u32>;
///Field `CFBADD` writer - color frame buffer start address
pub type CFBADD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - color frame buffer start address
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1CFBAR")
            .field("cfbadd", &self.cfbadd())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - color frame buffer start address
    #[inline(always)]
    pub fn cfbadd(&mut self) -> CFBADD_W<'_, L1CFBARrs> {
        CFBADD_W::new(self, 0)
    }
}
/**LTDC layerx color frame buffer address register

You can [`read`](crate::Reg::read) this register and get [`l1cfbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1CFBAR)*/
pub struct L1CFBARrs;
impl crate::RegisterSpec for L1CFBARrs {
    type Ux = u32;
}
///`read()` method returns [`l1cfbar::R`](R) reader structure
impl crate::Readable for L1CFBARrs {}
///`write(|w| ..)` method takes [`l1cfbar::W`](W) writer structure
impl crate::Writable for L1CFBARrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1CFBAR to value 0
impl crate::Resettable for L1CFBARrs {}
