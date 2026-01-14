///Register `HUFFBASE24` reader
pub type R = crate::R<HUFFBASE24rs>;
///Register `HUFFBASE24` writer
pub type W = crate::W<HUFFBASE24rs>;
///Field `DATA48` reader - Data 48
pub type DATA48_R = crate::FieldReader<u16>;
///Field `DATA48` writer - Data 48
pub type DATA48_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA49` reader - Data 49
pub type DATA49_R = crate::FieldReader<u16>;
///Field `DATA49` writer - Data 49
pub type DATA49_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 48
    #[inline(always)]
    pub fn data48(&self) -> DATA48_R {
        DATA48_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 49
    #[inline(always)]
    pub fn data49(&self) -> DATA49_R {
        DATA49_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE24")
            .field("data48", &self.data48())
            .field("data49", &self.data49())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 48
    #[inline(always)]
    pub fn data48(&mut self) -> DATA48_W<'_, HUFFBASE24rs> {
        DATA48_W::new(self, 0)
    }
    ///Bits 16:24 - Data 49
    #[inline(always)]
    pub fn data49(&mut self) -> DATA49_W<'_, HUFFBASE24rs> {
        DATA49_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFBASE24)*/
pub struct HUFFBASE24rs;
impl crate::RegisterSpec for HUFFBASE24rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase24::R`](R) reader structure
impl crate::Readable for HUFFBASE24rs {}
///`write(|w| ..)` method takes [`huffbase24::W`](W) writer structure
impl crate::Writable for HUFFBASE24rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE24 to value 0
impl crate::Resettable for HUFFBASE24rs {}
