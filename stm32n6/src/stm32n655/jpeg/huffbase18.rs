///Register `HUFFBASE18` reader
pub type R = crate::R<HUFFBASE18rs>;
///Register `HUFFBASE18` writer
pub type W = crate::W<HUFFBASE18rs>;
///Field `DATA36` reader - Data 36
pub type DATA36_R = crate::FieldReader<u16>;
///Field `DATA36` writer - Data 36
pub type DATA36_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA37` reader - Data 37
pub type DATA37_R = crate::FieldReader<u16>;
///Field `DATA37` writer - Data 37
pub type DATA37_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 36
    #[inline(always)]
    pub fn data36(&self) -> DATA36_R {
        DATA36_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 37
    #[inline(always)]
    pub fn data37(&self) -> DATA37_R {
        DATA37_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE18")
            .field("data36", &self.data36())
            .field("data37", &self.data37())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 36
    #[inline(always)]
    pub fn data36(&mut self) -> DATA36_W<'_, HUFFBASE18rs> {
        DATA36_W::new(self, 0)
    }
    ///Bits 16:24 - Data 37
    #[inline(always)]
    pub fn data37(&mut self) -> DATA37_W<'_, HUFFBASE18rs> {
        DATA37_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE18)*/
pub struct HUFFBASE18rs;
impl crate::RegisterSpec for HUFFBASE18rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase18::R`](R) reader structure
impl crate::Readable for HUFFBASE18rs {}
///`write(|w| ..)` method takes [`huffbase18::W`](W) writer structure
impl crate::Writable for HUFFBASE18rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE18 to value 0
impl crate::Resettable for HUFFBASE18rs {}
