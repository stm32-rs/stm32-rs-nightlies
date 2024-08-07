///Register `TZC_GATE_KEEPER` reader
pub type R = crate::R<TZC_GATE_KEEPERrs>;
///Register `TZC_GATE_KEEPER` writer
pub type W = crate::W<TZC_GATE_KEEPERrs>;
///Field `OPENREQ` reader - OPENREQ
pub type OPENREQ_R = crate::FieldReader;
///Field `OPENREQ` writer - OPENREQ
pub type OPENREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OPENSTAT` reader - OPENSTAT
pub type OPENSTAT_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - OPENREQ
    #[inline(always)]
    pub fn openreq(&self) -> OPENREQ_R {
        OPENREQ_R::new((self.bits & 3) as u8)
    }
    ///Bits 16:17 - OPENSTAT
    #[inline(always)]
    pub fn openstat(&self) -> OPENSTAT_R {
        OPENSTAT_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZC_GATE_KEEPER")
            .field("openreq", &self.openreq())
            .field("openstat", &self.openstat())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - OPENREQ
    #[inline(always)]
    #[must_use]
    pub fn openreq(&mut self) -> OPENREQ_W<TZC_GATE_KEEPERrs> {
        OPENREQ_W::new(self, 0)
    }
}
/**Provides control and status for the gate keeper in each filter unit implemented.

You can [`read`](crate::Reg::read) this register and get [`tzc_gate_keeper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_gate_keeper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TZC:TZC_GATE_KEEPER)*/
pub struct TZC_GATE_KEEPERrs;
impl crate::RegisterSpec for TZC_GATE_KEEPERrs {
    type Ux = u32;
}
///`read()` method returns [`tzc_gate_keeper::R`](R) reader structure
impl crate::Readable for TZC_GATE_KEEPERrs {}
///`write(|w| ..)` method takes [`tzc_gate_keeper::W`](W) writer structure
impl crate::Writable for TZC_GATE_KEEPERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TZC_GATE_KEEPER to value 0
impl crate::Resettable for TZC_GATE_KEEPERrs {
    const RESET_VALUE: u32 = 0;
}
