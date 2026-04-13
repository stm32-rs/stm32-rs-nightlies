///Register `ASMR2` reader
pub type R = crate::R<ASMR2rs>;
///Register `ASMR2` writer
pub type W = crate::W<ASMR2rs>;
///Field `PB` reader - Port B analog switch mode selection
pub type PB_R = crate::FieldReader<u16>;
///Field `PB` writer - Port B analog switch mode selection
pub type PB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Port B analog switch mode selection
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASMR2").field("pb", &self.pb()).finish()
    }
}
impl W {
    ///Bits 0:15 - Port B analog switch mode selection
    #[inline(always)]
    pub fn pb(&mut self) -> PB_W<'_, ASMR2rs> {
        PB_W::new(self, 0)
    }
}
/**Analog switch mode register

You can [`read`](crate::Reg::read) this register and get [`asmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L162.html#RI:ASMR2)*/
pub struct ASMR2rs;
impl crate::RegisterSpec for ASMR2rs {
    type Ux = u32;
}
///`read()` method returns [`asmr2::R`](R) reader structure
impl crate::Readable for ASMR2rs {}
///`write(|w| ..)` method takes [`asmr2::W`](W) writer structure
impl crate::Writable for ASMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ASMR2 to value 0
impl crate::Resettable for ASMR2rs {}
