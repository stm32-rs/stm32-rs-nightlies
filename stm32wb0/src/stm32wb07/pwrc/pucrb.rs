///Register `PUCRB` reader
pub type R = crate::R<PUCRBrs>;
///Register `PUCRB` writer
pub type W = crate::W<PUCRBrs>;
///Field `PUB` reader - PUB\[x\] : Pull Up Pull up activation on port B\[i\] pad when APC bit of PWRC CR3 is set
pub type PUB_R = crate::FieldReader<u16>;
///Field `PUB` writer - PUB\[x\] : Pull Up Pull up activation on port B\[i\] pad when APC bit of PWRC CR3 is set
pub type PUB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - PUB\[x\] : Pull Up Pull up activation on port B\[i\] pad when APC bit of PWRC CR3 is set
    #[inline(always)]
    pub fn pub_(&self) -> PUB_R {
        PUB_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRB").field("pub_", &self.pub_()).finish()
    }
}
impl W {
    ///Bits 0:15 - PUB\[x\] : Pull Up Pull up activation on port B\[i\] pad when APC bit of PWRC CR3 is set
    #[inline(always)]
    pub fn pub_(&mut self) -> PUB_W<'_, PUCRBrs> {
        PUB_W::new(self, 0)
    }
}
/**PUCRB register

You can [`read`](crate::Reg::read) this register and get [`pucrb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#PWRC:PUCRB)*/
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
