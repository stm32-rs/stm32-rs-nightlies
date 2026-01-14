///Register `HUFFSYMB24` reader
pub type R = crate::R<HUFFSYMB24rs>;
///Register `HUFFSYMB24` writer
pub type W = crate::W<HUFFSYMB24rs>;
///Field `DATA96` reader - Data 96
pub type DATA96_R = crate::FieldReader;
///Field `DATA96` writer - Data 96
pub type DATA96_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA97` reader - Data 97
pub type DATA97_R = crate::FieldReader;
///Field `DATA97` writer - Data 97
pub type DATA97_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA98` reader - Data 98
pub type DATA98_R = crate::FieldReader;
///Field `DATA98` writer - Data 98
pub type DATA98_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA99` reader - Data 99
pub type DATA99_R = crate::FieldReader;
///Field `DATA99` writer - Data 99
pub type DATA99_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 96
    #[inline(always)]
    pub fn data96(&self) -> DATA96_R {
        DATA96_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 97
    #[inline(always)]
    pub fn data97(&self) -> DATA97_R {
        DATA97_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 98
    #[inline(always)]
    pub fn data98(&self) -> DATA98_R {
        DATA98_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 99
    #[inline(always)]
    pub fn data99(&self) -> DATA99_R {
        DATA99_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB24")
            .field("data96", &self.data96())
            .field("data97", &self.data97())
            .field("data98", &self.data98())
            .field("data99", &self.data99())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 96
    #[inline(always)]
    pub fn data96(&mut self) -> DATA96_W<'_, HUFFSYMB24rs> {
        DATA96_W::new(self, 0)
    }
    ///Bits 8:15 - Data 97
    #[inline(always)]
    pub fn data97(&mut self) -> DATA97_W<'_, HUFFSYMB24rs> {
        DATA97_W::new(self, 8)
    }
    ///Bits 16:23 - Data 98
    #[inline(always)]
    pub fn data98(&mut self) -> DATA98_W<'_, HUFFSYMB24rs> {
        DATA98_W::new(self, 16)
    }
    ///Bits 24:31 - Data 99
    #[inline(always)]
    pub fn data99(&mut self) -> DATA99_W<'_, HUFFSYMB24rs> {
        DATA99_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#JPEG:HUFFSYMB24)*/
pub struct HUFFSYMB24rs;
impl crate::RegisterSpec for HUFFSYMB24rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb24::R`](R) reader structure
impl crate::Readable for HUFFSYMB24rs {}
///`write(|w| ..)` method takes [`huffsymb24::W`](W) writer structure
impl crate::Writable for HUFFSYMB24rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB24 to value 0
impl crate::Resettable for HUFFSYMB24rs {}
