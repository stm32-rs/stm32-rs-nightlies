///Register `BKP13R` reader
pub type R = crate::R<BKP13Rrs>;
///Register `BKP13R` writer
pub type W = crate::W<BKP13Rrs>;
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
        f.debug_struct("BKP13R").field("bkp", &self.bkp()).finish()
    }
}
impl W {
    ///Bits 0:31 - The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled.
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<BKP13Rrs> {
        BKP_W::new(self, 0)
    }
}
/**TAMP backup 13 register

You can [`read`](crate::Reg::read) this register and get [`bkp13r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp13r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#TAMP:BKP13R)*/
pub struct BKP13Rrs;
impl crate::RegisterSpec for BKP13Rrs {
    type Ux = u32;
}
///`read()` method returns [`bkp13r::R`](R) reader structure
impl crate::Readable for BKP13Rrs {}
///`write(|w| ..)` method takes [`bkp13r::W`](W) writer structure
impl crate::Writable for BKP13Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BKP13R to value 0
impl crate::Resettable for BKP13Rrs {}
