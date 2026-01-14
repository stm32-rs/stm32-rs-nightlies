///Register `HUFFSYMB56` reader
pub type R = crate::R<HUFFSYMB56rs>;
///Register `HUFFSYMB56` writer
pub type W = crate::W<HUFFSYMB56rs>;
///Field `DATA224` reader - Data 224
pub type DATA224_R = crate::FieldReader;
///Field `DATA224` writer - Data 224
pub type DATA224_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA225` reader - Data 225
pub type DATA225_R = crate::FieldReader;
///Field `DATA225` writer - Data 225
pub type DATA225_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA226` reader - Data 226
pub type DATA226_R = crate::FieldReader;
///Field `DATA226` writer - Data 226
pub type DATA226_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DATA227` reader - Data 227
pub type DATA227_R = crate::FieldReader;
///Field `DATA227` writer - Data 227
pub type DATA227_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Data 224
    #[inline(always)]
    pub fn data224(&self) -> DATA224_R {
        DATA224_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Data 225
    #[inline(always)]
    pub fn data225(&self) -> DATA225_R {
        DATA225_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Data 226
    #[inline(always)]
    pub fn data226(&self) -> DATA226_R {
        DATA226_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data 227
    #[inline(always)]
    pub fn data227(&self) -> DATA227_R {
        DATA227_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUFFSYMB56")
            .field("data224", &self.data224())
            .field("data225", &self.data225())
            .field("data226", &self.data226())
            .field("data227", &self.data227())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Data 224
    #[inline(always)]
    pub fn data224(&mut self) -> DATA224_W<'_, HUFFSYMB56rs> {
        DATA224_W::new(self, 0)
    }
    ///Bits 8:15 - Data 225
    #[inline(always)]
    pub fn data225(&mut self) -> DATA225_W<'_, HUFFSYMB56rs> {
        DATA225_W::new(self, 8)
    }
    ///Bits 16:23 - Data 226
    #[inline(always)]
    pub fn data226(&mut self) -> DATA226_W<'_, HUFFSYMB56rs> {
        DATA226_W::new(self, 16)
    }
    ///Bits 24:31 - Data 227
    #[inline(always)]
    pub fn data227(&mut self) -> DATA227_W<'_, HUFFSYMB56rs> {
        DATA227_W::new(self, 24)
    }
}
/**JPEG Huffman symbol

You can [`read`](crate::Reg::read) this register and get [`huffsymb56::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huffsymb56::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#JPEG:HUFFSYMB56)*/
pub struct HUFFSYMB56rs;
impl crate::RegisterSpec for HUFFSYMB56rs {
    type Ux = u32;
}
///`read()` method returns [`huffsymb56::R`](R) reader structure
impl crate::Readable for HUFFSYMB56rs {}
///`write(|w| ..)` method takes [`huffsymb56::W`](W) writer structure
impl crate::Writable for HUFFSYMB56rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HUFFSYMB56 to value 0
impl crate::Resettable for HUFFSYMB56rs {}
