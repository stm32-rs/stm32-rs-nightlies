///Register `HUFFBASE22` reader
pub type R = crate::R<HUFFBASE22rs>;
///Register `HUFFBASE22` writer
pub type W = crate::W<HUFFBASE22rs>;
///Field `DATA44` reader - Data 44
pub type DATA44_R = crate::FieldReader<u16>;
///Field `DATA44` writer - Data 44
pub type DATA44_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA45` reader - Data 45
pub type DATA45_R = crate::FieldReader<u16>;
///Field `DATA45` writer - Data 45
pub type DATA45_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 44
    #[inline(always)]
    pub fn data44(&self) -> DATA44_R {
        DATA44_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 45
    #[inline(always)]
    pub fn data45(&self) -> DATA45_R {
        DATA45_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE22")
            .field("data44", &self.data44())
            .field("data45", &self.data45())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 44
    #[inline(always)]
    pub fn data44(&mut self) -> DATA44_W<'_, HUFFBASE22rs> {
        DATA44_W::new(self, 0)
    }
    ///Bits 16:24 - Data 45
    #[inline(always)]
    pub fn data45(&mut self) -> DATA45_W<'_, HUFFBASE22rs> {
        DATA45_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFBASE22)*/
pub struct HUFFBASE22rs;
impl crate::RegisterSpec for HUFFBASE22rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase22::R`](R) reader structure
impl crate::Readable for HUFFBASE22rs {}
///`write(|w| ..)` method takes [`huffbase22::W`](W) writer structure
impl crate::Writable for HUFFBASE22rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE22 to value 0
impl crate::Resettable for HUFFBASE22rs {}
