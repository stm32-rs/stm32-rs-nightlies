///Register `IDR` reader
pub type R = crate::R<IDRrs>;
///Register `IDR` writer
pub type W = crate::W<IDRrs>;
///Field `IDR` reader - Independent data register
pub type IDR_R = crate::FieldReader;
///Field `IDR` writer - Independent data register
pub type IDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - Independent data register
    #[inline(always)]
    pub fn idr(&self) -> IDR_R {
        IDR_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR").field("idr", &self.idr()).finish()
    }
}
impl W {
    ///Bits 0:7 - Independent data register
    #[inline(always)]
    pub fn idr(&mut self) -> IDR_W<'_, IDRrs> {
        IDR_W::new(self, 0)
    }
}
/**Independent data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#CRC:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`write(|w| ..)` method takes [`idr::W`](W) writer structure
impl crate::Writable for IDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {}
