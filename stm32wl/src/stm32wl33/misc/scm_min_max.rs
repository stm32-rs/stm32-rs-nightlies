///Register `SCM_MIN_MAX` reader
pub type R = crate::R<SCM_MIN_MAXrs>;
///Register `SCM_MIN_MAX` writer
pub type W = crate::W<SCM_MIN_MAXrs>;
///Field `SCM_COUNTER_MINVAL` reader - Slow Clock Measurement: minimum SCM_COUNTER value seen since the counter is ON and since last clear request.
pub type SCM_COUNTER_MINVAL_R = crate::FieldReader<u16>;
///Field `SCM_COUNTER_MAXVAL` reader - Slow Clock Measurement: maximum SCM_COUNTER value seen since the counter is ON and since last clear request.
pub type SCM_COUNTER_MAXVAL_R = crate::FieldReader<u16>;
///Field `CLEAR_MIN_MAX` writer - Write 1' to clear the SCM_COUNTER_MINVAL and SCM_COUNTER_MAXVAL bit fields.
pub type CLEAR_MIN_MAX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:14 - Slow Clock Measurement: minimum SCM_COUNTER value seen since the counter is ON and since last clear request.
    #[inline(always)]
    pub fn scm_counter_minval(&self) -> SCM_COUNTER_MINVAL_R {
        SCM_COUNTER_MINVAL_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 16:30 - Slow Clock Measurement: maximum SCM_COUNTER value seen since the counter is ON and since last clear request.
    #[inline(always)]
    pub fn scm_counter_maxval(&self) -> SCM_COUNTER_MAXVAL_R {
        SCM_COUNTER_MAXVAL_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCM_MIN_MAX")
            .field("scm_counter_minval", &self.scm_counter_minval())
            .field("scm_counter_maxval", &self.scm_counter_maxval())
            .finish()
    }
}
impl W {
    ///Bit 31 - Write 1' to clear the SCM_COUNTER_MINVAL and SCM_COUNTER_MAXVAL bit fields.
    #[inline(always)]
    pub fn clear_min_max(&mut self) -> CLEAR_MIN_MAX_W<'_, SCM_MIN_MAXrs> {
        CLEAR_MIN_MAX_W::new(self, 31)
    }
}
/**SCM_MIN_MAX register

You can [`read`](crate::Reg::read) this register and get [`scm_min_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scm_min_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MISC:SCM_MIN_MAX)*/
pub struct SCM_MIN_MAXrs;
impl crate::RegisterSpec for SCM_MIN_MAXrs {
    type Ux = u32;
}
///`read()` method returns [`scm_min_max::R`](R) reader structure
impl crate::Readable for SCM_MIN_MAXrs {}
///`write(|w| ..)` method takes [`scm_min_max::W`](W) writer structure
impl crate::Writable for SCM_MIN_MAXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SCM_MIN_MAX to value 0x7fff
impl crate::Resettable for SCM_MIN_MAXrs {
    const RESET_VALUE: u32 = 0x7fff;
}
