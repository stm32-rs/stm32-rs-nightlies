///Register `PCSEL` reader
pub type R = crate::R<PCSELrs>;
///Register `PCSEL` writer
pub type W = crate::W<PCSELrs>;
///Field `PCSEL` reader - Channel i (V less than sub>INP less than /sub>\[i\]) preselection
pub type PCSEL_R = crate::FieldReader<u32>;
///Field `PCSEL` writer - Channel i (V less than sub>INP less than /sub>\[i\]) preselection
pub type PCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - Channel i (V less than sub>INP less than /sub>\[i\]) preselection
    #[inline(always)]
    pub fn pcsel(&self) -> PCSEL_R {
        PCSEL_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCSEL")
            .field("pcsel", &self.pcsel())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - Channel i (V less than sub>INP less than /sub>\[i\]) preselection
    #[inline(always)]
    pub fn pcsel(&mut self) -> PCSEL_W<'_, PCSELrs> {
        PCSEL_W::new(self, 0)
    }
}
/**ADC channel preselection register

You can [`read`](crate::Reg::read) this register and get [`pcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ADC1:PCSEL)*/
pub struct PCSELrs;
impl crate::RegisterSpec for PCSELrs {
    type Ux = u32;
}
///`read()` method returns [`pcsel::R`](R) reader structure
impl crate::Readable for PCSELrs {}
///`write(|w| ..)` method takes [`pcsel::W`](W) writer structure
impl crate::Writable for PCSELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCSEL to value 0
impl crate::Resettable for PCSELrs {}
