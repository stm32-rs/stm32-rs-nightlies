///Register `TAMP_BKP18R` reader
pub type R = crate::R<TAMP_BKP18Rrs>;
///Register `TAMP_BKP18R` writer
pub type W = crate::W<TAMP_BKP18Rrs>;
///Field `BKP` reader - The application can write or read data to and from these registers. They are powered-on by VBAT when VDD is switched off, so that they are not reset by System reset, and their contents remain valid when the device operates in low-power mode. In the default configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled.
pub type BKP_R = crate::FieldReader<u32>;
///Field `BKP` writer - The application can write or read data to and from these registers. They are powered-on by VBAT when VDD is switched off, so that they are not reset by System reset, and their contents remain valid when the device operates in low-power mode. In the default configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled.
pub type BKP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The application can write or read data to and from these registers. They are powered-on by VBAT when VDD is switched off, so that they are not reset by System reset, and their contents remain valid when the device operates in low-power mode. In the default configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled.
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMP_BKP18R")
            .field("bkp", &self.bkp())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - The application can write or read data to and from these registers. They are powered-on by VBAT when VDD is switched off, so that they are not reset by System reset, and their contents remain valid when the device operates in low-power mode. In the default configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled.
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<TAMP_BKP18Rrs> {
        BKP_W::new(self, 0)
    }
}
/**TAMP backup 18 register

You can [`read`](crate::Reg::read) this register and get [`tamp_bkp18r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tamp_bkp18r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#TAMP:TAMP_BKP18R)*/
pub struct TAMP_BKP18Rrs;
impl crate::RegisterSpec for TAMP_BKP18Rrs {
    type Ux = u32;
}
///`read()` method returns [`tamp_bkp18r::R`](R) reader structure
impl crate::Readable for TAMP_BKP18Rrs {}
///`write(|w| ..)` method takes [`tamp_bkp18r::W`](W) writer structure
impl crate::Writable for TAMP_BKP18Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TAMP_BKP18R to value 0
impl crate::Resettable for TAMP_BKP18Rrs {
    const RESET_VALUE: u32 = 0;
}
