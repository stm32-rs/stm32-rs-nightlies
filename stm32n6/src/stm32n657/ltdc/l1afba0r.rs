///Register `L1AFBA0R` reader
pub type R = crate::R<L1AFBA0Rrs>;
///Register `L1AFBA0R` writer
pub type W = crate::W<L1AFBA0Rrs>;
///Field `AFBADD0` reader - frame buffer start address
pub type AFBADD0_R = crate::FieldReader<u32>;
///Field `AFBADD0` writer - frame buffer start address
pub type AFBADD0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - frame buffer start address
    #[inline(always)]
    pub fn afbadd0(&self) -> AFBADD0_R {
        AFBADD0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1AFBA0R")
            .field("afbadd0", &self.afbadd0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - frame buffer start address
    #[inline(always)]
    pub fn afbadd0(&mut self) -> AFBADD0_W<'_, L1AFBA0Rrs> {
        AFBADD0_W::new(self, 0)
    }
}
/**LTDC layer1 auxiliary frame buffer address 0 register

You can [`read`](crate::Reg::read) this register and get [`l1afba0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1afba0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#LTDC:L1AFBA0R)*/
pub struct L1AFBA0Rrs;
impl crate::RegisterSpec for L1AFBA0Rrs {
    type Ux = u32;
}
///`read()` method returns [`l1afba0r::R`](R) reader structure
impl crate::Readable for L1AFBA0Rrs {}
///`write(|w| ..)` method takes [`l1afba0r::W`](W) writer structure
impl crate::Writable for L1AFBA0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1AFBA0R to value 0
impl crate::Resettable for L1AFBA0Rrs {}
