///Register `BKP1R` reader
pub type R = crate::R<BKP1Rrs>;
///Register `BKP1R` writer
pub type W = crate::W<BKP1Rrs>;
///Field `BKP` reader - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set.
pub type BKP_R = crate::FieldReader<u32>;
///Field `BKP` writer - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set.
pub type BKP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set.
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKP1R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    ///Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set.
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<BKP1Rrs> {
        BKP_W::new(self, 0)
    }
}
/**TAMP backup 1 register

You can [`read`](crate::Reg::read) this register and get [`bkp1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#TAMP:BKP1R)*/
pub struct BKP1Rrs;
impl crate::RegisterSpec for BKP1Rrs {
    type Ux = u32;
}
///`read()` method returns [`bkp1r::R`](R) reader structure
impl crate::Readable for BKP1Rrs {}
///`write(|w| ..)` method takes [`bkp1r::W`](W) writer structure
impl crate::Writable for BKP1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKP1R to value 0
impl crate::Resettable for BKP1Rrs {}
