///Register `DIFSEL` reader
pub type R = crate::R<DIFSELrs>;
///Register `DIFSEL` writer
pub type W = crate::W<DIFSELrs>;
///Field `DIFSEL` reader - DIFSEL
pub type DIFSEL_R = crate::FieldReader<u32>;
///Field `DIFSEL` writer - DIFSEL
pub type DIFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - DIFSEL
    #[inline(always)]
    pub fn difsel(&self) -> DIFSEL_R {
        DIFSEL_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIFSEL")
            .field("difsel", &self.difsel())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - DIFSEL
    #[inline(always)]
    pub fn difsel(&mut self) -> DIFSEL_W<'_, DIFSELrs> {
        DIFSEL_W::new(self, 0)
    }
}
/**ADC differential mode selection register

You can [`read`](crate::Reg::read) this register and get [`difsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ADC2:DIFSEL)*/
pub struct DIFSELrs;
impl crate::RegisterSpec for DIFSELrs {
    type Ux = u32;
}
///`read()` method returns [`difsel::R`](R) reader structure
impl crate::Readable for DIFSELrs {}
///`write(|w| ..)` method takes [`difsel::W`](W) writer structure
impl crate::Writable for DIFSELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIFSEL to value 0
impl crate::Resettable for DIFSELrs {}
