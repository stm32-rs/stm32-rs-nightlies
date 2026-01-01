///Register `PDCRA` reader
pub type R = crate::R<PDCRArs>;
///Register `PDCRA` writer
pub type W = crate::W<PDCRArs>;
///Field `PD` reader - PD\[x\]: Pull Down Pull Down activation on port A\[i\] pad when APC bit of PWRC CR3 is set
pub type PD_R = crate::FieldReader<u16>;
///Field `PD` writer - PD\[x\]: Pull Down Pull Down activation on port A\[i\] pad when APC bit of PWRC CR3 is set
pub type PD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - PD\[x\]: Pull Down Pull Down activation on port A\[i\] pad when APC bit of PWRC CR3 is set
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRA").field("pd", &self.pd()).finish()
    }
}
impl W {
    ///Bits 0:15 - PD\[x\]: Pull Down Pull Down activation on port A\[i\] pad when APC bit of PWRC CR3 is set
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W<'_, PDCRArs> {
        PD_W::new(self, 0)
    }
}
/**PDCRA register

You can [`read`](crate::Reg::read) this register and get [`pdcra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#PWRC:PDCRA)*/
pub struct PDCRArs;
impl crate::RegisterSpec for PDCRArs {
    type Ux = u32;
}
///`read()` method returns [`pdcra::R`](R) reader structure
impl crate::Readable for PDCRArs {}
///`write(|w| ..)` method takes [`pdcra::W`](W) writer structure
impl crate::Writable for PDCRArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRA to value 0x08
impl crate::Resettable for PDCRArs {
    const RESET_VALUE: u32 = 0x08;
}
