///Register `M2IER` reader
pub type R = crate::R<M2IERrs>;
///Register `M2IER` writer
pub type W = crate::W<M2IERrs>;
///Field `PEIE` reader - Parity error interrupt enable
pub type PEIE_R = crate::BitReader;
///Field `PEIE` writer - Parity error interrupt enable
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PENMI` reader - Parity error NMI. This bit is set by software and cleared only by a global RAMCFG reset Note: When PENMI bit is set, the RAMCFG maskable interrupt is not generated for a parity error whatever PEIE bit value.
pub type PENMI_R = crate::BitReader;
///Field `PENMI` writer - Parity error NMI. This bit is set by software and cleared only by a global RAMCFG reset Note: When PENMI bit is set, the RAMCFG maskable interrupt is not generated for a parity error whatever PEIE bit value.
pub type PENMI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1 - Parity error interrupt enable
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Parity error NMI. This bit is set by software and cleared only by a global RAMCFG reset Note: When PENMI bit is set, the RAMCFG maskable interrupt is not generated for a parity error whatever PEIE bit value.
    #[inline(always)]
    pub fn penmi(&self) -> PENMI_R {
        PENMI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2IER")
            .field("peie", &self.peie())
            .field("penmi", &self.penmi())
            .finish()
    }
}
impl W {
    ///Bit 1 - Parity error interrupt enable
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W<'_, M2IERrs> {
        PEIE_W::new(self, 1)
    }
    ///Bit 3 - Parity error NMI. This bit is set by software and cleared only by a global RAMCFG reset Note: When PENMI bit is set, the RAMCFG maskable interrupt is not generated for a parity error whatever PEIE bit value.
    #[inline(always)]
    pub fn penmi(&mut self) -> PENMI_W<'_, M2IERrs> {
        PENMI_W::new(self, 3)
    }
}
/**RAMCFG SRAM2 interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`m2ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#RAMCFG:M2IER)*/
pub struct M2IERrs;
impl crate::RegisterSpec for M2IERrs {
    type Ux = u32;
}
///`read()` method returns [`m2ier::R`](R) reader structure
impl crate::Readable for M2IERrs {}
///`write(|w| ..)` method takes [`m2ier::W`](W) writer structure
impl crate::Writable for M2IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M2IER to value 0
impl crate::Resettable for M2IERrs {}
