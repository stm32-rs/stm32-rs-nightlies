///Register `HEALTH_CR` reader
pub type R = crate::R<HEALTH_CRrs>;
///Register `HEALTH_CR` writer
pub type W = crate::W<HEALTH_CRrs>;
///Field `REPET_CUTOFF` reader - Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
pub type REPET_CUTOFF_R = crate::FieldReader;
///Field `REPET_CUTOFF` writer - Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
pub type REPET_CUTOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ADAP_CUTOFF` reader - Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
pub type ADAP_CUTOFF_R = crate::FieldReader<u16>;
///Field `ADAP_CUTOFF` writer - Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
pub type ADAP_CUTOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `ITER_ADAP` reader - Number of iterations minus 1 of Adaptive test during initialization phase. Default value is set to 0 i.e. 1 iteration.
pub type ITER_ADAP_R = crate::FieldReader;
///Field `ITER_ADAP` writer - Number of iterations minus 1 of Adaptive test during initialization phase. Default value is set to 0 i.e. 1 iteration.
pub type ITER_ADAP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:7 - Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
    #[inline(always)]
    pub fn repet_cutoff(&self) -> REPET_CUTOFF_R {
        REPET_CUTOFF_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:25 - Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
    #[inline(always)]
    pub fn adap_cutoff(&self) -> ADAP_CUTOFF_R {
        ADAP_CUTOFF_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 28:29 - Number of iterations minus 1 of Adaptive test during initialization phase. Default value is set to 0 i.e. 1 iteration.
    #[inline(always)]
    pub fn iter_adap(&self) -> ITER_ADAP_R {
        ITER_ADAP_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HEALTH_CR")
            .field("repet_cutoff", &self.repet_cutoff())
            .field("adap_cutoff", &self.adap_cutoff())
            .field("iter_adap", &self.iter_adap())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Cutoff value of Repetition Test. The default value is set to 51. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
    #[inline(always)]
    pub fn repet_cutoff(&mut self) -> REPET_CUTOFF_W<'_, HEALTH_CRrs> {
        REPET_CUTOFF_W::new(self, 0)
    }
    ///Bits 16:25 - Cutoff value of Adaptive Test. The default value is set to 699. Caution: To be handled with care as any change can lead to misbehavior of TRNG.
    #[inline(always)]
    pub fn adap_cutoff(&mut self) -> ADAP_CUTOFF_W<'_, HEALTH_CRrs> {
        ADAP_CUTOFF_W::new(self, 16)
    }
    ///Bits 28:29 - Number of iterations minus 1 of Adaptive test during initialization phase. Default value is set to 0 i.e. 1 iteration.
    #[inline(always)]
    pub fn iter_adap(&mut self) -> ITER_ADAP_W<'_, HEALTH_CRrs> {
        ITER_ADAP_W::new(self, 28)
    }
}
/**TRNG_HEALTH_CR register

You can [`read`](crate::Reg::read) this register and get [`health_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`health_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#TRNG:HEALTH_CR)*/
pub struct HEALTH_CRrs;
impl crate::RegisterSpec for HEALTH_CRrs {
    type Ux = u32;
}
///`read()` method returns [`health_cr::R`](R) reader structure
impl crate::Readable for HEALTH_CRrs {}
///`write(|w| ..)` method takes [`health_cr::W`](W) writer structure
impl crate::Writable for HEALTH_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HEALTH_CR to value 0x02bb_0033
impl crate::Resettable for HEALTH_CRrs {
    const RESET_VALUE: u32 = 0x02bb_0033;
}
