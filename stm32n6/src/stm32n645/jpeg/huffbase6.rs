///Register `HUFFBASE6` reader
pub type R = crate::R<HUFFBASE6rs>;
///Register `HUFFBASE6` writer
pub type W = crate::W<HUFFBASE6rs>;
///Field `DATA12` reader - Data 12
pub type DATA12_R = crate::FieldReader<u16>;
///Field `DATA12` writer - Data 12
pub type DATA12_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA13` reader - Data 13
pub type DATA13_R = crate::FieldReader<u16>;
///Field `DATA13` writer - Data 13
pub type DATA13_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 12
    #[inline(always)]
    pub fn data12(&self) -> DATA12_R {
        DATA12_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 13
    #[inline(always)]
    pub fn data13(&self) -> DATA13_R {
        DATA13_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE6")
            .field("data12", &self.data12())
            .field("data13", &self.data13())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 12
    #[inline(always)]
    pub fn data12(&mut self) -> DATA12_W<'_, HUFFBASE6rs> {
        DATA12_W::new(self, 0)
    }
    ///Bits 16:24 - Data 13
    #[inline(always)]
    pub fn data13(&mut self) -> DATA13_W<'_, HUFFBASE6rs> {
        DATA13_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFBASE6)*/
pub struct HUFFBASE6rs;
impl crate::RegisterSpec for HUFFBASE6rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase6::R`](R) reader structure
impl crate::Readable for HUFFBASE6rs {}
///`write(|w| ..)` method takes [`huffbase6::W`](W) writer structure
impl crate::Writable for HUFFBASE6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE6 to value 0
impl crate::Resettable for HUFFBASE6rs {}
