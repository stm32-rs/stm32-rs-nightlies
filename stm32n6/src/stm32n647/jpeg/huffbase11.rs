///Register `HUFFBASE11` reader
pub type R = crate::R<HUFFBASE11rs>;
///Register `HUFFBASE11` writer
pub type W = crate::W<HUFFBASE11rs>;
///Field `DATA22` reader - Data 22
pub type DATA22_R = crate::FieldReader<u16>;
///Field `DATA22` writer - Data 22
pub type DATA22_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA23` reader - Data 23
pub type DATA23_R = crate::FieldReader<u16>;
///Field `DATA23` writer - Data 23
pub type DATA23_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 22
    #[inline(always)]
    pub fn data22(&self) -> DATA22_R {
        DATA22_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 23
    #[inline(always)]
    pub fn data23(&self) -> DATA23_R {
        DATA23_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE11")
            .field("data22", &self.data22())
            .field("data23", &self.data23())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 22
    #[inline(always)]
    pub fn data22(&mut self) -> DATA22_W<'_, HUFFBASE11rs> {
        DATA22_W::new(self, 0)
    }
    ///Bits 16:24 - Data 23
    #[inline(always)]
    pub fn data23(&mut self) -> DATA23_W<'_, HUFFBASE11rs> {
        DATA23_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFBASE11)*/
pub struct HUFFBASE11rs;
impl crate::RegisterSpec for HUFFBASE11rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase11::R`](R) reader structure
impl crate::Readable for HUFFBASE11rs {}
///`write(|w| ..)` method takes [`huffbase11::W`](W) writer structure
impl crate::Writable for HUFFBASE11rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE11 to value 0
impl crate::Resettable for HUFFBASE11rs {}
