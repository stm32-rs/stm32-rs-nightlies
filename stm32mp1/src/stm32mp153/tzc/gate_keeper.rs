///Register `GATE_KEEPER` reader
pub type R = crate::R<GATE_KEEPERrs>;
///Register `GATE_KEEPER` writer
pub type W = crate::W<GATE_KEEPERrs>;
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
        f.debug_struct("GATE_KEEPER")
            .field("openreq", &self.openreq())
            .field("openstat", &self.openstat())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - OPENREQ
    #[inline(always)]
    pub fn openreq(&mut self) -> OPENREQ_W<'_, GATE_KEEPERrs> {
        OPENREQ_W::new(self, 0)
    }
}
/**Provides control and status for the gate keeper in each filter unit implemented.

You can [`read`](crate::Reg::read) this register and get [`gate_keeper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gate_keeper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TZC:GATE_KEEPER)*/
pub struct GATE_KEEPERrs;
impl crate::RegisterSpec for GATE_KEEPERrs {
    type Ux = u32;
}
///`read()` method returns [`gate_keeper::R`](R) reader structure
impl crate::Readable for GATE_KEEPERrs {}
///`write(|w| ..)` method takes [`gate_keeper::W`](W) writer structure
impl crate::Writable for GATE_KEEPERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GATE_KEEPER to value 0
impl crate::Resettable for GATE_KEEPERrs {}
