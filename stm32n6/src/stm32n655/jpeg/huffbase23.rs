///Register `HUFFBASE23` reader
pub type R = crate::R<HUFFBASE23rs>;
///Register `HUFFBASE23` writer
pub type W = crate::W<HUFFBASE23rs>;
///Field `DATA46` reader - Data 46
pub type DATA46_R = crate::FieldReader<u16>;
///Field `DATA46` writer - Data 46
pub type DATA46_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA47` reader - Data 47
pub type DATA47_R = crate::FieldReader<u16>;
///Field `DATA47` writer - Data 47
pub type DATA47_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 46
    #[inline(always)]
    pub fn data46(&self) -> DATA46_R {
        DATA46_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 47
    #[inline(always)]
    pub fn data47(&self) -> DATA47_R {
        DATA47_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE23")
            .field("data46", &self.data46())
            .field("data47", &self.data47())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 46
    #[inline(always)]
    pub fn data46(&mut self) -> DATA46_W<'_, HUFFBASE23rs> {
        DATA46_W::new(self, 0)
    }
    ///Bits 16:24 - Data 47
    #[inline(always)]
    pub fn data47(&mut self) -> DATA47_W<'_, HUFFBASE23rs> {
        DATA47_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE23)*/
pub struct HUFFBASE23rs;
impl crate::RegisterSpec for HUFFBASE23rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase23::R`](R) reader structure
impl crate::Readable for HUFFBASE23rs {}
///`write(|w| ..)` method takes [`huffbase23::W`](W) writer structure
impl crate::Writable for HUFFBASE23rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE23 to value 0
impl crate::Resettable for HUFFBASE23rs {}
