///Register `BKP6R` reader
pub type R = crate::R<BKP6Rrs>;
///Register `BKP6R` writer
pub type W = crate::W<BKP6Rrs>;
///Field `BKP` reader - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled.
pub type BKP_R = crate::FieldReader<u32>;
///Field `BKP` writer - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled.
pub type BKP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled.
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKP6R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    ///Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled.
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<BKP6Rrs> {
        BKP_W::new(self, 0)
    }
}
/**TAMP backup 6 register

You can [`read`](crate::Reg::read) this register and get [`bkp6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#TAMP:BKP6R)*/
pub struct BKP6Rrs;
impl crate::RegisterSpec for BKP6Rrs {
    type Ux = u32;
}
///`read()` method returns [`bkp6r::R`](R) reader structure
impl crate::Readable for BKP6Rrs {}
///`write(|w| ..)` method takes [`bkp6r::W`](W) writer structure
impl crate::Writable for BKP6Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKP6R to value 0
impl crate::Resettable for BKP6Rrs {}
