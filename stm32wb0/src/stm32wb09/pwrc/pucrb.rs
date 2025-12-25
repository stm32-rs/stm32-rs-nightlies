///Register `PUCRB` reader
pub type R = crate::R<PUCRBrs>;
///Register `PUCRB` writer
pub type W = crate::W<PUCRBrs>;
///Field `PU` reader - PU\[x\] : Pull Up Pull up activation on port B\[i\] pad when APC bit of PWRC CR3 is set
pub type PU_R = crate::FieldReader<u16>;
///Field `PU` writer - PU\[x\] : Pull Up Pull up activation on port B\[i\] pad when APC bit of PWRC CR3 is set
pub type PU_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - PU\[x\] : Pull Up Pull up activation on port B\[i\] pad when APC bit of PWRC CR3 is set
    #[inline(always)]
    pub fn pu(&self) -> PU_R {
        PU_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRB").field("pu", &self.pu()).finish()
    }
}
impl W {
    ///Bits 0:15 - PU\[x\] : Pull Up Pull up activation on port B\[i\] pad when APC bit of PWRC CR3 is set
    #[inline(always)]
    pub fn pu(&mut self) -> PU_W<'_, PUCRBrs> {
        PU_W::new(self, 0)
    }
}
/**PUCRB register

You can [`read`](crate::Reg::read) this register and get [`pucrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#PWRC:PUCRB)*/
pub struct PUCRBrs;
impl crate::RegisterSpec for PUCRBrs {
    type Ux = u32;
}
///`read()` method returns [`pucrb::R`](R) reader structure
impl crate::Readable for PUCRBrs {}
///`write(|w| ..)` method takes [`pucrb::W`](W) writer structure
impl crate::Writable for PUCRBrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRB to value 0xf0ff
impl crate::Resettable for PUCRBrs {
    const RESET_VALUE: u32 = 0xf0ff;
}
