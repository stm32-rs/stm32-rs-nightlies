///Register `HUFFBASE7` reader
pub type R = crate::R<HUFFBASE7rs>;
///Register `HUFFBASE7` writer
pub type W = crate::W<HUFFBASE7rs>;
///Field `DATA14` reader - Data 14
pub type DATA14_R = crate::FieldReader<u16>;
///Field `DATA14` writer - Data 14
pub type DATA14_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA15` reader - Data 15
pub type DATA15_R = crate::FieldReader<u16>;
///Field `DATA15` writer - Data 15
pub type DATA15_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 14
    #[inline(always)]
    pub fn data14(&self) -> DATA14_R {
        DATA14_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 15
    #[inline(always)]
    pub fn data15(&self) -> DATA15_R {
        DATA15_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE7")
            .field("data14", &self.data14())
            .field("data15", &self.data15())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 14
    #[inline(always)]
    pub fn data14(&mut self) -> DATA14_W<'_, HUFFBASE7rs> {
        DATA14_W::new(self, 0)
    }
    ///Bits 16:24 - Data 15
    #[inline(always)]
    pub fn data15(&mut self) -> DATA15_W<'_, HUFFBASE7rs> {
        DATA15_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFBASE7)*/
pub struct HUFFBASE7rs;
impl crate::RegisterSpec for HUFFBASE7rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase7::R`](R) reader structure
impl crate::Readable for HUFFBASE7rs {}
///`write(|w| ..)` method takes [`huffbase7::W`](W) writer structure
impl crate::Writable for HUFFBASE7rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE7 to value 0
impl crate::Resettable for HUFFBASE7rs {}
