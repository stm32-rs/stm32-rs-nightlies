///Register `HUFFBASE12` reader
pub type R = crate::R<HUFFBASE12rs>;
///Register `HUFFBASE12` writer
pub type W = crate::W<HUFFBASE12rs>;
///Field `DATA24` reader - Data 24
pub type DATA24_R = crate::FieldReader<u16>;
///Field `DATA24` writer - Data 24
pub type DATA24_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA25` reader - Data 25
pub type DATA25_R = crate::FieldReader<u16>;
///Field `DATA25` writer - Data 25
pub type DATA25_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 24
    #[inline(always)]
    pub fn data24(&self) -> DATA24_R {
        DATA24_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 25
    #[inline(always)]
    pub fn data25(&self) -> DATA25_R {
        DATA25_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE12")
            .field("data24", &self.data24())
            .field("data25", &self.data25())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 24
    #[inline(always)]
    pub fn data24(&mut self) -> DATA24_W<'_, HUFFBASE12rs> {
        DATA24_W::new(self, 0)
    }
    ///Bits 16:24 - Data 25
    #[inline(always)]
    pub fn data25(&mut self) -> DATA25_W<'_, HUFFBASE12rs> {
        DATA25_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFBASE12)*/
pub struct HUFFBASE12rs;
impl crate::RegisterSpec for HUFFBASE12rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase12::R`](R) reader structure
impl crate::Readable for HUFFBASE12rs {}
///`write(|w| ..)` method takes [`huffbase12::W`](W) writer structure
impl crate::Writable for HUFFBASE12rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE12 to value 0
impl crate::Resettable for HUFFBASE12rs {}
