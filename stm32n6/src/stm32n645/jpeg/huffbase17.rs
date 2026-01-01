///Register `HUFFBASE17` reader
pub type R = crate::R<HUFFBASE17rs>;
///Register `HUFFBASE17` writer
pub type W = crate::W<HUFFBASE17rs>;
///Field `DATA34` reader - Data 34
pub type DATA34_R = crate::FieldReader<u16>;
///Field `DATA34` writer - Data 34
pub type DATA34_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA35` reader - Data 35
pub type DATA35_R = crate::FieldReader<u16>;
///Field `DATA35` writer - Data 35
pub type DATA35_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 34
    #[inline(always)]
    pub fn data34(&self) -> DATA34_R {
        DATA34_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 35
    #[inline(always)]
    pub fn data35(&self) -> DATA35_R {
        DATA35_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE17")
            .field("data34", &self.data34())
            .field("data35", &self.data35())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 34
    #[inline(always)]
    pub fn data34(&mut self) -> DATA34_W<'_, HUFFBASE17rs> {
        DATA34_W::new(self, 0)
    }
    ///Bits 16:24 - Data 35
    #[inline(always)]
    pub fn data35(&mut self) -> DATA35_W<'_, HUFFBASE17rs> {
        DATA35_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFBASE17)*/
pub struct HUFFBASE17rs;
impl crate::RegisterSpec for HUFFBASE17rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase17::R`](R) reader structure
impl crate::Readable for HUFFBASE17rs {}
///`write(|w| ..)` method takes [`huffbase17::W`](W) writer structure
impl crate::Writable for HUFFBASE17rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE17 to value 0
impl crate::Resettable for HUFFBASE17rs {}
