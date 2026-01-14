///Register `HUFFBASE13` reader
pub type R = crate::R<HUFFBASE13rs>;
///Register `HUFFBASE13` writer
pub type W = crate::W<HUFFBASE13rs>;
///Field `DATA26` reader - Data 26
pub type DATA26_R = crate::FieldReader<u16>;
///Field `DATA26` writer - Data 26
pub type DATA26_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA27` reader - Data 27
pub type DATA27_R = crate::FieldReader<u16>;
///Field `DATA27` writer - Data 27
pub type DATA27_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 26
    #[inline(always)]
    pub fn data26(&self) -> DATA26_R {
        DATA26_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 27
    #[inline(always)]
    pub fn data27(&self) -> DATA27_R {
        DATA27_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE13")
            .field("data26", &self.data26())
            .field("data27", &self.data27())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 26
    #[inline(always)]
    pub fn data26(&mut self) -> DATA26_W<'_, HUFFBASE13rs> {
        DATA26_W::new(self, 0)
    }
    ///Bits 16:24 - Data 27
    #[inline(always)]
    pub fn data27(&mut self) -> DATA27_W<'_, HUFFBASE13rs> {
        DATA27_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE13)*/
pub struct HUFFBASE13rs;
impl crate::RegisterSpec for HUFFBASE13rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase13::R`](R) reader structure
impl crate::Readable for HUFFBASE13rs {}
///`write(|w| ..)` method takes [`huffbase13::W`](W) writer structure
impl crate::Writable for HUFFBASE13rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE13 to value 0
impl crate::Resettable for HUFFBASE13rs {}
