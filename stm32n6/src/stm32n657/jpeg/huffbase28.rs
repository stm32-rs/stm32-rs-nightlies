///Register `HUFFBASE28` reader
pub type R = crate::R<HUFFBASE28rs>;
///Register `HUFFBASE28` writer
pub type W = crate::W<HUFFBASE28rs>;
///Field `DATA56` reader - Data 56
pub type DATA56_R = crate::FieldReader<u16>;
///Field `DATA56` writer - Data 56
pub type DATA56_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA57` reader - Data 57
pub type DATA57_R = crate::FieldReader<u16>;
///Field `DATA57` writer - Data 57
pub type DATA57_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 56
    #[inline(always)]
    pub fn data56(&self) -> DATA56_R {
        DATA56_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 57
    #[inline(always)]
    pub fn data57(&self) -> DATA57_R {
        DATA57_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE28")
            .field("data56", &self.data56())
            .field("data57", &self.data57())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 56
    #[inline(always)]
    pub fn data56(&mut self) -> DATA56_W<'_, HUFFBASE28rs> {
        DATA56_W::new(self, 0)
    }
    ///Bits 16:24 - Data 57
    #[inline(always)]
    pub fn data57(&mut self) -> DATA57_W<'_, HUFFBASE28rs> {
        DATA57_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFBASE28)*/
pub struct HUFFBASE28rs;
impl crate::RegisterSpec for HUFFBASE28rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase28::R`](R) reader structure
impl crate::Readable for HUFFBASE28rs {}
///`write(|w| ..)` method takes [`huffbase28::W`](W) writer structure
impl crate::Writable for HUFFBASE28rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE28 to value 0
impl crate::Resettable for HUFFBASE28rs {}
