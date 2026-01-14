///Register `P1PPM0PR` reader
pub type R = crate::R<P1PPM0PRrs>;
///Register `P1PPM0PR` writer
pub type W = crate::W<P1PPM0PRrs>;
///Field `PITCH` reader - Number of bytes between the address of two consecutive lines.
pub type PITCH_R = crate::FieldReader<u16>;
///Field `PITCH` writer - Number of bytes between the address of two consecutive lines.
pub type PITCH_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 0:14 - Number of bytes between the address of two consecutive lines.
    #[inline(always)]
    pub fn pitch(&self) -> PITCH_R {
        PITCH_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1PPM0PR")
            .field("pitch", &self.pitch())
            .finish()
    }
}
impl W {
    ///Bits 0:14 - Number of bytes between the address of two consecutive lines.
    #[inline(always)]
    pub fn pitch(&mut self) -> PITCH_W<'_, P1PPM0PRrs> {
        PITCH_W::new(self, 0)
    }
}
/**DCMIPP Pipex pixel packer Memory0 pitch register

You can [`read`](crate::Reg::read) this register and get [`p1ppm0pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ppm0pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DCMIPP:P1PPM0PR)*/
pub struct P1PPM0PRrs;
impl crate::RegisterSpec for P1PPM0PRrs {
    type Ux = u32;
}
///`read()` method returns [`p1ppm0pr::R`](R) reader structure
impl crate::Readable for P1PPM0PRrs {}
///`write(|w| ..)` method takes [`p1ppm0pr::W`](W) writer structure
impl crate::Writable for P1PPM0PRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1PPM0PR to value 0
impl crate::Resettable for P1PPM0PRrs {}
