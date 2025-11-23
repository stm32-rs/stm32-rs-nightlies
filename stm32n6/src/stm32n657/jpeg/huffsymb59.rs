///Register `HUFFSYMB59` reader
pub type R = crate::R<HUFFSYMB59rs>;
///Register `HUFFSYMB59` writer
pub type W = crate::W<HUFFSYMB59rs>;
///Field `DATA236` reader - Data 236
pub type DATA236_R = crate::FieldReader;
///Field `DATA236` writer - Data 236
pub type DATA236_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA237` reader - Data 237
pub type DATA237_R = crate::FieldReader;
///Field `DATA237` writer - Data 237
pub type DATA237_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA238` reader - Data 238
pub type DATA238_R = crate::FieldReader;
///Field `DATA238` writer - Data 238
pub type DATA238_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA239` reader - Data 239
pub type DATA239_R = crate::FieldReader;
///Field `DATA239` writer - Data 239
pub type DATA239_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 236
    #[inline(always)]
    pub fn data236(&self) -> DATA236_R {
        DATA236_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 237
    #[inline(always)]
    pub fn data237(&self) -> DATA237_R {
        DATA237_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 238
    #[inline(always)]
    pub fn data238(&self) -> DATA238_R {
        DATA238_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 239
    #[inline(always)]
    pub fn data239(&self) -> DATA239_R {
        DATA239_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB59")
            .field("data236", &self.data236())
            .field("data237", &self.data237())
            .field("data238", &self.data238())
            .field("data239", &self.data239())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 236
    #[inline(always)]
    pub fn data236(&mut self) -> DATA236_W<'_, HUFFSYMB59rs> {
        DATA236_W::new(self, 0)
    }
    ///Bits 8:15 - Data 237
    #[inline(always)]
    pub fn data237(&mut self) -> DATA237_W<'_, HUFFSYMB59rs> {
        DATA237_W::new(self, 8)
    }
    ///Bits 16:23 - Data 238
    #[inline(always)]
    pub fn data238(&mut self) -> DATA238_W<'_, HUFFSYMB59rs> {
        DATA238_W::new(self, 16)
    }
    ///Bits 24:31 - Data 239
    #[inline(always)]
    pub fn data239(&mut self) -> DATA239_W<'_, HUFFSYMB59rs> {
        DATA239_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb59::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb59::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#JPEG:HUFFSYMB59)*/
pub struct HUFFSYMB59rs;
impl crate::RegisterSpec for HUFFSYMB59rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb59::R`](R) reader structure
impl crate::Readable for HUFFSYMB59rs {}
///`write(|w| ..)` method takes [`huffsymb59::W`](W) writer structure
impl crate::Writable for HUFFSYMB59rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB59 to value 0
impl crate::Resettable for HUFFSYMB59rs {}
