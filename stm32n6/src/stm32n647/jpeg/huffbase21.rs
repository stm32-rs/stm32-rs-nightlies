///Register `HUFFBASE21` reader
pub type R = crate::R<HUFFBASE21rs>;
///Register `HUFFBASE21` writer
pub type W = crate::W<HUFFBASE21rs>;
///Field `DATA42` reader - Data 42
pub type DATA42_R = crate::FieldReader<u16>;
///Field `DATA42` writer - Data 42
pub type DATA42_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DATA43` reader - Data 43
pub type DATA43_R = crate::FieldReader<u16>;
///Field `DATA43` writer - Data 43
pub type DATA43_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Data 42
    #[inline(always)]
    pub fn data42(&self) -> DATA42_R {
        DATA42_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 16:24 - Data 43
    #[inline(always)]
    pub fn data43(&self) -> DATA43_R {
        DATA43_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFBASE21")
            .field("data42", &self.data42())
            .field("data43", &self.data43())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Data 42
    #[inline(always)]
    pub fn data42(&mut self) -> DATA42_W<'_, HUFFBASE21rs> {
        DATA42_W::new(self, 0)
    }
    ///Bits 16:24 - Data 43
    #[inline(always)]
    pub fn data43(&mut self) -> DATA43_W<'_, HUFFBASE21rs> {
        DATA43_W::new(self, 16)
    }
}
/**JPEG Huffman base

You can [`read`](crate::Reg::read) this register and get [`huffbase21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffbase21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#JPEG:HUFFBASE21)*/
pub struct HUFFBASE21rs;
impl crate::RegisterSpec for HUFFBASE21rs {
    type Ux = u32;
}
///`read()` method returns [`huffbase21::R`](R) reader structure
impl crate::Readable for HUFFBASE21rs {}
///`write(|w| ..)` method takes [`huffbase21::W`](W) writer structure
impl crate::Writable for HUFFBASE21rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFBASE21 to value 0
impl crate::Resettable for HUFFBASE21rs {}
