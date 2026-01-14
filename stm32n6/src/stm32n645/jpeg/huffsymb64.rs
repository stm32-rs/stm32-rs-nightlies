///Register `HUFFSYMB64` reader
pub type R = crate::R<HUFFSYMB64rs>;
///Register `HUFFSYMB64` writer
pub type W = crate::W<HUFFSYMB64rs>;
///Field `DATA256` reader - Data 256
pub type DATA256_R = crate::FieldReader;
///Field `DATA256` writer - Data 256
pub type DATA256_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA257` reader - Data 257
pub type DATA257_R = crate::FieldReader;
///Field `DATA257` writer - Data 257
pub type DATA257_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA258` reader - Data 258
pub type DATA258_R = crate::FieldReader;
///Field `DATA258` writer - Data 258
pub type DATA258_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA259` reader - Data 259
pub type DATA259_R = crate::FieldReader;
///Field `DATA259` writer - Data 259
pub type DATA259_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 256
    #[inline(always)]
    pub fn data256(&self) -> DATA256_R {
        DATA256_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 257
    #[inline(always)]
    pub fn data257(&self) -> DATA257_R {
        DATA257_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 258
    #[inline(always)]
    pub fn data258(&self) -> DATA258_R {
        DATA258_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 259
    #[inline(always)]
    pub fn data259(&self) -> DATA259_R {
        DATA259_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB64")
            .field("data256", &self.data256())
            .field("data257", &self.data257())
            .field("data258", &self.data258())
            .field("data259", &self.data259())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 256
    #[inline(always)]
    pub fn data256(&mut self) -> DATA256_W<'_, HUFFSYMB64rs> {
        DATA256_W::new(self, 0)
    }
    ///Bits 8:15 - Data 257
    #[inline(always)]
    pub fn data257(&mut self) -> DATA257_W<'_, HUFFSYMB64rs> {
        DATA257_W::new(self, 8)
    }
    ///Bits 16:23 - Data 258
    #[inline(always)]
    pub fn data258(&mut self) -> DATA258_W<'_, HUFFSYMB64rs> {
        DATA258_W::new(self, 16)
    }
    ///Bits 24:31 - Data 259
    #[inline(always)]
    pub fn data259(&mut self) -> DATA259_W<'_, HUFFSYMB64rs> {
        DATA259_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB64)*/
pub struct HUFFSYMB64rs;
impl crate::RegisterSpec for HUFFSYMB64rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb64::R`](R) reader structure
impl crate::Readable for HUFFSYMB64rs {}
///`write(|w| ..)` method takes [`huffsymb64::W`](W) writer structure
impl crate::Writable for HUFFSYMB64rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB64 to value 0
impl crate::Resettable for HUFFSYMB64rs {}
