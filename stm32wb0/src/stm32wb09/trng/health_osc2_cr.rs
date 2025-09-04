///Register `HEALTH_OSC2_CR` reader
pub type R = crate::R<HEALTH_OSC2_CRrs>;
///Register `HEALTH_OSC2_CR` writer
pub type W = crate::W<HEALTH_OSC2_CRrs>;
///Field `REPET_CUTOFF_OSC2` reader - Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
pub type REPET_CUTOFF_OSC2_R = crate::FieldReader;
///Field `REPET_CUTOFF_OSC2` writer - Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
pub type REPET_CUTOFF_OSC2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ADAP_CUTOFF_OSC2` reader - Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
pub type ADAP_CUTOFF_OSC2_R = crate::FieldReader<u16>;
///Field `ADAP_CUTOFF_OSC2` writer - Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
pub type ADAP_CUTOFF_OSC2_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:7 - Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
    #[inline(always)]
    pub fn repet_cutoff_osc2(&self) -> REPET_CUTOFF_OSC2_R {
        REPET_CUTOFF_OSC2_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:25 - Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
    #[inline(always)]
    pub fn adap_cutoff_osc2(&self) -> ADAP_CUTOFF_OSC2_R {
        ADAP_CUTOFF_OSC2_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HEALTH_OSC2_CR")
            .field("repet_cutoff_osc2", &self.repet_cutoff_osc2())
            .field("adap_cutoff_osc2", &self.adap_cutoff_osc2())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
    #[inline(always)]
    pub fn repet_cutoff_osc2(&mut self) -> REPET_CUTOFF_OSC2_W<HEALTH_OSC2_CRrs> {
        REPET_CUTOFF_OSC2_W::new(self, 0)
    }
    ///Bits 16:25 - Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
    #[inline(always)]
    pub fn adap_cutoff_osc2(&mut self) -> ADAP_CUTOFF_OSC2_W<HEALTH_OSC2_CRrs> {
        ADAP_CUTOFF_OSC2_W::new(self, 16)
    }
}
/**TRNG_HEALTH_OSC2_CR register

You can [`read`](crate::Reg::read) this register and get [`health_osc2_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`health_osc2_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:HEALTH_OSC2_CR)*/
pub struct HEALTH_OSC2_CRrs;
impl crate::RegisterSpec for HEALTH_OSC2_CRrs {
    type Ux = u32;
}
///`read()` method returns [`health_osc2_cr::R`](R) reader structure
impl crate::Readable for HEALTH_OSC2_CRrs {}
///`write(|w| ..)` method takes [`health_osc2_cr::W`](W) writer structure
impl crate::Writable for HEALTH_OSC2_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HEALTH_OSC2_CR to value 0x03e3_00fb
impl crate::Resettable for HEALTH_OSC2_CRrs {
    const RESET_VALUE: u32 = 0x03e3_00fb;
}
