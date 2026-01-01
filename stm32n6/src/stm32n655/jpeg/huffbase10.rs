///Register `HUFFBASE10` reader
pub type R = crate::R<HUFFBASE10rs>;
///Register `HUFFBASE10` writer
pub type W = crate::W<HUFFBASE10rs>;
///Field `DATA20` reader - Data 20
pub type DATA20_R = crate::FieldReader<u16>;
///Field `DATA20` writer - Data 20
pub type DATA20_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA21` reader - Data 21
pub type DATA21_R = crate::FieldReader<u16>;
///Field `DATA21` writer - Data 21
pub type DATA21_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 20
    #[inline(always)]
    pub fn data20(&self) -> DATA20_R {
        DATA20_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 21
    #[inline(always)]
    pub fn data21(&self) -> DATA21_R {
        DATA21_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE10")
            .field("data20", &self.data20())
            .field("data21", &self.data21())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 20
    #[inline(always)]
    pub fn data20(&mut self) -> DATA20_W<'_, HUFFBASE10rs> {
        DATA20_W::new(self, 0)
    }
    ///Bits 16:24 - Data 21
    #[inline(always)]
    pub fn data21(&mut self) -> DATA21_W<'_, HUFFBASE10rs> {
        DATA21_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFBASE10)*/
pub struct HUFFBASE10rs;
impl crate::RegisterSpec for HUFFBASE10rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase10::R`](R) reader structure
impl crate::Readable for HUFFBASE10rs {}
///`write(|w| ..)` method takes [`huffbase10::W`](W) writer structure
impl crate::Writable for HUFFBASE10rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE10 to value 0
impl crate::Resettable for HUFFBASE10rs {}
