///Register `TTTS` reader
pub type R = crate::R<TTTSrs>;
///Register `TTTS` writer
pub type W = crate::W<TTTSrs>;
///Field `SWTDEL` reader - SWTDEL
pub type SWTDEL_R = crate::FieldReader;
///Field `SWTDEL` writer - SWTDEL
pub type SWTDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EVTSEL` reader - EVTSEL
pub type EVTSEL_R = crate::FieldReader;
///Field `EVTSEL` writer - EVTSEL
pub type EVTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - SWTDEL
    #[inline(always)]
    pub fn swtdel(&self) -> SWTDEL_R {
        SWTDEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - EVTSEL
    #[inline(always)]
    pub fn evtsel(&self) -> EVTSEL_R {
        EVTSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTTS")
            .field("swtdel", &self.swtdel())
            .field("evtsel", &self.evtsel())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - SWTDEL
    #[inline(always)]
    pub fn swtdel(&mut self) -> SWTDEL_W<TTTSrs> {
        SWTDEL_W::new(self, 0)
    }
    ///Bits 4:5 - EVTSEL
    #[inline(always)]
    pub fn evtsel(&mut self) -> EVTSEL_W<TTTSrs> {
        EVTSEL_W::new(self, 4)
    }
}
/**The settings in the FDCAN_TTTS register select the input to be used as event trigger and stop watch trigger.

You can [`read`](crate::Reg::read) this register and get [`ttts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:TTTS)*/
pub struct TTTSrs;
impl crate::RegisterSpec for TTTSrs {
    type Ux = u32;
}
///`read()` method returns [`ttts::R`](R) reader structure
impl crate::Readable for TTTSrs {}
///`write(|w| ..)` method takes [`ttts::W`](W) writer structure
impl crate::Writable for TTTSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TTTS to value 0
impl crate::Resettable for TTTSrs {}
