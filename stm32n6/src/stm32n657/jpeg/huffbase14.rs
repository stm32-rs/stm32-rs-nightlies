///Register `HUFFBASE14` reader
pub type R = crate::R<HUFFBASE14rs>;
///Register `HUFFBASE14` writer
pub type W = crate::W<HUFFBASE14rs>;
///Field `DATA28` reader - Data 28
pub type DATA28_R = crate::FieldReader<u16>;
///Field `DATA28` writer - Data 28
pub type DATA28_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA29` reader - Data 29
pub type DATA29_R = crate::FieldReader<u16>;
///Field `DATA29` writer - Data 29
pub type DATA29_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 28
    #[inline(always)]
    pub fn data28(&self) -> DATA28_R {
        DATA28_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 29
    #[inline(always)]
    pub fn data29(&self) -> DATA29_R {
        DATA29_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE14")
            .field("data28", &self.data28())
            .field("data29", &self.data29())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 28
    #[inline(always)]
    pub fn data28(&mut self) -> DATA28_W<'_, HUFFBASE14rs> {
        DATA28_W::new(self, 0)
    }
    ///Bits 16:24 - Data 29
    #[inline(always)]
    pub fn data29(&mut self) -> DATA29_W<'_, HUFFBASE14rs> {
        DATA29_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFBASE14)*/
pub struct HUFFBASE14rs;
impl crate::RegisterSpec for HUFFBASE14rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase14::R`](R) reader structure
impl crate::Readable for HUFFBASE14rs {}
///`write(|w| ..)` method takes [`huffbase14::W`](W) writer structure
impl crate::Writable for HUFFBASE14rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE14 to value 0
impl crate::Resettable for HUFFBASE14rs {}
