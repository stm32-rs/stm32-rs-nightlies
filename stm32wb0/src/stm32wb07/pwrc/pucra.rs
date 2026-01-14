///Register `PUCRA` reader
pub type R = crate::R<PUCRArs>;
///Register `PUCRA` writer
pub type W = crate::W<PUCRArs>;
///Field `PUA` reader - PUA\[x\] : Pull Up Pull up activation on port A\[i\] pad when APC bit of PWRC CR3 is set
pub type PUA_R = crate::FieldReader<u16>;
///Field `PUA` writer - PUA\[x\] : Pull Up Pull up activation on port A\[i\] pad when APC bit of PWRC CR3 is set
pub type PUA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - PUA\[x\] : Pull Up Pull up activation on port A\[i\] pad when APC bit of PWRC CR3 is set
    #[inline(always)]
    pub fn pua(&self) -> PUA_R {
        PUA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRA").field("pua", &self.pua()).finish()
    }
}
impl W {
    ///Bits 0:15 - PUA\[x\] : Pull Up Pull up activation on port A\[i\] pad when APC bit of PWRC CR3 is set
    #[inline(always)]
    pub fn pua(&mut self) -> PUA_W<'_, PUCRArs> {
        PUA_W::new(self, 0)
    }
}
/**PUCRA register

You can [`read`](crate::Reg::read) this register and get [`pucra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#PWRC:PUCRA)*/
pub struct PUCRArs;
impl crate::RegisterSpec for PUCRArs {
    type Ux = u32;
}
///`read()` method returns [`pucra::R`](R) reader structure
impl crate::Readable for PUCRArs {}
///`write(|w| ..)` method takes [`pucra::W`](W) writer structure
impl crate::Writable for PUCRArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRA to value 0x0f07
impl crate::Resettable for PUCRArs {
    const RESET_VALUE: u32 = 0x0f07;
}
