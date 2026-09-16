///Register `PVTTMR_CR` reader
pub type R = crate::R<PVTTMR_CRrs>;
///Register `PVTTMR_CR` writer
pub type W = crate::W<PVTTMR_CRrs>;
///Field `TMR_DELAY` reader - Timer delay
pub type TMR_DELAY_R = crate::FieldReader<u16>;
///Field `TMR_DELAY` writer - Timer delay
pub type TMR_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TMR_RUN` reader - Timer count enable bit
pub type TMR_RUN_R = crate::BitReader;
///Field `TMR_RUN` writer - Timer count enable bit
pub type TMR_RUN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Timer delay
    #[inline(always)]
    pub fn tmr_delay(&self) -> TMR_DELAY_R {
        TMR_DELAY_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - Timer count enable bit
    #[inline(always)]
    pub fn tmr_run(&self) -> TMR_RUN_R {
        TMR_RUN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVTTMR_CR")
            .field("tmr_delay", &self.tmr_delay())
            .field("tmr_run", &self.tmr_run())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Timer delay
    #[inline(always)]
    pub fn tmr_delay(&mut self) -> TMR_DELAY_W<'_, PVTTMR_CRrs> {
        TMR_DELAY_W::new(self, 0)
    }
    ///Bit 16 - Timer count enable bit
    #[inline(always)]
    pub fn tmr_run(&mut self) -> TMR_RUN_W<'_, PVTTMR_CRrs> {
        TMR_RUN_W::new(self, 16)
    }
}
/**DTS PVT timer control register

You can [`read`](crate::Reg::read) this register and get [`pvttmr_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pvttmr_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DTS:PVTTMR_CR)*/
pub struct PVTTMR_CRrs;
impl crate::RegisterSpec for PVTTMR_CRrs {
    type Ux = u32;
}
///`read()` method returns [`pvttmr_cr::R`](R) reader structure
impl crate::Readable for PVTTMR_CRrs {}
///`write(|w| ..)` method takes [`pvttmr_cr::W`](W) writer structure
impl crate::Writable for PVTTMR_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PVTTMR_CR to value 0
impl crate::Resettable for PVTTMR_CRrs {}
