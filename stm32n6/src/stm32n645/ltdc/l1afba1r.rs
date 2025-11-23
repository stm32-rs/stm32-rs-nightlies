///Register `L1AFBA1R` reader
pub type R = crate::R<L1AFBA1Rrs>;
///Register `L1AFBA1R` writer
pub type W = crate::W<L1AFBA1Rrs>;
///Field `AFBADD1` reader - auxiliary frame buffer start address
pub type AFBADD1_R = crate::FieldReader<u32>;
///Field `AFBADD1` writer - auxiliary frame buffer start address
pub type AFBADD1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - auxiliary frame buffer start address
    #[inline(always)]
    pub fn afbadd1(&self) -> AFBADD1_R {
        AFBADD1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1AFBA1R")
            .field("afbadd1", &self.afbadd1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - auxiliary frame buffer start address
    #[inline(always)]
    pub fn afbadd1(&mut self) -> AFBADD1_W<'_, L1AFBA1Rrs> {
        AFBADD1_W::new(self, 0)
    }
}
/**LTDC layer1 auxiliary frame buffer address 1 register

You can [`read`](crate::Reg::read) this register and get [`l1afba1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1afba1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1AFBA1R)*/
pub struct L1AFBA1Rrs;
impl crate::RegisterSpec for L1AFBA1Rrs {
    type Ux = u32;
}
///`read()` method returns [`l1afba1r::R`](R) reader structure
impl crate::Readable for L1AFBA1Rrs {}
///`write(|w| ..)` method takes [`l1afba1r::W`](W) writer structure
impl crate::Writable for L1AFBA1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1AFBA1R to value 0
impl crate::Resettable for L1AFBA1Rrs {}
