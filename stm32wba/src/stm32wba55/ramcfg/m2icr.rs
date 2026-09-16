///Register `M2ICR` reader
pub type R = crate::R<M2ICRrs>;
///Register `M2ICR` writer
pub type W = crate::W<M2ICRrs>;
///Field `CPED` reader - Clear parity error detect bit Writing 1 to this bit clears the PED bit in RAMCFG_M2ISR. Reading this bit returns the value of the RAMCFG_M2ISR PED bit.
pub type CPED_R = crate::BitReader;
///Field `CPED` writer - Clear parity error detect bit Writing 1 to this bit clears the PED bit in RAMCFG_M2ISR. Reading this bit returns the value of the RAMCFG_M2ISR PED bit.
pub type CPED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Clear parity error detect bit Writing 1 to this bit clears the PED bit in RAMCFG_M2ISR. Reading this bit returns the value of the RAMCFG_M2ISR PED bit.
    #[inline(always)]
    pub fn cped(&self) -> CPED_R {
        CPED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2ICR").field("cped", &self.cped()).finish()
    }
}
impl W {
    ///Bit 1 - Clear parity error detect bit Writing 1 to this bit clears the PED bit in RAMCFG_M2ISR. Reading this bit returns the value of the RAMCFG_M2ISR PED bit.
    #[inline(always)]
    pub fn cped(&mut self) -> CPED_W<'_, M2ICRrs> {
        CPED_W::new(self, 1)
    }
}
/**RAMCFG SRAM2 interrupt clear register

You can [`read`](crate::Reg::read) this register and get [`m2icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#RAMCFG:M2ICR)*/
pub struct M2ICRrs;
impl crate::RegisterSpec for M2ICRrs {
    type Ux = u32;
}
///`read()` method returns [`m2icr::R`](R) reader structure
impl crate::Readable for M2ICRrs {}
///`write(|w| ..)` method takes [`m2icr::W`](W) writer structure
impl crate::Writable for M2ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M2ICR to value 0
impl crate::Resettable for M2ICRrs {}
